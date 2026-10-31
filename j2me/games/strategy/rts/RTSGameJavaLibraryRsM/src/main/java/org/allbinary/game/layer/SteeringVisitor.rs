// 
// AllBinary Open License Version 1 
// Copyright (c) 2003 AllBinary 
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
// package org.allbinary.game.layer
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::logic::util::visitor::Visitor::Visitor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SteeringVisitor {
    pub base: Visitor,
    pub list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}


impl SteeringVisitor {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: Visitor::new_struct(),
            list: None,
        };
        this.list = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.list.clone();
    }
}
impl std::ops::Deref for SteeringVisitor {
    type Target = Visitor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for SteeringVisitor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait SteeringVisitorDyn: crate::org::allbinary::logic::util::visitor::Visitor::VisitorDyn {
    fn as_SteeringVisitor(&self) -> &SteeringVisitor;
    fn as_SteeringVisitor_mut(&mut self) -> &mut SteeringVisitor;
    fn getList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { SteeringVisitor::getList(self.as_SteeringVisitor_mut()) }
}
impl SteeringVisitorDyn for SteeringVisitor {
    fn as_SteeringVisitor(&self) -> &SteeringVisitor { self }
    fn as_SteeringVisitor_mut(&mut self) -> &mut SteeringVisitor { self }
}

impl crate::org::allbinary::logic::util::visitor::Visitor::VisitorDyn for SteeringVisitor {
    fn as_Visitor(&self) -> &crate::org::allbinary::logic::util::visitor::Visitor::Visitor { &self.base }
    fn as_Visitor_mut(&mut self) -> &mut crate::org::allbinary::logic::util::visitor::Visitor::Visitor { &mut self.base }
}
impl crate::org::allbinary::logic::util::visitor::VisitorInterface::VisitorInterface for SteeringVisitor {
    fn visit(&mut self, p1: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { crate::org::allbinary::logic::util::visitor::Visitor::Visitor::visit(crate::org::allbinary::logic::util::visitor::Visitor::VisitorDyn::as_Visitor_mut(self), p1) }
}


