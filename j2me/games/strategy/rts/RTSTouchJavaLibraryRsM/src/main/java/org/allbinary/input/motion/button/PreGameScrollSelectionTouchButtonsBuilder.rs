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
use crate::org::allbinary::util::BasicArrayListUtil::BasicArrayListUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use super::BaseTouchInput::BaseTouchInput;
use super::TouchButtonLocationHelper::TouchButtonLocationHelper;
use super::CommonButtons::CommonButtons;
use super::BasicTouchButtonCellPositionFactory::BasicTouchButtonCellPositionFactory;
use super::TouchButton::TouchButton;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PreGameScrollSelectionTouchButtonsBuilder {
    pub base: BaseTouchInput,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}


impl PreGameScrollSelectionTouchButtonsBuilder {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: BaseTouchInput::new_struct(),
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).GET_LIST.clone());
            let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            let mut touchButtonLocationHelper: Option<Rc<RefCell<TouchButtonLocationHelper>>> = TouchButtonLocationHelper::new();
            let mut commonButtons: Option<Rc<RefCell<CommonButtons>>> = CommonButtons::getInstance();
            let mut basicTouchButtonCellPositionFactory: Option<Rc<RefCell<BasicTouchButtonCellPositionFactory>>> = BasicTouchButtonCellPositionFactory::new();
            let mut LEFT: Option<Rc<RefCell<dyn crate::org::allbinary::input::motion::button::TouchButton::TouchButtonDyn>>> = TouchButton::createButton(jcall!(BasicTouchInputFactory::BasicTouchInputFactory::getInstance()).LEFT.clone(), TouchButtonStrafeLeftResource::TouchButtonStrafeLeftResource::getInstance(), jcall!(commonButtons).NORMAL_BUTTON.clone(), jcall!(basicTouchButtonCellPositionFactory).TOP_LEFT.clone(), jcall!(touchButtonLocationHelper).getColumnsRemainderHalf(), jcall!(touchButtonLocationHelper).getRowsRemainderHalf());
            let mut RIGHT: Option<Rc<RefCell<dyn crate::org::allbinary::input::motion::button::TouchButton::TouchButtonDyn>>> = TouchButton::createButton(jcall!(BasicTouchInputFactory::BasicTouchInputFactory::getInstance()).RIGHT.clone(), TouchButtonStrafeRightResource::TouchButtonStrafeRightResource::getInstance(), jcall!(commonButtons).NORMAL_BUTTON.clone(), jcall!(basicTouchButtonCellPositionFactory).TOP_RIGHT.clone(), jcall!(touchButtonLocationHelper).getColumnsRemainderHalf(), jcall!(touchButtonLocationHelper).getRowsRemainderHalf());
            let mut START: Option<Rc<RefCell<dyn crate::org::allbinary::input::motion::button::TouchButton::TouchButtonDyn>>> = TouchButton::createButton(jcall!(BasicTouchInputFactory::BasicTouchInputFactory::getInstance()).SPECIAL_BUTTON_FIVE.clone(), TouchButtonStartResource::TouchButtonStartResource::getInstance(), jcall!(commonButtons).NORMAL_BUTTON.clone(), jcall!(basicTouchButtonCellPositionFactory).BOTTOM_RIGHT.clone(), jcall!(touchButtonLocationHelper).getColumnsRemainderHalf(), jcall!(touchButtonLocationHelper).getRowsRemainderHalf());
            jcall!(list).add(crate::java_into_object(LEFT.clone()));
            jcall!(list).add(crate::java_into_object(RIGHT.clone()));
            jcall!(list).add(crate::java_into_object(START.clone()));
            return list.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).GET_LIST.clone(), crate::java_into_object(e.clone()));
            return jcall!(BasicArrayListUtil::getInstance()).getImmutableInstance();
            }
        }
    }
}
impl std::ops::Deref for PreGameScrollSelectionTouchButtonsBuilder {
    type Target = BaseTouchInput;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for PreGameScrollSelectionTouchButtonsBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::input::motion::button::BaseTouchInput::BaseTouchInputDyn for PreGameScrollSelectionTouchButtonsBuilder {
    fn as_BaseTouchInput(&self) -> &crate::org::allbinary::input::motion::button::BaseTouchInput::BaseTouchInput { &self.base }
    fn as_BaseTouchInput_mut(&mut self) -> &mut crate::org::allbinary::input::motion::button::BaseTouchInput::BaseTouchInput { &mut self.base }
    fn getList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Self::getList(self) }
}


