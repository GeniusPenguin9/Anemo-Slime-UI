use std::any::Any;
use std::collections::HashMap;
use uuid::Uuid;

pub struct ExampleResourceManager {
    viewmodel: ExampleViewModel,
    view: ExampleView,
}

impl ExampleResourceManager {
    pub fn new() -> Self {
        let click_fn: Box<dyn (Fn(&mut dyn ASViewModel) -> Result<(), String>) + Send> =
            Box::new(ExampleViewModel::add_value);
        let user_widgets: Vec<Box<dyn ASWidget>> = vec![
            Box::new(TextBox::new()),
            Box::new(Button::new_with_click(click_fn)),
        ];

        ExampleResourceManager {
            viewmodel: ExampleViewModel::new(),
            view: ExampleView::new(user_widgets),
        }
    }
}

impl ResourceManager for ExampleResourceManager {
    fn get_viewmodel_id(&self) -> String {
        self.viewmodel.viewmodel_id.clone()
    }

    fn get_widgets_data(&self) -> HashMap<String, HashMap<String, String>> {
        let mut widgets = HashMap::new();
        for (widget_id, widget) in self.view.widgets.iter() {
            widgets.insert(widget_id.clone(), widget.get_widget_parameters());
        }
        widgets
    }

    fn perform_action(
        &mut self,
        widget_id: String,
        action_type: String,
        _data: HashMap<String, String>,
    ) {
        log::info!(
            ">> perform_action with widget id {}, action type = {}, data + {:?}",
            &widget_id,
            &action_type,
            &_data
        );
        match self.view.widgets.get(&widget_id) {
            None => (),
            Some(box_widget) => {
                if let Some(binding_fn) = box_widget.get_binding_function(action_type) {
                    binding_fn(&mut self.viewmodel);
                }
            }
        }
    }
}

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

struct ExampleViewModel {
    viewmodel_id: String,
    custom_number: u32,
}

impl ExampleViewModel {
    fn new() -> Self {
        ExampleViewModel {
            viewmodel_id: Uuid::new_v4().to_string(),
            custom_number: 0,
        }
    }

    fn add_value(as_viewmodel: &mut dyn ASViewModel) -> Result<(), String> {
        log::info!(">> ExampleViewModel#add_value");
        if let Some(s) = as_viewmodel.as_mut_any().downcast_mut::<ExampleViewModel>() {
            log::info!("current custom_number = {}", s.custom_number);
            s.custom_number += 1;
            log::info!(
                "<< ExampleViewModel#add_value, custom_number = {}",
                s.custom_number
            )
        }
        Ok(())
    }
}
impl ASViewModel for ExampleViewModel {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}

trait ASViewModel: Send {
    fn as_any(&self) -> &dyn Any;
    fn as_mut_any(&mut self) -> &mut dyn Any;
}

struct ExampleView {
    widgets: HashMap<String, Box<dyn ASWidget>>,
}

impl ExampleView {
    fn new(user_widgets: Vec<Box<dyn ASWidget>>) -> Self {
        let mut widgets = HashMap::new() as HashMap<String, Box<dyn ASWidget>>;
        user_widgets.into_iter().for_each(|i| {
            let _ = widgets.insert(i.get_widget_id(), i);
        });
        ExampleView { widgets }
    }
}

struct Button {
    widget_id: String,
    width: u32,
    height: u32,
    label: String,
    click: Option<Box<dyn (Fn(&mut dyn ASViewModel) -> Result<(), String>) + Send>>,
}

impl Button {
    #[allow(dead_code)]
    fn new() -> Self {
        Button {
            widget_id: Uuid::new_v4().to_string(),
            width: 50,
            height: 50,
            label: String::from("Button"),
            click: None,
        }
    }

    fn new_with_click(
        click_fn: Box<dyn (Fn(&mut dyn ASViewModel) -> Result<(), String>) + Send>,
    ) -> Self {
        Button {
            widget_id: Uuid::new_v4().to_string(),
            width: 50,
            height: 50,
            label: String::from("Button"),
            click: Some(click_fn),
        }
    }
}

impl ASWidget for Button {
    fn get_binding_function(
        &self,
        action_type: String,
    ) -> Option<&Box<dyn (Fn(&mut dyn ASViewModel) -> Result<(), String>) + Send>> {
        match action_type.as_str() {
            "click" => self.click.as_ref(),
            _ => None,
        }
    }
    fn get_widget_id(&self) -> String {
        self.widget_id.clone()
    }
    fn get_widget_parameters(&self) -> HashMap<String, String> {
        HashMap::from([
            ("width".to_string(), self.width.to_string()),
            ("height".to_string(), self.height.to_string()),
            ("label".to_string(), self.label.clone()),
        ])
    }
}

struct TextBox {
    widget_id: String,
    content: String,    // TODO: how to bind from viewmodel to model, and display?
}

impl TextBox {
    fn new() -> Self {
        TextBox {
            widget_id: Uuid::new_v4().to_string(),
            content: String::new(),
        }
    }
}

impl ASWidget for TextBox {
    fn get_binding_function(
        &self,
        _action_type: String,
    ) -> Option<&Box<dyn (Fn(&mut dyn ASViewModel) -> Result<(), String>) + Send>> {
        None
    }

    fn get_widget_id(&self) -> String {
        self.widget_id.clone()
    }

    fn get_widget_parameters(&self) -> HashMap<String, String> {
        HashMap::from([("content".to_string(), self.content.clone())])
    }
}

trait ASWidget: Send {
    fn get_binding_function(
        &self,
        action_type: String,
    ) -> Option<&Box<dyn (Fn(&mut dyn ASViewModel) -> Result<(), String>) + Send>>;
    fn get_widget_id(&self) -> String;
    fn get_widget_parameters(&self) -> HashMap<String, String>;
}
