// 
// AllBinary Open License Version 1 
// Copyright (c) 2006 AllBinary 
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
// package org.allbinary.game.input
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::util::visitor::Visitor::Visitor;
use super::SelectRTSLayerVisitorFactoryInterface::SelectRTSLayerVisitorFactoryInterface;
use super::SelectAdvancedRTSLayerVisitor::SelectAdvancedRTSLayerVisitor;
use super::SelectedRTSLayersPlayerGameInput::SelectedRTSLayersPlayerGameInput;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SelectAdvancedRTSLayerVisitorFactory {
}

thread_local! { pub static SelectAdvancedRTSLayerVisitorFactory_instance: Option<Rc<RefCell<SelectAdvancedRTSLayerVisitorFactory>>> = SelectAdvancedRTSLayerVisitorFactory::new(); }
impl SelectRTSLayerVisitorFactoryInterface for SelectAdvancedRTSLayerVisitorFactory {
    fn create(&mut self, mut selectedRTSLayersPlayerGameInput: Option<Rc<RefCell<SelectedRTSLayersPlayerGameInput>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::visitor::Visitor::VisitorDyn>>> { Self::create(self, selectedRTSLayersPlayerGameInput) }
}

impl SelectAdvancedRTSLayerVisitorFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<SelectAdvancedRTSLayerVisitorFactory>>> {
        return SelectAdvancedRTSLayerVisitorFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn create(&mut self, mut selectedRTSLayersPlayerGameInput: Option<Rc<RefCell<SelectedRTSLayersPlayerGameInput>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::visitor::Visitor::VisitorDyn>>> {
        return SelectAdvancedRTSLayerVisitor::new(selectedRTSLayersPlayerGameInput.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::visitor::Visitor::VisitorDyn>>);
    }
}

