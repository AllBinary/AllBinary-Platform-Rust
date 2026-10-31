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
// package org.allbinary.input.motion
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::input::motion::gesture::TouchMotionGestureFactory::TouchMotionGestureFactory;
use crate::org::allbinary::input::motion::gesture::configuration::MotionGestureConfiguration::MotionGestureConfiguration;
use crate::org::allbinary::input::motion::gesture::configuration::MotionGestureConfigurationFactory::MotionGestureConfigurationFactory;
use crate::org::allbinary::input::motion::touch::action::DiagonalDownLeftTouchInputToGameKeyEventAction::DiagonalDownLeftTouchInputToGameKeyEventAction;
use crate::org::allbinary::input::motion::touch::action::DiagonalDownRightTouchInputToGameKeyEventAction::DiagonalDownRightTouchInputToGameKeyEventAction;
use crate::org::allbinary::input::motion::touch::action::DiagonalUpLeftTouchInputToGameKeyEventAction::DiagonalUpLeftTouchInputToGameKeyEventAction;
use crate::org::allbinary::input::motion::touch::action::DiagonalUpRightTouchInputToGameKeyEventAction::DiagonalUpRightTouchInputToGameKeyEventAction;
use crate::org::allbinary::input::motion::touch::action::DownTouchInputToGameKeyEventAction::DownTouchInputToGameKeyEventAction;
use crate::org::allbinary::input::motion::touch::action::LeftTouchInputToGameKeyEventAction::LeftTouchInputToGameKeyEventAction;
use crate::org::allbinary::input::motion::touch::action::ReleaseTouchInputToGameKeyEventAction::ReleaseTouchInputToGameKeyEventAction;
use crate::org::allbinary::input::motion::touch::action::RightTouchInputToGameKeyEventAction::RightTouchInputToGameKeyEventAction;
use crate::org::allbinary::input::motion::touch::action::UpTouchInputToGameKeyEventAction::UpTouchInputToGameKeyEventAction;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CompleteMotionGestureInputToGameMotionGestureInput {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

thread_local! { pub static CompleteMotionGestureInputToGameMotionGestureInput_instance: Option<Rc<RefCell<CompleteMotionGestureInputToGameMotionGestureInput>>> = CompleteMotionGestureInputToGameMotionGestureInput::new(); }

impl CompleteMotionGestureInputToGameMotionGestureInput {

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<CompleteMotionGestureInputToGameMotionGestureInput>>> {
        return CompleteMotionGestureInputToGameMotionGestureInput_instance.with(Clone::clone).clone();
    }
    

    pub fn init(&mut self) {
        let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            PreLogUtil::put(String::from("Compound Motion Gestures"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).INIT.clone());
            let mut motionGestureConfiguration: Option<Rc<RefCell<MotionGestureConfiguration>>> = MotionGestureConfigurationFactory::getInstance();
            let mut touchMotionGestureFactory: Option<Rc<RefCell<TouchMotionGestureFactory>>> = TouchMotionGestureFactory::getInstance();
            let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            jcall!(list).add(crate::java_into_object(jcall!(touchMotionGestureFactory).DIAGONAL_DOWN_LEFT.clone()));
            jcall!(motionGestureConfiguration).addMotionGestureAction(list.clone(), DiagonalDownLeftTouchInputToGameKeyEventAction::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::CompleteMotionGestureInputEvent::CompleteMotionGestureInputEventDyn>>));
            list = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            jcall!(list).add(crate::java_into_object(jcall!(touchMotionGestureFactory).DIAGONAL_DOWN_RIGHT.clone()));
            jcall!(motionGestureConfiguration).addMotionGestureAction(list.clone(), DiagonalDownRightTouchInputToGameKeyEventAction::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::CompleteMotionGestureInputEvent::CompleteMotionGestureInputEventDyn>>));
            list = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            jcall!(list).add(crate::java_into_object(jcall!(touchMotionGestureFactory).DIAGONAL_UP_LEFT.clone()));
            jcall!(motionGestureConfiguration).addMotionGestureAction(list.clone(), DiagonalUpLeftTouchInputToGameKeyEventAction::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::CompleteMotionGestureInputEvent::CompleteMotionGestureInputEventDyn>>));
            list = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            jcall!(list).add(crate::java_into_object(jcall!(touchMotionGestureFactory).DIAGONAL_UP_RIGHT.clone()));
            jcall!(motionGestureConfiguration).addMotionGestureAction(list.clone(), DiagonalUpRightTouchInputToGameKeyEventAction::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::CompleteMotionGestureInputEvent::CompleteMotionGestureInputEventDyn>>));
            list = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            jcall!(list).add(crate::java_into_object(jcall!(touchMotionGestureFactory).LEFT.clone()));
            jcall!(motionGestureConfiguration).addMotionGestureAction(list.clone(), LeftTouchInputToGameKeyEventAction::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::CompleteMotionGestureInputEvent::CompleteMotionGestureInputEventDyn>>));
            list = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            jcall!(list).add(crate::java_into_object(jcall!(touchMotionGestureFactory).RIGHT.clone()));
            jcall!(motionGestureConfiguration).addMotionGestureAction(list.clone(), RightTouchInputToGameKeyEventAction::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::CompleteMotionGestureInputEvent::CompleteMotionGestureInputEventDyn>>));
            list = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            jcall!(list).add(crate::java_into_object(jcall!(touchMotionGestureFactory).DOWN.clone()));
            jcall!(motionGestureConfiguration).addMotionGestureAction(list.clone(), DownTouchInputToGameKeyEventAction::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::CompleteMotionGestureInputEvent::CompleteMotionGestureInputEventDyn>>));
            list = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            jcall!(list).add(crate::java_into_object(jcall!(touchMotionGestureFactory).UP.clone()));
            jcall!(motionGestureConfiguration).addMotionGestureAction(list.clone(), UpTouchInputToGameKeyEventAction::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::CompleteMotionGestureInputEvent::CompleteMotionGestureInputEventDyn>>));
            list = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            jcall!(list).add(crate::java_into_object(jcall!(touchMotionGestureFactory).RELEASED.clone()));
            jcall!(motionGestureConfiguration).addMotionGestureAction(list.clone(), ReleaseTouchInputToGameKeyEventAction::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::CompleteMotionGestureInputEvent::CompleteMotionGestureInputEventDyn>>));
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).INIT.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
}

