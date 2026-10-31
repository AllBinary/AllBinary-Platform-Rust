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
// package org.allbinary.log
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::logic::StdUtil::StdUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LOGGING {
    pub name: String,
}

pub static mut LOGGING_logging: std::cell::LazyCell<Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>> = std::cell::LazyCell::new(|| BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>));
thread_local! { pub static LOGGING_GRAPHICSPAINT: Option<Rc<RefCell<LOGGING>>> = LOGGING::new(String::from("Graphics Paint")); }
thread_local! { pub static LOGGING_GRAPHICSCREATION: Option<Rc<RefCell<LOGGING>>> = LOGGING::new(String::from("Graphics Creation")); }
thread_local! { pub static LOGGING_GRAPHICSDELETION: Option<Rc<RefCell<LOGGING>>> = LOGGING::new(String::from("Graphics Deletion")); }
thread_local! { pub static LOGGING_LOADINGERROR: Option<Rc<RefCell<LOGGING>>> = LOGGING::new(String::from("Loading Error")); }
thread_local! { pub static LOGGING_MENUEVENT: Option<Rc<RefCell<LOGGING>>> = LOGGING::new(String::from("Menu Action Event")); }
thread_local! { pub static LOGGING_MOUSEEVENT: Option<Rc<RefCell<LOGGING>>> = LOGGING::new(String::from("Mouse Action Event")); }
thread_local! { pub static LOGGING_KEYEVENT: Option<Rc<RefCell<LOGGING>>> = LOGGING::new(String::from("Keyboard Action Event")); }
thread_local! { pub static LOGGING_ACTIONEVENT: Option<Rc<RefCell<LOGGING>>> = LOGGING::new(String::from("Action Event")); }
thread_local! { pub static LOGGING_ADJUSTMENTEVENT: Option<Rc<RefCell<LOGGING>>> = LOGGING::new(String::from("Adjustment Action Event")); }
thread_local! { pub static LOGGING_COMPONENTEVENT: Option<Rc<RefCell<LOGGING>>> = LOGGING::new(String::from("Component Action Event")); }
thread_local! { pub static LOGGING_CONTAINEREVENT: Option<Rc<RefCell<LOGGING>>> = LOGGING::new(String::from("Container Action Event")); }
thread_local! { pub static LOGGING_FOCUSEVENT: Option<Rc<RefCell<LOGGING>>> = LOGGING::new(String::from("Focus Action Event")); }
thread_local! { pub static LOGGING_HIERARCHYEVENT: Option<Rc<RefCell<LOGGING>>> = LOGGING::new(String::from("Hierarchy Action Event")); }
thread_local! { pub static LOGGING_INPUTEVENT: Option<Rc<RefCell<LOGGING>>> = LOGGING::new(String::from("Input Action Event")); }
thread_local! { pub static LOGGING_INVOCATIONEVENT: Option<Rc<RefCell<LOGGING>>> = LOGGING::new(String::from("Invocation Action Event")); }
thread_local! { pub static LOGGING_ITEMEVENT: Option<Rc<RefCell<LOGGING>>> = LOGGING::new(String::from("Item Action Event")); }
thread_local! { pub static LOGGING_PAINTEVENT: Option<Rc<RefCell<LOGGING>>> = LOGGING::new(String::from("Paint Action Event")); }
thread_local! { pub static LOGGING_TEXTEVENT: Option<Rc<RefCell<LOGGING>>> = LOGGING::new(String::from("Text Action Event")); }
thread_local! { pub static LOGGING_WINDOWEVENT: Option<Rc<RefCell<LOGGING>>> = LOGGING::new(String::from("Window Action Event")); }

impl LOGGING {
    

    pub fn new_struct(mut value: String) -> Self {
        let mut this = Self {
            name: String::new(),
        };
        this.name = value.clone();
        jcall!(unsafe { LOGGING_logging.clone() }).add(crate::java_into_object(value.clone()));
        this
    }

    pub fn new(mut value: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(value))))
    }
    

    pub fn contains(mut logType: Option<Rc<RefCell<LOGGING>>>) -> bool {
        return jcall!(unsafe { LOGGING_logging.clone() }).contains(crate::java_into_object(jcall!(logType).toString()));
    }
    

    pub fn toString(&mut self) -> String {
        return self.name.clone();
    }
}

