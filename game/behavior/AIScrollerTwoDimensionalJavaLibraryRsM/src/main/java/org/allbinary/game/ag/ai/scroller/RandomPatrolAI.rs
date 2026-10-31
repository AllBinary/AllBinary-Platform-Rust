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
// package org.allbinary.game.ag.ai.scroller
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Canvas::Canvas;
use crate::org::allbinary::game::input::GameInput::GameInput;
use crate::org::allbinary::game::rand::MyRandomFactory::MyRandomFactory;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use super::BasePatrolAI::BasePatrolAI;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RandomPatrolAI {
    pub base: BasePatrolAI,
    pub changedDirection: bool,
}

impl crate::org::allbinary::ai::ArtificialIntelligenceInterface::ArtificialIntelligenceInterface for RandomPatrolAI {
    fn processAI(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { Self::processAI(self, allBinaryLayerManager) }
}
impl crate::org::allbinary::graphics::IdentifierInterface::IdentifierInterface for RandomPatrolAI {
    fn getId(&mut self) -> i32 { self.base.getId() }
}

impl RandomPatrolAI {
    

    pub fn new_struct(mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>, mut ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut gameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInput::GameInputDyn>>>) -> Self {
        let mut this = Self {
            base: BasePatrolAI::new_struct(hashtable.clone(), ownerLayerInterface.clone(), gameInput.clone()),
            changedDirection: false,
        };
        this.changedDirection = true;
        // super(...) is in the struct literal above
        this.updateRandomDistance();
        this
    }

    pub fn new(mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>, mut ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut gameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInput::GameInputDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(hashtable, ownerLayerInterface, gameInput))))
    }
    

    pub fn updateRandomDistance(&mut self) {
        self.currentDistance = jcall!(MyRandomFactory::getInstance()).getAbsoluteNextInt(((self.maxDistance).unwrap() as i32));
    }
    

    // throws Exception
    pub fn processAI(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) {
        self.base.update();
        self.base.processKeyAI(self.keyDirection.clone());
        if self.changedDirection {
            self.base.processKeyAI(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM1.clone() }.clone());
            self.updateRandomDistance();
            self.changedDirection = false;
        }
    }
    

    pub fn nextDirection(&mut self) {
        self.base.nextDirection();
        self.changedDirection = true;
    }
}
impl std::ops::Deref for RandomPatrolAI {
    type Target = BasePatrolAI;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for RandomPatrolAI {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::ag::ai::scroller::BasePatrolAI::BasePatrolAIDyn for RandomPatrolAI {
    fn as_BasePatrolAI(&self) -> &crate::org::allbinary::game::ag::ai::scroller::BasePatrolAI::BasePatrolAI { &self.base }
    fn as_BasePatrolAI_mut(&mut self) -> &mut crate::org::allbinary::game::ag::ai::scroller::BasePatrolAI::BasePatrolAI { &mut self.base }
    fn nextDirection(&mut self) { Self::nextDirection(self) }
}
impl crate::org::allbinary::game::ai::BasicAI::BasicAIDyn for RandomPatrolAI {
    fn as_BasicAI(&self) -> &crate::org::allbinary::game::ai::BasicAI::BasicAI { &self.base.base }
    fn as_BasicAI_mut(&mut self) -> &mut crate::org::allbinary::game::ai::BasicAI::BasicAI { &mut self.base.base }
}
impl crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligenceDyn for RandomPatrolAI {
    fn as_ArtificialIntelligence(&self) -> &crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligence { &self.base.base.base }
    fn as_ArtificialIntelligence_mut(&mut self) -> &mut crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligence { &mut self.base.base.base }
    fn processAI(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { Self::processAI(self, allBinaryLayerManager) }
}
impl crate::org::allbinary::game::input::GameKeyEventSourceInterface::GameKeyEventSourceInterface for RandomPatrolAI {
    fn getSourceId(&mut self) -> i32 { crate::org::allbinary::game::ai::BasicAI::BasicAI::getSourceId(crate::org::allbinary::game::ai::BasicAI::BasicAIDyn::as_BasicAI_mut(self)) }
}


