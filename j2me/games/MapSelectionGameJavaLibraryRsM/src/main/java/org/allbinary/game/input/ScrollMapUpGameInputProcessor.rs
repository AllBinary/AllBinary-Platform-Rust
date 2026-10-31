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
use super::ScrollMapGameInputProcessor::ScrollMapGameInputProcessor;
use super::ScrollMapPlayerGameInput::ScrollMapPlayerGameInput;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ScrollMapUpGameInputProcessor {
    pub base: ScrollMapGameInputProcessor,
}


impl ScrollMapUpGameInputProcessor {
    

    pub fn new_struct(mut scrollMapPlayerGameInput: Option<Rc<RefCell<ScrollMapPlayerGameInput>>>) -> Self {
        let mut this = Self {
            base: ScrollMapGameInputProcessor::new_struct(scrollMapPlayerGameInput.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut scrollMapPlayerGameInput: Option<Rc<RefCell<ScrollMapPlayerGameInput>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(scrollMapPlayerGameInput))))
    }
    

    // throws Exception
    pub fn processEvent(&mut self, mut allbinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) {
        jcall!(self.scrollMapPlayerGameInput).up();
    }
}
impl std::ops::Deref for ScrollMapUpGameInputProcessor {
    type Target = ScrollMapGameInputProcessor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ScrollMapUpGameInputProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::input::ScrollMapGameInputProcessor::ScrollMapGameInputProcessorDyn for ScrollMapUpGameInputProcessor {
    fn as_ScrollMapGameInputProcessor(&self) -> &crate::org::allbinary::game::input::ScrollMapGameInputProcessor::ScrollMapGameInputProcessor { &self.base }
    fn as_ScrollMapGameInputProcessor_mut(&mut self) -> &mut crate::org::allbinary::game::input::ScrollMapGameInputProcessor::ScrollMapGameInputProcessor { &mut self.base }
}
impl crate::org::allbinary::game::input::GameInputProcessor::GameInputProcessorDyn for ScrollMapUpGameInputProcessor {
    fn as_GameInputProcessor(&self) -> &crate::org::allbinary::game::input::GameInputProcessor::GameInputProcessor { &self.base.base }
    fn as_GameInputProcessor_mut(&mut self) -> &mut crate::org::allbinary::game::input::GameInputProcessor::GameInputProcessor { &mut self.base.base }
    fn processEvent(&mut self, mut allbinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) { Self::processEvent(self, allbinaryLayerManager, gameKeyEvent) }
}


