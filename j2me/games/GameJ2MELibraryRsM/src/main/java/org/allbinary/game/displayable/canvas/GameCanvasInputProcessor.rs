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
use crate::javax::microedition::lcdui::Canvas::Canvas;
use crate::org::allbinary::game::input::GameKey::GameKey;
use crate::org::allbinary::game::input::GameKeyEventSourceInterface::GameKeyEventSourceInterface;
use crate::org::allbinary::game::input::GameKeyFactory::GameKeyFactory;
use crate::org::allbinary::game::input::InputProcessor::InputProcessor;
use crate::org::allbinary::game::input::PlatformInputMappingFactory::PlatformInputMappingFactory;
use crate::org::allbinary::game::input::event::DownGameKeyEventHandler::DownGameKeyEventHandler;
use crate::org::allbinary::game::input::event::DownKeyEventHandler::DownKeyEventHandler;
use crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent;
use crate::org::allbinary::game::input::event::GameKeyEventFactory::GameKeyEventFactory;
use crate::org::allbinary::game::input::event::UpGameKeyEventHandler::UpGameKeyEventHandler;
use crate::org::allbinary::game::input::mapping::InputToGameKeyMapping::InputToGameKeyMapping;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::math::SmallIntegerSingletonFactory::SmallIntegerSingletonFactory;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::java::lang::Exception::*;
use super::AllBinaryGameCanvas::AllBinaryGameCanvas;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GameCanvasInputProcessor {
    pub base: InputProcessor,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>,
    pub downGameKeyEventHandler: Option<Rc<RefCell<DownGameKeyEventHandler>>>,
    pub upGameKeyEventHandler: Option<Rc<RefCell<UpGameKeyEventHandler>>>,
    pub downKeyEventHandler: Option<Rc<RefCell<DownKeyEventHandler>>>,
    pub smallIntegerSingletonFactory: Option<Rc<RefCell<SmallIntegerSingletonFactory>>>,
    pub inputToGameKeyMapping: Option<Rc<RefCell<InputToGameKeyMapping>>>,
    pub NONE: Option<Rc<RefCell<GameKey>>>,
    pub gameKeyEventFactory: Option<Rc<RefCell<GameKeyEventFactory>>>,
}


impl GameCanvasInputProcessor {
    

    pub fn new_struct(mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) -> Self {
        let mut this = Self {
            base: InputProcessor::new_struct(),
            logUtil: None,
            allBinaryGameCanvas: None,
            downGameKeyEventHandler: None,
            upGameKeyEventHandler: None,
            downKeyEventHandler: None,
            smallIntegerSingletonFactory: None,
            inputToGameKeyMapping: None,
            NONE: None,
            gameKeyEventFactory: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.downGameKeyEventHandler = DownGameKeyEventHandler::getInstance();
        this.upGameKeyEventHandler = UpGameKeyEventHandler::getInstance();
        this.downKeyEventHandler = DownKeyEventHandler::getInstance();
        this.smallIntegerSingletonFactory = SmallIntegerSingletonFactory::getInstance();
        this.inputToGameKeyMapping = jcall!(jcall!(PlatformInputMappingFactory::getInstance()).getPersistentInputMappingInstance()).getInputMapping();
        this.NONE = { let java_field = jcall!(GameKeyFactory::getInstance()).NONE.clone(); java_field }.clone();
        this.gameKeyEventFactory = GameKeyEventFactory::getInstance();
        this.allBinaryGameCanvas = allBinaryGameCanvas.clone();
        jcall!(this.inputToGameKeyMapping).init(this.allBinaryGameCanvas.clone().map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Canvas::CanvasDyn>>));
        this
    }

