
use ggez::Context;
use ggez::graphics;
use ggez::graphics::Point2;
use ggez;
use ggez::event::{self, Button, MouseState, Keycode, Mod, Axis};
use ecsystem::component::Component;
use ecsystem::component::IComponent;
use ecsystem::gameobject::GameObject;
use ecsystem::ECSystem;

use ggez::GameResult;


struct Movement {
    left: bool,
    right: bool,
}

lazy_static! {
    static ref MOVEMENT: Movement = Movement {left: false, right: false};
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
                 go.pos = Point2::new(go.pos.x - 10.0, go.pos.y);
             }
             if ecs.input.keycode_down == Some(ggez::event::Keycode::Right) {
                 go.pos = Point2::new(go.pos.x + 10.0, go.pos.y);
             }
        }
        _ => (),
    }
}

pub fn player_controller_render(component: &Component, ctx: &mut Context) -> GameResult<()> {
    match component {
        _ => Ok(()),
    }
}