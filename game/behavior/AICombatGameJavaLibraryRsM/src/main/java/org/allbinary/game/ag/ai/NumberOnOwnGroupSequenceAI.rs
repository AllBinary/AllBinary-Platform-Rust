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
// package org.allbinary.game.ag.ai
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::ai::ArtificialIntelligenceInterface::ArtificialIntelligenceInterface;
use crate::org::allbinary::game::ag::ai::sequence::SequenceAI::SequenceAI;
use crate::org::allbinary::game::input::GameInput::GameInput;
use crate::org::allbinary::game::layer::identification::GroupLayerManagerListener::GroupLayerManagerListener;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NumberOnOwnGroupSequenceAI {
    pub base: SequenceAI,
    pub numberOnSameTeam: Vec<Option<i32>>,
}

impl ArtificialIntelligenceInterface for NumberOnOwnGroupSequenceAI {
    fn processAI(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { Self::processAI(self, allBinaryLayerManager) }
}
impl crate::org::allbinary::graphics::IdentifierInterface::IdentifierInterface for NumberOnOwnGroupSequenceAI {
    fn getId(&mut self) -> i32 { self.base.getId() }
}

impl NumberOnOwnGroupSequenceAI {
    

    pub fn new_struct(mut numberOnSameTeam: Vec<Option<i32>>, mut artificialIntelligenceInterface: Vec<Option<Rc<RefCell<dyn ArtificialIntelligenceInterface>>>>, mut ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut gameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInput::GameInputDyn>>>) -> Self {
        let mut this = Self {
            base: SequenceAI::new_struct(artificialIntelligenceInterface.clone(), ownerLayerInterface.clone(), gameInput.clone()),
            numberOnSameTeam: Vec::new(),
        };
        // super(...) is in the struct literal above
        this.numberOnSameTeam = numberOnSameTeam.clone();
        this
    }

    pub fn new(mut numberOnSameTeam: Vec<Option<i32>>, mut artificialIntelligenceInterface: Vec<Option<Rc<RefCell<dyn ArtificialIntelligenceInterface>>>>, mut ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut gameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInput::GameInputDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(numberOnSameTeam, artificialIntelligenceInterface, ownerLayerInterface, gameInput))))
    }
    

    // throws Exception
    pub fn processAI(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) {
        let mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> = self.getOwnerLayerInterface();
        let mut index: i32 = self.getIndex();
        if (self.numberOnSameTeam.len() as i32) > index {
            let mut size: i32 = jcall!(GroupLayerManagerListener::getInstance()).getGroupSizeFromInterface(layerInterface.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::identification::GroupInterfaceCompositeInterface::GroupInterfaceCompositeInterface>>));
            if ((self.numberOnSameTeam[(index) as usize]).unwrap() as i32) > size {
                self.next();
            }
        }
        self.base.processAI(allBinaryLayerManager.clone());
    }
}
impl std::ops::Deref for NumberOnOwnGroupSequenceAI {
    type Target = SequenceAI;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for NumberOnOwnGroupSequenceAI {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::ag::ai::ArrayAI::ArrayAIDyn for NumberOnOwnGroupSequenceAI {
    fn as_ArrayAI(&self) -> &crate::org::allbinary::game::ag::ai::ArrayAI::ArrayAI { &self.base.base }
    fn as_ArrayAI_mut(&mut self) -> &mut crate::org::allbinary::game::ag::ai::ArrayAI::ArrayAI { &mut self.base.base }
}
impl crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligenceDyn for NumberOnOwnGroupSequenceAI {
    fn as_ArtificialIntelligence(&self) -> &crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligence { &self.base.base.base.base }
    fn as_ArtificialIntelligence_mut(&mut self) -> &mut crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligence { &mut self.base.base.base.base }
    fn processAI(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { Self::processAI(self, allBinaryLayerManager) }
}
impl crate::org::allbinary::game::ai::BasicAI::BasicAIDyn for NumberOnOwnGroupSequenceAI {
    fn as_BasicAI(&self) -> &crate::org::allbinary::game::ai::BasicAI::BasicAI { &self.base.base.base }
    fn as_BasicAI_mut(&mut self) -> &mut crate::org::allbinary::game::ai::BasicAI::BasicAI { &mut self.base.base.base }
    fn getName(&mut self) -> String { crate::org::allbinary::game::ag::ai::sequence::SequenceAI::SequenceAI::getName(crate::org::allbinary::game::ag::ai::sequence::SequenceAI::SequenceAIDyn::as_SequenceAI_mut(self)) }
}
impl crate::org::allbinary::game::ag::ai::sequence::SequenceAI::SequenceAIDyn for NumberOnOwnGroupSequenceAI {
    fn as_SequenceAI(&self) -> &crate::org::allbinary::game::ag::ai::sequence::SequenceAI::SequenceAI { &self.base }
    fn as_SequenceAI_mut(&mut self) -> &mut crate::org::allbinary::game::ag::ai::sequence::SequenceAI::SequenceAI { &mut self.base }
}
impl crate::org::allbinary::game::input::GameKeyEventSourceInterface::GameKeyEventSourceInterface for NumberOnOwnGroupSequenceAI {
    fn getSourceId(&mut self) -> i32 { crate::org::allbinary::game::ai::BasicAI::BasicAI::getSourceId(crate::org::allbinary::game::ai::BasicAI::BasicAIDyn::as_BasicAI_mut(self)) }
}