    pub fn new(mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(allBinaryGameCanvas))))
    }
    

    pub fn keyPressedByDevice(&mut self, mut keyCode: i32, mut deviceId: i32) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut gameKey: Option<Rc<RefCell<GameKey>>> = jcall!(self.inputToGameKeyMapping).getInstanceForCanvas(self.allBinaryGameCanvas.clone().map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Canvas::CanvasDyn>>), keyCode.clone());
            if !match (&gameKey, &self.NONE) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                let mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>> = jcall!(self.gameKeyEventFactory).getInstanceForInput(self.allBinaryGameCanvas.clone().map(|_java_reference| -> Rc<RefCell<dyn GameKeyEventSourceInterface>> { panic!("ClassCastException: the Rust trait of AllBinaryGameCanvas does not have GameKeyEventSourceInterface as a base trait") }), gameKey.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>));
                jcall!(self.downGameKeyEventHandler).fireEvent(gameKeyEvent.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>));
                jcall!(jcall!(self.downGameKeyEventHandler).getInstanceForDevice(deviceId.clone())).as_DownGameKeyEventHandlerBase_mut().fireEvent(gameKeyEvent.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>));
            } else {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append({ let java_field = jcall!(self.gameInputStrings).NO_KEY.clone(); java_field }.clone()).appendint(keyCode.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.gameInputStrings).ADD_KEY_EVENT.clone(); java_field }.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            let mut keyCodeAsInteger: Option<i32> = jcall!(self.smallIntegerSingletonFactory).getAtNoThrow(keyCode.clone());
            jcall!(self.downKeyEventHandler).fireEventI(keyCodeAsInteger.clone());
            jcall!(jcall!(self.downKeyEventHandler).getInstanceForDevice(deviceId.clone())).fireEventI(keyCodeAsInteger.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put(String::from("Key Event Error"), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.gameInputStrings).ADD_KEY_EVENT.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn keyReleasedByDevice(&mut self, mut canvas: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Canvas::CanvasDyn>>>, mut keyCode: i32, mut deviceId: i32) {
        self.removeGameKeyEvent(canvas.clone(), keyCode.clone(), deviceId.clone(), false);
    }
    

    pub fn removeGameKeyEvent(&mut self, mut canvas: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Canvas::CanvasDyn>>>, mut keyCode: i32, mut deviceId: i32, mut repeated: bool) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut gameKey: Option<Rc<RefCell<GameKey>>> = jcall!(self.inputToGameKeyMapping).getInstanceForCanvas(canvas.clone(), keyCode.clone());
            if !match (&gameKey, &self.NONE) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                let mut gameKeyEventSourceInterface: Option<Rc<RefCell<dyn GameKeyEventSourceInterface>>> = (|| -> Option<Rc<RefCell<dyn GameKeyEventSourceInterface>>> { panic!("ClassCastException: a Rust dyn Canvas cannot be cast to GameKeyEventSourceInterface") })();
                let mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>> = jcall!(self.gameKeyEventFactory).getInstanceForInput(gameKeyEventSourceInterface.clone(), gameKey.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>));
                jcall!(self.upGameKeyEventHandler).fireEvent(gameKeyEvent.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>));
                jcall!(jcall!(self.upGameKeyEventHandler).getInstanceForDevice(deviceId.clone())).as_UpGameKeyEventHandlerBase_mut().fireEvent(gameKeyEvent.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>));
            } else {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append({ let java_field = jcall!(self.gameInputStrings).NO_KEY.clone(); java_field }.clone()).appendint(keyCode.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.gameInputStrings).REMOVE_KEY_EVENT.clone(); java_field }.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put(String::from("Key Event Error"), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.gameInputStrings).REMOVE_KEY_EVENT.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
}
impl std::ops::Deref for GameCanvasInputProcessor {
    type Target = InputProcessor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for GameCanvasInputProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::input::InputProcessor::InputProcessorDyn for GameCanvasInputProcessor {
    fn as_InputProcessor(&self) -> &crate::org::allbinary::game::input::InputProcessor::InputProcessor { &self.base }
    fn as_InputProcessor_mut(&mut self) -> &mut crate::org::allbinary::game::input::InputProcessor::InputProcessor { &mut self.base }
    fn keyPressedByDevice(&mut self, mut keyCode: i32, mut deviceId: i32) { Self::keyPressedByDevice(self, keyCode, deviceId) }
    fn keyReleasedByDevice(&mut self, mut canvas: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Canvas::CanvasDyn>>>, mut keyCode: i32, mut deviceId: i32) { Self::keyReleasedByDevice(self, canvas, keyCode, deviceId) }
}


