
use ggez::Context;
use ggez::graphics;
use ggez::graphics::Point2;
use ecsystem::component::Component;
use ecsystem::component::IComponent;
use ecsystem::gameobject::GameObject;
use ecsystem::ECSystem;




use ggez::GameResult;


pub fn renderable_text_start(component: &mut Component, ecs: &mut ECSystem) {
    match component {
        &mut Component::RenderableText{..} => {
            //println!("RT START DEBUG");
        }
        _ => (),
    }
}

pub fn renderable_text_update(component: &mut Component, ctx: &mut Context, ecs: &mut ECSystem, go: &mut GameObject) {
    match component {
        &mut Component::RenderableText{..} => {
           // println!("RT UPDATE");
           
            //println!("{:?}", ecs.blackboard.panel().get(&go.tag().to_string()));
        }
        _ => (),
    }
}

pub fn renderable_text_render(component: &Component, ctx: &mut Context, go: &GameObject) -> GameResult<()> {
    match component {
        &Component::RenderableText{ref text} => {
            {
            let font = graphics::Font::new(ctx, "/font/FiraSans-Regular.ttf", 16)?;
            let text = graphics::Text::new(ctx, text, &font)?;
            graphics::draw(ctx, &text, go.pos, 0.0)
            }
            
            
        }
        _ => Ok(()),
    }
}