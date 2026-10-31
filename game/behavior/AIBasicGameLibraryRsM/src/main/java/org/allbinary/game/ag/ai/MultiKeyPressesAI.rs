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
use crate::org::allbinary::game::input::GameInput::GameInput;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager;
use crate::org::allbinary::time::GameTickTimeDelayHelper::GameTickTimeDelayHelper;
use crate::org::allbinary::time::GameTickTimeDelayHelperFactory::GameTickTimeDelayHelperFactory;
use crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelper;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use super::KeyPressesAI::KeyPressesAI;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MultiKeyPressesAI {
    pub base: KeyPressesAI,
    pub toggleTimeHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>>,
    pub gameTickTimeDelayHelper: Option<Rc<RefCell<GameTickTimeDelayHelper>>>,
}

impl crate::org::allbinary::ai::ArtificialIntelligenceInterface::ArtificialIntelligenceInterface for MultiKeyPressesAI {
    fn processAI(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { Self::processAI(self, allBinaryLayerManager) }
}
impl crate::org::allbinary::graphics::IdentifierInterface::IdentifierInterface for MultiKeyPressesAI {
    fn getId(&mut self) -> i32 { self.base.getId() }
}

impl MultiKeyPressesAI {
    

    pub fn new_struct(mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>, mut ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut gameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInput::GameInputDyn>>>) -> Self {
        let mut this = Self {
            base: KeyPressesAI::new_struct(hashtable.clone(), ownerLayerInterface.clone(), gameInput.clone()),
            toggleTimeHelper: None,
            gameTickTimeDelayHelper: None,
        };
        this.toggleTimeHelper = TimeDelayHelper::new(600).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>);
        this.gameTickTimeDelayHelper = GameTickTimeDelayHelperFactory::getInstance();
        // super(...) is in the struct literal above
        this.update();
        this
    }

    pub fn new(mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>, mut ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut gameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInput::GameInputDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(hashtable, ownerLayerInterface, gameInput))))
    }
    

    // throws Exception
    pub fn processAI(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) {
    }
    

    // throws Exception
    pub fn update(&mut self) {
        let mut gameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInput::GameInputDyn>>> = self.getGameInput();
        let mut keys: Vec<Option<i32>> = self.keys.clone();
        let mut size: i32 = (keys.len() as i32).clone();
        if self.isOn() {
            for index in (0..size) {
                let mut key: i32 = ((keys[(index) as usize]).unwrap() as i32);
                if key != -1 {
                    jcall!(gameInput).add(jcall!(self.gameKeyEventFactory).getInstanceForKey(None /* Java this: a &mut self method has no Rc of itself */, key.clone()));
                }
            }
        } else {
            for index in (0..size) {
                let mut key: i32 = ((keys[(index) as usize]).unwrap() as i32);
                if key != -1 {
                    jcall!(gameInput).addForRemoval(jcall!(self.gameKeyEventFactory).getInstanceForKey(None /* Java this: a &mut self method has no Rc of itself */, key.clone()));
                }
            }
        }
    }
    

    // throws Exception
    pub fn toggle(&mut self) {
        if jcall!(self.toggleTimeHelper).isTime(jcall!(self.gameTickTimeDelayHelper).startTime.clone()) {
            self.base.toggle();
            self.update();
        }
    }
    

    // throws Exception
    pub fn disable(&mut self) {
        if jcall!(self.toggleTimeHelper).isTime(jcall!(self.gameTickTimeDelayHelper).startTime.clone()) {
            self.base.disable();
            self.update();
        }
    }
    

    // throws Exception
    pub fn enable(&mut self) {
        if jcall!(self.toggleTimeHelper).isTime(jcall!(self.gameTickTimeDelayHelper).startTime.clone()) {
            self.base.enable();
            self.update();
        }
    }
}
impl std::ops::Deref for MultiKeyPressesAI {
    type Target = KeyPressesAI;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for MultiKeyPressesAI {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::ag::ai::KeyPressesAI::KeyPressesAIDyn for MultiKeyPressesAI {
    fn as_KeyPressesAI(&self) -> &crate::org::allbinary::game::ag::ai::KeyPressesAI::KeyPressesAI { &self.base }
    fn as_KeyPressesAI_mut(&mut self) -> &mut crate::org::allbinary::game::ag::ai::KeyPressesAI::KeyPressesAI { &mut self.base }
    fn toggle(&mut self) { Self::toggle(self) }
    fn disable(&mut self) { Self::disable(self) }
    fn enable(&mut self) { Self::enable(self) }
}
impl crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligenceDyn for MultiKeyPressesAI {
    fn as_ArtificialIntelligence(&self) -> &crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligence { &self.base.base.base }
    fn as_ArtificialIntelligence_mut(&mut self) -> &mut crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligence { &mut self.base.base.base }
    fn processAI(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { Self::processAI(self, allBinaryLayerManager) }
}
impl crate::org::allbinary::game::ai::BasicAI::BasicAIDyn for MultiKeyPressesAI {
    fn as_BasicAI(&self) -> &crate::org::allbinary::game::ai::BasicAI::BasicAI { &self.base.base }
    fn as_BasicAI_mut(&mut self) -> &mut crate::org::allbinary::game::ai::BasicAI::BasicAI { &mut self.base.base }
}
impl crate::org::allbinary::game::input::GameKeyEventSourceInterface::GameKeyEventSourceInterface for MultiKeyPressesAI {
    fn getSourceId(&mut self) -> i32 { crate::org::allbinary::game::ai::BasicAI::BasicAI::getSourceId(crate::org::allbinary::game::ai::BasicAI::BasicAIDyn::as_BasicAI_mut(self)) }
}


