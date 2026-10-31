// 
// AllBinary Open License Version 1 
// Copyright (c) 2025 AllBinary 
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
// package org.allbinary.logic.system.security.crypt.jcehelper
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::NullUtil::NullUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BaseSecretComposite {
}

thread_local! { pub static BaseSecretComposite_NULL_SECRET_COMPOSITE: Option<Rc<RefCell<dyn BaseSecretCompositeDyn>>> = BaseSecretComposite::new().map(|reference| reference as Rc<RefCell<dyn BaseSecretCompositeDyn>>); }

impl BaseSecretComposite {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn encrypt(&mut self, mut array: Vec<i8>) -> Vec<i8> {
        return jcall!(NullUtil::getInstance()).NULL_BYTE_ARRAY.clone();
    }
    

    // throws Exception
    pub fn decrypt(&mut self, mut array: Vec<i8>) -> Vec<i8> {
        return jcall!(NullUtil::getInstance()).NULL_BYTE_ARRAY.clone();
    }
}
pub trait BaseSecretCompositeDyn {
    fn as_BaseSecretComposite(&self) -> &BaseSecretComposite;
    fn as_BaseSecretComposite_mut(&mut self) -> &mut BaseSecretComposite;
    fn encrypt(&mut self, mut array: Vec<i8>) -> Vec<i8> { BaseSecretComposite::encrypt(self.as_BaseSecretComposite_mut(), array) }
    fn decrypt(&mut self, mut array: Vec<i8>) -> Vec<i8> { BaseSecretComposite::decrypt(self.as_BaseSecretComposite_mut(), array) }
}
impl BaseSecretCompositeDyn for BaseSecretComposite {
    fn as_BaseSecretComposite(&self) -> &BaseSecretComposite { self }
    fn as_BaseSecretComposite_mut(&mut self) -> &mut BaseSecretComposite { self }
}


