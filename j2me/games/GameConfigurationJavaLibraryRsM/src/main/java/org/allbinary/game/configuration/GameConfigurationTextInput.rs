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
// package org.allbinary.game.configuration
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::TextField::TextField;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::util::ABHashtable::ABHashtable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GameConfigurationTextInput {
    pub label: String,
    pub text: String,
}

pub static mut GameConfigurationTextInput_hashtable: std::cell::LazyCell<Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>> = std::cell::LazyCell::new(|| jcall!(StdUtil::getInstance()).createHashtable());
pub static mut GameConfigurationTextInput_SIMULATOR_IP: std::cell::LazyCell<Option<Rc<RefCell<GameConfigurationTextInput>>>> = std::cell::LazyCell::new(|| GameConfigurationTextInput::new(String::from("IP: "), String::from("192.168.1.3")));
pub static mut GameConfigurationTextInput_SIMULATOR_PORT: std::cell::LazyCell<Option<Rc<RefCell<GameConfigurationTextInput>>>> = std::cell::LazyCell::new(|| GameConfigurationTextInput::new(String::from("Port: "), String::from("8010")));

impl GameConfigurationTextInput {
    

    pub fn new_struct(mut label: String, mut text: String) -> Self {
        let mut this = Self {
            label: String::new(),
            text: String::new(),
        };
        this.label = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        this.text = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        this.setText(text.clone());
        this.setLabel(label.clone());
        this
    }

    pub fn new(mut label: String, mut text: String) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(label, text))));
            jcall!(unsafe { GameConfigurationTextInput_hashtable.clone() }).put(crate::java_into_object(this.getLabel()), (|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()));
        this
    }
    

    pub fn update(mut textField: Option<Rc<RefCell<TextField>>>) {
        let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = (jcall!(textField).getLabel()).clone() /* as Any */;
        let mut gameConfigurationTextInput: Option<Rc<RefCell<GameConfigurationTextInput>>> = { let java_object = (jcall!(unsafe { GameConfigurationTextInput_hashtable.clone() }).get(anyType.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<GameConfigurationTextInput>>>>().expect("ClassCastException").clone()) };
        jcall!(gameConfigurationTextInput).setText(jcall!(textField).getString());
    }
    

    pub fn setText(&mut self, mut text: String) {
        self.text = text.clone();
    }
    

    pub fn getText(&mut self) -> String {
        return self.text.clone();
    }
    

    pub fn setLabel(&mut self, mut label: String) {
        self.label = label.clone();
    }
    

    pub fn getLabel(&mut self) -> String {
        return self.label.clone();
    }
    

    pub fn getHashtable() -> Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> {
        return unsafe { GameConfigurationTextInput_hashtable.clone() }.clone();
    }
}

