use std::{collections::HashMap, fmt::Display};
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
    click: ASType<TViewModel, IgnoreValue>,
}

impl<TViewModel: ASViewModel> Button<TViewModel> {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Button {
            widget_id: Uuid::new_v4().to_string(),
            width: 50,
            height: 50,
            label: String::from("Button"),
            click: ASType::Default,
        }
    }

    pub fn click(
        mut self,
        click_fn: Box<
            dyn (Fn(&mut TViewModel, HashMap<String, String>) -> Result<(), String>) + Send,
        >,
    ) -> Self {
        self.click = ASType::View2ViewModel(click_fn);
        self
    }
}

impl<TViewModel: ASViewModel> ASWidget<TViewModel> for Button<TViewModel> {
    fn perform_action(
        &self,
        action_type: String,
        data: HashMap<String, String>,
        viewmodel: &mut TViewModel,
    ) {
        log::info!(">> Button#perform_action");
        match action_type.as_str() {
            "click" => {
                if let ASType::View2ViewModel(click_fn) = &self.click {
                    let _ = click_fn(viewmodel, data); // TODO: should return Result<>
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

pub struct TextBox<TViewModel: ASViewModel, TValue: Display + Send> {
    widget_id: String,
    content: ASType<TViewModel, TValue>,
}

impl<TViewModel: ASViewModel, TValue: Display + Send> TextBox<TViewModel, TValue> {
    pub fn new() -> Self {
        TextBox {
            widget_id: Uuid::new_v4().to_string(),
            content: ASType::Default,
        }
    }

    pub fn content(mut self, user_content: ASType<TViewModel, TValue>) -> Self {
        self.content = user_content;
        self
    }
}

impl<TViewModel: ASViewModel, TValue: Display + Send> ASWidget<TViewModel>
    for TextBox<TViewModel, TValue>
{
    fn get_widget_id(&self) -> String {
        self.widget_id.clone()
    }

    fn get_widget_parameters(&self, viewmodel: &TViewModel) -> HashMap<String, String> {
        match &self.content {
            ASType::Default => HashMap::from([("content".to_string(), "".to_string())]),
            ASType::Static(s) => HashMap::from([("content".to_string(), s.to_string())]),
            ASType::ViewModel2View(f) => match f(viewmodel) {
                Ok(res) => HashMap::from([("content".to_string(), res.to_string())]),
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

pub enum ASType<TViewModel: ASViewModel, TValue: Display + Send> {
    Default,
    Static(TValue),
    // if viewmodel change, it should affect view
    ViewModel2View(Box<dyn (Fn(&TViewModel) -> Result<TValue, String>) + Send>),
    // if view change, it should affect view model.
    View2ViewModel(
        Box<dyn (Fn(&mut TViewModel, HashMap<String, String>) -> Result<(), String>) + Send>,
    ),
}

pub struct IgnoreValue;
impl Display for IgnoreValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
unsafe impl Send for IgnoreValue {}
