// Generated Code Do Not Modify
// package javax.microedition.lcdui
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::CommandListener::CommandListener;
use super::Command::Command;
use super::Displayable::Displayable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NullCommandListener {
}

thread_local! { pub static NullCommandListener_NULL_COMMAND_LISTENER: Option<Rc<RefCell<dyn NullCommandListenerDyn>>> = NullCommandListener::new().map(|reference| reference as Rc<RefCell<dyn NullCommandListenerDyn>>); }
// impl CommandListener for NullCommandListener with the methods of the trait

impl NullCommandListener {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn commandAction(&mut self, mut c: Option<Rc<RefCell<Command>>>, mut d: Option<Rc<RefCell<Displayable>>>) {
    }
}
pub trait NullCommandListenerDyn {
    fn as_NullCommandListener(&self) -> &NullCommandListener;
    fn as_NullCommandListener_mut(&mut self) -> &mut NullCommandListener;
    fn commandAction(&mut self, mut c: Option<Rc<RefCell<Command>>>, mut d: Option<Rc<RefCell<Displayable>>>) { NullCommandListener::commandAction(self.as_NullCommandListener_mut(), c, d) }
}
impl NullCommandListenerDyn for NullCommandListener {
    fn as_NullCommandListener(&self) -> &NullCommandListener { self }
    fn as_NullCommandListener_mut(&mut self) -> &mut NullCommandListener { self }
}


