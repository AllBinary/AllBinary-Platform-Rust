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
use crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GameInputProcessor {
}

thread_local! { pub static GameInputProcessor_instance: Option<Rc<RefCell<dyn GameInputProcessorDyn>>> = GameInputProcessor::new().map(|reference| reference as Rc<RefCell<dyn GameInputProcessorDyn>>); }

impl GameInputProcessor {
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn GameInputProcessorDyn>>> {
        return GameInputProcessor_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn processEvent(&mut self, mut allbinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) {
    }
    

    // throws Exception
    pub fn processReleasedEvent(&mut self, mut allbinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) {
    }
    

    // throws Exception
    pub fn processAnalog(&mut self, mut allbinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>, mut analogValue: i32) {
    }
    

    // throws Exception
    pub fn process(&mut self, mut allbinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut keyAsInteger: Option<i32>) {
    }
    

    // throws Exception
    pub fn processReleased(&mut self, mut allbinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut keyAsInteger: Option<i32>) {
    }
}
pub trait GameInputProcessorDyn {
    fn as_GameInputProcessor(&self) -> &GameInputProcessor;
    fn as_GameInputProcessor_mut(&mut self) -> &mut GameInputProcessor;
    fn processEvent(&mut self, mut allbinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) { GameInputProcessor::processEvent(self.as_GameInputProcessor_mut(), allbinaryLayerManager, gameKeyEvent) }
    fn processReleasedEvent(&mut self, mut allbinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) { GameInputProcessor::processReleasedEvent(self.as_GameInputProcessor_mut(), allbinaryLayerManager, gameKeyEvent) }
    fn processAnalog(&mut self, mut allbinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>, mut analogValue: i32) { GameInputProcessor::processAnalog(self.as_GameInputProcessor_mut(), allbinaryLayerManager, gameKeyEvent, analogValue) }
    fn process(&mut self, mut allbinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut keyAsInteger: Option<i32>) { GameInputProcessor::process(self.as_GameInputProcessor_mut(), allbinaryLayerManager, keyAsInteger) }
    fn processReleased(&mut self, mut allbinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut keyAsInteger: Option<i32>) { GameInputProcessor::processReleased(self.as_GameInputProcessor_mut(), allbinaryLayerManager, keyAsInteger) }
}
impl GameInputProcessorDyn for GameInputProcessor {
    fn as_GameInputProcessor(&self) -> &GameInputProcessor { self }
    fn as_GameInputProcessor_mut(&mut self) -> &mut GameInputProcessor { self }
}


