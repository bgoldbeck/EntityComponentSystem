
use ggez::Context;
use ggez::graphics;
use ggez::graphics::Image;
use ggez::graphics::Point2;
use ecsystem::ECSystem;
use std::hash::Hasher;
use std::hash::Hash;
use std::fmt;

use ecsystem::gameobject::GameObject;
use ecsystem::renderable::*;
use ecsystem::renderable_text::*;
use ecsystem::player_controller::*;


pub trait IComponent {
    
    fn start(&mut self, ecs: &mut ECSystem);
    fn update(&mut self, ecs: &mut ECSystem, go: &mut GameObject);
    
    fn render(&self, ctx: &mut Context, go: &GameObject);
    
}

#[derive(Debug, Clone)]
pub enum Component {

    Renderable {
        sprite: Image,
    },

    RenderableText {
        text: String,
    },

    PlayerController {
    },
}

impl IComponent for Component {
    fn start(&mut self, ecs: &mut ECSystem) {
        match self {
            &mut Component::Renderable{..} => renderable_start(self, ecs),
            &mut Component::RenderableText{..} => renderable_text_start(self, ecs),
            &mut Component::PlayerController{..} => player_controller_start(self, ecs),
            _ => (),
        }
    }

    fn update(&mut self, ecs: &mut ECSystem, go: &mut GameObject) {
        match self {
            &mut Component::Renderable{..} => renderable_update(self, ecs, go),
            &mut Component::RenderableText{..} => renderable_text_update(self, ecs, go),
            &mut Component::PlayerController{..} => player_controller_update(self, ecs, go),
            _ => (),
        }
    }

  
 
    fn render(&self, ctx: &mut Context, go: &GameObject) {
        match self {
            &Component::Renderable{..} => renderable_render(self, ctx, go),
            &Component::RenderableText{..} => renderable_text_render(self, ctx, go),
            _ => Ok(()),
        };
    }

}

impl fmt::Debug for IComponent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

