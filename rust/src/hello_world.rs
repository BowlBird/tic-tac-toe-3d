use godot::prelude::*;


//acts as base class for each file
//change from 'Node' to whatever component you are using.
#[derive(GodotClass)]
#[class(base=Node)]
pub struct HelloWorld {

    //add extra fields here

    #[base]
    base: Base<Node>,
}

//add functions to be used here
#[godot_api]
impl HelloWorld {
    #[func]
    pub fn example(&mut self) {

    }
}

//for initalizing and frame by frame logic.
#[godot_api]
impl GodotExt for HelloWorld {

    //initalize
    fn init(base: Base<Node>) -> Self {

        godot_print!("Hello, World!");
        
        HelloWorld {
            base,
        }
    }

    //after initalized
    fn ready(&mut self) {
        
    }

    //per frame method call
    fn process(&mut self, _delta:f64) {

    }
}