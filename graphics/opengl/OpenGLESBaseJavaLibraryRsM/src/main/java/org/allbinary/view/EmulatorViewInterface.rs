// Generated Code Do Not Modify
// package org.allbinary.view
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Displayable::Displayable;
// use crate::javax::microedition::midlet::MIDlet::MIDlet;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait EmulatorViewInterface {
    

    fn setMidlet(&mut self, midlet: Option<Rc<RefCell<MIDlet>>>);
    

    fn onEmulatorInitComplete(&mut self, midletActivity: Option<Rc<RefCell<Box<dyn std::any::Any>>>>);
    

    fn onSetDisplayable(&mut self, displayable: Option<Rc<RefCell<Displayable>>>);

    fn toString(&mut self) -> String { String::from(std::any::type_name::<Self>()) }
}


