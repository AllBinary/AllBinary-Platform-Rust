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
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use super::Input::Input;
use super::InputFactory::InputFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct J2MEGameKey {
    pub base: Input,
}


impl J2MEGameKey {
    

    pub fn new_struct(mut key: i32, mut keyName: String) -> Self {
        let mut this = Self {
            base: Input::new_struct2(key.clone(), keyName.clone()),
        };
        // super(...) is in the struct literal above
        let mut id: i32 = this.getId();
        this
    }

    pub fn new(mut key: i32, mut keyName: String) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(key, keyName))));
            if id >= 0 {
            let mut inputFactory: Option<Rc<RefCell<InputFactory>>> = InputFactory::getInstance();
            jcall!(inputFactory).add(id.clone(), (|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()));
        }
        this
    }
    

    pub fn toString(&mut self) -> String {
        return jcall!(StringMaker::new()).append(String::from("J2MEGameKey: ")).append(self.base.toString()).toString();
    }
    

    pub fn getString(mut keyCode: i32) -> String {
        let mut input: Option<Rc<RefCell<Input>>> = jcall!(InputFactory::getInstance()).getInstanceById(keyCode.clone());
        if input.is_some() {
            return jcall!(input).getName();
        } else {
            return jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        }
    }
}
impl std::ops::Deref for J2MEGameKey {
    type Target = Input;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for J2MEGameKey {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



