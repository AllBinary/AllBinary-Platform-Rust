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
// package org.allbinary.input.motion.button
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::input::InputFactory::InputFactory;
use crate::org::allbinary::game::input::mapping::InputToGameKeyMapping::InputToGameKeyMapping;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use super::TouchButtonInput::TouchButtonInput;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicTouchInputFactory {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub SPECIAL_BUTTON_SIX: Option<Rc<RefCell<TouchButtonInput>>>,
    pub SPECIAL_BUTTON_FIVE: Option<Rc<RefCell<TouchButtonInput>>>,
    pub SPECIAL_BUTTON_FOUR: Option<Rc<RefCell<TouchButtonInput>>>,
    pub SPECIAL_BUTTON_THREE: Option<Rc<RefCell<TouchButtonInput>>>,
    pub SPECIAL_BUTTON_TWO: Option<Rc<RefCell<TouchButtonInput>>>,
    pub SPECIAL_BUTTON_ONE: Option<Rc<RefCell<TouchButtonInput>>>,
    pub UP: Option<Rc<RefCell<TouchButtonInput>>>,
    pub LEFT: Option<Rc<RefCell<TouchButtonInput>>>,
    pub RIGHT: Option<Rc<RefCell<TouchButtonInput>>>,
    pub DOWN: Option<Rc<RefCell<TouchButtonInput>>>,
    pub NONE: Option<Rc<RefCell<TouchButtonInput>>>,
    pub initialized: bool,
    pub list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}

pub static mut BasicTouchInputFactory_instance: std::cell::LazyCell<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = std::cell::LazyCell::new(|| jcall!(NullUtil::getInstance()).NULL_OBJECT.clone());

impl BasicTouchInputFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            SPECIAL_BUTTON_SIX: None,
            SPECIAL_BUTTON_FIVE: None,
            SPECIAL_BUTTON_FOUR: None,
            SPECIAL_BUTTON_THREE: None,
            SPECIAL_BUTTON_TWO: None,
            SPECIAL_BUTTON_ONE: None,
            UP: None,
            LEFT: None,
            RIGHT: None,
            DOWN: None,
            NONE: None,
            initialized: false,
            list: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.SPECIAL_BUTTON_SIX = TouchButtonInput::new(jcall!(InputFactory::getInstance()).MAX - 30, String::from("Button 6"));
        this.SPECIAL_BUTTON_FIVE = TouchButtonInput::new(jcall!(InputFactory::getInstance()).MAX - 31, String::from("Button 5"));
        this.SPECIAL_BUTTON_FOUR = TouchButtonInput::new(jcall!(InputFactory::getInstance()).MAX - 32, String::from("Button 4"));
        this.SPECIAL_BUTTON_THREE = TouchButtonInput::new(jcall!(InputFactory::getInstance()).MAX - 33, String::from("Button 3"));
        this.SPECIAL_BUTTON_TWO = TouchButtonInput::new(jcall!(InputFactory::getInstance()).MAX - 34, String::from("Button 2"));
        this.SPECIAL_BUTTON_ONE = TouchButtonInput::new(jcall!(InputFactory::getInstance()).MAX - 35, String::from("Button 1"));
        this.UP = TouchButtonInput::new(jcall!(InputFactory::getInstance()).MAX - 36, String::from("Up Button"));
        this.LEFT = TouchButtonInput::new(jcall!(InputFactory::getInstance()).MAX - 37, String::from("Left Button"));
        this.RIGHT = TouchButtonInput::new(jcall!(InputFactory::getInstance()).MAX - 38, String::from("Right Button"));
        this.DOWN = TouchButtonInput::new(jcall!(InputFactory::getInstance()).MAX - 39, String::from("Down Button"));
        this.NONE = TouchButtonInput::new(jcall!(InputFactory::getInstance()).MAX - 40, String::from("No Button"));
        this.initialized = false;
        this.list = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<BasicTouchInputFactory>>> {
        if match (&unsafe { BasicTouchInputFactory_instance.clone() }, &jcall!(NullUtil::getInstance()).NULL_OBJECT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            unsafe { *BasicTouchInputFactory_instance = BasicTouchInputFactory::new() };
        }
        return (unsafe { BasicTouchInputFactory_instance.clone() }).clone() /* as BasicTouchInputFactory */;
    }
    

    // synchronized
    pub fn init(&mut self, mut inputToGameKeyMapping: Option<Rc<RefCell<InputToGameKeyMapping>>>) {
        if !self.initialized {
            self.initialized = true;
            jcall!(self.list).add(crate::java_into_object(self.UP.clone()));
            jcall!(self.list).add(crate::java_into_object(self.LEFT.clone()));
            jcall!(self.list).add(crate::java_into_object(self.RIGHT.clone()));
            jcall!(self.list).add(crate::java_into_object(self.DOWN.clone()));
            jcall!(self.list).add(crate::java_into_object(self.SPECIAL_BUTTON_TWO.clone()));
            jcall!(self.list).add(crate::java_into_object(self.SPECIAL_BUTTON_FOUR.clone()));
            jcall!(self.list).add(crate::java_into_object(self.SPECIAL_BUTTON_THREE.clone()));
            jcall!(self.list).add(crate::java_into_object(self.SPECIAL_BUTTON_ONE.clone()));
            jcall!(self.list).add(crate::java_into_object(self.SPECIAL_BUTTON_FIVE.clone()));
            jcall!(self.list).add(crate::java_into_object(self.SPECIAL_BUTTON_SIX.clone()));
            self.updateAllList(self.list.clone(), inputToGameKeyMapping.clone());
        }
    }
    

    pub fn updateAll(&mut self, mut inputToGameKeyMapping: Option<Rc<RefCell<InputToGameKeyMapping>>>) {
        { let java_arguments = (self.getList(), inputToGameKeyMapping.clone(),); self.updateAllList(java_arguments.0, java_arguments.1) };
    }
    

    pub fn updateAllList(&mut self, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut inputToGameKeyMapping: Option<Rc<RefCell<InputToGameKeyMapping>>>) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(CommonLabels::getInstance()).START.clone()).appendint(jcall!(list).size()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("updateAll"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        for index in (0..=jcall!(list).size() - 1).rev() {
            let mut touchButtonInput: Option<Rc<RefCell<TouchButtonInput>>> = { let java_object = (jcall!(list).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<TouchButtonInput>>>>().expect("ClassCastException").clone()) };
            jcall!(touchButtonInput).update(inputToGameKeyMapping.clone());
        }
    }
    

    pub fn getList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.list.clone();
    }
}

