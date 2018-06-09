
use ggez::Context;
use ggez::graphics;
use ggez::graphics::Point2;
use ggez;
use ggez::event::{self, Button, MouseState, Keycode, Mod, Axis};
use ecsystem::component::Component;
use ecsystem::component::IComponent;
use ecsystem::gameobject::GameObject;
use ecsystem::ECSystem;
use ecsystem::*;
use std::sync::Mutex;
use ecsystem::Input;
use std::borrow::Borrow;
use ggez::GameResult;
use self::blackboard::*;

pub struct Controls {
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub shoot: bool,
}

lazy_static! {
    static ref CONTROLS: Mutex<Controls> = Mutex::new(Controls {left: false, right: false, up: false, down: false, shoot: false});
    static ref SPEED: Mutex<f32> = Mutex::new(10.0);
}

pub fn player_controller_start(component: &mut Component, ecs: &mut ECSystem) {
    match component {
        &mut Component::PlayerController{..} => {
        }
        _ => (),
    }
}

pub fn player_controller_update(component: &mut Component, ctx: &mut Context, ecs: &mut ECSystem, go: &mut GameObject) {

    
    match component {
        &mut Component::PlayerController{..} => {
            

            // Keycode DOWN
            if ecs.input.keycode_down == Some(ggez::event::Keycode::Space) {
                CONTROLS.lock().unwrap().shoot = true;
                //ecs.input.keycode_down = None;
            }

            if ecs.input.keycode_down == Some(ggez::event::Keycode::Left) {
                CONTROLS.lock().unwrap().left = true;
                CONTROLS.lock().unwrap().right = false;
                ecs.input.keycode_down = None;
            }
            else if ecs.input.keycode_down == Some(ggez::event::Keycode::Right) {
                CONTROLS.lock().unwrap().right = true; 
                CONTROLS.lock().unwrap().left = false;
                ecs.input.keycode_down = None;
            }

            if ecs.input.keycode_down == Some(ggez::event::Keycode::Up) {
                CONTROLS.lock().unwrap().up = true;
                CONTROLS.lock().unwrap().down = false;
                ecs.input.keycode_down = None;
            }
            else if ecs.input.keycode_down == Some(ggez::event::Keycode::Down) {
                CONTROLS.lock().unwrap().down = true;    
                CONTROLS.lock().unwrap().up = false;    
                ecs.input.keycode_down = None;
            }   

            // Keycode UP
            if ecs.input.keycode_up == Some(ggez::event::Keycode::Space) {
                CONTROLS.lock().unwrap().shoot = false;
                //ecs.input.keycode_up = None;
            }


            if ecs.input.keycode_up == Some(ggez::event::Keycode::Up) {
                CONTROLS.lock().unwrap().up = false; 
                ecs.input.keycode_up = None;
            }
            else if ecs.input.keycode_up == Some(ggez::event::Keycode::Down) {
                CONTROLS.lock().unwrap().down = false;   
                ecs.input.keycode_up = None;
            }             

            if ecs.input.keycode_up == Some(ggez::event::Keycode::Left) {
                CONTROLS.lock().unwrap().left = false;
                ecs.input.keycode_up = None;
            }
            else if ecs.input.keycode_up == Some(ggez::event::Keycode::Right) {
                CONTROLS.lock().unwrap().right = false;  
                ecs.input.keycode_up = None;
            }

            ecs.blackboard.scribble(go.tag().clone(), BlackboardObject::Doodle{object: Box::new(32 as u32)});
          
          
            let mut left: bool = CONTROLS.lock().unwrap().left;
            let mut right: bool = CONTROLS.lock().unwrap().right;
            let mut up: bool = CONTROLS.lock().unwrap().up;
            let mut down: bool = CONTROLS.lock().unwrap().down;
            let mut shoot: bool = CONTROLS.lock().unwrap().shoot;

            if shoot {    
                // Deadlock as soon as access game objects.
                //GAME_OBJECTS.lock().unwrap();
                println!("spawn bullet");
                CONTROLS.lock().unwrap().shoot = false;

                let mut bullet = GameObject::new("bullet".to_string());
                bullet.add_component(Box::new(Component::Renderable {
                    sprite: graphics::Image::new(ctx, "/texture/background.png").unwrap(),
                }));
                
                ecs.add_game_object(bullet);
            }
            

            match (up, right, down, left) {
                ( true, false, false, false) => {                                    go.pos.y -= *SPEED.lock().unwrap()}, //self.translate(0.0, -vel),
                ( true,  true, false, false) => {go.pos.x += *SPEED.lock().unwrap(); go.pos.y -= *SPEED.lock().unwrap()}, //self.translate(vel*0.707, -vel*0.707),
                (false,  true, false, false) => {go.pos.x += *SPEED.lock().unwrap()                                    }, //self.translate(vel, 0.0),
                (false,  true,  true, false) => {go.pos.x += *SPEED.lock().unwrap(); go.pos.y += *SPEED.lock().unwrap()}, //self.translate(vel*0.707, vel*0.707),
                (false, false,  true, false) => {                                    go.pos.y += *SPEED.lock().unwrap()}, //self.translate(0.0, vel),
                (false, false,  true,  true) => {go.pos.x -= *SPEED.lock().unwrap(); go.pos.y += *SPEED.lock().unwrap()}, //self.translate(-vel*0.707, vel*0.707),
                (false, false, false,  true) => {go.pos.x -= *SPEED.lock().unwrap()                                    }, //self.translate(-vel, 0.0),
                ( true, false, false,  true) => {go.pos.x -= *SPEED.lock().unwrap(); go.pos.y -= *SPEED.lock().unwrap()}, //self.translate(-vel*0.707, -vel*0.707),
                _ => (),
            }
    
            // Consume the input.
            //ecs.input = Input {keycode_up: None, keymod_up: None, keycode_down: None, keymod_down: None};
        }
        _ => (),
    }
    
   
}

pub fn player_controller_render(component: &Component, ctx: &mut Context) -> GameResult<()> {
    match component {
        _ => Ok(()),
    }
}