


extern crate ggez;
use ecsystem::*;


use ggez::conf;
use ggez::{Context, GameResult};
use ggez::event::{self, Button, MouseState, Keycode, Mod, Axis};
use ggez::graphics;
use std::env;
use std::any::Any;
use std::path;
use std::mem::transmute;
use std::collections::HashMap;
use ecsystem::gameobject::*;
use ecsystem::component::*;
//use ecsystem::playercontroller::PlayerController;
//use ecsystem::gameobject::GAME_OBJECTS;
//use game::renderable::Renderable;
use ecsystem::component::IComponent;

pub struct Game {
}

impl Game {
    pub fn new() -> Self {
        Game {
        }
    }

    pub fn init(&mut self) {
        let c = conf::Conf::new();
        let ctx = &mut Context::load_from_conf("helloworld", "ggez", c).unwrap();   

        // We add the CARGO_MANIFEST_DIR/resources to the filesystem's path
        // so that ggez will look in our cargo project directory for files.
        if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
            let mut path = path::PathBuf::from(manifest_dir);
            path.push("assets");
            ctx.filesystem.mount(&path, true);
        }

        
//        let mut game_objects: &HashMap<String, Box<GameObject>> = &self.ecsystem.game_objects;
//        let go: &GameObject = game_objects.get(&tag).unwrap();

  //      go

        //if let Some(x) = game_objects.get_mut(&tag) {
           // *x = Box::new(*go);
       // }

        
        let state = &mut MainState::new(ctx).unwrap();

        if let Err(e) = event::run(ctx, state) {
            println!("Error encountered: {}", e);
        } else {
            println!("Game exited cleanly.");
        }
       
 

    }
}

// First we make a structure to contain the game's state
pub struct MainState {
    pub ecsystem: ECSystem,
    pub frames: usize,
}

impl MainState {


    pub fn new(ctx: &mut Context) -> GameResult<MainState> {
        // The ttf file will be in your resources directory. Later, we
        // will mount that directory so we can omit it in the path here.
        
        let font = graphics::Font::new(ctx, "/font/FiraSans-Regular.ttf", 48)?;
        let score_text = graphics::Text::new(ctx, "Score: ", &font)?;
        let mut ecs: ECSystem = ECSystem::new();

       
        // Create and add a player game object to the system. 
        let tag: String = "player".to_string();
        println!("creating player");
        let mut player = GameObject::new(tag.clone());

        println!("add renderable component");
        player.add_component(Box::new(Component::Renderable {
            sprite: graphics::Image::new(ctx, "/texture/crab.png").unwrap(),
            }));
        
        println!("add renderable text component");
        player.add_component(Box::new(Component::RenderableText {
            text: "Player".to_string(),
            }));
        
        println!("add player controller");
        player.add_component(Box::new(Component::PlayerController {
            }));

        println!("add game object to ecs");
        ecs.add_game_object(player);
        
        println!("create mainstate");
        let mut s = MainState {
            ecsystem: ecs,
            frames: 0,
        };

        Ok(s)
    }
    
}

// Then we implement the `ggez:event::EventHandler` trait on it, which
// requires callbacks for updating and drawing the game state each frame.
//
// The `EventHandler` trait also contains callbacks for event handling
// that you can override if you wish, but the defaults are fine.
impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        /*
        if self.ecsystem.input.keycode_up != None {
            self.ecsystem.input = Input {
                keycode_down: None,
                keymod_down: None,
                keycode_up: None,
                keymod_up: None,
            };
        }
        */
        self.ecsystem.update(ctx);
        self.ecsystem.late_update();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        // Drawables are drawn from their top-left corner.
        //graphics::draw(ctx, &self.background, graphics::Point2::new(0.0, 0.0), 0.0)?;


        self.ecsystem.render(ctx);

        graphics::present(ctx);

        self.frames += 1;
        if (self.frames % 100) == 0 {
            println!("FPS: {}", ggez::timer::get_fps(ctx));
        }

        Ok(())
    }

    // Event is triggered when the player presses keydowns
    fn key_down_event(&mut self, _ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool) {
        println!("keydown: {:?}", keycode);
        
        self.ecsystem.input.keycode_down = Some(keycode);
        self.ecsystem.input.keymod_down = Some(keymod);

        /*
        match self.ecsystem.input.keycode_down {
            None => self.ecsystem.input.keycode_down = Some(keycode),
            _ => (),
        }
        match self.ecsystem.input.keymod_down {
            None => self.ecsystem.input.keymod_down = Some(keymod),
            _ => (),
        }
        */
        //self.ecsystem.input = Input {
        //    keycode_down: Some(keycode),
        //    keymod_down: Some(keymod),
        //    keymod_up: self.ecsystem.input.keymod_down,
        //    keycode_up: self.ecsystem.input.keycode_down,
        //};
        
    }
    
	// Event is triggered when player lifts up on a keys
    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool) {
        println!("keyup: {:?}", keycode);

        self.ecsystem.input.keycode_up = Some(keycode);
        self.ecsystem.input.keymod_up = Some(keymod);
        /*
        match self.ecsystem.input.keycode_up {
            None => self.ecsystem.input.keycode_up = Some(keycode),
            _ => (),
        }
        match self.ecsystem.input.keymod_up {
            None => self.ecsystem.input.keymod_up = Some(keymod),
            _ => (),
        }
        */
        //self.ecsystem.input = Input {
            //keycode_down: self.ecsystem.input.keycode_down,
            //keymod_down: self.ecsystem.input.keymod_down,
            //keycode_up: Some(keycode),
            //keymod_up: Some(keymod),
        //};
    }
    
}


