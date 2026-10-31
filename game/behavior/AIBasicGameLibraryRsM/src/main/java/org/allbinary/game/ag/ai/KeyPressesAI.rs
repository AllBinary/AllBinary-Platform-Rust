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
use crate::org::allbinary::util::HashtableUtil::HashtableUtil;
use crate::org::allbinary::game::ai::BasicAI::BasicAI;
use crate::org::allbinary::util::ABHashtable::ABHashtable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct KeyPressesAI {
    pub base: BasicAI,
    pub on: bool,
    pub keys: Vec<Option<i32>>,
}

impl crate::org::allbinary::ai::ArtificialIntelligenceInterface::ArtificialIntelligenceInterface for KeyPressesAI {
    fn processAI(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { Self::processAI(self, allBinaryLayerManager) }
}
impl crate::org::allbinary::graphics::IdentifierInterface::IdentifierInterface for KeyPressesAI {
    fn getId(&mut self) -> i32 { self.base.getId() }
}

impl KeyPressesAI {
    

    pub fn new_struct(mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>, mut ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut gameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInput::GameInputDyn>>>) -> Self {
        let mut this = Self {
            base: BasicAI::new_struct(ownerLayerInterface.clone(), gameInput.clone()),
            on: false,
            keys: Vec::new(),
        };
        this.on = true;
        // super(...) is in the struct literal above
        this.keys = vec![None; (jcall!(hashtable).size()) as usize];
        let mut objectArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(HashtableUtil::getInstance()).getKeysAsArray(hashtable.clone());
        let mut size: i32 = (objectArray.len() as i32).clone();
        for index in (0..size) {
            this.keys[(index) as usize] = { let java_object = jcall!(hashtable).get(crate::java_into_object((objectArray[(index) as usize]).clone() /* as Any */)); java_object.as_ref().map(|object| { let object = object.borrow(); object.downcast_ref::<i32>().copied().or_else(|| object.downcast_ref::<Option<i32>>().copied().flatten()).expect("ClassCastException") }) };
        }
        this
    }

    pub fn new(mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>, mut ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut gameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInput::GameInputDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(hashtable, ownerLayerInterface, gameInput))))
    }
    

    // throws Exception
    pub fn processAI(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) {
        if self.on {
            for index in (0..(self.keys.len() as i32)) {
                self.base.processKeyAI(((self.keys[(index) as usize]).unwrap() as i32));
            }
        }
    }
    

    // throws Exception
    pub fn toggle(&mut self) {
        if self.isOn() {
            self.setOn(false);
        } else {
            self.setOn(true);
        }
    }
    

    // throws Exception
    pub fn disable(&mut self) {
        self.setOn(false);
    }
    

    // throws Exception
    pub fn enable(&mut self) {
        self.setOn(true);
    }
    

    pub fn setOn(&mut self, mut on: bool) {
        self.on = on.clone();
    }
    

    pub fn isOn(&mut self) -> bool {
        return self.on.clone();
    }
}
impl std::ops::Deref for KeyPressesAI {
    type Target = BasicAI;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for KeyPressesAI {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait KeyPressesAIDyn: crate::org::allbinary::game::ai::BasicAI::BasicAIDyn {
    fn as_KeyPressesAI(&self) -> &KeyPressesAI;
    fn as_KeyPressesAI_mut(&mut self) -> &mut KeyPressesAI;
    fn toggle(&mut self) { KeyPressesAI::toggle(self.as_KeyPressesAI_mut()) }
    fn disable(&mut self) { KeyPressesAI::disable(self.as_KeyPressesAI_mut()) }
    fn enable(&mut self) { KeyPressesAI::enable(self.as_KeyPressesAI_mut()) }
    fn setOn(&mut self, mut on: bool) { KeyPressesAI::setOn(self.as_KeyPressesAI_mut(), on) }
    fn isOn(&mut self) -> bool { KeyPressesAI::isOn(self.as_KeyPressesAI_mut()) }
}
impl KeyPressesAIDyn for KeyPressesAI {
    fn as_KeyPressesAI(&self) -> &KeyPressesAI { self }
    fn as_KeyPressesAI_mut(&mut self) -> &mut KeyPressesAI { self }
}

impl crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligenceDyn for KeyPressesAI {
    fn as_ArtificialIntelligence(&self) -> &crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligence { &self.base.base }
    fn as_ArtificialIntelligence_mut(&mut self) -> &mut crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligence { &mut self.base.base }
    fn processAI(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { Self::processAI(self, allBinaryLayerManager) }
}
impl crate::org::allbinary::game::ai::BasicAI::BasicAIDyn for KeyPressesAI {
    fn as_BasicAI(&self) -> &crate::org::allbinary::game::ai::BasicAI::BasicAI { &self.base }
    fn as_BasicAI_mut(&mut self) -> &mut crate::org::allbinary::game::ai::BasicAI::BasicAI { &mut self.base }
}
impl crate::org::allbinary::game::input::GameKeyEventSourceInterface::GameKeyEventSourceInterface for KeyPressesAI {
    fn getSourceId(&mut self) -> i32 { crate::org::allbinary::game::ai::BasicAI::BasicAI::getSourceId(crate::org::allbinary::game::ai::BasicAI::BasicAIDyn::as_BasicAI_mut(self)) }
}


