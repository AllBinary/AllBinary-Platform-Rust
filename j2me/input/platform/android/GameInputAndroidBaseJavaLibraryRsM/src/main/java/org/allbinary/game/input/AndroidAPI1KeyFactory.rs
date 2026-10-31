// Generated Code Do Not Modify
// package org.allbinary.game.input
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use super::AndroidKeyFactory::AndroidKeyFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AndroidAPI1KeyFactory {
    pub base: AndroidKeyFactory,
}

pub static mut AndroidAPI1KeyFactory_instance: std::cell::LazyCell<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = std::cell::LazyCell::new(|| jcall!(NullUtil::getInstance()).NULL_OBJECT.clone());

impl AndroidAPI1KeyFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<AndroidAPI1KeyFactory>>> {
        if match (&unsafe { AndroidAPI1KeyFactory_instance.clone() }, &jcall!(NullUtil::getInstance()).NULL_OBJECT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            unsafe { *AndroidAPI1KeyFactory_instance = AndroidAPI1KeyFactory::new() };
        }
        return (unsafe { AndroidAPI1KeyFactory_instance.clone() }).clone() /* as AndroidAPI1KeyFactory */;
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: AndroidKeyFactory::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}
impl std::ops::Deref for AndroidAPI1KeyFactory {
    type Target = AndroidKeyFactory;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AndroidAPI1KeyFactory {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::input::AndroidKeyFactory::AndroidKeyFactoryDyn for AndroidAPI1KeyFactory {
    fn as_AndroidKeyFactory(&self) -> &crate::org::allbinary::game::input::AndroidKeyFactory::AndroidKeyFactory { &self.base }
    fn as_AndroidKeyFactory_mut(&mut self) -> &mut crate::org::allbinary::game::input::AndroidKeyFactory::AndroidKeyFactory { &mut self.base }
}


