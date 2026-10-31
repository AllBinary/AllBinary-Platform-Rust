// 
// AllBinary Open License Version 1 
// Copyright (c) 2007 AllBinary 
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
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use super::BaseTouchInput::BaseTouchInput;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TouchButtonsListBuilder {
    pub base: BaseTouchInput,
    pub baseTouchInputList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}


impl TouchButtonsListBuilder {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: BaseTouchInput::new_struct(),
            baseTouchInputList: None,
        };
        this.baseTouchInputList = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn addList(&mut self, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        let mut size: i32 = jcall!(self.baseTouchInputList).size();
        let mut baseTouchInput: Option<Rc<RefCell<dyn crate::org::allbinary::input::motion::button::BaseTouchInput::BaseTouchInputDyn>>> = None;
        for index in (0..size) {
            baseTouchInput = { let java_object = (jcall!(self.baseTouchInputList).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::input::motion::button::BaseTouchInput::BaseTouchInputDyn>>>>().expect("ClassCastException").clone()) };
            jcall!(list).addAllList(jcall!(baseTouchInput).getList());
        }
    }
    

    pub fn add(&mut self, mut baseTouchInput: Option<Rc<RefCell<dyn crate::org::allbinary::input::motion::button::BaseTouchInput::BaseTouchInputDyn>>>) {
        jcall!(self.baseTouchInputList).add(crate::java_into_object(baseTouchInput.clone()));
    }
}
impl std::ops::Deref for TouchButtonsListBuilder {
    type Target = BaseTouchInput;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for TouchButtonsListBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait TouchButtonsListBuilderDyn: crate::org::allbinary::input::motion::button::BaseTouchInput::BaseTouchInputDyn {
    fn as_TouchButtonsListBuilder(&self) -> &TouchButtonsListBuilder;
    fn as_TouchButtonsListBuilder_mut(&mut self) -> &mut TouchButtonsListBuilder;
    fn addList(&mut self, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { TouchButtonsListBuilder::addList(self.as_TouchButtonsListBuilder_mut(), list) }
    fn add(&mut self, mut baseTouchInput: Option<Rc<RefCell<dyn crate::org::allbinary::input::motion::button::BaseTouchInput::BaseTouchInputDyn>>>) { TouchButtonsListBuilder::add(self.as_TouchButtonsListBuilder_mut(), baseTouchInput) }
}
impl TouchButtonsListBuilderDyn for TouchButtonsListBuilder {
    fn as_TouchButtonsListBuilder(&self) -> &TouchButtonsListBuilder { self }
    fn as_TouchButtonsListBuilder_mut(&mut self) -> &mut TouchButtonsListBuilder { self }
}

impl crate::org::allbinary::input::motion::button::BaseTouchInput::BaseTouchInputDyn for TouchButtonsListBuilder {
    fn as_BaseTouchInput(&self) -> &crate::org::allbinary::input::motion::button::BaseTouchInput::BaseTouchInput { &self.base }
    fn as_BaseTouchInput_mut(&mut self) -> &mut crate::org::allbinary::input::motion::button::BaseTouchInput::BaseTouchInput { &mut self.base }
}


