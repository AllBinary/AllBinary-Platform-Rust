// Generated Code Do Not Modify
// package org.allbinary.game.input
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::android::view::KeyEvent::KeyEvent;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use super::AndroidAPI9KeyFactory::AndroidAPI9KeyFactory;
use super::AndroidGameKey::AndroidGameKey;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AndroidAPI12KeyFactory {
    pub base: AndroidAPI9KeyFactory,
}

pub static mut AndroidAPI12KeyFactory_instance12: std::cell::LazyCell<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = std::cell::LazyCell::new(|| jcall!(NullUtil::getInstance()).NULL_OBJECT.clone());

impl AndroidAPI12KeyFactory {
    

    pub fn getAPI12Instance() -> Option<Rc<RefCell<AndroidAPI12KeyFactory>>> {
        if match (&unsafe { AndroidAPI12KeyFactory_instance12.clone() }, &jcall!(NullUtil::getInstance()).NULL_OBJECT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            unsafe { *AndroidAPI12KeyFactory_instance12 = AndroidAPI12KeyFactory::new() };
        }
        return (unsafe { AndroidAPI12KeyFactory_instance12.clone() }).clone() /* as AndroidAPI12KeyFactory */;
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: AndroidAPI9KeyFactory::new_struct(),
        };
        this.KEYCODE_BUTTON_1 = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_BUTTON_1.clone() }.clone(), String::from("BUTTON 1"));
        this.KEYCODE_BUTTON_10 = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_BUTTON_10.clone() }.clone(), String::from("BUTTON 10"));
        this.KEYCODE_BUTTON_11 = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_BUTTON_11.clone() }.clone(), String::from("BUTTON 11"));
        this.KEYCODE_BUTTON_12 = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_BUTTON_12.clone() }.clone(), String::from("BUTTON 12"));
        this.KEYCODE_BUTTON_13 = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_BUTTON_13.clone() }.clone(), String::from("BUTTON 13"));
        this.KEYCODE_BUTTON_14 = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_BUTTON_14.clone() }.clone(), String::from("BUTTON 14"));
        this.KEYCODE_BUTTON_15 = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_BUTTON_15.clone() }.clone(), String::from("BUTTON 15"));
        this.KEYCODE_BUTTON_16 = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_BUTTON_16.clone() }.clone(), String::from("BUTTON 16"));
        this.KEYCODE_BUTTON_2 = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_BUTTON_2.clone() }.clone(), String::from("BUTTON 2"));
        this.KEYCODE_BUTTON_3 = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_BUTTON_3.clone() }.clone(), String::from("BUTTON 3"));
        this.KEYCODE_BUTTON_4 = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_BUTTON_4.clone() }.clone(), String::from("BUTTON 4"));
        this.KEYCODE_BUTTON_5 = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_BUTTON_5.clone() }.clone(), String::from("BUTTON 5"));
        this.KEYCODE_BUTTON_6 = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_BUTTON_6.clone() }.clone(), String::from("BUTTON 6"));
        this.KEYCODE_BUTTON_7 = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_BUTTON_7.clone() }.clone(), String::from("BUTTON 7"));
        this.KEYCODE_BUTTON_8 = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_BUTTON_8.clone() }.clone(), String::from("BUTTON 8"));
        this.KEYCODE_BUTTON_9 = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_BUTTON_9.clone() }.clone(), String::from("BUTTON 9"));
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}
impl std::ops::Deref for AndroidAPI12KeyFactory {
    type Target = AndroidAPI9KeyFactory;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AndroidAPI12KeyFactory {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::input::AndroidAPI9KeyFactory::AndroidAPI9KeyFactoryDyn for AndroidAPI12KeyFactory {
    fn as_AndroidAPI9KeyFactory(&self) -> &crate::org::allbinary::game::input::AndroidAPI9KeyFactory::AndroidAPI9KeyFactory { &self.base }
    fn as_AndroidAPI9KeyFactory_mut(&mut self) -> &mut crate::org::allbinary::game::input::AndroidAPI9KeyFactory::AndroidAPI9KeyFactory { &mut self.base }
}
impl crate::org::allbinary::game::input::AndroidKeyFactory::AndroidKeyFactoryDyn for AndroidAPI12KeyFactory {
    fn as_AndroidKeyFactory(&self) -> &crate::org::allbinary::game::input::AndroidKeyFactory::AndroidKeyFactory { &self.base.base }
    fn as_AndroidKeyFactory_mut(&mut self) -> &mut crate::org::allbinary::game::input::AndroidKeyFactory::AndroidKeyFactory { &mut self.base.base }
}


