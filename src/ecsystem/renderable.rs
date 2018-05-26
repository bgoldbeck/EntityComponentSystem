
use ggez::Context;
use ggez::graphics;
use ggez::graphics::Point2;
use ecsystem::component::Component;
use ecsystem::component::IComponent;
use ecsystem::gameobject::GameObject;
use ecsystem::ECSystem;




use ggez::GameResult;

pub fn renderable_start(component: &mut Component, ecs: &mut ECSystem) {
    match component {
        &mut Component::Renderable{..} => {
            println!("R START DEBUG");
        }
        _ => (),
    }
}

pub fn renderable_update(component: &mut Component, ecs: &mut ECSystem, go: &mut GameObject) {
    match component {
        &mut Component::Renderable{..} => {
            println!("R UPDATE");
        
        }
        _ => (),
    }
}

pub fn renderable_render(component: &Component, ctx: &mut Context, go: &GameObject) -> GameResult<()> {
    match component {
        &Component::Renderable{ref sprite, ..} => {
            graphics::draw(ctx, sprite, go.pos, 0.0)
        }
        _ => Ok(()),
    }
}