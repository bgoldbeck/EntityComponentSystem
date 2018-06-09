
use ggez::Context;
use ggez::graphics;
use ggez::graphics::Point2;
use ecsystem::component::Component;
use ecsystem::component::IComponent;
use ecsystem::gameobject::GameObject;
use ecsystem::ECSystem;
use ecsystem::blackboard::*;



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
            let panel = ecs.blackboard.panel();
            let option_obj: Option<&BlackboardObject> = panel.get(&go.tag().to_string());
            
            if option_obj.is_some() {
                let blackboard_obj: &BlackboardObject = option_obj.unwrap();
                let blackboard_obj_value = blackboard_obj.to_value::<u32>().unwrap();

                println!("{:?}", blackboard_obj_value);
            }
            
            
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