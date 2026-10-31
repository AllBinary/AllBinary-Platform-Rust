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
// package org.allbinary.game.ag.ai.sequence
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::ai::ArtificialIntelligenceInterface::ArtificialIntelligenceInterface;
use crate::org::allbinary::game::input::GameInput::GameInput;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager;
use super::SequenceAI::SequenceAI;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ComboSequenceAI {
    pub base: SequenceAI,
}

impl ArtificialIntelligenceInterface for ComboSequenceAI {
    fn processAI(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { Self::processAI(self, allBinaryLayerManager) }
}
impl crate::org::allbinary::graphics::IdentifierInterface::IdentifierInterface for ComboSequenceAI {
    fn getId(&mut self) -> i32 { self.base.getId() }
}

impl ComboSequenceAI {
    

    pub fn new_struct(mut artificialIntelligenceInterface: Vec<Option<Rc<RefCell<dyn ArtificialIntelligenceInterface>>>>, mut ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut gameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInput::GameInputDyn>>>) -> Self {
        let mut this = Self {
            base: SequenceAI::new_struct(artificialIntelligenceInterface.clone(), ownerLayerInterface.clone(), gameInput.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut artificialIntelligenceInterface: Vec<Option<Rc<RefCell<dyn ArtificialIntelligenceInterface>>>>, mut ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut gameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInput::GameInputDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(artificialIntelligenceInterface, ownerLayerInterface, gameInput))))
    }
    

    // throws Exception
    pub fn processAI(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) {
        let mut artificialIntelligenceInterfaceArray: Vec<Option<Rc<RefCell<dyn ArtificialIntelligenceInterface>>>> = self.getArtificialIntelligenceInterface();
        let mut size: i32 = (artificialIntelligenceInterfaceArray.len() as i32).clone();
        for index in (0..size) {
            self.setIndex(index.clone());
            self.base.processAI(allBinaryLayerManager.clone());
        }
    }
}
impl std::ops::Deref for ComboSequenceAI {
    type Target = SequenceAI;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ComboSequenceAI {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::ag::ai::sequence::SequenceAI::SequenceAIDyn for ComboSequenceAI {
    fn as_SequenceAI(&self) -> &crate::org::allbinary::game::ag::ai::sequence::SequenceAI::SequenceAI { &self.base }
    fn as_SequenceAI_mut(&mut self) -> &mut crate::org::allbinary::game::ag::ai::sequence::SequenceAI::SequenceAI { &mut self.base }
}
impl crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligenceDyn for ComboSequenceAI {
    fn as_ArtificialIntelligence(&self) -> &crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligence { &self.base.base.base.base }
    fn as_ArtificialIntelligence_mut(&mut self) -> &mut crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligence { &mut self.base.base.base.base }
    fn processAI(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { Self::processAI(self, allBinaryLayerManager) }
}
impl crate::org::allbinary::game::ag::ai::ArrayAI::ArrayAIDyn for ComboSequenceAI {
    fn as_ArrayAI(&self) -> &crate::org::allbinary::game::ag::ai::ArrayAI::ArrayAI { &self.base.base }
    fn as_ArrayAI_mut(&mut self) -> &mut crate::org::allbinary::game::ag::ai::ArrayAI::ArrayAI { &mut self.base.base }
}
impl crate::org::allbinary::game::ai::BasicAI::BasicAIDyn for ComboSequenceAI {
    fn as_BasicAI(&self) -> &crate::org::allbinary::game::ai::BasicAI::BasicAI { &self.base.base.base }
    fn as_BasicAI_mut(&mut self) -> &mut crate::org::allbinary::game::ai::BasicAI::BasicAI { &mut self.base.base.base }
    fn getName(&mut self) -> String { crate::org::allbinary::game::ag::ai::sequence::SequenceAI::SequenceAI::getName(crate::org::allbinary::game::ag::ai::sequence::SequenceAI::SequenceAIDyn::as_SequenceAI_mut(self)) }
}
impl crate::org::allbinary::game::input::GameKeyEventSourceInterface::GameKeyEventSourceInterface for ComboSequenceAI {
    fn getSourceId(&mut self) -> i32 { crate::org::allbinary::game::ai::BasicAI::BasicAI::getSourceId(crate::org::allbinary::game::ai::BasicAI::BasicAIDyn::as_BasicAI_mut(self)) }
}


