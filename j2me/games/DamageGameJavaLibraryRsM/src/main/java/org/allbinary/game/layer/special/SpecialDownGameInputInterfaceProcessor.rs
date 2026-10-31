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
// package org.allbinary.game.layer.special
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent;
use crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager;
use super::SpecialGameInputInterfaceProcessor::SpecialGameInputInterfaceProcessor;
use super::SpecialGameInputInterface::SpecialGameInputInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SpecialDownGameInputInterfaceProcessor {
    pub base: SpecialGameInputInterfaceProcessor,
}


impl SpecialDownGameInputInterfaceProcessor {
    

    pub fn new_struct(mut collidableDestroyableDamageableLayer: Option<Rc<RefCell<dyn SpecialGameInputInterface>>>) -> Self {
        let mut this = Self {
            base: SpecialGameInputInterfaceProcessor::new_struct(collidableDestroyableDamageableLayer.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut collidableDestroyableDamageableLayer: Option<Rc<RefCell<dyn SpecialGameInputInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(collidableDestroyableDamageableLayer))))
    }
    

    // throws Exception
    pub fn processEvent(&mut self, mut allbinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) {
        jcall!(self.specialGameInputInterface).down();
    }
}
impl std::ops::Deref for SpecialDownGameInputInterfaceProcessor {
    type Target = SpecialGameInputInterfaceProcessor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for SpecialDownGameInputInterfaceProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::layer::special::SpecialGameInputInterfaceProcessor::SpecialGameInputInterfaceProcessorDyn for SpecialDownGameInputInterfaceProcessor {
    fn as_SpecialGameInputInterfaceProcessor(&self) -> &crate::org::allbinary::game::layer::special::SpecialGameInputInterfaceProcessor::SpecialGameInputInterfaceProcessor { &self.base }
    fn as_SpecialGameInputInterfaceProcessor_mut(&mut self) -> &mut crate::org::allbinary::game::layer::special::SpecialGameInputInterfaceProcessor::SpecialGameInputInterfaceProcessor { &mut self.base }
}
impl crate::org::allbinary::game::input::GameInputProcessor::GameInputProcessorDyn for SpecialDownGameInputInterfaceProcessor {
    fn as_GameInputProcessor(&self) -> &crate::org::allbinary::game::input::GameInputProcessor::GameInputProcessor { &self.base.base }
    fn as_GameInputProcessor_mut(&mut self) -> &mut crate::org::allbinary::game::input::GameInputProcessor::GameInputProcessor { &mut self.base.base }
    fn processEvent(&mut self, mut allbinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) { Self::processEvent(self, allbinaryLayerManager, gameKeyEvent) }
}


