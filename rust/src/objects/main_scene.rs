use godot::engine::node::InternalMode;
use godot::{prelude::*};

use crate::objects::game::Game;

use super::game::player::Player;

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
        let players = vec!(
            Player::new(), Player::new(),
        );
        let board_size = (4,4,4);

        let mut game = Game::new(board_size, players);
        self.add_child(game.upcast(), true, InternalMode::INTERNAL_MODE_DISABLED);
    }

    //per frame method call
    fn process(&mut self, _delta:f64) {
        let input = Input::singleton();
    
    }
}