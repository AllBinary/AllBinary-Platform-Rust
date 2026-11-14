// Generated Code Do Not Modify
// package org.allbinary.game.input
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::android::view::KeyEvent::KeyEvent;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use super::AndroidKeyFactory::AndroidKeyFactory;
use super::AndroidGameKey::AndroidGameKey;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AndroidAPI9KeyFactory {
    pub base: AndroidKeyFactory,
}

pub static mut AndroidAPI9KeyFactory_instance: std::cell::LazyCell<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = std::cell::LazyCell::new(|| { let java_field = jcall!(NullUtil::getInstance()).NULL_OBJECT.clone(); java_field }.clone());

impl AndroidAPI9KeyFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn AndroidAPI9KeyFactoryDyn>>> {
        if match (&unsafe { AndroidAPI9KeyFactory_instance.clone() }, &{ let java_field = jcall!(NullUtil::getInstance()).NULL_OBJECT.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            unsafe { *AndroidAPI9KeyFactory_instance = crate::java_into_object(AndroidAPI9KeyFactory::new()) };
        }
        return { let java_object = (unsafe { AndroidAPI9KeyFactory_instance.clone() }).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn AndroidAPI9KeyFactoryDyn>>>>().expect("ClassCastException").clone()) };
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: AndroidKeyFactory::new_struct(),
        };
        this.KEYCODE_BUTTON_L1 = { let java_value = (AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_BUTTON_L1.clone() }.clone(), String::from("BUTTON L1"))).clone(); java_value };
        this.KEYCODE_BUTTON_R1 = { let java_value = (AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_BUTTON_R1.clone() }.clone(), String::from("BUTTON R1"))).clone(); java_value };
        this.KEYCODE_BUTTON_THUMBR = { let java_value = (AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_BUTTON_THUMBR.clone() }.clone(), String::from("BUTTON THUMBR"))).clone(); java_value };
        this.KEYCODE_BUTTON_THUMBL = { let java_value = (AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_BUTTON_THUMBL.clone() }.clone(), String::from("BUTTON THUMBL"))).clone(); java_value };
        this.KEYCODE_BUTTON_START = { let java_value = (AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_BUTTON_START.clone() }.clone(), String::from("BUTTON START"))).clone(); java_value };
        this.KEYCODE_BUTTON_MODE = { let java_value = (AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_BUTTON_MODE.clone() }.clone(), String::from("BUTTON MODE"))).clone(); java_value };
        this.KEYCODE_BUTTON_B = { let java_value = (AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_BUTTON_B.clone() }.clone(), String::from("BUTTON B"))).clone(); java_value };
        this.KEYCODE_BUTTON_A = { let java_value = (AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_BUTTON_A.clone() }.clone(), String::from("BUTTON A"))).clone(); java_value };
        this.KEYCODE_BUTTON_X = { let java_value = (AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_BUTTON_X.clone() }.clone(), String::from("BUTTON X"))).clone(); java_value };
        this.KEYCODE_BUTTON_Y = { let java_value = (AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_BUTTON_Y.clone() }.clone(), String::from("BUTTON Y"))).clone(); java_value };
        this.KEYCODE_BUTTON_L2 = { let java_value = (AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_BUTTON_L2.clone() }.clone(), String::from("BUTTON L2"))).clone(); java_value };
        this.KEYCODE_BUTTON_R2 = { let java_value = (AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_BUTTON_R2.clone() }.clone(), String::from("BUTTON R2"))).clone(); java_value };
        this.KEYCODE_BUTTON_SELECT = { let java_value = (AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_BUTTON_SELECT.clone() }.clone(), String::from("BUTTON SELECT"))).clone(); java_value };
        this.KEYCODE_BUTTON_C = { let java_value = (AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_BUTTON_C.clone() }.clone(), String::from("BUTTON C"))).clone(); java_value };
        this.KEYCODE_BUTTON_Z = { let java_value = (AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_BUTTON_Z.clone() }.clone(), String::from("BUTTON Z"))).clone(); java_value };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}
impl std::ops::Deref for AndroidAPI9KeyFactory {
    type Target = AndroidKeyFactory;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AndroidAPI9KeyFactory {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait AndroidAPI9KeyFactoryDyn: crate::org::allbinary::game::input::AndroidKeyFactory::AndroidKeyFactoryDyn {
    fn as_AndroidAPI9KeyFactory(&self) -> &AndroidAPI9KeyFactory;
    fn as_AndroidAPI9KeyFactory_mut(&mut self) -> &mut AndroidAPI9KeyFactory;
}
impl AndroidAPI9KeyFactoryDyn for AndroidAPI9KeyFactory {
    fn as_AndroidAPI9KeyFactory(&self) -> &AndroidAPI9KeyFactory { self }
    fn as_AndroidAPI9KeyFactory_mut(&mut self) -> &mut AndroidAPI9KeyFactory { self }
}

impl crate::org::allbinary::game::input::AndroidKeyFactory::AndroidKeyFactoryDyn for AndroidAPI9KeyFactory {
    fn as_AndroidKeyFactory(&self) -> &crate::org::allbinary::game::input::AndroidKeyFactory::AndroidKeyFactory { &self.base }
    fn as_AndroidKeyFactory_mut(&mut self) -> &mut crate::org::allbinary::game::input::AndroidKeyFactory::AndroidKeyFactory { &mut self.base }
}
impl crate::org::allbinary::android::ActivityFractureUtil::ActivityFractureUtilDyn for AndroidAPI9KeyFactory {
    fn as_ActivityFractureUtil(&self) -> &crate::org::allbinary::android::ActivityFractureUtil::ActivityFractureUtil { &self.base.base }
    fn as_ActivityFractureUtil_mut(&mut self) -> &mut crate::org::allbinary::android::ActivityFractureUtil::ActivityFractureUtil { &mut self.base.base }
}


