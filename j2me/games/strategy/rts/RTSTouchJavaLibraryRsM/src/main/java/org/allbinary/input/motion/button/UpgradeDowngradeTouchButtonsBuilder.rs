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
use crate::org::allbinary::game::input::resource::TouchButtonDowngradeResource::TouchButtonDowngradeResource;
use crate::org::allbinary::game::input::resource::TouchButtonUpgradeResource::TouchButtonUpgradeResource;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::util::BasicArrayListUtil::BasicArrayListUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::graphics::CellPositionFactory::CellPositionFactory;
use crate::java::lang::Exception::*;
use super::BaseTouchInput::BaseTouchInput;
use super::TouchButtonLocationHelper::TouchButtonLocationHelper;
use super::BasicTouchButtonCellPositionFactory::BasicTouchButtonCellPositionFactory;
use super::CommonButtons::CommonButtons;
use super::TouchButton::TouchButton;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct UpgradeDowngradeTouchButtonsBuilder {
    pub base: BaseTouchInput,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}


impl UpgradeDowngradeTouchButtonsBuilder {

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
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).putF({ let java_field = jcall!(self.commonStrings).START.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).CONSTRUCTOR.clone(); java_field }.clone());
            let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            let mut touchButtonLocationHelper: Option<Rc<RefCell<TouchButtonLocationHelper>>> = TouchButtonLocationHelper::new();
            let mut cellPositionFactory: Option<Rc<RefCell<CellPositionFactory>>> = CellPositionFactory::getInstance();
            let mut basicTouchButtonCellPositionFactory: Option<Rc<RefCell<BasicTouchButtonCellPositionFactory>>> = BasicTouchButtonCellPositionFactory::new();
            if !match (&{ let java_field = jcall!(basicTouchButtonCellPositionFactory).SECOND_FROM_TOP_LEFT.clone(); java_field }, &{ let java_field = jcall!(cellPositionFactory).NONE.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } && !match (&{ let java_field = jcall!(basicTouchButtonCellPositionFactory).SECOND_FROM_TOP_RIGHT.clone(); java_field }, &{ let java_field = jcall!(cellPositionFactory).NONE.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                let mut commonButtons: Option<Rc<RefCell<CommonButtons>>> = CommonButtons::getInstance();
                let mut UP: Option<Rc<RefCell<dyn crate::org::allbinary::input::motion::button::TouchButton::TouchButtonDyn>>> = TouchButton::createButton({ let java_field = jcall!(BasicTouchInputFactory::BasicTouchInputFactory::getInstance()).SPECIAL_BUTTON_TWO.clone(); java_field }.clone(), TouchButtonUpgradeResource::getInstance(), { let java_field = jcall!(commonButtons).NORMAL_BUTTON.clone(); java_field }.clone(), { let java_field = jcall!(basicTouchButtonCellPositionFactory).SECOND_FROM_TOP_LEFT.clone(); java_field }.clone(), jcall!(touchButtonLocationHelper).getColumnsRemainderHalf(), jcall!(touchButtonLocationHelper).getRowsRemainderHalf());
                let mut DOWN: Option<Rc<RefCell<dyn crate::org::allbinary::input::motion::button::TouchButton::TouchButtonDyn>>> = TouchButton::createButton({ let java_field = jcall!(BasicTouchInputFactory::BasicTouchInputFactory::getInstance()).SPECIAL_BUTTON_ONE.clone(); java_field }.clone(), TouchButtonDowngradeResource::getInstance(), { let java_field = jcall!(commonButtons).NORMAL_BUTTON.clone(); java_field }.clone(), { let java_field = jcall!(basicTouchButtonCellPositionFactory).SECOND_FROM_TOP_RIGHT.clone(); java_field }.clone(), jcall!(touchButtonLocationHelper).getColumnsRemainderHalf(), jcall!(touchButtonLocationHelper).getRowsRemainderHalf());
                jcall!(list).add(crate::java_into_object(UP.clone()));
                jcall!(list).add(crate::java_into_object(DOWN.clone()));
            }
            return Some(list.clone());
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).CONSTRUCTOR.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            return jcall!(BasicArrayListUtil::getInstance()).getImmutableInstance();
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
}
impl std::ops::Deref for UpgradeDowngradeTouchButtonsBuilder {
    type Target = BaseTouchInput;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for UpgradeDowngradeTouchButtonsBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::input::motion::button::BaseTouchInput::BaseTouchInputDyn for UpgradeDowngradeTouchButtonsBuilder {
    fn as_BaseTouchInput(&self) -> &crate::org::allbinary::input::motion::button::BaseTouchInput::BaseTouchInput { &self.base }
    fn as_BaseTouchInput_mut(&mut self) -> &mut crate::org::allbinary::input::motion::button::BaseTouchInput::BaseTouchInput { &mut self.base }
    fn getList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Self::getList(self) }
}


