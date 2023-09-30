use actix_cors::Cors;
use actix_files::{Files, NamedFile};
use actix_web::{
    get,
    http::{
        header::{self, ContentType},
        Method, StatusCode,
    },
    middleware,
    web::{self, to, Data},
    App, Either, HttpRequest, HttpResponse, HttpServer, Responder, Result,
};
use serde::{Deserialize, Serialize};
use std::any::{Any, TypeId};
use std::{
    collections::HashMap,
    hash::Hash,
    sync::mpsc::{self, Receiver, Sender},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Mutex,
    },
};
use std::{io, mem, thread, vec};
use uuid::Uuid;
mod user_example;
use user_example::{ExampleResourceManager, ResourceManager};

fn main() {
    let (gc2server_tx, gc2server_rx) = mpsc::channel();

    thread::spawn(move || {
        gc_thread(gc2server_tx);
    });

    let _ = server_thread(gc2server_rx);
}

#[actix_web::main]
async fn server_thread(_gc2server_rx: Receiver<String>) -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    let reource_managers: Data<Mutex<HashMap<String, ResourceManagerState>>> =
        Data::new(Mutex::new(HashMap::new()));

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin("http://localhost:8080")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .app_data(reource_managers.clone())
            .service(index_html)
            .service(favicon_ico)
            .service(Files::new("/assets", "../../frontend/dist/assets"))
            .service(web::resource("/api/view/{viewName}").route(web::post().to(view)))
            .service(web::resource("/api/action").route(web::post().to(action)))
            .wrap(cors)
            .default_service(web::to(|| HttpResponse::Ok()))
    })
    .bind(("127.0.0.1", 8080))?
    .workers(2)
    .run()
    .await
}

async fn view(
    resource_managers: Data<Mutex<HashMap<String, ResourceManagerState>>>,
    path_params: web::Path<String>,
    body: web::Json<ViewRequestBody>,
) -> HttpResponse {
    log::info!(">> view()");
    match path_params.into_inner().as_str() {
        "example" => match body.0.viewmodel_id {
            None => {
                log::info!("call view without viewmodel_id");

                let resource_manager = ExampleResourceManager::new();
                let viewmodel_id = resource_manager.get_viewmodel_id();
                let response_body =
                    generate_base_response_body(&resource_manager, viewmodel_id.clone()).await;

                (*resource_managers.lock().unwrap()).insert(
                    viewmodel_id.clone(),
                    ResourceManagerState::Idle(
                        Box::new(resource_manager) as Box<dyn ResourceManager + Send>
                    ),
                );
                HttpResponse::Ok().json(response_body)
            }
            Some(viewmodel_id) => {
                log::info!("call view with viewmodel_id {}", viewmodel_id);
                match (*resource_managers.lock().unwrap()).get(&viewmodel_id) {
                    None => HttpResponse::NotFound().finish(),
                    Some(ResourceManagerState::Busy) => HttpResponse::TooManyRequests().finish(),
                    Some(ResourceManagerState::Idle(resource_manager)) => HttpResponse::Ok().json(
                        generate_base_response_body(
                            resource_manager.as_ref(),
                            viewmodel_id.clone(),
                        )
                        .await,
                    ),
                }
            }
        },
        _ => HttpResponse::NotFound().finish(),
    }
}

async fn generate_base_response_body(
    resource_manager: &dyn ResourceManager,
    viewmodel_id: String,
) -> BaseResponseBody {
    let widgets_data = resource_manager.get_widgets_data();
    BaseResponseBody {
        viewmodel_id,
        widgets_data,
    }
}

async fn action(
    resource_managers: Data<Mutex<HashMap<String, ResourceManagerState>>>,
    body: web::Json<ActionRequestBody>,
) -> HttpResponse {
    let viewmodel_id = body.0.viewmodel_id;
    log::info!("get viewmodel id {} from action request", &viewmodel_id);
    match (*resource_managers.lock().unwrap()).get_mut(&viewmodel_id) {
        None => HttpResponse::NotFound().finish(),
        Some(ResourceManagerState::Busy) => {
            HttpResponse::InternalServerError().body(format!("{:?} busy", viewmodel_id))
        }
        Some(registered_resource_manager_state) => {
            let mut operating_resource_manager_state = ResourceManagerState::Busy;
            mem::swap(
                registered_resource_manager_state,
                &mut operating_resource_manager_state,
            );
            let response_body = if let ResourceManagerState::Idle(resource_manager) =
                &mut operating_resource_manager_state
            {
                resource_manager.perform_action(body.0.widget_id, body.0.action_type, body.0.data);
                generate_base_response_body(resource_manager.as_ref(), viewmodel_id.clone()).await
            } else {
                panic!()
            };

            mem::swap(
                registered_resource_manager_state,
                &mut operating_resource_manager_state,
            );
            
            HttpResponse::Ok().json(response_body)
        }
    }
}

#[get("/index.html")]
async fn index_html() -> Result<impl Responder> {
    Ok(NamedFile::open("../../frontend/dist/index.html")?)
}

#[get("/favicon.ico")]
async fn favicon_ico() -> Result<impl Responder> {
    Ok(NamedFile::open("../../frontend/dist/favicon.ico")?)
}

fn gc_thread(_gc2server_tx: Sender<String>) {}

enum ResourceManagerState {
    Idle(Box<dyn ResourceManager + Send>),
    Busy,
}

// =========================================== Request ===========================================
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ViewRequestBody {
    #[serde(default)]
    viewmodel_id: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ActionRequestBody {
    viewmodel_id: String,
    widget_id: String,
    action_type: String,
    data: HashMap<String, String>,
}

// =========================================== Reponse ===========================================

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct BaseResponseBody {
    viewmodel_id: String,
    widgets_data: HashMap<String, HashMap<String, String>>,
}

// =========================================== test ===========================================
#[derive(Serialize, Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct WidgetParameters {
    text: Option<String>,
    visible: Option<bool>,
    select: Option<bool>,
}

#[test]
fn test_deserialize_hashmap() {
    let json = r#"{
        "wwwwwwww": {
            "text": "hello",
            "visible": true
        },
        "yyyyyyyy": {
            "text": "morning",
            "select": false
        }
    }"#;

    let lookup: HashMap<String, WidgetParameters> = serde_json::from_str(json).unwrap();
    println!("{:?}", lookup);
    println!("{:?}", lookup.get("wwwwwwww").map_or(None, |v| v.visible))
}

#[test]
fn test_deserialize_option_value() {
    let empty_json = r#"{     
    }"#;
    let lookup: ViewRequestBody = serde_json::from_str(empty_json).unwrap();
    println!("{:?}", lookup);

    let json_with_value = r#"{"viewmodelId":"abc"}"#;
    let lookup2: ViewRequestBody = serde_json::from_str(json_with_value).unwrap();
    println!("{:?}", lookup2);
}
