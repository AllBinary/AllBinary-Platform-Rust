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
// package org.allbinary.media.audio
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::sound::sampled::Control::Control;
use super::Type::Type;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NullJ2SEControl {
    pub base: Control,
}


impl NullJ2SEControl {
    

    pub fn new_struct(mut r#type: Option<Rc<RefCell<Type>>>) -> Self {
        let mut this = Self {
            base: Control::new_struct1(r#type.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut r#type: Option<Rc<RefCell<Type>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(r#type))))
    }
}
impl std::ops::Deref for NullJ2SEControl {
    type Target = Control;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for NullJ2SEControl {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



