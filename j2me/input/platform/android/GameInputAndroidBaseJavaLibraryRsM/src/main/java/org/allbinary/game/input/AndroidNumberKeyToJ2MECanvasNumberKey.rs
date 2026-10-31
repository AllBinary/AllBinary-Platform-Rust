// Generated Code Do Not Modify
// package org.allbinary.game.input
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Canvas::Canvas;
use crate::android::view::KeyEvent::KeyEvent;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AndroidNumberKeyToJ2MECanvasNumberKey {
}

thread_local! { pub static AndroidNumberKeyToJ2MECanvasNumberKey_androidtoJavaMicroEditionKeyMap: Vec<i32> = vec![0; (17) as usize]; }

impl AndroidNumberKeyToJ2MECanvasNumberKey {
    

    pub fn init() {
        AndroidNumberKeyToJ2MECanvasNumberKey_androidtoJavaMicroEditionKeyMap.with(Clone::clone)[(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_0.clone() }) as usize] = unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM0.clone() }.clone();
        AndroidNumberKeyToJ2MECanvasNumberKey_androidtoJavaMicroEditionKeyMap.with(Clone::clone)[(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_1.clone() }) as usize] = unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM1.clone() }.clone();
        AndroidNumberKeyToJ2MECanvasNumberKey_androidtoJavaMicroEditionKeyMap.with(Clone::clone)[(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_2.clone() }) as usize] = unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM2.clone() }.clone();
        AndroidNumberKeyToJ2MECanvasNumberKey_androidtoJavaMicroEditionKeyMap.with(Clone::clone)[(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_3.clone() }) as usize] = unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM3.clone() }.clone();
        AndroidNumberKeyToJ2MECanvasNumberKey_androidtoJavaMicroEditionKeyMap.with(Clone::clone)[(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_4.clone() }) as usize] = unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM4.clone() }.clone();
        AndroidNumberKeyToJ2MECanvasNumberKey_androidtoJavaMicroEditionKeyMap.with(Clone::clone)[(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_5.clone() }) as usize] = unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM5.clone() }.clone();
        AndroidNumberKeyToJ2MECanvasNumberKey_androidtoJavaMicroEditionKeyMap.with(Clone::clone)[(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_6.clone() }) as usize] = unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM6.clone() }.clone();
        AndroidNumberKeyToJ2MECanvasNumberKey_androidtoJavaMicroEditionKeyMap.with(Clone::clone)[(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_7.clone() }) as usize] = unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM7.clone() }.clone();
        AndroidNumberKeyToJ2MECanvasNumberKey_androidtoJavaMicroEditionKeyMap.with(Clone::clone)[(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_8.clone() }) as usize] = unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM8.clone() }.clone();
        AndroidNumberKeyToJ2MECanvasNumberKey_androidtoJavaMicroEditionKeyMap.with(Clone::clone)[(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_9.clone() }) as usize] = unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM9.clone() }.clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getKey(mut key: i32) -> i32 {
        if key < (AndroidNumberKeyToJ2MECanvasNumberKey_androidtoJavaMicroEditionKeyMap.with(Clone::clone).len() as i32) {
            let mut value: i32 = AndroidNumberKeyToJ2MECanvasNumberKey_androidtoJavaMicroEditionKeyMap.with(Clone::clone)[(key) as usize].clone();
            if value != 0 {
                return value.clone();
            }
        }
        return key.clone();
    }
}

