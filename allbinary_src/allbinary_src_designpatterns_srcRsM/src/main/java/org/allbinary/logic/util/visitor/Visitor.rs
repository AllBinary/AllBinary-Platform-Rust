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
// package org.allbinary.logic.util.visitor
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use super::VisitorInterface::VisitorInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct Visitor {
    pub nullUtil: Option<Rc<RefCell<NullUtil>>>,
}

thread_local! { pub static Visitor_instance: Option<Rc<RefCell<dyn VisitorDyn>>> = Visitor::new().map(|reference| reference as Rc<RefCell<dyn VisitorDyn>>); }
impl VisitorInterface for Visitor {
    fn visit(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { Self::visit(self, anyType) }
}

impl Visitor {

    pub fn new_struct() -> Self {
        let mut this = Self {
            nullUtil: None,
        };
        this.nullUtil = NullUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn visit(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return jcall!(self.nullUtil).NULL_OBJECT.clone();
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn VisitorDyn>>> {
        return Visitor_instance.with(Clone::clone).clone();
    }
}
pub trait VisitorDyn: crate::org::allbinary::logic::util::visitor::VisitorInterface::VisitorInterface {
    fn as_Visitor(&self) -> &Visitor;
    fn as_Visitor_mut(&mut self) -> &mut Visitor;
}
impl VisitorDyn for Visitor {
    fn as_Visitor(&self) -> &Visitor { self }
    fn as_Visitor_mut(&mut self) -> &mut Visitor { self }
}


