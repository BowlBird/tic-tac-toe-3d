use godot::prelude::*;

use crate::{circular_vector::CircularVector, board::Board};

#[derive(Debug)]
pub struct Player {
    pub id: i32,
}
// -----------------------------------------------------
#[derive(GodotClass)]
#[class(base=Node)]
pub struct Game {
    board: Board<Player>,
    players: CircularVector<Player>,
    
    node: Base<Node>,
}

impl Game {
    /// after init, set player count
    pub fn set_players(&mut self, num: i32) {
        for i in 0..num {
            self.players.insert_after(Player {id: i});
            self.players.rotate();
        }
        self.players.rotate(); //go back to index 0
    }

    /// if different board size is selected, it can be set here
    /// bool will return false if it failed.
    pub fn set_board_size(&mut self, board_size: (i32,i32,i32)) -> bool {
        let new_board: Board<Player> = match Board::new(board_size.0, board_size.1, board_size.2) {
            Ok(it) => it,
            Err(_) => return false,
        };
        self.board = new_board;
        return true;
    }
}

#[godot_api]
impl GodotExt for Game {
    fn init(base: Base<Node>) -> Self {

        godot_print!("Hello World from game!");

        Game {
            //should always work
            board: Board::new(4,4,4).unwrap(),
            players: CircularVector::new(),
            node: base,
        }
    }
}