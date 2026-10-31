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
use crate::org::allbinary::game::ai::BasicAI::BasicAI;
use crate::org::allbinary::game::input::GameInput::GameInput;
use crate::org::allbinary::graphics::displayable::DisplayInfoSingleton::DisplayInfoSingleton;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ScrollOverAtHalfAI {
    pub base: BasicAI,
}

impl crate::org::allbinary::ai::ArtificialIntelligenceInterface::ArtificialIntelligenceInterface for ScrollOverAtHalfAI {
    fn processAI(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { Self::processAI(self, allBinaryLayerManager) }
}
impl crate::org::allbinary::graphics::IdentifierInterface::IdentifierInterface for ScrollOverAtHalfAI {
    fn getId(&mut self) -> i32 { self.base.getId() }
}

impl ScrollOverAtHalfAI {
    

    pub fn new_struct(mut ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut gameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInput::GameInputDyn>>>) -> Self {
        let mut this = Self {
            base: BasicAI::new_struct(ownerLayerInterface.clone(), gameInput.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut gameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInput::GameInputDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(ownerLayerInterface, gameInput))))
    }
    

    // throws Exception
    pub fn processAI(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) {
        let mut ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> = self.getOwnerLayerInterface();
        let mut x: i32 = jcall!(ownerLayerInterface).getXP();
        let mut y: i32 = jcall!(ownerLayerInterface).getYP();
        let mut halfWidth: i32 = jcall!(ownerLayerInterface).getHalfWidth();
        let mut halfHeight: i32 = jcall!(ownerLayerInterface).getHalfHeight();
        let mut displayInfo: Option<Rc<RefCell<DisplayInfoSingleton>>> = DisplayInfoSingleton::getInstance();
        if x > jcall!(displayInfo).getLastWidth() - halfWidth {
            jcall!(ownerLayerInterface).setPosition(halfWidth + 1, y.clone(), jcall!(ownerLayerInterface).getZP());
        }
        if y > jcall!(displayInfo).getLastHeight() - halfHeight {
            jcall!(ownerLayerInterface).setPosition(x.clone(), halfHeight + 1, jcall!(ownerLayerInterface).getZP());
        }
        if x < -halfWidth {
            jcall!(ownerLayerInterface).setPosition(jcall!(displayInfo).getLastWidth() - halfWidth, y.clone(), jcall!(ownerLayerInterface).getZP());
        }
        if y < -halfHeight {
            jcall!(ownerLayerInterface).setPosition(x.clone(), jcall!(displayInfo).getLastHeight() - halfHeight, jcall!(ownerLayerInterface).getZP());
        }
    }
}
impl std::ops::Deref for ScrollOverAtHalfAI {
    type Target = BasicAI;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ScrollOverAtHalfAI {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligenceDyn for ScrollOverAtHalfAI {
    fn as_ArtificialIntelligence(&self) -> &crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligence { &self.base.base }
    fn as_ArtificialIntelligence_mut(&mut self) -> &mut crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligence { &mut self.base.base }
    fn processAI(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { Self::processAI(self, allBinaryLayerManager) }
}
impl crate::org::allbinary::game::ai::BasicAI::BasicAIDyn for ScrollOverAtHalfAI {
    fn as_BasicAI(&self) -> &crate::org::allbinary::game::ai::BasicAI::BasicAI { &self.base }
    fn as_BasicAI_mut(&mut self) -> &mut crate::org::allbinary::game::ai::BasicAI::BasicAI { &mut self.base }
}
impl crate::org::allbinary::game::input::GameKeyEventSourceInterface::GameKeyEventSourceInterface for ScrollOverAtHalfAI {
    fn getSourceId(&mut self) -> i32 { crate::org::allbinary::game::ai::BasicAI::BasicAI::getSourceId(crate::org::allbinary::game::ai::BasicAI::BasicAIDyn::as_BasicAI_mut(self)) }
}


