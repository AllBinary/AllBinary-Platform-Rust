// Generated Code Do Not Modify
// package org.allbinary.game.input
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::android::AndroidInfoFactory::AndroidInfoFactory;
use super::AndroidKeyFactory::AndroidKeyFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ActivityFractureControllerUtilFactory {
}


impl ActivityFractureControllerUtilFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn crate::org::allbinary::game::input::AndroidKeyFactory::AndroidKeyFactoryDyn>>> {
        let mut SDK_VERSION: i32 = jcall!(AndroidInfoFactory::getInstance()).getVersion();
        if SDK_VERSION <= 8 {
            return AndroidAPI1KeyFactory::AndroidAPI1KeyFactory::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::AndroidKeyFactory::AndroidKeyFactoryDyn>>);
        } else if SDK_VERSION <= 11 {
            return AndroidAPI9KeyFactory::AndroidAPI9KeyFactory::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::AndroidKeyFactory::AndroidKeyFactoryDyn>>);
        } else {
            return AndroidAPI12KeyFactory::AndroidAPI12KeyFactory::getAPI12Instance();
        }
    }
}

