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

pub static mut LOGGING_logging: std::cell::LazyCell<Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>> = std::cell::LazyCell::new(|| BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>));
pub static LOGGING_GRAPHICSPAINT: crate::JavaStatic<Option<Rc<RefCell<LOGGING>>>> = crate::JavaStatic::new(|| LOGGING::new(String::from("Graphics Paint")));
pub static LOGGING_GRAPHICSCREATION: crate::JavaStatic<Option<Rc<RefCell<LOGGING>>>> = crate::JavaStatic::new(|| LOGGING::new(String::from("Graphics Creation")));
pub static LOGGING_GRAPHICSDELETION: crate::JavaStatic<Option<Rc<RefCell<LOGGING>>>> = crate::JavaStatic::new(|| LOGGING::new(String::from("Graphics Deletion")));
pub static LOGGING_LOADINGERROR: crate::JavaStatic<Option<Rc<RefCell<LOGGING>>>> = crate::JavaStatic::new(|| LOGGING::new(String::from("Loading Error")));
pub static LOGGING_MENUEVENT: crate::JavaStatic<Option<Rc<RefCell<LOGGING>>>> = crate::JavaStatic::new(|| LOGGING::new(String::from("Menu Action Event")));
pub static LOGGING_MOUSEEVENT: crate::JavaStatic<Option<Rc<RefCell<LOGGING>>>> = crate::JavaStatic::new(|| LOGGING::new(String::from("Mouse Action Event")));
pub static LOGGING_KEYEVENT: crate::JavaStatic<Option<Rc<RefCell<LOGGING>>>> = crate::JavaStatic::new(|| LOGGING::new(String::from("Keyboard Action Event")));
pub static LOGGING_ACTIONEVENT: crate::JavaStatic<Option<Rc<RefCell<LOGGING>>>> = crate::JavaStatic::new(|| LOGGING::new(String::from("Action Event")));
pub static LOGGING_ADJUSTMENTEVENT: crate::JavaStatic<Option<Rc<RefCell<LOGGING>>>> = crate::JavaStatic::new(|| LOGGING::new(String::from("Adjustment Action Event")));
pub static LOGGING_COMPONENTEVENT: crate::JavaStatic<Option<Rc<RefCell<LOGGING>>>> = crate::JavaStatic::new(|| LOGGING::new(String::from("Component Action Event")));
pub static LOGGING_CONTAINEREVENT: crate::JavaStatic<Option<Rc<RefCell<LOGGING>>>> = crate::JavaStatic::new(|| LOGGING::new(String::from("Container Action Event")));
pub static LOGGING_FOCUSEVENT: crate::JavaStatic<Option<Rc<RefCell<LOGGING>>>> = crate::JavaStatic::new(|| LOGGING::new(String::from("Focus Action Event")));
pub static LOGGING_HIERARCHYEVENT: crate::JavaStatic<Option<Rc<RefCell<LOGGING>>>> = crate::JavaStatic::new(|| LOGGING::new(String::from("Hierarchy Action Event")));
pub static LOGGING_INPUTEVENT: crate::JavaStatic<Option<Rc<RefCell<LOGGING>>>> = crate::JavaStatic::new(|| LOGGING::new(String::from("Input Action Event")));
pub static LOGGING_INVOCATIONEVENT: crate::JavaStatic<Option<Rc<RefCell<LOGGING>>>> = crate::JavaStatic::new(|| LOGGING::new(String::from("Invocation Action Event")));
pub static LOGGING_ITEMEVENT: crate::JavaStatic<Option<Rc<RefCell<LOGGING>>>> = crate::JavaStatic::new(|| LOGGING::new(String::from("Item Action Event")));
pub static LOGGING_PAINTEVENT: crate::JavaStatic<Option<Rc<RefCell<LOGGING>>>> = crate::JavaStatic::new(|| LOGGING::new(String::from("Paint Action Event")));
pub static LOGGING_TEXTEVENT: crate::JavaStatic<Option<Rc<RefCell<LOGGING>>>> = crate::JavaStatic::new(|| LOGGING::new(String::from("Text Action Event")));
pub static LOGGING_WINDOWEVENT: crate::JavaStatic<Option<Rc<RefCell<LOGGING>>>> = crate::JavaStatic::new(|| LOGGING::new(String::from("Window Action Event")));

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

