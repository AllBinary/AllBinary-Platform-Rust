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
// package org.allbinary.graphics.form.item
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Command::Command;
use crate::org::allbinary::game::commands::GameCommandsFactory::GameCommandsFactory;
use crate::org::allbinary::graphics::displayable::DisplayInfoSingleton::DisplayInfoSingleton;
use crate::org::allbinary::graphics::form::ScreenInfo::ScreenInfo;
use crate::org::allbinary::logic::java::bool::BooleanFactory::BooleanFactory;
use crate::org::allbinary::logic::util::visitor::Visitor::Visitor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LimitCommandsVisitor {
    pub base: Visitor,
    pub isNotSmallDisplay: bool,
}


impl LimitCommandsVisitor {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: Visitor::new_struct(),
            isNotSmallDisplay: false,
        };
        let mut displayInfo: Option<Rc<RefCell<DisplayInfoSingleton>>> = DisplayInfoSingleton::getInstance();
        this.isNotSmallDisplay = (jcall!(displayInfo).isPortrait() && jcall!(displayInfo).getLastHeight() > jcall!(ScreenInfo::getInstance()).SMALL_WIDTH) || (!jcall!(displayInfo).isPortrait() && jcall!(displayInfo).getLastHeight() > jcall!(ScreenInfo::getInstance()).SMALL_HEIGHT);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn visit(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        let mut command: Option<Rc<RefCell<Command>>> = (anyType).clone() /* as Command */;
        let mut booleanFactory: Option<Rc<RefCell<BooleanFactory>>> = BooleanFactory::getInstance();
        if self.isNotSmallDisplay || jcall!(command).getPriority() == 1 || match (&command, &jcall!(GameCommandsFactory::getInstance()).EXIT_COMMAND) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } || match (&command, &jcall!(GameCommandsFactory::getInstance()).EXIT_WITHOUT_PROGRESS_COMMAND) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return jcall!(booleanFactory).TRUE.clone();
        } else {
            return jcall!(booleanFactory).FALSE.clone();
        }
    }
}
impl std::ops::Deref for LimitCommandsVisitor {
    type Target = Visitor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for LimitCommandsVisitor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::util::visitor::Visitor::VisitorDyn for LimitCommandsVisitor {
    fn as_Visitor(&self) -> &crate::org::allbinary::logic::util::visitor::Visitor::Visitor { &self.base }
    fn as_Visitor_mut(&mut self) -> &mut crate::org::allbinary::logic::util::visitor::Visitor::Visitor { &mut self.base }
}
impl crate::org::allbinary::logic::util::visitor::VisitorInterface::VisitorInterface for LimitCommandsVisitor {
    fn visit(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { Self::visit(self, anyType) }
}


