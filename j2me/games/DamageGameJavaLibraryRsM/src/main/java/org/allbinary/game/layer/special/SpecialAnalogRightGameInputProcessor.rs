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
use super::SpecialGameInputProcessor::SpecialGameInputProcessor;
use super::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SpecialAnalogRightGameInputProcessor {
    pub base: SpecialGameInputProcessor,
}


impl SpecialAnalogRightGameInputProcessor {
    

    pub fn new_struct(mut collidableDestroyableDamageableLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>) -> Self {
        let mut this = Self {
            base: SpecialGameInputProcessor::new_struct(collidableDestroyableDamageableLayer.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut collidableDestroyableDamageableLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(collidableDestroyableDamageableLayer))))
    }
    

    // throws Exception
    pub fn processAnalog(&mut self, mut allbinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>, mut xAnalogValue: i32) {
        jcall!(self.collidableDestroyableDamageableLayer).rightAnalog(xAnalogValue.clone());
    }
}
impl std::ops::Deref for SpecialAnalogRightGameInputProcessor {
    type Target = SpecialGameInputProcessor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for SpecialAnalogRightGameInputProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::layer::special::SpecialGameInputProcessor::SpecialGameInputProcessorDyn for SpecialAnalogRightGameInputProcessor {
    fn as_SpecialGameInputProcessor(&self) -> &crate::org::allbinary::game::layer::special::SpecialGameInputProcessor::SpecialGameInputProcessor { &self.base }
    fn as_SpecialGameInputProcessor_mut(&mut self) -> &mut crate::org::allbinary::game::layer::special::SpecialGameInputProcessor::SpecialGameInputProcessor { &mut self.base }
}
impl crate::org::allbinary::game::input::GameInputProcessor::GameInputProcessorDyn for SpecialAnalogRightGameInputProcessor {
    fn as_GameInputProcessor(&self) -> &crate::org::allbinary::game::input::GameInputProcessor::GameInputProcessor { &self.base.base }
    fn as_GameInputProcessor_mut(&mut self) -> &mut crate::org::allbinary::game::input::GameInputProcessor::GameInputProcessor { &mut self.base.base }
    fn processAnalog(&mut self, mut allbinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>, mut xAnalogValue: i32) { Self::processAnalog(self, allbinaryLayerManager, gameKeyEvent, xAnalogValue) }
}


