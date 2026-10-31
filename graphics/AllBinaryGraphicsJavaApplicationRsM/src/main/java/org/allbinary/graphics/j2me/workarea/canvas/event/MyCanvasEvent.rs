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
// use crate::javax::swing::tree::MutableTreeNode::MutableTreeNode;
use super::EventObject::EventObject;
use super::util::util;
use super::java::java;
use super::MyCanvasEventSource::MyCanvasEventSource;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MyCanvasEvent {
    pub base: EventObject,
}


impl MyCanvasEvent {
    

    pub fn new_struct(mut source: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Self {
        let mut this = Self {
            base: EventObject::new_struct1(source.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut source: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(source))))
    }
    

    pub fn getSource(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return self.base.getSource();
    }
    

    pub fn toString(&mut self) -> String {
        return self.base.toString();
    }
    

    pub fn getCommand(&mut self) -> String {
        let mut myEventSource: Option<Rc<RefCell<MyCanvasEventSource>>> = { let java_object = (self.getSource()).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<MyCanvasEventSource>>>>().expect("ClassCastException").clone()) };
        return jcall!(myEventSource).getCommand();
    }
    

    pub fn getTreeNode(&mut self) -> Option<Rc<RefCell<MutableTreeNode>>> {
        let mut myEventSource: Option<Rc<RefCell<MyCanvasEventSource>>> = { let java_object = (self.getSource()).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<MyCanvasEventSource>>>>().expect("ClassCastException").clone()) };
        return jcall!(myEventSource).getTreeNode();
    }
    

    pub fn getAngle(&mut self) -> f64 {
        let mut myEventSource: Option<Rc<RefCell<MyCanvasEventSource>>> = { let java_object = (self.getSource()).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<MyCanvasEventSource>>>>().expect("ClassCastException").clone()) };
        return jcall!(myEventSource).getAngle();
    }
}
impl std::ops::Deref for MyCanvasEvent {
    type Target = EventObject;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for MyCanvasEvent {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



