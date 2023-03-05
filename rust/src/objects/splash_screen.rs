use godot::{prelude::*, engine::{CenterContainer, AnimationPlayer}};

/// base definition for Splash Screen Object
#[derive(GodotClass)]
#[class(base=CenterContainer)]
pub struct SplashScreen {

    //add extra fields here

    #[base]
    base: Base<CenterContainer>,
}

/// function implementations
#[godot_api]
impl SplashScreen {
    
    /// signal listener that changes scene to main scene after animation for splash screen is done.
    #[func]
    fn on_animation_finished(&self) {
        match self.get_tree() {
            Some(mut it) => {
                it.change_scene_to_file("res://scenes/main.tscn".into());
            },
            None => {}
        }
        
    }
}

//for initalizing and frame by frame logic.
#[godot_api]
impl GodotExt for SplashScreen {

    //initalize
    fn init(base: Base<CenterContainer>) -> Self {

        godot_print!("Hello, World from Splash Screen!");
        
        SplashScreen {
            base,
        }
    }

    //after initalized
    fn ready(&mut self) {

        // connects animation player 'animation_finished' signal with the function 'on_animation_finished' defined above.
        let mut animation_player = self.base.get_node_as::<AnimationPlayer>("BoxContainer/AnimationPlayer");

        animation_player.connect(
            "animation_finished".into(),
            Callable::from_object_method(self.base.share(), "on_animation_finished"),
            0
        );
    }

    //per frame method call
    fn process(&mut self, _delta:f64) {

    }
}