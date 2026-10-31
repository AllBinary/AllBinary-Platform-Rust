// Generated Code Do Not Modify
// package javax.microedition.lcdui
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use crate::Stack;
use super::*;
// use crate::java::util::Stack::Stack;
use super::Canvas::Canvas;
use super::Command::Command;
use super::CommandListener::CommandListener;
use super::Graphics::Graphics;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NullCanvas {
    pub base: Canvas,
}

thread_local! { pub static NullCanvas_NULL_CANVAS: Option<Rc<RefCell<NullCanvas>>> = NullCanvas::new(); }

impl NullCanvas {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: Canvas::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn setFullScreenMode(&mut self, mut mode: bool) {
    }
    

    pub fn sizeChanged(&mut self, mut w: i32, mut h: i32) {
    }
    

    pub fn getCommandStack(&mut self) -> Option<Rc<RefCell<Stack<Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn isCommand(&mut self, mut command: Option<Rc<RefCell<Command>>>) -> bool {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn addCommand(&mut self, mut command: Option<Rc<RefCell<Command>>>) {
    }
    

    pub fn removeCommand(&mut self, mut command: Option<Rc<RefCell<Command>>>) {
    }
    

    pub fn removeAllCommands(&mut self) {
    }
    

    pub fn setCommandListener(&mut self, mut l: Option<Rc<RefCell<CommandListener>>>) {
    }
    

    pub fn getCustomCommandListener(&mut self) -> Option<Rc<RefCell<CommandListener>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn isPaused(&mut self) -> bool {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn removePauseCommand(&mut self) {
    }
    

    pub fn pause(&mut self) {
    }
    

    pub fn unPause(&mut self) {
    }
    

    // throws Exception
    pub fn process(&mut self) {
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
    }
    

    pub fn destroy(&mut self) {
    }
    

    pub fn setPaused(&mut self, mut isPaused: bool) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn pointerDragged(&mut self, mut x: i32, mut y: i32) {
    }
    

    pub fn pointerPressed(&mut self, mut x: i32, mut y: i32) {
    }
    

    pub fn pointerReleased(&mut self, mut x: i32, mut y: i32) {
    }
}
impl std::ops::Deref for NullCanvas {
    type Target = Canvas;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for NullCanvas {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



