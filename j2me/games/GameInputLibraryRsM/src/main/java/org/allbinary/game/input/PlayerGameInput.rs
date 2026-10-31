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
// package org.allbinary.game.input
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent;
use crate::org::allbinary::game::input::event::GameKeyEventListenerInterface::GameKeyEventListenerInterface;
use crate::org::allbinary::logic::communication::log::ForcedLogUtil::ForcedLogUtil;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject;
use crate::org::allbinary::logic::util::event::EventStrings::EventStrings;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use super::GameInput::GameInput;
use super::GameInputStrings::GameInputStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PlayerGameInput {
    pub base: GameInput,
    pub gameInputStrings: Option<Rc<RefCell<GameInputStrings>>>,
    pub playerInputId: i32,
}

// impl GameKeyEventListenerInterface for PlayerGameInput with the methods of the trait

impl PlayerGameInput {
    

    pub fn new_struct(mut gameKeyEventList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut removalGameKeyEventList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut playerInputId: i32) -> Self {
        let mut this = Self {
            base: GameInput::new_struct(gameKeyEventList.clone(), removalGameKeyEventList.clone()),
            gameInputStrings: None,
            playerInputId: 0,
        };
        this.gameInputStrings = GameInputStrings::getInstance();
        // super(...) is in the struct literal above
        this.playerInputId = playerInputId.clone();
        this
    }

    pub fn new(mut gameKeyEventList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut removalGameKeyEventList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut playerInputId: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(gameKeyEventList, removalGameKeyEventList, playerInputId))))
    }
    

    // synchronized
    pub fn onPressGameKeyEvent(&mut self, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) {
        self.add(gameKeyEvent.clone());
        self.addForRemoval(gameKeyEvent.clone());
    }
    

    // synchronized
    pub fn onDownGameKeyEvent(&mut self, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) {
        self.add(gameKeyEvent.clone());
    }
    

    // synchronized
    pub fn onDownKeyEvent(&mut self, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) {
        self.add(gameKeyEvent.clone());
        self.addForRemoval(gameKeyEvent.clone());
    }
    

    // synchronized
    pub fn onDownKey(&mut self, mut gameKeyEvent: Option<i32>) {
        PreLogUtil::put(jcall!(StringMaker::new()).append(jcall!(CommonLabels::getInstance()).START_LABEL.clone()).append(jcall!(gameKeyEvent).toString()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.gameInputStrings).ON_DOWN_GAME_KEY.clone());
    }
    

    // synchronized
    pub fn onUpGameKeyEvent(&mut self, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) {
        self.addForRemoval(gameKeyEvent.clone());
    }
    

    // synchronized
    pub fn onUpKeyEvent(&mut self, mut gameKeyEvent: Option<i32>) {
    }
    

    pub fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) {
        ForcedLogUtil::log(jcall!(EventStrings::getInstance()).PERFORMANCE_MESSAGE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())));
    }
    

    pub fn getPlayerInputId(&mut self) -> i32 {
        return self.playerInputId.clone();
    }
}
impl std::ops::Deref for PlayerGameInput {
    type Target = GameInput;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for PlayerGameInput {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait PlayerGameInputDyn: crate::org::allbinary::game::input::GameInput::GameInputDyn {
    fn as_PlayerGameInput(&self) -> &PlayerGameInput;
    fn as_PlayerGameInput_mut(&mut self) -> &mut PlayerGameInput;
    fn onPressGameKeyEvent(&mut self, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) { PlayerGameInput::onPressGameKeyEvent(self.as_PlayerGameInput_mut(), gameKeyEvent) }
    fn onDownGameKeyEvent(&mut self, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) { PlayerGameInput::onDownGameKeyEvent(self.as_PlayerGameInput_mut(), gameKeyEvent) }
    fn onDownKeyEvent(&mut self, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) { PlayerGameInput::onDownKeyEvent(self.as_PlayerGameInput_mut(), gameKeyEvent) }
    fn onDownKey(&mut self, mut gameKeyEvent: Option<i32>) { PlayerGameInput::onDownKey(self.as_PlayerGameInput_mut(), gameKeyEvent) }
    fn onUpGameKeyEvent(&mut self, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) { PlayerGameInput::onUpGameKeyEvent(self.as_PlayerGameInput_mut(), gameKeyEvent) }
    fn onUpKeyEvent(&mut self, mut gameKeyEvent: Option<i32>) { PlayerGameInput::onUpKeyEvent(self.as_PlayerGameInput_mut(), gameKeyEvent) }
    fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { PlayerGameInput::onEvent(self.as_PlayerGameInput_mut(), eventObject) }
    fn getPlayerInputId(&mut self) -> i32 { PlayerGameInput::getPlayerInputId(self.as_PlayerGameInput_mut()) }
}
impl PlayerGameInputDyn for PlayerGameInput {
    fn as_PlayerGameInput(&self) -> &PlayerGameInput { self }
    fn as_PlayerGameInput_mut(&mut self) -> &mut PlayerGameInput { self }
}

impl crate::org::allbinary::game::input::GameInput::GameInputDyn for PlayerGameInput {
    fn as_GameInput(&self) -> &crate::org::allbinary::game::input::GameInput::GameInput { &self.base }
    fn as_GameInput_mut(&mut self) -> &mut crate::org::allbinary::game::input::GameInput::GameInput { &mut self.base }
}


