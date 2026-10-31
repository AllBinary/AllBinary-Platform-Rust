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
// package org.allbinary.graphics.form.item.validation
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::input::TextNotificationUtil::TextNotificationUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::java::bool::BooleanFactory::BooleanFactory;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use crate::org::allbinary::logic::util::visitor::Visitor::Visitor;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TextItemUserIdentifierVisitor {
    pub base: Visitor,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}


impl TextItemUserIdentifierVisitor {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: Visitor::new_struct(),
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn visit(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        let mut value: String = (anyType).clone() /* as String */;
        if (value.chars().count() as i32) < 2 {
            let mut charValue: char = value.chars().nth((0) as usize).unwrap();
            let mut stringValidationUtil: Option<Rc<RefCell<StringValidationUtil>>> = StringValidationUtil::getInstance();
            if jcall!(stringValidationUtil).isNumber(value.clone()) {
                return jcall!(BooleanFactory::getInstance()).TRUE.clone();
            } else if self.isLetter(charValue.clone()) {
                return jcall!(BooleanFactory::getInstance()).TRUE.clone();
            } else {
                match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    jcall!(TextNotificationUtil::getInstance()).fireError(String::from("Only Letters & Numbers Allowed"));
                })) {
                    Ok(_) => {}
                    Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

                    let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
                    jcall!(self.logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).VISIT.clone(), crate::java_into_object(e.clone()));
                    }
                }
                return jcall!(BooleanFactory::getInstance()).FALSE.clone();
            }
        } else {
            return jcall!(BooleanFactory::getInstance()).FALSE.clone();
        }
    }
    

    pub fn isLetter(&mut self, mut charValue: char) -> bool {
        let mut value: i32 = ((charValue) as i32);
        return (value > 64 && value < 91) || (value > 96 && value < 123);
    }
}
impl std::ops::Deref for TextItemUserIdentifierVisitor {
    type Target = Visitor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for TextItemUserIdentifierVisitor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::util::visitor::Visitor::VisitorDyn for TextItemUserIdentifierVisitor {
    fn as_Visitor(&self) -> &crate::org::allbinary::logic::util::visitor::Visitor::Visitor { &self.base }
    fn as_Visitor_mut(&mut self) -> &mut crate::org::allbinary::logic::util::visitor::Visitor::Visitor { &mut self.base }
}
impl crate::org::allbinary::logic::util::visitor::VisitorInterface::VisitorInterface for TextItemUserIdentifierVisitor {
    fn visit(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { Self::visit(self, anyType) }
}


