use super::mvvm_core::{ASView, ASViewModel, ASWidget, Button, ResourceManager, TextBox};
use std::collections::HashMap;
use uuid::Uuid;

pub struct ExampleResourceManager {
    viewmodel: ExampleViewModel,
    view: ExampleView,
}

impl ExampleResourceManager {
    pub fn new() -> Self {
        let click_fn: Box<dyn (Fn(&mut ExampleViewModel) -> Result<(), String>) + Send> =
            Box::new(ExampleViewModel::add_value);
        let user_widgets: Vec<Box<dyn ASWidget<ExampleViewModel>>> = vec![
            Box::new(TextBox::new()),
            Box::new(Button::<ExampleViewModel>::new_with_click(click_fn)),
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
                box_widget.perform_action(action_type, _data, &mut self.viewmodel);
            }
        }
    }
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

    fn add_value(&mut self) -> Result<(), String> {
        log::info!(">> ExampleViewModel#add_value");

        log::info!("current custom_number = {}", self.custom_number);
        self.custom_number += 1;
        log::info!(
            "<< ExampleViewModel#add_value, custom_number = {}",
            self.custom_number
        );

        Ok(())
    }
}
impl ASViewModel for ExampleViewModel {}

struct ExampleView {
    widgets: HashMap<String, Box<dyn ASWidget<ExampleViewModel>>>,
}

impl ExampleView {
    fn new(user_widgets: Vec<Box<dyn ASWidget<ExampleViewModel>>>) -> Self {
        let mut widgets = HashMap::new() as HashMap<String, Box<dyn ASWidget<ExampleViewModel>>>;
        user_widgets.into_iter().for_each(|i| {
            let _ = widgets.insert(i.get_widget_id(), i);
        });
        ExampleView { widgets }
    }
}

impl ASView for ExampleView {}
