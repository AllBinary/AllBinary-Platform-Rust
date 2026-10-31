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
use crate::org::allbinary::graphics::GPoint::GPoint;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use super::CurrentlyPressedTouchButtonSingleton::CurrentlyPressedTouchButtonSingleton;
use super::TouchButtonInput::TouchButtonInput;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CurrentlyPressedTouchButtonSingletonDebug {
    pub base: CurrentlyPressedTouchButtonSingleton,
    pub string: String,
    pub listString: String,
    pub append: String,
    pub PRESSED_AND_FIRED: String,
    pub PRESSED_AND_NOT_FIRED: String,
    pub ASSOCIATED_RELEASED_AND_FIRED: String,
    pub RELEASED_AND_FIRED: String,
    pub RELEASED_AND_FIRED_2: String,
    pub RELEASED_AND_NOT_FIRED: String,
}

thread_local! { pub static CurrentlyPressedTouchButtonSingletonDebug_instanceC: Option<Rc<RefCell<CurrentlyPressedTouchButtonSingletonDebug>>> = CurrentlyPressedTouchButtonSingletonDebug::new(); }

impl CurrentlyPressedTouchButtonSingletonDebug {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: CurrentlyPressedTouchButtonSingleton::new_struct(),
            string: String::new(),
            listString: String::new(),
            append: String::new(),
            PRESSED_AND_FIRED: String::new(),
            PRESSED_AND_NOT_FIRED: String::new(),
            ASSOCIATED_RELEASED_AND_FIRED: String::new(),
            RELEASED_AND_FIRED: String::new(),
            RELEASED_AND_FIRED_2: String::new(),
            RELEASED_AND_NOT_FIRED: String::new(),
        };
        this.string = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        this.listString = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        this.append = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        this.PRESSED_AND_FIRED = String::from("pressed & fired");
        this.PRESSED_AND_NOT_FIRED = String::from("pressed in button but not fired");
        this.ASSOCIATED_RELEASED_AND_FIRED = String::from("assoc. rel & fired");
        this.RELEASED_AND_FIRED = String::from("rel & fired 1");
        this.RELEASED_AND_FIRED_2 = String::from("rel & fired 2");
        this.RELEASED_AND_NOT_FIRED = String::from("rel & not fired");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn crate::org::allbinary::input::motion::button::CurrentlyPressedTouchButtonSingleton::CurrentlyPressedTouchButtonSingletonDyn>>> {
        return CurrentlyPressedTouchButtonSingletonDebug_instanceC.with(Clone::clone).clone();
    }
    

    pub fn appendTouchButtonInput(&mut self, mut append: String, mut touchButtonInput: Option<Rc<RefCell<TouchButtonInput>>>) {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(jcall!(touchButtonInput).toString());
        jcall!(stringBuffer).append(jcall!(CommonSeps::getInstance()).SPACE.clone());
        jcall!(stringBuffer).append(append.clone());
        self.append = jcall!(stringBuffer).toString();
        self.string = jcall!(StringMaker::new()).append(self.listString.clone()).append(self.append.clone()).toString();
    }
    

    pub fn append(&mut self, mut append: String) {
        self.append = jcall!(StringMaker::new()).append(self.append.clone()).append(append.clone()).toString();
        self.string = jcall!(StringMaker::new()).append(self.listString.clone()).append(self.append.clone()).toString();
    }
    

    pub fn clearLog(&mut self, mut x: i32, mut y: i32) {
        self.append = jcall!(StringMaker::new()).append(GPoint::toStringStatic(x.clone(), y.clone(), 0)).append(jcall!(CommonSeps::getInstance()).SPACE.clone()).toString();
        self.string = jcall!(StringMaker::new()).append(self.listString.clone()).append(self.append.clone()).toString();
    }
    

    pub fn releaseAndNotFired(&mut self) {
        self.append(self.RELEASED_AND_NOT_FIRED.clone());
    }
    

    pub fn releaseAndFired(&mut self, mut touchButtonInput: Option<Rc<RefCell<TouchButtonInput>>>) {
        self.appendTouchButtonInput(self.RELEASED_AND_FIRED.clone(), touchButtonInput.clone());
    }
    

    pub fn releaseAndFired2(&mut self, mut touchButtonInput: Option<Rc<RefCell<TouchButtonInput>>>) {
        self.appendTouchButtonInput(self.RELEASED_AND_FIRED_2.clone(), touchButtonInput.clone());
    }
    

    pub fn releaseAndFiredAssociated(&mut self, mut touchButtonInput: Option<Rc<RefCell<TouchButtonInput>>>) {
        self.appendTouchButtonInput(self.ASSOCIATED_RELEASED_AND_FIRED.clone(), touchButtonInput.clone());
    }
    

    pub fn pressedAndFired(&mut self, mut touchButtonInput: Option<Rc<RefCell<TouchButtonInput>>>) {
        self.appendTouchButtonInput(self.PRESSED_AND_FIRED.clone(), touchButtonInput.clone());
    }
    

    pub fn pressedAndNotFired(&mut self, mut touchButtonInput: Option<Rc<RefCell<TouchButtonInput>>>) {
        self.appendTouchButtonInput(self.PRESSED_AND_NOT_FIRED.clone(), touchButtonInput.clone());
    }
    

    pub fn removeAt(&mut self, mut index: i32) -> Option<Rc<RefCell<TouchButtonInput>>> {
        let mut touchButtonInput: Option<Rc<RefCell<TouchButtonInput>>> = self.base.removeAt(index.clone());
        self.listString = jcall!(self.list).toString();
        self.string = jcall!(StringMaker::new()).append(self.listString.clone()).append(self.append.clone()).toString();
        return touchButtonInput.clone();
    }
    

    pub fn remove(&mut self, mut touchButtonInput: Option<Rc<RefCell<TouchButtonInput>>>) -> bool {
        let mut isRemoved: bool = self.base.remove(touchButtonInput.clone());
        self.listString = jcall!(self.list).toString();
        self.string = jcall!(StringMaker::new()).append(self.listString.clone()).append(self.append.clone()).toString();
        return isRemoved.clone();
    }
    

    pub fn add(&mut self, mut touchButtonInput: Option<Rc<RefCell<TouchButtonInput>>>) {
        self.base.add(touchButtonInput.clone());
        self.listString = jcall!(self.list).toString();
        self.string = jcall!(StringMaker::new()).append(self.listString.clone()).append(self.append.clone()).toString();
    }
    

    pub fn toString(&mut self) -> String {
        return self.string.clone();
    }
}
impl std::ops::Deref for CurrentlyPressedTouchButtonSingletonDebug {
    type Target = CurrentlyPressedTouchButtonSingleton;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for CurrentlyPressedTouchButtonSingletonDebug {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::input::motion::button::CurrentlyPressedTouchButtonSingleton::CurrentlyPressedTouchButtonSingletonDyn for CurrentlyPressedTouchButtonSingletonDebug {
    fn as_CurrentlyPressedTouchButtonSingleton(&self) -> &crate::org::allbinary::input::motion::button::CurrentlyPressedTouchButtonSingleton::CurrentlyPressedTouchButtonSingleton { &self.base }
    fn as_CurrentlyPressedTouchButtonSingleton_mut(&mut self) -> &mut crate::org::allbinary::input::motion::button::CurrentlyPressedTouchButtonSingleton::CurrentlyPressedTouchButtonSingleton { &mut self.base }
    fn removeAt(&mut self, mut index: i32) -> Option<Rc<RefCell<TouchButtonInput>>> { Self::removeAt(self, index) }
    fn remove(&mut self, mut touchButtonInput: Option<Rc<RefCell<TouchButtonInput>>>) -> bool { Self::remove(self, touchButtonInput) }
    fn add(&mut self, mut touchButtonInput: Option<Rc<RefCell<TouchButtonInput>>>) { Self::add(self, touchButtonInput) }
}


