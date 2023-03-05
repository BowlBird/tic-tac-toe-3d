use godot::prelude::*;


static mut ID_COUNTER: i32 = 0;
//------------------------------------------------------


#[derive(Debug)]
#[derive(GodotClass)]
#[class(base=Node)]
pub struct Player {
    id: i32,

    #[base]
    base: Base<Node>,
}

/// unsafe counter implementations
fn get_counter() -> i32 {
    return unsafe {ID_COUNTER};
}

fn increment_counter() {
    unsafe {
        ID_COUNTER += 1;
    }
}

impl Player {

    /// creates new player object
    pub fn new() -> Gd<Self> {
        let mut player = Gd::with_base(|base| {
            Player {
                id: get_counter(),
                base,
            }
        });

        increment_counter();
        return player;
    }

    /// returns private id player
    pub fn get_id(&self) -> i32 {
        return self.id;
    }
}

#[godot_api]
impl GodotExt for Player {}