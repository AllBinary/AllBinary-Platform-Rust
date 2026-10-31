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
// package org.allbinary.graphics.j2me.workarea.canvas.event
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::MyCanvasEvent::MyCanvasEvent;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait MyCanvasEventListener {
    

    fn delete(&mut self, evt: Option<Rc<RefCell<MyCanvasEvent>>>);
    

    // throws Exception
    fn duplicate(&mut self, evt: Option<Rc<RefCell<MyCanvasEvent>>>);
    

    fn select(&mut self, evt: Option<Rc<RefCell<MyCanvasEvent>>>);
    

    fn rotate(&mut self, evt: Option<Rc<RefCell<MyCanvasEvent>>>);
    

    fn explode(&mut self, evt: Option<Rc<RefCell<MyCanvasEvent>>>);
    

    // throws Exception
    fn autoExplode(&mut self, evt: Option<Rc<RefCell<MyCanvasEvent>>>);
    

    // throws Exception
    fn center(&mut self, evt: Option<Rc<RefCell<MyCanvasEvent>>>);

    fn toString(&mut self) -> String { String::from(std::any::type_name::<Self>()) }
}


