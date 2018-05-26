// Basic hello world example.

#![allow(unused)]
extern crate ggez;
#[macro_use]
extern crate lazy_static;



use ggez::conf;
use ggez::{Context, GameResult};
use ggez::event::{self, Button, MouseState, Keycode, Mod, Axis};
use ggez::graphics;
use std::env;
use std::path;

mod ecsystem;
mod game;

use ecsystem::gameobject::*;
use ecsystem::component::*;
use ecsystem::ECSystem;
use game::Game;
// Now our main function, which does three things:
//
// * First, create a new `ggez::conf::Conf`
// object which contains configuration info on things such
// as screen resolution and window title.
// * Second, create a `ggez::game::Game` object which will
// do the work of creating our MainState and running our game.
// * Then, just call `game.run()` which runs the `Game` mainloop.
pub fn main() {    
    let mut game = Game::new();
    game.init();
    //GAME_OBJECTS

}
