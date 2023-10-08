use std::collections::HashMap;
use uuid::Uuid;

// =================================== ResourceManager ===================================
pub trait ResourceManager {
    fn get_viewmodel_id(&self) -> String;
    fn get_widgets_data(&self) -> HashMap<String, HashMap<String, String>>;
    fn perform_action(
        &mut self,
        widget_id: String,
        action_type: String,
        _data: HashMap<String, String>,
    );
}

// =================================== ViewModel ===================================
pub trait ASViewModel {}

// =================================== View ===================================
pub trait ASView {}

// =================================== Widget ===================================
pub struct Button<TViewModel: ASViewModel> {
    widget_id: String,
    width: u32,
    height: u32,
    label: String,
    click: Option<Box<dyn (Fn(&mut TViewModel) -> Result<(), String>) + Send>>,
}

impl<TViewModel: ASViewModel> Button<TViewModel> {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Button {
            widget_id: Uuid::new_v4().to_string(),
            width: 50,
            height: 50,
            label: String::from("Button"),
            click: None,
        }
    }

    pub fn click(
        mut self,
        click_fn: Box<dyn (Fn(&mut TViewModel) -> Result<(), String>) + Send>,
    ) -> Self {
        self.click = Some(click_fn);
        self
    }
}

impl<TViewModel: ASViewModel> ASWidget<TViewModel> for Button<TViewModel> {
    fn perform_action(
        &self,
        action_type: String,
        _data: HashMap<String, String>,
        viewmodel: &mut TViewModel,
    ) {
        log::info!(">> Button#perform_action");
        match action_type.as_str() {
            "click" => {
                if let Some(f) = self.click.as_ref() {
                    let _ = f(viewmodel); // TODO: should return Result<>
                }
            }
            _ => (),
        }
    }

    fn get_widget_id(&self) -> String {
        self.widget_id.clone()
    }

    fn get_widget_parameters(&self, viewmodel: &TViewModel) -> HashMap<String, String> {
        HashMap::from([
            ("width".to_string(), self.width.to_string()),
            ("height".to_string(), self.height.to_string()),
            ("label".to_string(), self.label.clone()),
        ])
    }
}

pub struct TextBox<TViewModel: ASViewModel> {
    widget_id: String,
    content: ASString<TViewModel>,
}

pub enum ASString<TViewModel: ASViewModel> {
    Default,
    StaticString(String),
    // if viewmodel change, it should affect view
    ViewModel2View(Box<dyn (Fn(&TViewModel) -> Result<String, String>) + Send>),
    // if view change, it should affect view model.
    View2ViewModel(),
}

impl<TViewModel: ASViewModel> TextBox<TViewModel> {
    pub fn new() -> Self {
        TextBox {
            widget_id: Uuid::new_v4().to_string(),
            content: ASString::Default,
        }
    }

    pub fn content(mut self, user_content: ASString<TViewModel>) -> Self {
        self.content = user_content;
        self
    }
}

impl<TViewModel: ASViewModel> ASWidget<TViewModel> for TextBox<TViewModel> {
    fn get_widget_id(&self) -> String {
        self.widget_id.clone()
    }

    fn get_widget_parameters(&self, viewmodel: &TViewModel) -> HashMap<String, String> {
        match &self.content {
            ASString::Default => HashMap::from([("content".to_string(), "".to_string())]),
            ASString::StaticString(s) => HashMap::from([("content".to_string(), s.clone())]),
            ASString::ViewModel2View(f) => match f(viewmodel) {
                Ok(res) => HashMap::from([("content".to_string(), res)]),
                Err(err) => HashMap::from([("content".to_string(), format!("Error : {err}"))]),
            },
            _ => HashMap::from([("content".to_string(), "unknown".to_string())]),
        }
    }
}

pub trait ASWidget<TViewModel: ASViewModel>: Send {
    fn perform_action(
        &self,
        action_type: String,
        _data: HashMap<String, String>,
        viewmodel: &mut TViewModel,
    ) {
        log::info!(">> ASWidget#perform_action")
    }

    fn get_widget_id(&self) -> String;

    fn get_widget_parameters(&self, viewmodel: &TViewModel) -> HashMap<String, String>;
}
