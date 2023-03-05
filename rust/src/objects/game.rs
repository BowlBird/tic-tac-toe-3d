use godot::engine::node::InternalMode;
use godot::prelude::*;

use crate::structures::{circular_vector::CircularVector, map_3d::Map3D};
use crate::objects::game::player::Player;
pub mod player;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct Game {
    board: Map3D<Player>,
    players: CircularVector<Gd<Player>>,
    #[base]
    base: Base<Node>,
}

impl Game {
    /// used to construct new Gd<Game> objects
    /// 
    /// this constructor is unregistered with Godot
    /// and should be manually set up in the tree
    /// after a reference is gotten.
    pub fn new(board_size: (i32,i32,i32), players: Vec<Gd<Player>>) -> Gd<Self> {
        let mut game = Gd::with_base(|base| {
            Game {
                board: Map3D::new(4, 4, 4).unwrap(),
                players: CircularVector::new(),
                base,
            }
        });

        //sets the variables after defaults are made.
        game.bind_mut().set_players(players);
        game.bind_mut().set_board_size(board_size);

        return game;
    }

    /// after init, set player count
    pub fn set_players(&mut self, players: Vec<Gd<Player>>) {

        //remove all current players
        while !self.players.is_empty() {
            let removed_player = self.players.remove().expect("Couldn't get Player!");
            self.remove_child(removed_player.upcast());
        }

        //next add the new players

        let mut cv = CircularVector::new();

        for player in players {
            self.add_child(player.share().upcast(), true, InternalMode::INTERNAL_MODE_DISABLED);
            cv.insert_after(player);
        }
        self.players = cv;
    }

    pub fn get_player_count(&self) -> i32 {
        self.players.size() as i32
    }

    /// if different board size is selected, it can be set here
    /// bool will return false if it failed.
    pub fn set_board_size(&mut self, board_size: (i32,i32,i32)) -> bool {
        let new_board: Map3D<Player> = match Map3D::new(board_size.0, board_size.1, board_size.2) {
            Ok(it) => it,
            Err(_) => return false,
        };
        self.board = new_board;
        return true;
    }

    pub fn set_current_player_by_id(&mut self, id: i32) -> bool {
        if (0..self.get_player_count()).contains(&id) {
            while self.get_current_player().bind().get_id() != id {
                self.players.rotate()
            }
            return true
        }
        return false
    }

    pub fn get_current_player(&self) -> &Gd<Player> {
        self.players.get().expect("Could not find player!")
    }

}

#[godot_api]
impl GodotExt for Game {
    fn ready(&mut self) {
        godot_print!("In ready for game!");
    }
}