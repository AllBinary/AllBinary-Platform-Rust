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
// package org.allbinary.game.input.motion.action
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::input::CompleteMotionGestureInputEvent::CompleteMotionGestureInputEvent;
use crate::org::allbinary::game::input::GameKey::GameKey;
use crate::org::allbinary::game::input::GameKeyEventSourceInterface::GameKeyEventSourceInterface;
use crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent;
use crate::org::allbinary::game::input::event::GameKeyEventFactory::GameKeyEventFactory;
use crate::org::allbinary::game::input::mapping::InputToGameKeyMapping::InputToGameKeyMapping;
use crate::org::allbinary::input::motion::gesture::MotionGestureInput::MotionGestureInput;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GameKeyCompleteMotionGestureInputEvent {
    pub base: CompleteMotionGestureInputEvent,
    pub gameKeyEventFactory: Option<Rc<RefCell<GameKeyEventFactory>>>,
    pub SOURCE_ID: i32,
    pub gameKey: Option<Rc<RefCell<GameKey>>>,
    pub gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>,
    pub inputToGameKeyMapping: Option<Rc<RefCell<InputToGameKeyMapping>>>,
}

impl GameKeyEventSourceInterface for GameKeyCompleteMotionGestureInputEvent {
    fn getSourceId(&mut self) -> i32 { Self::getSourceId(self) }
}
impl crate::org::allbinary::game::input::CompleteMotionGestureInputInterface::CompleteMotionGestureInputInterface for GameKeyCompleteMotionGestureInputEvent {
}
impl crate::org::allbinary::layer::NamedInterface::NamedInterface for GameKeyCompleteMotionGestureInputEvent {
    fn getName(&mut self) -> String { self.base.getName() }
}

impl GameKeyCompleteMotionGestureInputEvent {
    

    pub fn new_struct(mut name: String, mut motionGestureInput: Option<Rc<RefCell<MotionGestureInput>>>, mut inputToGameKeyMapping: Option<Rc<RefCell<InputToGameKeyMapping>>>) -> Self {
        let mut this = Self {
            base: CompleteMotionGestureInputEvent::new_struct(name.clone(), motionGestureInput.clone()),
            gameKeyEventFactory: None,
            SOURCE_ID: 0,
            gameKey: None,
            gameKeyEvent: None,
            inputToGameKeyMapping: None,
        };
        this.gameKeyEventFactory = GameKeyEventFactory::getInstance();
        this.SOURCE_ID = jcall!(this.gameKeyEventFactory).MOTION_GESTURE_SOURCE_ID.clone();
        this.gameKey = crate::org::allbinary::game::input::GameKey::GameKey_NULL_GAME_KEY.with(Clone::clone).clone();
        this.gameKeyEvent = crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent_NONE.with(Clone::clone).clone();
        // super(...) is in the struct literal above
        this.inputToGameKeyMapping = inputToGameKeyMapping.clone();
        this.update();
        this
    }

