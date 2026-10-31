// Generated Code Do Not Modify
// package org.allbinary.graphics.form.item
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Graphics::Graphics;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait ABCustomItemInterface {
    

    fn getMinimumWidth(&mut self) -> i32;
    

    fn getMinimumHeight(&mut self) -> i32;
    

    fn getLabel(&mut self) -> String;
    

    fn paintXY(&mut self, graphics: Option<Rc<RefCell<Graphics>>>, x: i32, y: i32);
    

    fn paintUnselected(&mut self, graphics: Option<Rc<RefCell<Graphics>>>, x: i32, y: i32);

    fn toString(&mut self) -> String { String::from(std::any::type_name::<Self>()) }
}


