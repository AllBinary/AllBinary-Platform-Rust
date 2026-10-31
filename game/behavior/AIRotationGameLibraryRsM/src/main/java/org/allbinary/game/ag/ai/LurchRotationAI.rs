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
use crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation;
use crate::org::allbinary::animation::RotationAnimationInterface::RotationAnimationInterface;
use crate::org::allbinary::animation::RotationAnimationInterfaceCompositeInterface::RotationAnimationInterfaceCompositeInterface;
use crate::org::allbinary::game::ag::ai::tactical::LurchAI::LurchAI;
use crate::org::allbinary::game::input::GameInput::GameInput;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::math::AngleIncrementInfo::AngleIncrementInfo;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LurchRotationAI {
    pub base: LurchAI,
    pub rotationAnimationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn>>>,
}

impl crate::org::allbinary::graphics::IdentifierInterface::IdentifierInterface for LurchRotationAI {
    fn getId(&mut self) -> i32 { self.base.getId() }
}

impl LurchRotationAI {
    

    pub fn new_struct(mut ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut gameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInput::GameInputDyn>>>) -> Self {
        let mut this = Self {
            base: LurchAI::new_struct(ownerLayerInterface.clone(), gameInput.clone()),
            rotationAnimationInterface: None,
        };
        // super(...) is in the struct literal above
        let mut rotationAnimationInterfaceCompositeInterface: Option<Rc<RefCell<dyn RotationAnimationInterfaceCompositeInterface>>> = (this.getOwnerLayerInterface()).clone() /* as RotationAnimationInterfaceCompositeInterface */;
        this.rotationAnimationInterface = jcall!(rotationAnimationInterfaceCompositeInterface).getRotationAnimationInterface();
        this.update();
        this
    }

    pub fn new(mut ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut gameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInput::GameInputDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(ownerLayerInterface, gameInput))))
    }
    

    pub fn reverse(&mut self) {
        self.base.reverse();
        self.update();
    }
    

    pub fn update(&mut self) {
        if self.currentRelativeAngle == 0 {
            let mut rotationAnimationInterface: Option<Rc<RefCell<dyn RotationAnimationInterface>>> = (self.rotationAnimationInterface).clone() /* as RotationAnimationInterface */;
            let mut angleIncrementInfo: Option<Rc<RefCell<AngleIncrementInfo>>> = jcall!(jcall!(rotationAnimationInterface).as_IndexedAnimation_mut().getAngleInfoP()).getAngleIncrementInfo();
            let mut rotationAnimationInterfaceCompositeInterface: Option<Rc<RefCell<dyn RotationAnimationInterfaceCompositeInterface>>> = (self.getOwnerLayerInterface()).clone() /* as RotationAnimationInterfaceCompositeInterface */;
            jcall!(rotationAnimationInterfaceCompositeInterface).setFrame(((jcall!(angleIncrementInfo).RIGHT_FRAME).unwrap() as i32));
        } else if self.currentRelativeAngle == 180 {
            let mut rotationAnimationInterface: Option<Rc<RefCell<dyn RotationAnimationInterface>>> = (self.rotationAnimationInterface).clone() /* as RotationAnimationInterface */;
            let mut angleIncrementInfo: Option<Rc<RefCell<AngleIncrementInfo>>> = jcall!(jcall!(rotationAnimationInterface).as_IndexedAnimation_mut().getAngleInfoP()).getAngleIncrementInfo();
            let mut rotationAnimationInterfaceCompositeInterface: Option<Rc<RefCell<dyn RotationAnimationInterfaceCompositeInterface>>> = (self.getOwnerLayerInterface()).clone() /* as RotationAnimationInterfaceCompositeInterface */;
            jcall!(rotationAnimationInterfaceCompositeInterface).setFrame(((jcall!(angleIncrementInfo).LEFT_FRAME).unwrap() as i32));
        }
    }
}
impl std::ops::Deref for LurchRotationAI {
    type Target = LurchAI;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for LurchRotationAI {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligenceDyn for LurchRotationAI {
    fn as_ArtificialIntelligence(&self) -> &crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligence { &self.base.base.base }
    fn as_ArtificialIntelligence_mut(&mut self) -> &mut crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligence { &mut self.base.base.base }
    fn processAI(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { crate::org::allbinary::game::ag::ai::tactical::LurchAI::LurchAI::processAI(crate::org::allbinary::game::ag::ai::tactical::LurchAI::LurchAIDyn::as_LurchAI_mut(self), p1) }
}
impl crate::org::allbinary::game::ai::BasicAI::BasicAIDyn for LurchRotationAI {
    fn as_BasicAI(&self) -> &crate::org::allbinary::game::ai::BasicAI::BasicAI { &self.base.base }
    fn as_BasicAI_mut(&mut self) -> &mut crate::org::allbinary::game::ai::BasicAI::BasicAI { &mut self.base.base }
}
impl crate::org::allbinary::game::ag::ai::tactical::LurchAI::LurchAIDyn for LurchRotationAI {
    fn as_LurchAI(&self) -> &crate::org::allbinary::game::ag::ai::tactical::LurchAI::LurchAI { &self.base }
    fn as_LurchAI_mut(&mut self) -> &mut crate::org::allbinary::game::ag::ai::tactical::LurchAI::LurchAI { &mut self.base }
    fn reverse(&mut self) { Self::reverse(self) }
}
impl crate::org::allbinary::game::input::GameKeyEventSourceInterface::GameKeyEventSourceInterface for LurchRotationAI {
    fn getSourceId(&mut self) -> i32 { crate::org::allbinary::game::ai::BasicAI::BasicAI::getSourceId(crate::org::allbinary::game::ai::BasicAI::BasicAIDyn::as_BasicAI_mut(self)) }
}


