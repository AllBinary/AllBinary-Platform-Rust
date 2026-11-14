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
use crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent;
use crate::java::lang::Exception::*;
use super::ReleaseHelper::ReleaseHelper;
use super::TouchButtonRecognizer::TouchButtonRecognizer;
use super::TouchButtonInput::TouchButtonInput;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MultitouchReleaseHelper {
    pub base: ReleaseHelper,
}


impl MultitouchReleaseHelper {
    

    pub fn new_struct(mut touchButtonRecognizer: Option<Rc<RefCell<TouchButtonRecognizer>>>) -> Self {
        let mut this = Self {
            base: ReleaseHelper::new_struct(touchButtonRecognizer.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut touchButtonRecognizer: Option<Rc<RefCell<TouchButtonRecognizer>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(touchButtonRecognizer))))
    }
    

    // throws Exception
    pub fn release(&mut self, mut touchButtonInput: Option<Rc<RefCell<TouchButtonInput>>>, mut deviceId: i32) {
        let mut cancelTouchButtonInput: Option<Rc<RefCell<TouchButtonInput>>> = jcall!(CancelTouchButtonInputFactory::CancelTouchButtonInputFactory::getInstance()).getCancel(touchButtonInput.clone());
        let mut nextTouchButtonInput: Option<Rc<RefCell<TouchButtonInput>>> = None;
        let mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>> = None;
        for index in (0..={ let java_bound = jcall!({ let java_field = jcall!(self.touchButtonRecognizer).currentlyPressedTouchButtonSingleton.clone(); java_field }).size() - 1; java_bound }).rev() {
            nextTouchButtonInput = { let java_value = (jcall!({ let java_field = jcall!(self.touchButtonRecognizer).currentlyPressedTouchButtonSingleton.clone(); java_field }).get(index.clone())).clone(); java_value };
            if match (&cancelTouchButtonInput, &nextTouchButtonInput) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                gameKeyEvent = { let java_value = (jcall!(nextTouchButtonInput).getGameKeyEvent()).clone(); java_value };
                jcall!({ let java_field = jcall!(self.touchButtonRecognizer).upGameKeyEventHandler.clone(); java_field }).fireEvent(gameKeyEvent.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>));
                jcall!(jcall!({ let java_field = jcall!(self.touchButtonRecognizer).upGameKeyEventHandler.clone(); java_field }).getInstanceForDevice(deviceId.clone())).as_UpGameKeyEventHandlerBase_mut().fireEvent(gameKeyEvent.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>));
                jcall!({ let java_field = jcall!(self.touchButtonRecognizer).currentlyPressedTouchButtonSingleton.clone(); java_field }).removeAt(index.clone());
            }
        }
    }
}
impl std::ops::Deref for MultitouchReleaseHelper {
    type Target = ReleaseHelper;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for MultitouchReleaseHelper {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::input::motion::button::ReleaseHelper::ReleaseHelperDyn for MultitouchReleaseHelper {
    fn as_ReleaseHelper(&self) -> &crate::org::allbinary::input::motion::button::ReleaseHelper::ReleaseHelper { &self.base }
    fn as_ReleaseHelper_mut(&mut self) -> &mut crate::org::allbinary::input::motion::button::ReleaseHelper::ReleaseHelper { &mut self.base }
    fn release(&mut self, mut touchButtonInput: Option<Rc<RefCell<TouchButtonInput>>>, mut deviceId: i32) { Self::release(self, touchButtonInput, deviceId) }
}


