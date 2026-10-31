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
// package org.allbinary.game.displayable.canvas
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Canvas::Canvas;
use crate::org::allbinary::game::input::GameKey::GameKey;
use crate::org::allbinary::game::input::GameKeyEventSourceInterface::GameKeyEventSourceInterface;
use crate::org::allbinary::game::input::GameKeyFactory::GameKeyFactory;
use crate::org::allbinary::game::input::Input::Input;
use crate::org::allbinary::game::input::InputFactory::InputFactory;
use crate::org::allbinary::game::input::InputProcessor::InputProcessor;
use crate::org::allbinary::game::input::PlatformKeyFactory::PlatformKeyFactory;
use crate::org::allbinary::game::input::event::DownKeyEventHandler::DownKeyEventHandler;
use crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent;
use crate::org::allbinary::game::input::event::GameKeyEventFactory::GameKeyEventFactory;
use crate::org::allbinary::game::input::event::UpGameKeyEventHandler::UpGameKeyEventHandler;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::AllBinaryGameCanvas::AllBinaryGameCanvas;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct FormInputProcessor {
    pub base: InputProcessor,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub gameKeyEventFactory: Option<Rc<RefCell<GameKeyEventFactory>>>,
    pub downKeyEventHandler: Option<Rc<RefCell<DownKeyEventHandler>>>,
    pub upGameKeyEventHandler: Option<Rc<RefCell<UpGameKeyEventHandler>>>,
    pub inputFactory: Option<Rc<RefCell<InputFactory>>>,
    pub platformKeyFactory: Option<Rc<RefCell<PlatformKeyFactory>>>,
    pub allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>,
}


impl FormInputProcessor {
    

    pub fn new_struct(mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) -> Self {
        let mut this = Self {
            base: InputProcessor::new_struct(),
            logUtil: None,
            commonStrings: None,
            gameKeyEventFactory: None,
            downKeyEventHandler: None,
            upGameKeyEventHandler: None,
            inputFactory: None,
            platformKeyFactory: None,
            allBinaryGameCanvas: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.gameKeyEventFactory = GameKeyEventFactory::getInstance();
        this.downKeyEventHandler = DownKeyEventHandler::getInstance();
        this.upGameKeyEventHandler = UpGameKeyEventHandler::getInstance();
        this.inputFactory = InputFactory::getInstance();
        this.platformKeyFactory = PlatformKeyFactory::getInstance();
        this.allBinaryGameCanvas = allBinaryGameCanvas.clone();
        this
    }

    pub fn new(mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(allBinaryGameCanvas))))
    }
    

    pub fn keyPressedByDevice(&mut self, mut keyCode: i32, mut deviceId: i32) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            PreLogUtil::put(jcall!(StringMaker::new()).append(jcall!(self.inputFactory).KEY_CODE_LABEL.clone()).appendint(keyCode.clone()).append(jcall!(CommonSeps::getInstance()).SPACE.clone()).append(jcall!(self.inputFactory).DEVICE_ID_LABEL.clone()).appendint(deviceId.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.gameInputStrings).KEY_PRESSED.clone());
            let mut input: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>> = jcall!(self.inputFactory).getInstanceById(keyCode.clone());
            let mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>> = jcall!(self.gameKeyEventFactory).getInstanceForInput(self.allBinaryGameCanvas.clone().map(|reference| reference as Rc<RefCell<dyn GameKeyEventSourceInterface>>), input.clone());
            jcall!(self.downKeyEventHandler).fireEventForGameKeyEvent(gameKeyEvent.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.gameInputStrings).ADD_KEY_EVENT.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn keyReleasedByDevice(&mut self, mut canvas: Option<Rc<RefCell<Canvas>>>, mut keyCode: i32, mut deviceId: i32) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut input: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>> = jcall!(self.inputFactory).getInstanceById(keyCode.clone());
            if jcall!(self.platformKeyFactory).isEnter(input.clone()) {
                PreLogUtil::put(jcall!(StringMaker::new()).append(jcall!(self.inputFactory).KEY_CODE_LABEL.clone()).appendint(keyCode.clone()).append(jcall!(CommonSeps::getInstance()).SPACE.clone()).append(jcall!(self.inputFactory).DEVICE_ID_LABEL.clone()).appendint(deviceId.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.gameInputStrings).KEY_RELEASED.clone());
                let mut gameKey: Option<Rc<RefCell<GameKey>>> = jcall!(GameKeyFactory::getInstance()).KEY_NUM0.clone();
                let mut gameKeyEventSourceInterface: Option<Rc<RefCell<dyn GameKeyEventSourceInterface>>> = (canvas).clone() /* as GameKeyEventSourceInterface */;
                let mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>> = jcall!(self.gameKeyEventFactory).getInstanceForInput(gameKeyEventSourceInterface.clone(), gameKey.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>));
                jcall!(self.upGameKeyEventHandler).fireEvent(gameKeyEvent.clone());
                jcall!(jcall!(self.upGameKeyEventHandler).getInstanceForDevice(deviceId.clone())).fireEvent(gameKeyEvent.clone());
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.gameInputStrings).ADD_KEY_EVENT.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
}
impl std::ops::Deref for FormInputProcessor {
    type Target = InputProcessor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for FormInputProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::input::InputProcessor::InputProcessorDyn for FormInputProcessor {
    fn as_InputProcessor(&self) -> &crate::org::allbinary::game::input::InputProcessor::InputProcessor { &self.base }
    fn as_InputProcessor_mut(&mut self) -> &mut crate::org::allbinary::game::input::InputProcessor::InputProcessor { &mut self.base }
    fn keyPressedByDevice(&mut self, mut keyCode: i32, mut deviceId: i32) { Self::keyPressedByDevice(self, keyCode, deviceId) }
    fn keyReleasedByDevice(&mut self, mut canvas: Option<Rc<RefCell<Canvas>>>, mut keyCode: i32, mut deviceId: i32) { Self::keyReleasedByDevice(self, canvas, keyCode, deviceId) }
}


