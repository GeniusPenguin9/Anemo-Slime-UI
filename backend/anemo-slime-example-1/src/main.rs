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

fn main() {
    let (gc2server_tx, gc2server_rx) = mpsc::channel();

    thread::spawn(move || {
        gc_thread(gc2server_tx);
    });

    server_thread(gc2server_rx);
}

#[actix_web::main]
async fn server_thread(gc2server_rx: Receiver<String>) -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    let viewmodel_map: Data<Mutex<HashMap<String, ViewmodelState>>> =
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
            .app_data(viewmodel_map.clone())
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
    viewmodel_map: Data<Mutex<HashMap<String, ViewmodelState>>>,
    path_params: web::Path<String>,
    body: web::Json<ViewRequestBody>,
) -> HttpResponse {
    log::info!(">> view()");
    match path_params.into_inner().as_str() {
        "example" => match body.0.viewmodel_id {
            None => {
                log::info!("call view without viewmodel_id");
                let viewmodel = ExampleViewModel::new();
                let viewmodel_id = viewmodel.viewmodel_id.clone();
                let viewmodel_clone = viewmodel.clone();

                (*viewmodel_map.lock().unwrap())
                    .insert(viewmodel_id.clone(), ViewmodelState::Idle(viewmodel));

                HttpResponse::Ok().json(viewmodel_clone)
            }
            Some(viewmodel_id) => {
                log::info!("call view with viewmodel_id {}", viewmodel_id);
                match (*viewmodel_map.lock().unwrap()).get(&viewmodel_id) {
                    None => HttpResponse::NotFound().finish(),
                    Some(_) => HttpResponse::Ok()
                        .content_type(ContentType::plaintext())
                        .body(format!("viewmodel_id = {}", viewmodel_id)),
                }
            }
        },
        _ => HttpResponse::NotFound().finish(),
    }
}

async fn action(
    viewmodel_map: Data<Mutex<HashMap<String, ViewmodelState>>>,
    body: web::Json<ActionRequestBody>,
) -> HttpResponse {
    let viewmodel_id = body.0.viewmodel_id;
    log::info!("get {} from action request", &viewmodel_id);
    match (*viewmodel_map.lock().unwrap()).get_mut(&viewmodel_id) {
        None => HttpResponse::NotFound().finish(),
        Some(ViewmodelState::Busy) => {
            HttpResponse::InternalServerError().body(format!("{:?} busy", viewmodel_id))
        }
        Some(registered_viewmodel_state) => {
            let mut operating_viewmodel_state = ViewmodelState::Busy;
            mem::swap(registered_viewmodel_state, &mut operating_viewmodel_state);

            let return_value: String =
                if let ViewmodelState::Idle(viewmodel) = &mut operating_viewmodel_state {
                    viewmodel
                        .widgets_data
                        .get("TextBox1")
                        .map_or("0".to_string(), |v| {
                            let mut str = v.text.as_ref().unwrap().clone();
                            str.push('1');
                            str
                        })
                } else {
                    "0".to_string()
                };
            let as_response = BaseResponseBody {
                viewmodel_id,
                widgets_data: HashMap::from([(
                    "TextBox1".to_string(),
                    WidgetParameters::new_with_text(return_value),
                )]),
            };

            mem::swap(registered_viewmodel_state, &mut operating_viewmodel_state);
            HttpResponse::Ok().json(as_response)
        }
    }
}

#[get("/")]
async fn index_html() -> Result<impl Responder> {
    Ok(NamedFile::open("../../frontend/dist/index.html")?)
}

#[get("/favicon.ico")]
async fn favicon_ico() -> Result<impl Responder> {
    Ok(NamedFile::open("../../frontend/dist/favicon.ico")?)
}

fn gc_thread(gc2server_tx: Sender<String>) {}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct ExampleViewModel {
    viewmodel_id: String,
    widgets_data: HashMap<String, WidgetParameters>,
}

impl ExampleViewModel {
    fn new() -> Self {
        let mut widgets_data = HashMap::new();
        widgets_data.insert("AddButton1".to_string(), WidgetParameters::new());
        widgets_data.insert(
            "TextBox1".to_string(),
            WidgetParameters::new_with_text("0".to_string()),
        );
        ExampleViewModel {
            viewmodel_id: Self::uuid_string(),
            widgets_data,
        }
    }

    fn uuid_string() -> String {
        Uuid::new_v4().to_string()
    }
}

#[derive(Debug)]
enum ViewmodelState {
    Idle(ExampleViewModel),
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
    widgets_data: HashMap<String, WidgetParameters>,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct WidgetParameters {
    text: Option<String>,
    visible: Option<bool>,
    select: Option<bool>,
}

impl WidgetParameters {
    fn new() -> Self {
        WidgetParameters {
            text: None,
            visible: None,
            select: None,
        }
    }
    fn new_with_text(text: String) -> Self {
        WidgetParameters {
            text: Some(text),
            visible: None,
            select: None,
        }
    }
    fn new_with_select(select: bool) -> Self {
        WidgetParameters {
            text: None,
            visible: None,
            select: Some(select),
        }
    }
}

// =========================================== test ===========================================
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
