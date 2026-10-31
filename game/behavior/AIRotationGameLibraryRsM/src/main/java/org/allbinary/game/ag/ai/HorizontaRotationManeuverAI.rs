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
// use crate::javax::microedition::lcdui::Canvas::Canvas;
use crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligence;
use crate::org::allbinary::ai::ArtificialIntelligenceTransitionInterface::ArtificialIntelligenceTransitionInterface;
use crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation;
use crate::org::allbinary::animation::RotationAnimationInterface::RotationAnimationInterface;
use crate::org::allbinary::animation::RotationAnimationInterfaceCompositeInterface::RotationAnimationInterfaceCompositeInterface;
use crate::org::allbinary::game::ai::BasicAI::BasicAI;
use crate::org::allbinary::game::input::GameInput::GameInput;
use crate::org::allbinary::game::physics::velocity::BasicVelocityProperties::BasicVelocityProperties;
use crate::org::allbinary::game::physics::velocity::VelocityInterface::VelocityInterface;
use crate::org::allbinary::game::physics::velocity::VelocityInterfaceCompositeInterface::VelocityInterfaceCompositeInterface;
use crate::org::allbinary::graphics::displayable::DisplayInfoSingleton::DisplayInfoSingleton;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager;
use crate::org::allbinary::math::AngleIncrementInfo::AngleIncrementInfo;
use crate::org::allbinary::util::CircularIndexUtil::CircularIndexUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct HorizontaRotationManeuverAI {
    pub base: BasicAI,
    pub circularIndexUtil: Option<Rc<RefCell<CircularIndexUtil>>>,
    pub currentSpeedDivisor: i32,
    pub rotationAnimationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn>>>,
    pub baseVelocityInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::physics::velocity::BasicVelocityProperties::BasicVelocityPropertiesDyn>>>,
    pub velocityInterface: Option<Rc<RefCell<dyn VelocityInterface>>>,
}

impl ArtificialIntelligenceTransitionInterface for HorizontaRotationManeuverAI {
    fn transition(&mut self) { Self::transition(self) }
}
impl crate::org::allbinary::ai::ArtificialIntelligenceInterface::ArtificialIntelligenceInterface for HorizontaRotationManeuverAI {
    fn processAI(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { Self::processAI(self, allBinaryLayerManager) }
}
impl crate::org::allbinary::graphics::IdentifierInterface::IdentifierInterface for HorizontaRotationManeuverAI {
    fn getId(&mut self) -> i32 { Self::getId(self) }
}

impl HorizontaRotationManeuverAI {
    

    pub fn new_struct(mut ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut gameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInput::GameInputDyn>>>) -> Self {
        let mut this = Self {
            base: BasicAI::new_struct(ownerLayerInterface.clone(), gameInput.clone()),
            circularIndexUtil: None,
            currentSpeedDivisor: 0,
            rotationAnimationInterface: None,
            baseVelocityInterface: None,
            velocityInterface: None,
        };
        this.circularIndexUtil = CircularIndexUtil::createInstanceAt(10, i32::MAX.clone());
        this.currentSpeedDivisor = 5;
        // super(...) is in the struct literal above
        let mut rotationAnimationInterfaceCompositeInterface: Option<Rc<RefCell<dyn RotationAnimationInterfaceCompositeInterface>>> = (this.getOwnerLayerInterface()).clone() /* as RotationAnimationInterfaceCompositeInterface */;
        this.rotationAnimationInterface = jcall!(rotationAnimationInterfaceCompositeInterface).getRotationAnimationInterface();
        let mut velocityInterfaceCompositeInterface: Option<Rc<RefCell<dyn VelocityInterfaceCompositeInterface>>> = (this.getOwnerLayerInterface()).clone() /* as VelocityInterfaceCompositeInterface */;
        this.baseVelocityInterface = jcall!(velocityInterfaceCompositeInterface).getVelocityProperties();
        this.velocityInterface = (this.baseVelocityInterface).clone() /* as VelocityInterface */;
        let mut rotationAnimationInterface: Option<Rc<RefCell<dyn RotationAnimationInterface>>> = (this.rotationAnimationInterface).clone() /* as RotationAnimationInterface */;
        let mut angleIncrementInfo: Option<Rc<RefCell<AngleIncrementInfo>>> = jcall!(jcall!(rotationAnimationInterface).as_IndexedAnimation_mut().getAngleInfoP()).getAngleIncrementInfo();
        jcall!(rotationAnimationInterfaceCompositeInterface).setFrame(((jcall!(angleIncrementInfo).RIGHT_FRAME).unwrap() as i32));
        this
    }

