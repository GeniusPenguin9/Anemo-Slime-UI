/**
 * 1. imple MVVM
 * 2. register itself
 */

struct ExampleView {}
impl ExampleView for ASView {
    fn get_component() -> Vec<Box<dyn Component>> {
        vec![
           Box::new(Table {
               value: bind(get_animals)
               row_view_template: HashMap<String, Box<dyn Component>> {
                    "Animal Name": Label { 
                        text: bind(get_name)
                    },
                    "Age": Label {
                        text: bind(get_age)
                    }
                    "Actions": Button {
                        click: bind(delete)
                    }
               },
               column_headers: {
                    "Animal Name": Label {
                        text: "Animal Name"
                    },
                    "Age": Label {
                        text: "Animal Name"
                    }
               }
           }),
          
        ]
    }
}

// ============================== View Model ==================================
struct ExampleViewModel {
    animals : Vec<AnimalViewModel>
}
impl ExampleViewModel{
    fn get_animals(&self) -> Vec<AnimalViewModel> {
        self.animals
    }
}

struct AnimalViewModel {
    parent: &ExampleViewModel,
    animalName: String,
    age: u32,
    sex: SexEnum,
}

impl AnimalViewModel {
    fn get_name() -> String{
        
    }

    fn get_age(&self) -> String {
        format!("{} years", self.age)
    }

    fn delete(&self)  {
        self.parent.animals.remove(self);
        self.parent.notify_change(ExampleViewModel::get_animals)
    }
}
