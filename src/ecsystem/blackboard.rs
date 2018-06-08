

use std::collections::HashMap;
use std::cell::RefCell;
use std::cell::Ref;
use std::rc::Rc;

#[derive(Debug)]
pub enum BlackboardObject {
    Int(i64),
    UInt(u64),
    String(String),
}


pub struct Blackboard {
    pub map: Rc<RefCell<HashMap<String, BlackboardObject>>>,
    //pub map: HashMap<String, BlackboardObject>;
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