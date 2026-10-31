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
use crate::org::allbinary::game::physics::velocity::BasicVelocityProperties::BasicVelocityProperties;
use crate::org::allbinary::game::physics::velocity::VelocityInterfaceCompositeInterface::VelocityInterfaceCompositeInterface;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager;
use crate::org::allbinary::logic::math::BasicDecimal::BasicDecimal;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct UpDownVectorAI {
    pub base: BasicAI,
    pub velocityInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::physics::velocity::BasicVelocityProperties::BasicVelocityPropertiesDyn>>>,
    pub index: i32,
}

impl crate::org::allbinary::ai::ArtificialIntelligenceInterface::ArtificialIntelligenceInterface for UpDownVectorAI {
    fn processAI(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { Self::processAI(self, allBinaryLayerManager) }
}
impl crate::org::allbinary::graphics::IdentifierInterface::IdentifierInterface for UpDownVectorAI {
    fn getId(&mut self) -> i32 { self.base.getId() }
}

impl UpDownVectorAI {
    

    pub fn new_struct(mut ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut gameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInput::GameInputDyn>>>) -> Self {
        let mut this = Self {
            base: BasicAI::new_struct(ownerLayerInterface.clone(), gameInput.clone()),
            velocityInterface: None,
            index: 0,
        };
        this.index = 0;
        // super(...) is in the struct literal above
        let mut velocityInterfaceCompositeInterface: Option<Rc<RefCell<dyn VelocityInterfaceCompositeInterface>>> = (this.getOwnerLayerInterface()).clone() /* as VelocityInterfaceCompositeInterface */;
        this.velocityInterface = jcall!(velocityInterfaceCompositeInterface).getVelocityProperties();
        jcall!(jcall!(this.velocityInterface).getVelocityYBasicDecimalP()).setint(0);
        this
    }

    pub fn new(mut ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut gameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInput::GameInputDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(ownerLayerInterface, gameInput))))
    }
    

    // throws Exception
    pub fn processAI(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) {
        let mut y: i32 = jcall!(self.getOwnerLayerInterface()).getHeight() * 270;
        if self.index < 5 {
            let mut basicDecimal: Option<Rc<RefCell<BasicDecimal>>> = jcall!(self.velocityInterface).getVelocityYBasicDecimalP();
            jcall!(basicDecimal).setint(0);
            jcall!(basicDecimal).addint(y.clone());
            self.index += 1;
        } else if self.index < 10 {
            let mut basicDecimal: Option<Rc<RefCell<BasicDecimal>>> = jcall!(self.velocityInterface).getVelocityYBasicDecimalP();
            jcall!(basicDecimal).setint(0);
            jcall!(basicDecimal).subtractint(y.clone());
            self.index += 1;
        } else {
            self.index = 0;
            self.processAI(allBinaryLayerManager.clone());
        }
    }
}
impl std::ops::Deref for UpDownVectorAI {
    type Target = BasicAI;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for UpDownVectorAI {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligenceDyn for UpDownVectorAI {
    fn as_ArtificialIntelligence(&self) -> &crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligence { &self.base.base }
    fn as_ArtificialIntelligence_mut(&mut self) -> &mut crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligence { &mut self.base.base }
    fn processAI(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { Self::processAI(self, allBinaryLayerManager) }
}
impl crate::org::allbinary::game::ai::BasicAI::BasicAIDyn for UpDownVectorAI {
    fn as_BasicAI(&self) -> &crate::org::allbinary::game::ai::BasicAI::BasicAI { &self.base }
    fn as_BasicAI_mut(&mut self) -> &mut crate::org::allbinary::game::ai::BasicAI::BasicAI { &mut self.base }
}
impl crate::org::allbinary::game::input::GameKeyEventSourceInterface::GameKeyEventSourceInterface for UpDownVectorAI {
    fn getSourceId(&mut self) -> i32 { crate::org::allbinary::game::ai::BasicAI::BasicAI::getSourceId(crate::org::allbinary::game::ai::BasicAI::BasicAIDyn::as_BasicAI_mut(self)) }
}


