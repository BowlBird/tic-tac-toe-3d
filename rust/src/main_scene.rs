use godot::engine::node::InternalMode;
use godot::engine::packed_scene::*;
use godot::{prelude::*};

use crate::game::*;

//acts as base class for each file
//change from 'Node' to whatever component you are using.
#[derive(GodotClass)]
#[class(base=Node)]
pub struct Main {
    #[base]
    base: Base<Node>,
}

//add functions to be used here
#[godot_api]
impl Main {}

//for initalizing and frame by frame logic.
#[godot_api]
impl GodotExt for Main {

    //initalize
    fn init(base: Base<Node>) -> Self {

        godot_print!("Hello, World from Main!");
        Main {
            base,
        }
    }

    //after initalized
    fn ready(&mut self) {
        let game: Gd<Node> = (load("res://scenes/game.tscn") as Gd<PackedScene>)
                                        .instantiate(GenEditState::GEN_EDIT_STATE_DISABLED)
                                        .expect("Could not load res://scenes/game.tscn!");
        self.add_child(game.share(), false, InternalMode::INTERNAL_MODE_DISABLED);
        
        let mut binding = game.cast::<Game>();
        let mut game = binding.bind_mut();
        
        game.set_players(2)
    }

    //per frame method call
    fn process(&mut self, _delta:f64) {
        let input = Input::singleton();
    
    }
}