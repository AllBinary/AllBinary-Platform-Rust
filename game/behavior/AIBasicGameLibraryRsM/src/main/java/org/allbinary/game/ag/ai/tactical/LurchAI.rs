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
// package org.allbinary.game.ag.ai.tactical
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Canvas::Canvas;
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

pub struct LurchAI {
    pub base: BasicAI,
    pub currentRelativeAngle: i32,
    pub currentSpeed: i32,
}

impl crate::org::allbinary::ai::ArtificialIntelligenceInterface::ArtificialIntelligenceInterface for LurchAI {
    fn processAI(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { Self::processAI(self, allBinaryLayerManager) }
}
impl crate::org::allbinary::graphics::IdentifierInterface::IdentifierInterface for LurchAI {
    fn getId(&mut self) -> i32 { self.base.getId() }
}

impl LurchAI {
    

    pub fn new_struct(mut ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut gameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInput::GameInputDyn>>>) -> Self {
        let mut this = Self {
            base: BasicAI::new_struct(ownerLayerInterface.clone(), gameInput.clone()),
            currentRelativeAngle: 0,
            currentSpeed: 0,
        };
        this.currentRelativeAngle = 0;
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut gameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInput::GameInputDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(ownerLayerInterface, gameInput))))
    }
    

    // throws Exception
    pub fn processAI(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) {
        if self.currentSpeed == -1 {
            self.currentSpeed = 5;
        } else {
            self.currentSpeed = -1;
        }
        let mut ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> = self.getOwnerLayerInterface();
        let mut x: i32 = jcall!(ownerLayerInterface).getXP();
        if jcall!(ownerLayerInterface).getXP() - self.currentSpeed <= 0 {
            self.reverse();
            self.drop();
        }
        if jcall!(ownerLayerInterface).getX2() + self.currentSpeed > jcall!(DisplayInfoSingleton::getInstance()).getLastWidth() {
            self.reverse();
            self.accelerate();
            self.drop();
        }
        if self.currentRelativeAngle == 0 {
            x += self.currentSpeed.clone();
        } else if self.currentRelativeAngle == 180 {
            x -= self.currentSpeed.clone();
        }
        jcall!(ownerLayerInterface).setPosition(x.clone(), jcall!(ownerLayerInterface).getYP(), jcall!(ownerLayerInterface).getZP());
        if self.currentRelativeAngle == 0 {
            self.base.processKeyAI(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM0.clone() }.clone());
        } else if self.currentRelativeAngle == 180 {
            self.base.processKeyAI(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_POUND.clone() }.clone());
        }
    }
    

    pub fn reverse(&mut self) {
        if self.currentRelativeAngle == 180 {
            self.currentRelativeAngle = 0;
        } else if self.currentRelativeAngle == 0 {
            self.currentRelativeAngle = 180;
        }
    }
    

    pub fn accelerate(&mut self) {
        if self.currentSpeed < 20 {
            self.currentSpeed += 1;
        }
    }
    

    pub fn drop(&mut self) {
        let mut ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> = self.getOwnerLayerInterface();
        let mut y: i32 = jcall!(ownerLayerInterface).getYP();
        if jcall!(ownerLayerInterface).getY2() + jcall!(ownerLayerInterface).getHeight() > jcall!(DisplayInfoSingleton::getInstance()).getLastHeight() {
            y = 0;
        } else {
            y += jcall!(ownerLayerInterface).getHeight() + 1;
        }
        jcall!(ownerLayerInterface).setPosition(jcall!(ownerLayerInterface).getXP(), y.clone(), jcall!(ownerLayerInterface).getZP());
    }
}
impl std::ops::Deref for LurchAI {
    type Target = BasicAI;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for LurchAI {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait LurchAIDyn: crate::org::allbinary::game::ai::BasicAI::BasicAIDyn {
    fn as_LurchAI(&self) -> &LurchAI;
    fn as_LurchAI_mut(&mut self) -> &mut LurchAI;
    fn reverse(&mut self) { LurchAI::reverse(self.as_LurchAI_mut()) }
    fn accelerate(&mut self) { LurchAI::accelerate(self.as_LurchAI_mut()) }
    fn drop(&mut self) { LurchAI::drop(self.as_LurchAI_mut()) }
}
impl LurchAIDyn for LurchAI {
    fn as_LurchAI(&self) -> &LurchAI { self }
    fn as_LurchAI_mut(&mut self) -> &mut LurchAI { self }
}

impl crate::org::allbinary::game::ai::BasicAI::BasicAIDyn for LurchAI {
    fn as_BasicAI(&self) -> &crate::org::allbinary::game::ai::BasicAI::BasicAI { &self.base }
    fn as_BasicAI_mut(&mut self) -> &mut crate::org::allbinary::game::ai::BasicAI::BasicAI { &mut self.base }
}
impl crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligenceDyn for LurchAI {
    fn as_ArtificialIntelligence(&self) -> &crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligence { &self.base.base }
    fn as_ArtificialIntelligence_mut(&mut self) -> &mut crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligence { &mut self.base.base }
    fn processAI(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { Self::processAI(self, allBinaryLayerManager) }
}
impl crate::org::allbinary::game::input::GameKeyEventSourceInterface::GameKeyEventSourceInterface for LurchAI {
    fn getSourceId(&mut self) -> i32 { crate::org::allbinary::game::ai::BasicAI::BasicAI::getSourceId(crate::org::allbinary::game::ai::BasicAI::BasicAIDyn::as_BasicAI_mut(self)) }
}


