

use std::collections::HashMap;
use std::cell::RefCell;
use std::cell::Ref;
use std::rc::Rc;
use std::any::*;

pub trait IBObject {
    fn to_value<T: 'static>(&self) -> Option<&T>;
}

#[derive(Debug)]
pub enum BlackboardObject {
    Doodle {
        object: (Box<Any>),
    },
}

impl<'a> IBObject for BlackboardObject {
    fn to_value<T: 'static>(&self) -> Option<&T> {
        match self {
            &BlackboardObject::Doodle{ref object} => object.downcast_ref::<T>(),
            _ => None,
        }
    }
}

pub struct Blackboard {
    pub map: Rc<RefCell<HashMap<String, BlackboardObject>>>,

}

impl Blackboard {
    pub fn new() -> Blackboard {
        Blackboard {
            map: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn scribble(&mut self, tag: String, object: BlackboardObject) {
        self.map.borrow_mut().insert(tag, object);
    }

    pub fn erase(&mut self, tag: String) {
        self.map.borrow_mut().remove(&tag);
    }

    pub fn panel(&mut self) -> Ref<HashMap<String, BlackboardObject>> {
        self.map.borrow()
    }
}