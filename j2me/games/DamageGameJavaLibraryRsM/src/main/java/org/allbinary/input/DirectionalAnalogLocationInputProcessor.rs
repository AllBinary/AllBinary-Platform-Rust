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
// package org.allbinary.input
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::android::input::motion::AnalogLocationInputProcessor::AnalogLocationInputProcessor;
use crate::org::allbinary::game::input::GameInputProcessor::GameInputProcessor;
use crate::org::allbinary::game::input::GameKeyEventSourceInterface::GameKeyEventSourceInterface;
use crate::org::allbinary::game::input::GameKeyFactory::GameKeyFactory;
use crate::org::allbinary::game::input::analog::AnalogLocationInput::AnalogLocationInput;
use crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent;
use crate::org::allbinary::game::input::event::GameKeyEventFactory::GameKeyEventFactory;
use crate::org::allbinary::graphics::CustomGPoint::CustomGPoint;
use crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct DirectionalAnalogLocationInputProcessor {
    pub base: AnalogLocationInputProcessor,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub inputProcessorArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInputProcessor::GameInputProcessorDyn>>>>,
    pub gameKeyFactory: Option<Rc<RefCell<GameKeyFactory>>>,
    pub leftGameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>,
    pub rightGameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>,
    pub upGameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>,
    pub downGameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>,
    pub leftTriggerGameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>,
    pub rightTriggerGameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>,
}

impl GameKeyEventSourceInterface for DirectionalAnalogLocationInputProcessor {
    fn getSourceId(&mut self) -> i32 { Self::getSourceId(self) }
}

impl DirectionalAnalogLocationInputProcessor {
    

    pub fn new_struct(mut inputProcessorArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInputProcessor::GameInputProcessorDyn>>>>) -> Self {
        let mut this = Self {
            base: AnalogLocationInputProcessor::new_struct(),
            logUtil: None,
            inputProcessorArray: Vec::new(),
            gameKeyFactory: None,
            leftGameKeyEvent: None,
            rightGameKeyEvent: None,
            upGameKeyEvent: None,
            downGameKeyEvent: None,
            leftTriggerGameKeyEvent: None,
            rightTriggerGameKeyEvent: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.gameKeyFactory = GameKeyFactory::getInstance();
        this.leftGameKeyEvent = crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent_NONE.with(Clone::clone).clone();
        this.rightGameKeyEvent = crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent_NONE.with(Clone::clone).clone();
        this.upGameKeyEvent = crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent_NONE.with(Clone::clone).clone();
        this.downGameKeyEvent = crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent_NONE.with(Clone::clone).clone();
        this.leftTriggerGameKeyEvent = crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent_NONE.with(Clone::clone).clone();
        this.rightTriggerGameKeyEvent = crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent_NONE.with(Clone::clone).clone();
        this.inputProcessorArray = inputProcessorArray.clone();
        this
    }

    pub fn new(mut inputProcessorArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInputProcessor::GameInputProcessorDyn>>>>) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(inputProcessorArray))));
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            this.leftGameKeyEvent = jcall!(GameKeyEventFactory::getInstance()).getInstanceForInput((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()), jcall!(this.gameKeyFactory).LEFT.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>));
            this.rightGameKeyEvent = jcall!(GameKeyEventFactory::getInstance()).getInstanceForInput((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()), jcall!(this.gameKeyFactory).RIGHT.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>));
            this.upGameKeyEvent = jcall!(GameKeyEventFactory::getInstance()).getInstanceForInput((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()), jcall!(this.gameKeyFactory).UP.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>));
            this.downGameKeyEvent = jcall!(GameKeyEventFactory::getInstance()).getInstanceForInput((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()), jcall!(this.gameKeyFactory).DOWN.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>));
            this.leftTriggerGameKeyEvent = jcall!(GameKeyEventFactory::getInstance()).getInstanceForInput((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()), jcall!(this.gameKeyFactory).KEY_NUM0.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>));
            this.rightTriggerGameKeyEvent = jcall!(GameKeyEventFactory::getInstance()).getInstanceForInput((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()), jcall!(this.gameKeyFactory).KEY_NUM5.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>));
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(this.logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).CONSTRUCTOR.clone(), crate::java_into_object(e.clone()));
            }
        }
        this
    }
    

    pub fn process(&mut self, mut allbinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut analogLocationInput: Option<Rc<RefCell<AnalogLocationInput>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut customGPoint: Option<Rc<RefCell<CustomGPoint>>> = None;
            customGPoint = jcall!(analogLocationInput).getCustomGPoint();
            let mut x: i32 = jcall!(customGPoint).getX();
            let mut y: i32 = jcall!(customGPoint).getY();
            let mut leftTrigger: i32 = jcall!(analogLocationInput).getLeftTrigger();
            let mut rightTrigger: i32 = jcall!(analogLocationInput).getRightTrigger();
            if x < 0 {
                jcall!(self.inputProcessorArray[(jcall!(self.leftGameKeyEvent).getKey()) as usize]).processAnalog(allbinaryLayerManager.clone(), self.leftGameKeyEvent.clone(), x.clone());
            } else if x > 0 {
                jcall!(self.inputProcessorArray[(jcall!(self.rightGameKeyEvent).getKey()) as usize]).processAnalog(allbinaryLayerManager.clone(), self.rightGameKeyEvent.clone(), x.clone());
            }
            if y < 0 {
                jcall!(self.inputProcessorArray[(jcall!(self.downGameKeyEvent).getKey()) as usize]).processAnalog(allbinaryLayerManager.clone(), self.downGameKeyEvent.clone(), y.clone());
            } else if y > 0 {
                jcall!(self.inputProcessorArray[(jcall!(self.upGameKeyEvent).getKey()) as usize]).processAnalog(allbinaryLayerManager.clone(), self.upGameKeyEvent.clone(), y.clone());
            }
            if leftTrigger > 0 {
                jcall!(self.inputProcessorArray[(jcall!(self.leftTriggerGameKeyEvent).getKey()) as usize]).processAnalog(allbinaryLayerManager.clone(), self.leftTriggerGameKeyEvent.clone(), leftTrigger.clone());
            }
            if rightTrigger > 0 {
                jcall!(self.inputProcessorArray[(jcall!(self.rightTriggerGameKeyEvent).getKey()) as usize]).processAnalog(allbinaryLayerManager.clone(), self.rightTriggerGameKeyEvent.clone(), rightTrigger.clone());
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(self.logUtil).put(String::from("Unable to process analog input"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).PROCESS.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn getSourceId(&mut self) -> i32 {
        return 0;
    }
}
impl std::ops::Deref for DirectionalAnalogLocationInputProcessor {
    type Target = AnalogLocationInputProcessor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for DirectionalAnalogLocationInputProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::android::input::motion::AnalogLocationInputProcessor::AnalogLocationInputProcessorDyn for DirectionalAnalogLocationInputProcessor {
    fn as_AnalogLocationInputProcessor(&self) -> &crate::org::allbinary::android::input::motion::AnalogLocationInputProcessor::AnalogLocationInputProcessor { &self.base }
    fn as_AnalogLocationInputProcessor_mut(&mut self) -> &mut crate::org::allbinary::android::input::motion::AnalogLocationInputProcessor::AnalogLocationInputProcessor { &mut self.base }
    fn process(&mut self, mut allbinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut analogLocationInput: Option<Rc<RefCell<AnalogLocationInput>>>) { Self::process(self, allbinaryLayerManager, analogLocationInput) }
}


