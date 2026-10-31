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
// package org.allbinary.input
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::awt::event::KeyEvent::KeyEvent;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct KeyUtil {
}


impl KeyUtil {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn isNormallyDisplayed(mut keyCode: i32) -> bool {
        match keyCode {
            switch_value if switch_value == unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_SPACE.clone() } || switch_value == unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_COMMA.clone() } || switch_value == unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_MINUS.clone() } || switch_value == unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_PERIOD.clone() } || switch_value == unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_SLASH.clone() } || switch_value == unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_0.clone() } || switch_value == unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_1.clone() } || switch_value == unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_2.clone() } || switch_value == unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_3.clone() } || switch_value == unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_4.clone() } || switch_value == unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_5.clone() } || switch_value == unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_6.clone() } || switch_value == unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_7.clone() } || switch_value == unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_8.clone() } || switch_value == unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_9.clone() } || switch_value == unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_SEMICOLON.clone() } || switch_value == unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_EQUALS.clone() } || switch_value == unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_A.clone() } || switch_value == unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_B.clone() } || switch_value == unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_C.clone() } || switch_value == unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_D.clone() } || switch_value == unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_E.clone() } || switch_value == unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_F.clone() } || switch_value == unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_G.clone() } || switch_value == unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_H.clone() } || switch_value == unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_I.clone() } || switch_value == unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_J.clone() } || switch_value == unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_K.clone() } || switch_value == unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_L.clone() } || switch_value == unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_M.clone() } || switch_value == unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_N.clone() } || switch_value == unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_O.clone() } || switch_value == unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_P.clone() } || switch_value == unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_Q.clone() } || switch_value == unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_R.clone() } || switch_value == unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_S.clone() } || switch_value == unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_T.clone() } || switch_value == unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_U.clone() } || switch_value == unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_V.clone() } || switch_value == unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_W.clone() } || switch_value == unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_X.clone() } || switch_value == unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_Y.clone() } || switch_value == unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_Z.clone() } || switch_value == unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_OPEN_BRACKET.clone() } || switch_value == unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_BACK_SLASH.clone() } || switch_value == unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_CLOSE_BRACKET.clone() } || switch_value == unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_NUMPAD0.clone() } || switch_value == unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_NUMPAD1.clone() } || switch_value == unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_NUMPAD2.clone() } || switch_value == unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_NUMPAD3.clone() } || switch_value == unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_NUMPAD4.clone() } || switch_value == unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_NUMPAD5.clone() } || switch_value == unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_NUMPAD6.clone() } || switch_value == unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_NUMPAD7.clone() } || switch_value == unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_NUMPAD8.clone() } || switch_value == unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_NUMPAD9.clone() } || switch_value == unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_MULTIPLY.clone() } || switch_value == unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_ADD.clone() } || switch_value == unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_SEPARATER.clone() } || switch_value == unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_SUBTRACT.clone() } || switch_value == unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_DECIMAL.clone() } || switch_value == unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_DIVIDE.clone() } => {
                return true;
            }
            _ => {
                return false;
            }
        }
    }
}

