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
// package org.allbinary.game.configuration
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::CommandListener::CommandListener;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandForm;
use crate::org::allbinary::init::Init::Init;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::InGameOptionsForm::InGameOptionsForm;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InGameOptionsFormFactory {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub SINGLETON: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandFormDyn>>>,
}

pub static mut InGameOptionsFormFactory_instance: std::cell::LazyCell<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = std::cell::LazyCell::new(|| jcall!(NullUtil::getInstance()).NULL_OBJECT.clone());

impl InGameOptionsFormFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            SINGLETON: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.SINGLETON = CommandForm::getNullCommandForm();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<InGameOptionsFormFactory>>> {
        if match (&unsafe { InGameOptionsFormFactory_instance.clone() }, &jcall!(NullUtil::getInstance()).NULL_OBJECT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            unsafe { *InGameOptionsFormFactory_instance = InGameOptionsFormFactory::new() };
        }
        return (unsafe { InGameOptionsFormFactory_instance.clone() }).clone() /* as InGameOptionsFormFactory */;
    }
    

    pub fn init(&mut self, mut commandListener: Option<Rc<RefCell<CommandListener>>>, mut initInterface: Option<Rc<RefCell<dyn crate::org::allbinary::init::Init::InitDyn>>>, mut title: String, mut backgrounBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if match (&self.SINGLETON, &CommandForm::getNullCommandForm()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                jcall!(initInterface).init();
                self.SINGLETON = InGameOptionsForm::new(commandListener.clone(), title.clone(), backgrounBasicColor.clone(), foregroundBasicColor.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandFormDyn>>);
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(self.logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).INIT.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn get(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandFormDyn>>> {
        return self.SINGLETON.clone();
    }
}

