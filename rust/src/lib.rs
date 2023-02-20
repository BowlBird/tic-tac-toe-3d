use godot::prelude::*;

//put references to other rust scripts here
mod board;
mod main_scene;
mod splash_screen;

//name of struct doesn't really matter, usually it is the name of the project.
struct Parent;

//starts rust integration
#[gdextension]
unsafe impl ExtensionLibrary for Parent {}
