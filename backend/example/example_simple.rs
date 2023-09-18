/**
 * 1. imple MVVM
 * 2. register itself
 */

struct ExampleView {}
impl ExampleView for ASView {
    fn get_component() -> Vec<Box<dyn Component>> {
        vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Cat"),
                    String::from("Dog"),
                    String::from("Others"),
                ],
                on_change: bind(update_animal),
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("Add value"),
                click: bind(add_value),
            }),
            Box::new(Text {
                width: 50,
                height: 10,
                value: bind(get_value),
            }),
        ]
    }
}

struct ExampleViewModel {
    cache_number: u32,
    model: ExampleModel,
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
    fn update_animal(&mut self, animal: String) {
        self.model.update_animal(animal)
    }
}

struct ExampleModel {
    animal: String,
}

impl ExampleModel {
    fn update_animal(&mut self, animal: String) {
        self.animal = animal
    }
}
