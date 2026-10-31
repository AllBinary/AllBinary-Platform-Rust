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
// package org.allbinary.game.input
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use super::Input::Input;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PCGameKey {
    pub base: Input,
}


impl PCGameKey {
    

    pub fn new_struct(mut keyCode: i32, mut name: String) -> Self {
        let mut this = Self {
            base: Input::new_struct2(keyCode.clone(), name.clone()),
        };
        // super(...) is in the struct literal above
        let mut id: i32 = this.getId();
        this
    }

    pub fn new(mut keyCode: i32, mut name: String) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(keyCode, name))));
            if id >= 0 {
            jcall!(InputFactory::InputFactory::getInstance()).add(id.clone(), (|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()));
        }
        this
    }
    

    pub fn toString(&mut self) -> String {
        return format!("{}{}", String::from("PCGameKey: "), self.base.toString());
    }
    

    pub fn getString(mut keyCode: i32) -> String {
        let mut input: Option<Rc<RefCell<Input>>> = jcall!(InputFactory::InputFactory::getInstance()).getInstanceById(keyCode.clone());
        if input.is_some() {
            return jcall!(input).getName();
        } else {
            return jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        }
    }
}
impl std::ops::Deref for PCGameKey {
    type Target = Input;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for PCGameKey {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