    pub fn new(mut ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut gameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInput::GameInputDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(ownerLayerInterface, gameInput))))
    }
    

    pub fn transition(&mut self) {
        let mut rotationAnimationInterface: Option<Rc<RefCell<dyn RotationAnimationInterface>>> = (self.rotationAnimationInterface).clone() /* as RotationAnimationInterface */;
        let mut angleIncrementInfo: Option<Rc<RefCell<AngleIncrementInfo>>> = jcall!(jcall!(rotationAnimationInterface).as_IndexedAnimation_mut().getAngleInfoP()).getAngleIncrementInfo();
        let mut rotationAnimationInterfaceCompositeInterface: Option<Rc<RefCell<dyn RotationAnimationInterfaceCompositeInterface>>> = (self.getOwnerLayerInterface()).clone() /* as RotationAnimationInterfaceCompositeInterface */;
        jcall!(rotationAnimationInterfaceCompositeInterface).setFrame(((jcall!(angleIncrementInfo).RIGHT_FRAME).unwrap() as i32));
        let mut allbinaryLayer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> = self.getOwnerLayerInterface();
        jcall!(allbinaryLayer).setPosition(jcall!(allbinaryLayer).getXP(), 1, jcall!(allbinaryLayer).getZP());
    }
    

    // throws Exception
    pub fn processAI(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) {
        let mut rotationAnimationInterface: Option<Rc<RefCell<dyn RotationAnimationInterface>>> = ((self.rotationAnimationInterface).clone() /* as RotationAnimationInterface */);
        let mut angleIncrementInfo: Option<Rc<RefCell<AngleIncrementInfo>>> = jcall!(jcall!(rotationAnimationInterface).as_IndexedAnimation_mut().getAngleInfoP()).getAngleIncrementInfo();
        let mut frame: i32 = jcall!(self.rotationAnimationInterface).getFrame();
        let mut ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> = self.getOwnerLayerInterface();
        if jcall!(ownerLayerInterface).getXP() - jcall!(ownerLayerInterface).getWidth() <= 0 && frame == ((jcall!(angleIncrementInfo).LEFT_FRAME).unwrap() as i32) {
            self.reverse();
            self.drop();
        }
        let mut displayInfo: Option<Rc<RefCell<DisplayInfoSingleton>>> = DisplayInfoSingleton::getInstance();
        if jcall!(ownerLayerInterface).getX2() + jcall!(ownerLayerInterface).getWidth() > jcall!(displayInfo).getLastWidth() && frame == ((jcall!(angleIncrementInfo).RIGHT_FRAME).unwrap() as i32) {
            self.reverse();
            self.accelerate();
            self.drop();
        }
        let mut index: i32 = jcall!(self.circularIndexUtil).getIndex();
        if index % self.currentSpeedDivisor == 0 && index % 2 == 0 {
            self.base.processKeyAI(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_UP.clone() }.clone());
            { let java_receiver = self.velocityInterface.clone(); let java_arguments = (jcall!(self.velocityInterface).getMaxForwardVelocity() / self.currentSpeedDivisor,); jcall!(java_receiver).limitMaxXYVelocity(java_arguments.0); };
        }
        jcall!(self.circularIndexUtil).next();
        if frame == ((jcall!(angleIncrementInfo).LEFT_FRAME).unwrap() as i32) {
            self.base.processKeyAI(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM0.clone() }.clone());
        } else if frame == ((jcall!(angleIncrementInfo).RIGHT_FRAME).unwrap() as i32) {
            self.base.processKeyAI(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_POUND.clone() }.clone());
        }
    }
    

    pub fn reverse(&mut self) {
        let mut rotationAnimationInterface: Option<Rc<RefCell<dyn RotationAnimationInterface>>> = (self.rotationAnimationInterface).clone() /* as RotationAnimationInterface */;
        let mut angleIncrementInfo: Option<Rc<RefCell<AngleIncrementInfo>>> = jcall!(jcall!(rotationAnimationInterface).as_IndexedAnimation_mut().getAngleInfoP()).getAngleIncrementInfo();
        let mut frame: i32 = jcall!(self.rotationAnimationInterface).getFrame();
        if frame == ((jcall!(angleIncrementInfo).LEFT_FRAME).unwrap() as i32) {
            let mut rotationAnimationInterfaceCompositeInterface: Option<Rc<RefCell<dyn RotationAnimationInterfaceCompositeInterface>>> = (self.getOwnerLayerInterface()).clone() /* as RotationAnimationInterfaceCompositeInterface */;
            jcall!(rotationAnimationInterfaceCompositeInterface).setFrame(((jcall!(angleIncrementInfo).RIGHT_FRAME).unwrap() as i32));
            jcall!(self.baseVelocityInterface).zero();
        } else if frame == ((jcall!(angleIncrementInfo).RIGHT_FRAME).unwrap() as i32) {
            let mut rotationAnimationInterfaceCompositeInterface: Option<Rc<RefCell<dyn RotationAnimationInterfaceCompositeInterface>>> = (self.getOwnerLayerInterface()).clone() /* as RotationAnimationInterfaceCompositeInterface */;
            jcall!(rotationAnimationInterfaceCompositeInterface).setFrame(((jcall!(angleIncrementInfo).LEFT_FRAME).unwrap() as i32));
            jcall!(self.baseVelocityInterface).zero();
        }
    }
    

    pub fn accelerate(&mut self) {
        if self.currentSpeedDivisor > 1 {
            self.currentSpeedDivisor -= 1;
        }
    }
    

    pub fn drop(&mut self) {
        let mut displayInfo: Option<Rc<RefCell<DisplayInfoSingleton>>> = DisplayInfoSingleton::getInstance();
        let mut ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> = self.getOwnerLayerInterface();
        let mut y: i32 = jcall!(ownerLayerInterface).getYP();
        if jcall!(ownerLayerInterface).getY2() + jcall!(ownerLayerInterface).getHeight() > jcall!(displayInfo).getLastHeight() {
            y = 0;
        } else {
            y += jcall!(ownerLayerInterface).getHeight() + 1;
        }
        jcall!(ownerLayerInterface).setPosition(jcall!(ownerLayerInterface).getXP(), y.clone(), jcall!(ownerLayerInterface).getZP());
    }
    

    pub fn getId(&mut self) -> i32 {
        return crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligence_AI_ID.with(Clone::clone).clone();
    }
}
impl std::ops::Deref for HorizontaRotationManeuverAI {
    type Target = BasicAI;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for HorizontaRotationManeuverAI {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligenceDyn for HorizontaRotationManeuverAI {
    fn as_ArtificialIntelligence(&self) -> &crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligence { &self.base.base }
    fn as_ArtificialIntelligence_mut(&mut self) -> &mut crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligence { &mut self.base.base }
    fn processAI(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { Self::processAI(self, allBinaryLayerManager) }
    fn getId(&mut self) -> i32 { Self::getId(self) }
}
impl crate::org::allbinary::game::ai::BasicAI::BasicAIDyn for HorizontaRotationManeuverAI {
    fn as_BasicAI(&self) -> &crate::org::allbinary::game::ai::BasicAI::BasicAI { &self.base }
    fn as_BasicAI_mut(&mut self) -> &mut crate::org::allbinary::game::ai::BasicAI::BasicAI { &mut self.base }
}
impl crate::org::allbinary::game::input::GameKeyEventSourceInterface::GameKeyEventSourceInterface for HorizontaRotationManeuverAI {
    fn getSourceId(&mut self) -> i32 { crate::org::allbinary::game::ai::BasicAI::BasicAI::getSourceId(crate::org::allbinary::game::ai::BasicAI::BasicAIDyn::as_BasicAI_mut(self)) }
}


