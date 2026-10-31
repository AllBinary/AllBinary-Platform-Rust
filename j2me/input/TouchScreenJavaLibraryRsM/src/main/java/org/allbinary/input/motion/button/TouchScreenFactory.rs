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
// package org.allbinary.input.motion.button
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::TouchScreenType::TouchScreenType;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TouchScreenFactory {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub touch: bool,
    pub multiTouch: bool,
    pub multiTouchDistinct: bool,
    pub touchScreenType: Option<Rc<RefCell<TouchScreenType>>>,
}

thread_local! { pub static TouchScreenFactory_SINGLETON: Option<Rc<RefCell<TouchScreenFactory>>> = TouchScreenFactory::new(); }

impl TouchScreenFactory {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            touch: false,
            multiTouch: false,
            multiTouchDistinct: false,
            touchScreenType: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.touchScreenType = jcall!(TouchScreenTypesFactory::TouchScreenTypesFactory::getInstance()).NOTOUCH.clone();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            this.setTouch(true);
            this.setMultiTouch(false);
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(this.logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(TouchScreenFactory_SINGLETON.with(Clone::clone).clone()), jcall!(commonStrings).GET_INSTANCE.clone(), crate::java_into_object(e.clone()));
            }
        }
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<TouchScreenFactory>>> {
        return TouchScreenFactory_SINGLETON.with(Clone::clone).clone();
    }
    

    pub fn setMultiTouch(&mut self, mut multiTouch: bool) {
        self.multiTouch = multiTouch.clone();
    }
    

    pub fn isMultiTouch(&mut self) -> bool {
        return self.multiTouch.clone();
    }
    

    pub fn setTouch(&mut self, mut touch: bool) {
        self.touch = touch.clone();
    }
    

    pub fn isTouch(&mut self) -> bool {
        return self.touch.clone();
    }
    

    pub fn setMultiTouchDistinct(&mut self, mut multiTouchDistinct: bool) {
        self.multiTouchDistinct = multiTouchDistinct.clone();
    }
    

    pub fn isMultiTouchDistinct(&mut self) -> bool {
        return self.multiTouchDistinct.clone();
    }
    

    pub fn setTouchScreenType(&mut self, mut touchScreenType: Option<Rc<RefCell<TouchScreenType>>>) {
        self.touchScreenType = touchScreenType.clone();
    }
    

    pub fn getTouchScreenType(&mut self) -> Option<Rc<RefCell<TouchScreenType>>> {
        return self.touchScreenType.clone();
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(String::from("TouchScreen: "));
        jcall!(stringBuffer).appendboolean(self.isTouch());
        jcall!(stringBuffer).append(String::from(" MultiTouch: "));
        jcall!(stringBuffer).appendboolean(self.isMultiTouch());
        jcall!(stringBuffer).append(String::from(" Distinct MultiTouch: "));
        jcall!(stringBuffer).appendboolean(self.isMultiTouchDistinct());
        jcall!(stringBuffer).append(String::from(" TouchScreenType: "));
        jcall!(stringBuffer).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(self.getTouchScreenType())));
        return jcall!(stringBuffer).toString();
    }
}

