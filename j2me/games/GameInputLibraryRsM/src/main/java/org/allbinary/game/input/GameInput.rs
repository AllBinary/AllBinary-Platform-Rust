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
use crate::org::allbinary::game::configuration::feature::Features::Features;
use crate::org::allbinary::game::configuration::feature::InputFeatureFactory::InputFeatureFactory;
use crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GameInput {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub gameKeyEventList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub removalGameKeyEventList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub isRemoveDuplicateKeyPresses: bool,
}


impl GameInput {
    

    pub fn new_struct(mut gameKeyEventList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut removalGameKeyEventList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            gameKeyEventList: None,
            removalGameKeyEventList: None,
            isRemoveDuplicateKeyPresses: false,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.gameKeyEventList = gameKeyEventList.clone();
        this.removalGameKeyEventList = removalGameKeyEventList.clone();
        this.isRemoveDuplicateKeyPresses = jcall!(Features::getInstance()).isFeature(jcall!(InputFeatureFactory::getInstance()).REMOVE_DUPLICATE_KEY_PRESSES.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>));
        this
    }

    pub fn new(mut gameKeyEventList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut removalGameKeyEventList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(gameKeyEventList, removalGameKeyEventList))))
    }
    

    // synchronized
    pub fn add(&mut self, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) {
        if self.isRemoveDuplicateKeyPresses && jcall!(self.gameKeyEventList).contains(crate::java_into_object(gameKeyEvent.clone())) {
            return;
        }
        if gameKeyEvent.is_some() {
            jcall!(self.gameKeyEventList).add(crate::java_into_object(gameKeyEvent.clone()));
        } else {
            jcall!(self.logUtil).putF(String::from("Danger Passed Null GameKeyEvent"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).ADD.clone());
        }
    }
    

    // synchronized
    pub fn addForRemoval(&mut self, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) {
        jcall!(self.removalGameKeyEventList).add(crate::java_into_object(gameKeyEvent.clone()));
    }
    

    // synchronized
    pub fn isForRemoval(&mut self, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) -> bool {
        return jcall!(self.removalGameKeyEventList).contains(crate::java_into_object(gameKeyEvent.clone()));
    }
    

    // synchronized
    pub fn clear(&mut self) {
        jcall!(self.gameKeyEventList).clear();
    }
    

    // synchronized
    pub fn removeNonAIInputGameKeyEvents(&mut self) {
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.gameKeyEventList.clone();
        let mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>> = None;
        {
            let mut index: i32 = jcall!(list).size();
            while { index -= 1; index } >= 0 {
            gameKeyEvent = { let java_object = (jcall!(list).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<GameKeyEvent>>>>().expect("ClassCastException").clone()) };
            if jcall!(gameKeyEvent).getSourceId() != 1 {
                jcall!(list).removeAt(index.clone());
            }
            }
        }
    }
    

    // synchronized
    pub fn update(&mut self) {
        let mut removeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.removalGameKeyEventList.clone();
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.gameKeyEventList.clone();
        let mut size: i32 = jcall!(removeList).size();
        for index in (0..size) {
            let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(removeList).as_BasicArrayList_mut().objectArray[(index) as usize].clone();
            {
                let mut index2: i32 = jcall!(list).size();
                while { index2 -= 1; index2 } >= 0 {
                if match (&jcall!(list).as_BasicArrayList_mut().objectArray[(index2) as usize], &anyType) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                    jcall!(list).removeAt(index2.clone());
                }
                }
            }
        }
        jcall!(removeList).clear();
    }
    

    pub fn getRemovalGameKeyEventList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.removalGameKeyEventList.clone();
    }
    

    pub fn getGameKeyEventList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.gameKeyEventList.clone();
    }
}
pub trait GameInputDyn {
    fn as_GameInput(&self) -> &GameInput;
    fn as_GameInput_mut(&mut self) -> &mut GameInput;
    fn add(&mut self, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) { GameInput::add(self.as_GameInput_mut(), gameKeyEvent) }
    fn addForRemoval(&mut self, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) { GameInput::addForRemoval(self.as_GameInput_mut(), gameKeyEvent) }
    fn isForRemoval(&mut self, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) -> bool { GameInput::isForRemoval(self.as_GameInput_mut(), gameKeyEvent) }
    fn clear(&mut self) { GameInput::clear(self.as_GameInput_mut()) }
    fn removeNonAIInputGameKeyEvents(&mut self) { GameInput::removeNonAIInputGameKeyEvents(self.as_GameInput_mut()) }
    fn update(&mut self) { GameInput::update(self.as_GameInput_mut()) }
    fn getRemovalGameKeyEventList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { GameInput::getRemovalGameKeyEventList(self.as_GameInput_mut()) }
    fn getGameKeyEventList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { GameInput::getGameKeyEventList(self.as_GameInput_mut()) }
}
impl GameInputDyn for GameInput {
    fn as_GameInput(&self) -> &GameInput { self }
    fn as_GameInput_mut(&mut self) -> &mut GameInput { self }
}


