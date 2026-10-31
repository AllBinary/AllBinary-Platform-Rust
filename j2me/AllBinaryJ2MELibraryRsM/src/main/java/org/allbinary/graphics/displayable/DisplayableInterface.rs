//
// AllBinary Open License Version 1
// Copyright (c) 2011 AllBinary
// 
// By agreeing to this license you and any business entity you represent are
// legally bound to the AllBinary Open License Version 1 legal agreement.
// 
// You may obtain the AllBinary Open License Version 1 legal agreement from
// AllBinary or the root directory of AllBinary's AllBinary Platform repository.
// 
// Created By: Travis Berthelot
//  
// Generated Code Do Not Modify
// package org.allbinary.graphics.displayable
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Command::Command;
// use crate::javax::microedition::lcdui::CommandListener::CommandListener;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait DisplayableInterface {
    

    fn addCommand(&mut self, cmd: Option<Rc<RefCell<Command>>>);
    

    fn getHeight(&mut self) -> i32;
    

    fn getWidth(&mut self) -> i32;
    

    fn removeCommand(&mut self, cmd: Option<Rc<RefCell<Command>>>);
    

    fn setCommandListener(&mut self, l: Option<Rc<RefCell<CommandListener>>>);

    fn toString(&mut self) -> String { String::from(std::any::type_name::<Self>()) }
}


