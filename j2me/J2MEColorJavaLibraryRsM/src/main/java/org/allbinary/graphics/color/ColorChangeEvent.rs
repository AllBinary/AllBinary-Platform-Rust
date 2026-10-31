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
// package org.allbinary.graphics.color
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject;
use super::BasicColor::BasicColor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ColorChangeEvent {
    pub base: AllBinaryEventObject,
    pub basicColor: Option<Rc<RefCell<BasicColor>>>,
}


impl ColorChangeEvent {
    

    pub fn new_struct(mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Self {
        let mut this = Self {
            base: AllBinaryEventObject::new_struct(anyType.clone()),
            basicColor: None,
        };
        this.basicColor = jcall!(BasicColorFactory::BasicColorFactory::getInstance()).WHITE.clone();
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(anyType))))
    }
    

    pub fn setBasicColorP(&mut self, mut basicColor: Option<Rc<RefCell<BasicColor>>>) {
        self.basicColor = basicColor.clone();
    }
    

    pub fn getBasicColorP(&mut self) -> Option<Rc<RefCell<BasicColor>>> {
        return self.basicColor.clone();
    }
}
impl std::ops::Deref for ColorChangeEvent {
    type Target = AllBinaryEventObject;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ColorChangeEvent {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn for ColorChangeEvent {
    fn as_AllBinaryEventObject(&self) -> &crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject { &self.base }
    fn as_AllBinaryEventObject_mut(&mut self) -> &mut crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject { &mut self.base }
}


