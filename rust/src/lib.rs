use godot::prelude::*;

//put references to other rust scripts here

//objects
mod board;
mod game;
mod circular_vector;


//scenes
mod main_scene;
mod splash_screen;

//name of struct doesn't really matter, usually it is the name of the project.
struct Parent;

//starts rust integration
#[gdextension]
unsafe impl ExtensionLibrary for Parent {}
