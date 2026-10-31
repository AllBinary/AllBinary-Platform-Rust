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
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::util::BasicArrayListUtil::BasicArrayListUtil;
use super::BaseTouchInput::BaseTouchInput;
use super::CommonButtons::CommonButtons;
use super::TouchButtonLocationHelper::TouchButtonLocationHelper;
use super::BasicTouchButtonCellPositionFactory::BasicTouchButtonCellPositionFactory;
use super::TouchButton::TouchButton;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicTouchButtonsBuilder {
    pub base: BaseTouchInput,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}


impl BasicTouchButtonsBuilder {

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
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).CONSTRUCTOR.clone());
            let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            let mut commonButtons: Option<Rc<RefCell<CommonButtons>>> = CommonButtons::getInstance();
            let mut touchButtonLocationHelper: Option<Rc<RefCell<TouchButtonLocationHelper>>> = TouchButtonLocationHelper::new();
            let mut basicTouchButtonCellPositionFactory: Option<Rc<RefCell<BasicTouchButtonCellPositionFactory>>> = BasicTouchButtonCellPositionFactory::new();
            let mut UP: Option<Rc<RefCell<dyn crate::org::allbinary::input::motion::button::TouchButton::TouchButtonDyn>>> = TouchButton::createButton(jcall!(BasicTouchInputFactory::BasicTouchInputFactory::getInstance()).UP.clone(), TouchButtonUpResource::TouchButtonUpResource::getInstance(), jcall!(commonButtons).NORMAL_BUTTON.clone(), jcall!(basicTouchButtonCellPositionFactory).SECOND_FROM_BOTTOM_LEFT.clone(), jcall!(touchButtonLocationHelper).getColumnsRemainderHalf(), jcall!(touchButtonLocationHelper).getRowsRemainderHalf());
            let mut DOWN: Option<Rc<RefCell<dyn crate::org::allbinary::input::motion::button::TouchButton::TouchButtonDyn>>> = TouchButton::createButton(jcall!(BasicTouchInputFactory::BasicTouchInputFactory::getInstance()).DOWN.clone(), TouchButtonDownResource::TouchButtonDownResource::getInstance(), jcall!(commonButtons).NORMAL_BUTTON.clone(), jcall!(basicTouchButtonCellPositionFactory).BOTTOM_LEFT.clone(), jcall!(touchButtonLocationHelper).getColumnsRemainderHalf(), jcall!(touchButtonLocationHelper).getRowsRemainderHalf());
            let mut LEFT: Option<Rc<RefCell<dyn crate::org::allbinary::input::motion::button::TouchButton::TouchButtonDyn>>> = TouchButton::createButton(jcall!(BasicTouchInputFactory::BasicTouchInputFactory::getInstance()).SPECIAL_BUTTON_FOUR.clone(), TouchButtonTurnLeftResource::TouchButtonTurnLeftResource::getInstance(), jcall!(commonButtons).NORMAL_BUTTON.clone(), jcall!(basicTouchButtonCellPositionFactory).BOTTOM_SECOND_FROM_RIGHT.clone(), jcall!(touchButtonLocationHelper).getColumnsRemainderHalf(), jcall!(touchButtonLocationHelper).getRowsRemainderHalf());
            let mut RIGHT: Option<Rc<RefCell<dyn crate::org::allbinary::input::motion::button::TouchButton::TouchButtonDyn>>> = TouchButton::createButton(jcall!(BasicTouchInputFactory::BasicTouchInputFactory::getInstance()).SPECIAL_BUTTON_THREE.clone(), TouchButtonTurnRightResource::TouchButtonTurnRightResource::getInstance(), jcall!(commonButtons).NORMAL_BUTTON.clone(), jcall!(basicTouchButtonCellPositionFactory).BOTTOM_RIGHT.clone(), jcall!(touchButtonLocationHelper).getColumnsRemainderHalf(), jcall!(touchButtonLocationHelper).getRowsRemainderHalf());
            jcall!(list).add(crate::java_into_object(UP.clone()));
            jcall!(list).add(crate::java_into_object(LEFT.clone()));
            jcall!(list).add(crate::java_into_object(RIGHT.clone()));
            jcall!(list).add(crate::java_into_object(DOWN.clone()));
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
impl std::ops::Deref for BasicTouchButtonsBuilder {
    type Target = BaseTouchInput;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for BasicTouchButtonsBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::input::motion::button::BaseTouchInput::BaseTouchInputDyn for BasicTouchButtonsBuilder {
    fn as_BaseTouchInput(&self) -> &crate::org::allbinary::input::motion::button::BaseTouchInput::BaseTouchInput { &self.base }
    fn as_BaseTouchInput_mut(&mut self) -> &mut crate::org::allbinary::input::motion::button::BaseTouchInput::BaseTouchInput { &mut self.base }
    fn getList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Self::getList(self) }
}


