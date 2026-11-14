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

pub static mut AndroidAPI12KeyFactory_instance12: std::cell::LazyCell<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = std::cell::LazyCell::new(|| { let java_field = jcall!(NullUtil::getInstance()).NULL_OBJECT.clone(); java_field }.clone());

impl AndroidAPI12KeyFactory {
    

    pub fn getAPI12Instance() -> Option<Rc<RefCell<AndroidAPI12KeyFactory>>> {
        if match (&unsafe { AndroidAPI12KeyFactory_instance12.clone() }, &{ let java_field = jcall!(NullUtil::getInstance()).NULL_OBJECT.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            unsafe { *AndroidAPI12KeyFactory_instance12 = crate::java_into_object(AndroidAPI12KeyFactory::new()) };
        }
        return { let java_object = (unsafe { AndroidAPI12KeyFactory_instance12.clone() }).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<AndroidAPI12KeyFactory>>>>().expect("ClassCastException").clone()) };
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: AndroidAPI9KeyFactory::new_struct(),
        };
        this.KEYCODE_BUTTON_1 = { let java_value = (AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_BUTTON_1.clone() }.clone(), String::from("BUTTON 1"))).clone(); java_value };
        this.KEYCODE_BUTTON_10 = { let java_value = (AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_BUTTON_10.clone() }.clone(), String::from("BUTTON 10"))).clone(); java_value };
        this.KEYCODE_BUTTON_11 = { let java_value = (AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_BUTTON_11.clone() }.clone(), String::from("BUTTON 11"))).clone(); java_value };
        this.KEYCODE_BUTTON_12 = { let java_value = (AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_BUTTON_12.clone() }.clone(), String::from("BUTTON 12"))).clone(); java_value };
        this.KEYCODE_BUTTON_13 = { let java_value = (AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_BUTTON_13.clone() }.clone(), String::from("BUTTON 13"))).clone(); java_value };
        this.KEYCODE_BUTTON_14 = { let java_value = (AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_BUTTON_14.clone() }.clone(), String::from("BUTTON 14"))).clone(); java_value };
        this.KEYCODE_BUTTON_15 = { let java_value = (AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_BUTTON_15.clone() }.clone(), String::from("BUTTON 15"))).clone(); java_value };
        this.KEYCODE_BUTTON_16 = { let java_value = (AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_BUTTON_16.clone() }.clone(), String::from("BUTTON 16"))).clone(); java_value };
        this.KEYCODE_BUTTON_2 = { let java_value = (AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_BUTTON_2.clone() }.clone(), String::from("BUTTON 2"))).clone(); java_value };
        this.KEYCODE_BUTTON_3 = { let java_value = (AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_BUTTON_3.clone() }.clone(), String::from("BUTTON 3"))).clone(); java_value };
        this.KEYCODE_BUTTON_4 = { let java_value = (AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_BUTTON_4.clone() }.clone(), String::from("BUTTON 4"))).clone(); java_value };
        this.KEYCODE_BUTTON_5 = { let java_value = (AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_BUTTON_5.clone() }.clone(), String::from("BUTTON 5"))).clone(); java_value };
        this.KEYCODE_BUTTON_6 = { let java_value = (AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_BUTTON_6.clone() }.clone(), String::from("BUTTON 6"))).clone(); java_value };
        this.KEYCODE_BUTTON_7 = { let java_value = (AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_BUTTON_7.clone() }.clone(), String::from("BUTTON 7"))).clone(); java_value };
        this.KEYCODE_BUTTON_8 = { let java_value = (AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_BUTTON_8.clone() }.clone(), String::from("BUTTON 8"))).clone(); java_value };
        this.KEYCODE_BUTTON_9 = { let java_value = (AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_BUTTON_9.clone() }.clone(), String::from("BUTTON 9"))).clone(); java_value };
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
impl crate::org::allbinary::android::ActivityFractureUtil::ActivityFractureUtilDyn for AndroidAPI12KeyFactory {
    fn as_ActivityFractureUtil(&self) -> &crate::org::allbinary::android::ActivityFractureUtil::ActivityFractureUtil { &self.base.base.base }
    fn as_ActivityFractureUtil_mut(&mut self) -> &mut crate::org::allbinary::android::ActivityFractureUtil::ActivityFractureUtil { &mut self.base.base.base }
}


