
use ggez::Context;
use ggez::graphics;
use ggez::graphics::Point2;
use ggez;
use ggez::event::{self, Button, MouseState, Keycode, Mod, Axis};
use ecsystem::component::Component;
use ecsystem::component::IComponent;
use ecsystem::gameobject::GameObject;
use ecsystem::ECSystem;
use std::sync::Mutex;
use ecsystem::Input;

use ggez::GameResult;


pub struct Movement {
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
}

lazy_static! {
    static ref MOVEMENT: Mutex<Movement> = Mutex::new(Movement {left: false, right: false, up: false, down: false});
    static ref SPEED: Mutex<f32> = Mutex::new(10.0);
}

pub fn player_controller_start(component: &mut Component, ecs: &mut ECSystem) {
    match component {
        &mut Component::PlayerController{..} => {
        }
        _ => (),
    }
}

pub fn player_controller_update(component: &mut Component, ecs: &mut ECSystem, go: &mut GameObject) {

    

    match component {
        &mut Component::PlayerController{..} => {
            
          
            if ecs.input.keycode_down == Some(ggez::event::Keycode::Left) {
                MOVEMENT.lock().unwrap().left = true;
                ecs.input.keycode_down = None;
            }

            if ecs.input.keycode_down == Some(ggez::event::Keycode::Right) {
                MOVEMENT.lock().unwrap().right = true; 
                ecs.input.keycode_down = None;
            }

            if ecs.input.keycode_down == Some(ggez::event::Keycode::Up) {
                MOVEMENT.lock().unwrap().up = true;
                ecs.input.keycode_down = None;
            }

            if ecs.input.keycode_down == Some(ggez::event::Keycode::Down) {
                MOVEMENT.lock().unwrap().down = true;    
                ecs.input.keycode_down = None;
            }

            if ecs.input.keycode_up == Some(ggez::event::Keycode::Up) {
                MOVEMENT.lock().unwrap().up = false; 
                ecs.input.keycode_up = None;
            }
             
            if ecs.input.keycode_up == Some(ggez::event::Keycode::Down) {
                MOVEMENT.lock().unwrap().down = false;   
                ecs.input.keycode_up = None;
            }
             
            if ecs.input.keycode_up == Some(ggez::event::Keycode::Left) {
                MOVEMENT.lock().unwrap().left = false;
                ecs.input.keycode_up = None;
            }
             
            if ecs.input.keycode_up == Some(ggez::event::Keycode::Right) {
                MOVEMENT.lock().unwrap().right = false;  
                ecs.input.keycode_up = None;
            }
         


            let mut left: bool = MOVEMENT.lock().unwrap().left;
            let mut right: bool = MOVEMENT.lock().unwrap().right;
            let mut up: bool = MOVEMENT.lock().unwrap().up;
            let mut down: bool = MOVEMENT.lock().unwrap().down;

            match (up, right, down, left) {
                ( true, false, false, false) => (                                    go.pos.y -= *SPEED.lock().unwrap()), //self.translate(0.0, -vel),
                ( true,  true, false, false) => {go.pos.x += *SPEED.lock().unwrap(); go.pos.y -= *SPEED.lock().unwrap()}, //self.translate(vel*0.707, -vel*0.707),
                (false,  true, false, false) => (go.pos.x += *SPEED.lock().unwrap()                                    ), //self.translate(vel, 0.0),
                (false,  true,  true, false) => {go.pos.x += *SPEED.lock().unwrap(); go.pos.y += *SPEED.lock().unwrap()}, //self.translate(vel*0.707, vel*0.707),
                (false, false,  true, false) => (                                    go.pos.y += *SPEED.lock().unwrap()), //self.translate(0.0, vel),
                (false, false,  true,  true) => {go.pos.x -= *SPEED.lock().unwrap(); go.pos.y += *SPEED.lock().unwrap()}, //self.translate(-vel*0.707, vel*0.707),
                (false, false, false,  true) => (go.pos.x -= *SPEED.lock().unwrap()                                    ), //self.translate(-vel, 0.0),
                ( true, false, false,  true) => {go.pos.x -= *SPEED.lock().unwrap(); go.pos.y -= *SPEED.lock().unwrap()}, //self.translate(-vel*0.707, -vel*0.707),
                _ => (),
            }
    
            // Consume the input.
            ecs.input = Input {keycode_up: None, keymod_up: None, keycode_down: None, keymod_down: None};
        }
        _ => (),
    }
    
   
}

pub fn player_controller_render(component: &Component, ctx: &mut Context) -> GameResult<()> {
    match component {
        _ => Ok(()),
    }
}