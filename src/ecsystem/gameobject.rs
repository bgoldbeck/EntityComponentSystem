
use ggez::graphics;
use ggez::Context;
use std::sync::Mutex;
use std::collections::{HashMap, HashSet};
use std::ops::{Deref, DerefMut, Drop};
use std::any::{Any, TypeId};
use lazy_static::lazy::Lazy;
use ecsystem::component::IComponent;
use ecsystem::ECSystem;
use std::thread;
use ecsystem::component::Component;
use ggez::graphics::Point2;
use ecsystem::TAGS;


#[derive(Debug, Clone)]
pub struct GameObject {
    pub tag: String,
    pub pos: Point2,
    //pub components: Vec<Box<IComponent>>,
    pub comp_to_add: HashMap<String, Vec<Box<Component>>>,
    
}




lazy_static! {
    pub static ref COMPONENTS: Mutex<HashMap<String, Vec<Box<Component>>>> = Mutex::new(HashMap::new());
}

impl GameObject {
    
     
    pub fn new<'a>(tag: String) -> Box<GameObject> {
        
        
        //let len = GAME_OBJECTS.lock().unwrap().len().to_string();
        //let len: String = "1".to_string();

        let go: Box<GameObject> = Box::new(GameObject{
            tag: tag.to_string(),
            pos: graphics::Point2::new(0.0, 0.0),
            comp_to_add: HashMap::new(),
            //components: Vec::new(),
        });

        //ecs.add_game_object(id, go)
        
        //GAME_OBJECTS.lock().unwrap().insert(id.clone(), &go);

        go
    }


    pub fn update(&mut self, ctx: &mut Context, ecs: &mut ECSystem) {
        
        //println!("update gameobject");
        //println!("update gameobject");
        let mut component_map = COMPONENTS.lock().unwrap();
        
        //println!("update gameobject");
        if component_map.contains_key(&self.tag) {
            
            let mut my_components: &mut Vec<Box<Component>> = component_map.get_mut(&self.tag).unwrap();
            
            for comp_idx in 0..my_components.len() {
                my_components[comp_idx].update(ctx, ecs, self);   
            }
        }
        
        //println!("update gameobject");
    }

    pub fn late_update(&mut self) {
        //println!("late-update gameobject");
        //let mut component_map = COMPONENTS.lock().unwrap();

        //let keys = self.comp_to_add.keys().clone();
        let len = self.comp_to_add.len();

        //println!("update ecs");
        //for key in keys {
        if len > 0 {
            let mut component_map = COMPONENTS.lock().unwrap(); {
                // Add a new vector if we need to.
                if !component_map.contains_key(&self.tag) {
                    component_map.insert(self.tag.clone(), Vec::new());  
                }

                let mut my_components: &mut Vec<Box<Component>> = component_map.get_mut(&self.tag).unwrap();
                
                let components_to_add: &mut Vec<Box<Component>> = self.comp_to_add.get_mut(&self.tag).unwrap();

                
                for comp in components_to_add.iter() {
                    my_components.push(comp.clone());
                }
                println!("Number of Components: {}", my_components.len());
            }
            
            self.comp_to_add.clear();
        }
        //println!("late-later update gameobject");
    }

    pub fn render(&self, ctx: &mut Context) {
        
        let mut component_map = COMPONENTS.lock().unwrap();
        if component_map.contains_key(&self.tag) {
        let mut my_components: &mut Vec<Box<Component>> = component_map.get_mut(&self.tag).unwrap();
            //println!("render gameobject");
            for comp_idx in 0..my_components.len() {
                //let go = &entity_comp_system.game_objects[&key];
                my_components[comp_idx].render(ctx, self);
            }
        }
    }

  
    //pub fn destroy(tag: String) -> Option<GameObject> {
        //GAME_OBJECTS.lock().unwrap().remove(&tag)    
    //}

    pub fn add_component<'a>(&'a mut self, mut component: Box<Component>) {
        // Start the component.
        //component.start(ecs);

        // SPNLOCK
        //let mut component_map = &mut COMPONENTS.lock().unwrap();

        // Add a new vector if we need to.
        if !self.comp_to_add.contains_key(&self.tag) {
            self.comp_to_add.insert(self.tag.clone(), Vec::new());  
        }
        
        // Push the new component onto the map of components for this key.
        let mut my_components: &mut Vec<Box<Component>> = self.comp_to_add.get_mut(&self.tag).unwrap();
        my_components.push(component);

    }
    
    pub fn get_component<'a>(&'a self, type_name: String) -> Option<&'a Box<Component>> {

        None
    }

    pub fn tag(&self) -> &String {
        &self.tag
    }
    //pub fn remove_component(&mut self, component: IComponent) {
        //let index = self.components.iter().position(|&ref x| x == &component).unwrap();

        //self.components.remove(index);
    //}
}

//impl ToOwned for 


