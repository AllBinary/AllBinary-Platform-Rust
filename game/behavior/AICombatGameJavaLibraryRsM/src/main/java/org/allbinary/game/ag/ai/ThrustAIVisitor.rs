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
// package org.allbinary.game.ag.ai
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Canvas::Canvas;
use crate::org::allbinary::game::ai::BasicAI::BasicAI;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::java::bool::BooleanFactory::BooleanFactory;
use crate::org::allbinary::logic::util::visitor::Visitor::Visitor;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ThrustAIVisitor {
    pub base: Visitor,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub TRUE: Option<bool>,
}


impl ThrustAIVisitor {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: Visitor::new_struct(),
            logUtil: None,
            TRUE: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.TRUE = jcall!(BooleanFactory::getInstance()).TRUE.clone();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn visit(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut basicAI: Option<Rc<RefCell<dyn crate::org::allbinary::game::ai::BasicAI::BasicAIDyn>>> = (anyType).clone() /* as BasicAI */;
            jcall!(basicAI).processKeyAI(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_UP.clone() }.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(self.logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).VISIT.clone(), crate::java_into_object(e.clone()));
            }
        }
        return self.TRUE.clone();
    }
}
impl std::ops::Deref for ThrustAIVisitor {
    type Target = Visitor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ThrustAIVisitor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::util::visitor::Visitor::VisitorDyn for ThrustAIVisitor {
    fn as_Visitor(&self) -> &crate::org::allbinary::logic::util::visitor::Visitor::Visitor { &self.base }
    fn as_Visitor_mut(&mut self) -> &mut crate::org::allbinary::logic::util::visitor::Visitor::Visitor { &mut self.base }
}
impl crate::org::allbinary::logic::util::visitor::VisitorInterface::VisitorInterface for ThrustAIVisitor {
    fn visit(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { Self::visit(self, anyType) }
}


