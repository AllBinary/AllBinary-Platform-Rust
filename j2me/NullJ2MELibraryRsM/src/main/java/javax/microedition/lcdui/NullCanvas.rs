// Generated Code Do Not Modify
// package javax.microedition.lcdui
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use crate::Stack;
use super::*;
// use crate::java::util::Stack::Stack;
use crate::java::lang::Exception::*;
use crate::java::lang::RuntimeException::*;
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

pub static NullCanvas_NULL_CANVAS: crate::JavaStatic<Option<Rc<RefCell<NullCanvas>>>> = crate::JavaStatic::new(|| NullCanvas::new());

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
    

    pub fn getCommandStack(&mut self) -> Option<Rc<RefCell<Stack<crate::JavaObject>>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn isCommand(&mut self, mut command: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) -> bool {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn addCommand(&mut self, mut command: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) {
    }
    

    pub fn removeCommand(&mut self, mut command: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) {
    }
    

    pub fn removeAllCommands(&mut self) {
    }
    

    pub fn setCommandListener(&mut self, mut l: Option<Rc<RefCell<dyn CommandListener>>>) {
    }
    

    pub fn getCustomCommandListener(&mut self) -> Option<Rc<RefCell<dyn CommandListener>>> {
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
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
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

impl crate::javax::microedition::lcdui::Canvas::CanvasDyn for NullCanvas {
    fn as_Canvas(&self) -> &crate::javax::microedition::lcdui::Canvas::Canvas { &self.base }
    fn as_Canvas_mut(&mut self) -> &mut crate::javax::microedition::lcdui::Canvas::Canvas { &mut self.base }
}
impl crate::javax::microedition::lcdui::Displayable::DisplayableDyn for NullCanvas {
    fn as_Displayable(&self) -> &crate::javax::microedition::lcdui::Displayable::Displayable { &self.base.base }
    fn as_Displayable_mut(&mut self) -> &mut crate::javax::microedition::lcdui::Displayable::Displayable { &mut self.base.base }
    fn addCommand(&mut self, mut command: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) { Self::addCommand(self, command) }
    fn removeCommand(&mut self, mut command: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) { Self::removeCommand(self, command) }
    fn setCommandListener(&mut self, mut l: Option<Rc<RefCell<dyn CommandListener>>>) { Self::setCommandListener(self, l) }
    fn setFullScreenMode(&mut self, mut mode: bool) { Self::setFullScreenMode(self, mode) }
    fn hideNotify(&mut self) { crate::javax::microedition::lcdui::Canvas::Canvas::hideNotify(crate::javax::microedition::lcdui::Canvas::CanvasDyn::as_Canvas_mut(self)) }
    fn keyPressed(&mut self, p1: i32) { crate::javax::microedition::lcdui::Canvas::Canvas::keyPressed(crate::javax::microedition::lcdui::Canvas::CanvasDyn::as_Canvas_mut(self), p1) }
    fn keyRepeated(&mut self, p1: i32) { crate::javax::microedition::lcdui::Canvas::Canvas::keyRepeated(crate::javax::microedition::lcdui::Canvas::CanvasDyn::as_Canvas_mut(self), p1) }
    fn keyReleased(&mut self, p1: i32) { crate::javax::microedition::lcdui::Canvas::Canvas::keyReleased(crate::javax::microedition::lcdui::Canvas::CanvasDyn::as_Canvas_mut(self), p1) }
    fn pointerPressed(&mut self, mut x: i32, mut y: i32) { Self::pointerPressed(self, x, y) }
    fn pointerReleased(&mut self, mut x: i32, mut y: i32) { Self::pointerReleased(self, x, y) }
    fn pointerDragged(&mut self, mut x: i32, mut y: i32) { Self::pointerDragged(self, x, y) }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::paint(self, graphics) }
    fn repaint(&mut self) { crate::javax::microedition::lcdui::Canvas::Canvas::repaint(crate::javax::microedition::lcdui::Canvas::CanvasDyn::as_Canvas_mut(self)) }
    fn repaintXYWH(&mut self, p1: i32, p2: i32, p3: i32, p4: i32) { crate::javax::microedition::lcdui::Canvas::Canvas::repaintXYWH(crate::javax::microedition::lcdui::Canvas::CanvasDyn::as_Canvas_mut(self), p1, p2, p3, p4) }
    fn sizeChanged(&mut self, mut w: i32, mut h: i32) { Self::sizeChanged(self, w, h) }
    fn showNotify(&mut self) { crate::javax::microedition::lcdui::Canvas::Canvas::showNotify(crate::javax::microedition::lcdui::Canvas::CanvasDyn::as_Canvas_mut(self)) }
}


