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
use crate::org::allbinary::game::input::GameKey::GameKey;
use crate::org::allbinary::game::input::GameKeyEventSourceInterface::GameKeyEventSourceInterface;
use crate::org::allbinary::game::input::Input::Input;
use crate::org::allbinary::game::input::InputFactory::InputFactory;
use crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent;
use crate::org::allbinary::game::input::event::GameKeyEventFactory::GameKeyEventFactory;
use crate::org::allbinary::game::input::mapping::InputToGameKeyMapping::InputToGameKeyMapping;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::java::lang::Exception::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TouchButtonInput {
    pub base: Input,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub gameKeyEventFactory: Option<Rc<RefCell<GameKeyEventFactory>>>,
    pub TOUCH_BUTTON_SOURCE_ID: i32,
    pub gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>,
}

impl GameKeyEventSourceInterface for TouchButtonInput {
    fn getSourceId(&mut self) -> i32 { Self::getSourceId(self) }
}

impl TouchButtonInput {
    

    pub fn new_struct(mut id: i32, mut name: String) -> Self {
        let mut this = Self {
            base: Input::new_struct(id.clone(), name.clone()),
            logUtil: None,
            gameKeyEventFactory: None,
            TOUCH_BUTTON_SOURCE_ID: 0,
            gameKeyEvent: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.gameKeyEventFactory = GameKeyEventFactory::getInstance();
        this.TOUCH_BUTTON_SOURCE_ID = { let java_field = jcall!(this.gameKeyEventFactory).TOUCH_BUTTON_SOURCE_ID.clone(); java_field }.clone();
        this.gameKeyEvent = crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent_NONE.with(Clone::clone).clone();
        // super(...) is in the struct literal above
        let mut inputFactory: Option<Rc<RefCell<InputFactory>>> = InputFactory::getInstance();
        this
    }

    pub fn new(mut id: i32, mut name: String) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(id.clone(), name.clone()))));
            let mut inputFactory: Option<Rc<RefCell<InputFactory>>> = InputFactory::getInstance();
            jcall!(inputFactory).add(jcall!(this).getId(), this.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>));
        this
    }
    

    pub fn update(&mut self, mut inputToGameKeyMapping: Option<Rc<RefCell<InputToGameKeyMapping>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut gameKey: Option<Rc<RefCell<GameKey>>> = jcall!(inputToGameKeyMapping).getInstance(self.getId());
            self.gameKeyEvent = { let java_value = (jcall!(self.gameKeyEventFactory).getInstanceForInput(None /* Java this: a &mut self method has no Rc of itself */, gameKey.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>))).clone(); java_value };
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(self.logUtil).put({ let java_field = jcall!(commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(commonStrings).UPDATE.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn getSourceId(&mut self) -> i32 {
        return self.TOUCH_BUTTON_SOURCE_ID.clone();
    }
    

    pub fn getGameKeyEvent(&mut self) -> Option<Rc<RefCell<GameKeyEvent>>> {
        return self.gameKeyEvent.clone();
    }
}
impl std::ops::Deref for TouchButtonInput {
    type Target = Input;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for TouchButtonInput {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::input::Input::InputDyn for TouchButtonInput {
    fn as_Input(&self) -> &crate::org::allbinary::game::input::Input::Input { &self.base }
    fn as_Input_mut(&mut self) -> &mut crate::org::allbinary::game::input::Input::Input { &mut self.base }
}