    pub fn new(mut name: String, mut motionGestureInput: Option<Rc<RefCell<MotionGestureInput>>>, mut inputToGameKeyMapping: Option<Rc<RefCell<InputToGameKeyMapping>>>) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(name, motionGestureInput, inputToGameKeyMapping))));
            jcall!(GameKeyCompleteMotionGestureInputEventFactory::GameKeyCompleteMotionGestureInputEventFactory::getInstance()).add((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()));
        this
    }
    

    pub fn getSourceId(&mut self) -> i32 {
        return self.SOURCE_ID.clone();
    }
    

    pub fn update(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            { let java_arguments = ({ let java_receiver = self.inputToGameKeyMapping.clone(); let java_arguments = (jcall!(self.getMotionGestureInput()).getId(),); let java_result = jcall!(java_receiver).getInstance(java_arguments.0); java_result },); self.setGameKey(java_arguments.0) };
            { let java_arguments = ({ let java_receiver = self.gameKeyEventFactory.clone(); let java_arguments = (None /* Java this: a &mut self method has no Rc of itself */, self.getGameKey().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>),); let java_result = jcall!(java_receiver).getInstanceForInput(java_arguments.0, java_arguments.1); java_result },); self.setGameKeyEvent(java_arguments.0) };
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(self.logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).UPDATE.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn setGameKeyEvent(&mut self, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) {
        self.gameKeyEvent = gameKeyEvent.clone();
    }
    

    pub fn getGameKeyEvent(&mut self) -> Option<Rc<RefCell<GameKeyEvent>>> {
        return self.gameKeyEvent.clone();
    }
    

    pub fn setGameKey(&mut self, mut gameKey: Option<Rc<RefCell<GameKey>>>) {
        self.gameKey = gameKey.clone();
    }
    

    pub fn getGameKey(&mut self) -> Option<Rc<RefCell<GameKey>>> {
        return self.gameKey.clone();
    }
}
impl std::ops::Deref for GameKeyCompleteMotionGestureInputEvent {
    type Target = CompleteMotionGestureInputEvent;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for GameKeyCompleteMotionGestureInputEvent {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait GameKeyCompleteMotionGestureInputEventDyn: crate::org::allbinary::game::input::CompleteMotionGestureInputEvent::CompleteMotionGestureInputEventDyn + crate::org::allbinary::game::input::GameKeyEventSourceInterface::GameKeyEventSourceInterface {
    fn as_GameKeyCompleteMotionGestureInputEvent(&self) -> &GameKeyCompleteMotionGestureInputEvent;
    fn as_GameKeyCompleteMotionGestureInputEvent_mut(&mut self) -> &mut GameKeyCompleteMotionGestureInputEvent;
    fn update(&mut self) { GameKeyCompleteMotionGestureInputEvent::update(self.as_GameKeyCompleteMotionGestureInputEvent_mut()) }
    fn setGameKeyEvent(&mut self, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) { GameKeyCompleteMotionGestureInputEvent::setGameKeyEvent(self.as_GameKeyCompleteMotionGestureInputEvent_mut(), gameKeyEvent) }
    fn getGameKeyEvent(&mut self) -> Option<Rc<RefCell<GameKeyEvent>>> { GameKeyCompleteMotionGestureInputEvent::getGameKeyEvent(self.as_GameKeyCompleteMotionGestureInputEvent_mut()) }
    fn setGameKey(&mut self, mut gameKey: Option<Rc<RefCell<GameKey>>>) { GameKeyCompleteMotionGestureInputEvent::setGameKey(self.as_GameKeyCompleteMotionGestureInputEvent_mut(), gameKey) }
    fn getGameKey(&mut self) -> Option<Rc<RefCell<GameKey>>> { GameKeyCompleteMotionGestureInputEvent::getGameKey(self.as_GameKeyCompleteMotionGestureInputEvent_mut()) }
}
impl GameKeyCompleteMotionGestureInputEventDyn for GameKeyCompleteMotionGestureInputEvent {
    fn as_GameKeyCompleteMotionGestureInputEvent(&self) -> &GameKeyCompleteMotionGestureInputEvent { self }
    fn as_GameKeyCompleteMotionGestureInputEvent_mut(&mut self) -> &mut GameKeyCompleteMotionGestureInputEvent { self }
}

impl crate::org::allbinary::game::input::CompleteMotionGestureInputEvent::CompleteMotionGestureInputEventDyn for GameKeyCompleteMotionGestureInputEvent {
    fn as_CompleteMotionGestureInputEvent(&self) -> &crate::org::allbinary::game::input::CompleteMotionGestureInputEvent::CompleteMotionGestureInputEvent { &self.base }
    fn as_CompleteMotionGestureInputEvent_mut(&mut self) -> &mut crate::org::allbinary::game::input::CompleteMotionGestureInputEvent::CompleteMotionGestureInputEvent { &mut self.base }
}
impl crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn for GameKeyCompleteMotionGestureInputEvent {
    fn as_AllBinaryEventObject(&self) -> &crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject { &self.base.base }
    fn as_AllBinaryEventObject_mut(&mut self) -> &mut crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject { &mut self.base.base }
}


