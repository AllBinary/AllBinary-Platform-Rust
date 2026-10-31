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
// package org.allbinary.logic.java.help
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::net::URL::URL;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
// use crate::javax::help::HelpSet::HelpSet;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct JavaHelpSetNotifier {
}

pub static mut JavaHelpSetNotifier_vector: std::cell::LazyCell<Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>> = std::cell::LazyCell::new(|| BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>));

impl JavaHelpSetNotifier {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn get() -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return unsafe { JavaHelpSetNotifier_vector.clone() }.clone();
    }
    

    pub fn isNotified(mut helpSet: Option<Rc<RefCell<HelpSet>>>) -> bool {
        let mut isNotified: bool = false;
        let mut url: Option<Rc<RefCell<URL>>> = jcall!(helpSet).getHelpSetURL();
        let mut size: i32 = jcall!(unsafe { JavaHelpSetNotifier_vector.clone() }).size();
        let mut urlArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(unsafe { JavaHelpSetNotifier_vector.clone() }).toArray();
        for i in (0..size) {
            let mut nextURL: Option<Rc<RefCell<URL>>> = (urlArray[(i) as usize]).clone() /* as URL */;
            if (jcall!(url).toString().cmp(&jcall!(nextURL).toString()) as i32) == 0 {
                isNotified = true;
            }
        }
        jcall!(unsafe { JavaHelpSetNotifier_vector.clone() }).add(crate::java_into_object(url.clone()));
        return isNotified.clone();
    }
}

