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
use super::TouchButtonRecognizer::TouchButtonRecognizer;
use super::TouchButtonInput::TouchButtonInput;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ReleaseHelper {
    pub touchButtonRecognizer: Option<Rc<RefCell<TouchButtonRecognizer>>>,
}


impl ReleaseHelper {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn new_struct(mut touchButtonRecognizer: Option<Rc<RefCell<TouchButtonRecognizer>>>) -> Self {
        let mut this = Self {
            touchButtonRecognizer: None,
        };
        this.touchButtonRecognizer = touchButtonRecognizer.clone();
        this
    }

    pub fn new(mut touchButtonRecognizer: Option<Rc<RefCell<TouchButtonRecognizer>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(touchButtonRecognizer))))
    }
    

    // throws Exception
    pub fn release(&mut self, mut touchButtonInput: Option<Rc<RefCell<TouchButtonInput>>>, mut deviceId: i32) {
        let mut nextTouchButtonInput: Option<Rc<RefCell<TouchButtonInput>>> = None;
        let mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>> = None;
        for index in (0..={ let java_bound = jcall!({ let java_field = jcall!(self.touchButtonRecognizer).currentlyPressedTouchButtonSingleton.clone(); java_field }).size() - 1; java_bound }).rev() {
            nextTouchButtonInput = { let java_value = (jcall!({ let java_field = jcall!(self.touchButtonRecognizer).currentlyPressedTouchButtonSingleton.clone(); java_field }).get(index.clone())).clone(); java_value };
            gameKeyEvent = { let java_value = (jcall!(nextTouchButtonInput).getGameKeyEvent()).clone(); java_value };
            jcall!({ let java_field = jcall!(self.touchButtonRecognizer).upGameKeyEventHandler.clone(); java_field }).fireEvent(gameKeyEvent.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>));
            jcall!(jcall!({ let java_field = jcall!(self.touchButtonRecognizer).upGameKeyEventHandler.clone(); java_field }).getInstanceForDevice(deviceId.clone())).as_UpGameKeyEventHandlerBase_mut().fireEvent(gameKeyEvent.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>));
            jcall!({ let java_field = jcall!(self.touchButtonRecognizer).currentlyPressedTouchButtonSingleton.clone(); java_field }).removeAt(index.clone());
        }
    }
}
pub trait ReleaseHelperDyn: std::any::Any {
    fn as_ReleaseHelper(&self) -> &ReleaseHelper;
    fn as_ReleaseHelper_mut(&mut self) -> &mut ReleaseHelper;
    fn release(&mut self, mut touchButtonInput: Option<Rc<RefCell<TouchButtonInput>>>, mut deviceId: i32) { ReleaseHelper::release(self.as_ReleaseHelper_mut(), touchButtonInput, deviceId) }
}
impl ReleaseHelperDyn for ReleaseHelper {
    fn as_ReleaseHelper(&self) -> &ReleaseHelper { self }
    fn as_ReleaseHelper_mut(&mut self) -> &mut ReleaseHelper { self }
}


