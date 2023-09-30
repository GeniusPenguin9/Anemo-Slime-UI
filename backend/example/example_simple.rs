/**
 * 1. imple MVVM
 * 2. register itself
 */

struct ExampleView {}
impl ExampleView for ASView {
    fn get_component() -> Vec<Box<dyn Component>> {
        vec![
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("Add value"),
                click: bind(ExampleViewModel::add_value),
            }),
            Box::new(Text {
                width: 50,
                height: 10,
                value: bind(ExampleViewModel::get_value),
            }),
        ]
    }
}

struct ExampleViewModel {
    cache_number: u32,
}

impl ExampleViewModel {
    fn new() -> Self {
        ExampleViewModel { cache_number: 0 }
    }
    fn add_value(&mut self) {
        self.cache_number += 1;
    }
    fn get_value(&self) -> u32 {
        self.cache_number
    }
}
