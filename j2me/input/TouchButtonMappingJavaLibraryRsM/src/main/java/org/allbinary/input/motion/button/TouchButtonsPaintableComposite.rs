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
use crate::org::allbinary::game::input::CompleteMotionGestureInputEvent::CompleteMotionGestureInputEvent;
use crate::org::allbinary::game::input::CompleteMotionGestureInputEventHandler::CompleteMotionGestureInputEventHandler;
use crate::org::allbinary::game::input::CompleteMotionGestureInputEventListenerInterface::CompleteMotionGestureInputEventListenerInterface;
use crate::org::allbinary::game::input::GameKey::GameKey;
use crate::org::allbinary::game::input::mapping::InputMappingInterface::InputMappingInterface;
use crate::org::allbinary::game::input::motion::action::GameKeyCompleteMotionGestureInputEvent::GameKeyCompleteMotionGestureInputEvent;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::paint::ProcessPaintable::ProcessPaintable;
use crate::org::allbinary::input::motion::gesture::MotionGestureInput::MotionGestureInput;
use crate::org::allbinary::input::motion::gesture::TouchMotionGestureFactory::TouchMotionGestureFactory;
use crate::org::allbinary::logic::communication::log::ForcedLogUtil::ForcedLogUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject;
use crate::org::allbinary::logic::util::event::EventStrings::EventStrings;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelper;
use super::TouchButtonsMappingPaintable::TouchButtonsMappingPaintable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TouchButtonsPaintableComposite {
    pub base: ProcessPaintable,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub inputMappingInterface: Option<Rc<RefCell<dyn InputMappingInterface>>>,
    pub touchButtonsPaintable: Option<Rc<RefCell<TouchButtonsMappingPaintable>>>,
    pub timeHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>>,
    pub released: bool,
    pub METHOD_NAME: String,
    pub RELEASE: String,
    pub FAST_REPEAT: String,
    pub IGNORE: String,
}

impl CompleteMotionGestureInputEventListenerInterface for TouchButtonsPaintableComposite {
    fn onCompleteMotionGestureInputEvent(&mut self, mut completeMotionGestureInputEvent: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::CompleteMotionGestureInputEvent::CompleteMotionGestureInputEventDyn>>>) { Self::onCompleteMotionGestureInputEvent(self, completeMotionGestureInputEvent) }
}
impl crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface for TouchButtonsPaintableComposite {
    fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { Self::onEvent(self, eventObject) }
}

impl TouchButtonsPaintableComposite {
    

    pub fn new_struct(mut inputMappingInterface: Option<Rc<RefCell<dyn InputMappingInterface>>>, mut basicColor: Option<Rc<RefCell<BasicColor>>>) -> Self {
        let mut this = Self {
            base: ProcessPaintable::new_struct(),
            logUtil: None,
            commonStrings: None,
            inputMappingInterface: None,
            touchButtonsPaintable: None,
            timeHelper: None,
            released: false,
            METHOD_NAME: String::new(),
            RELEASE: String::new(),
            FAST_REPEAT: String::new(),
            IGNORE: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.timeHelper = TimeDelayHelper::new(250).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>);
        this.released = true;
        this.METHOD_NAME = String::from("onCompleteMotionGestureInputEvent");
        this.RELEASE = String::from("Ignoring: MotionGestureInput Release");
        this.FAST_REPEAT = String::from("Ignoring: MotionGestureInput Repeated To Quickly");
        this.IGNORE = String::from("Ignoring: Until Released");
        this.inputMappingInterface = inputMappingInterface.clone();
        this.touchButtonsPaintable = TouchButtonsMappingPaintable::new(basicColor.clone());
        this
    }

    pub fn new(mut inputMappingInterface: Option<Rc<RefCell<dyn InputMappingInterface>>>, mut basicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(inputMappingInterface, basicColor))));
            jcall!(CompleteMotionGestureInputEventHandler::getInstance()).addListenerInterface((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()));
        this
    }
    

    pub fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) {
        ForcedLogUtil::log(jcall!(EventStrings::getInstance()).PERFORMANCE_MESSAGE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())));
    }
    

    pub fn onCompleteMotionGestureInputEvent(&mut self, mut completeMotionGestureInputEvent: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::CompleteMotionGestureInputEvent::CompleteMotionGestureInputEventDyn>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("onCompleteMotionGestureInputEvent"));
            let mut touchMotionGestureFactory: Option<Rc<RefCell<TouchMotionGestureFactory>>> = TouchMotionGestureFactory::getInstance();
            let mut gameKeyCompleteMotionGestureInputEvent: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::motion::action::GameKeyCompleteMotionGestureInputEvent::GameKeyCompleteMotionGestureInputEventDyn>>> = (completeMotionGestureInputEvent).clone() /* as GameKeyCompleteMotionGestureInputEvent */;
            let mut gameKey: Option<Rc<RefCell<GameKey>>> = jcall!(gameKeyCompleteMotionGestureInputEvent).getGameKey();
            let mut motionGestureInput: Option<Rc<RefCell<MotionGestureInput>>> = jcall!(completeMotionGestureInputEvent).getMotionGestureInput();
            if match (&motionGestureInput, &jcall!(touchMotionGestureFactory).RELEASED) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                jcall!(self.logUtil).putF(self.RELEASE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_NAME.clone());
                self.released = true;
                return;
            }
            if match (&motionGestureInput, &jcall!(TouchMotionGestureFactory::getInstance()).NO_MOTION) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } && !jcall!(self.timeHelper).isTimeTNT() {
                jcall!(self.logUtil).putF(self.FAST_REPEAT.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_NAME.clone());
                return;
            }
            if !self.released {
                jcall!(self.logUtil).putF(self.IGNORE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_NAME.clone());
                return;
            }
            self.released = false;
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("GameKey: ")).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(gameKey.clone()))).append(String::from(" MotionGestureInput: ")).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(motionGestureInput.clone()))).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_NAME.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            jcall!(self.inputMappingInterface).processInputMapping(gameKey.clone(), motionGestureInput.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>));
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_NAME.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn process(&mut self) {
        jcall!(CompleteMotionGestureInputEventHandler::getInstance()).removeListener(None /* Java this: a &mut self method has no Rc of itself */);
    }
}
impl std::ops::Deref for TouchButtonsPaintableComposite {
    type Target = ProcessPaintable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for TouchButtonsPaintableComposite {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::paint::Paintable::PaintableDyn for TouchButtonsPaintableComposite {
    fn as_Paintable(&self) -> &crate::org::allbinary::graphics::paint::Paintable::Paintable { &self.base.base }
    fn as_Paintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::Paintable::Paintable { &mut self.base.base }
}
impl crate::org::allbinary::graphics::paint::ProcessPaintable::ProcessPaintableDyn for TouchButtonsPaintableComposite {
    fn as_ProcessPaintable(&self) -> &crate::org::allbinary::graphics::paint::ProcessPaintable::ProcessPaintable { &self.base }
    fn as_ProcessPaintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::ProcessPaintable::ProcessPaintable { &mut self.base }
    fn process(&mut self) { Self::process(self) }
}


