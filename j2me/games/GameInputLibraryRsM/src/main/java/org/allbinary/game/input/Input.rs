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

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct Input {
    pub id: i32,
    pub name: String,
}


impl Input {
    

    pub fn new_struct(mut id: i32, mut name: String) -> Self {
        let mut this = Self {
            id: 0,
            name: String::new(),
        };
        this.name = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        this.setId(id.clone());
        this.setName(name.clone());
        this
    }

    pub fn new(mut id: i32, mut name: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(id, name))))
    }
    

    pub fn getId(&mut self) -> i32 {
        return self.id.clone();
    }
    

    pub fn setId(&mut self, mut key: i32) {
        self.id = key.clone();
    }
    

    pub fn getName(&mut self) -> String {
        return self.name.clone();
    }
    

    pub fn setName(&mut self, mut keyName: String) {
        self.name = keyName.clone();
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(String::from("Name: "));
        jcall!(stringBuffer).append(self.getName());
        jcall!(stringBuffer).append(String::from(" Id: "));
        jcall!(stringBuffer).appendint(self.getId());
        return jcall!(stringBuffer).toString();
    }
}
pub trait InputDyn {
    fn as_Input(&self) -> &Input;
    fn as_Input_mut(&mut self) -> &mut Input;
    fn getId(&mut self) -> i32 { Input::getId(self.as_Input_mut()) }
    fn setId(&mut self, mut key: i32) { Input::setId(self.as_Input_mut(), key) }
    fn getName(&mut self) -> String { Input::getName(self.as_Input_mut()) }
    fn setName(&mut self, mut keyName: String) { Input::setName(self.as_Input_mut(), keyName) }
    fn toString(&mut self) -> String { Input::toString(self.as_Input_mut()) }
}
impl InputDyn for Input {
    fn as_Input(&self) -> &Input { self }
    fn as_Input_mut(&mut self) -> &mut Input { self }
}


