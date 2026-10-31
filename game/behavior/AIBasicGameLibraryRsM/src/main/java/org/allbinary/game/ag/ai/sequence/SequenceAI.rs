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
use crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligence;
use crate::org::allbinary::ai::ArtificialIntelligenceInterface::ArtificialIntelligenceInterface;
use crate::org::allbinary::ai::ArtificialIntelligenceTransitionInterface::ArtificialIntelligenceTransitionInterface;
use crate::org::allbinary::game::ag::ai::ArrayAI::ArrayAI;
use crate::org::allbinary::game::input::GameInput::GameInput;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SequenceAI {
    pub base: ArrayAI,
    pub index: i32,
}

thread_local! { pub static SequenceAI_SEQUENCE_AI: String = String::from("Sequence AI"); }
impl ArtificialIntelligenceInterface for SequenceAI {
    fn processAI(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { Self::processAI(self, allBinaryLayerManager) }
}
impl crate::org::allbinary::graphics::IdentifierInterface::IdentifierInterface for SequenceAI {
    fn getId(&mut self) -> i32 { self.base.getId() }
}

impl SequenceAI {
    

    pub fn new_struct(mut artificialIntelligenceInterfaceArray: Vec<Option<Rc<RefCell<dyn ArtificialIntelligenceInterface>>>>, mut ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut gameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInput::GameInputDyn>>>) -> Self {
        let mut this = Self {
            base: ArrayAI::new_struct(artificialIntelligenceInterfaceArray.clone(), ownerLayerInterface.clone(), gameInput.clone()),
            index: 0,
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut artificialIntelligenceInterfaceArray: Vec<Option<Rc<RefCell<dyn ArtificialIntelligenceInterface>>>>, mut ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut gameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInput::GameInputDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(artificialIntelligenceInterfaceArray, ownerLayerInterface, gameInput))))
    }
    

    // throws Exception
    pub fn processAI(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) {
        jcall!(self.getArtificialIntelligenceInterface()[(self.index) as usize]).processAI(allBinaryLayerManager.clone());
    }
    

    pub fn next(&mut self) {
        self.index += 1;
        let mut artificialIntelligenceInterface: Option<Rc<RefCell<dyn ArtificialIntelligenceInterface>>> = self.getSelectedArtificialIntelligenceInterface();
        if jcall!(artificialIntelligenceInterface).getId() == crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligence_AI_ID.with(Clone::clone) {
            let mut artificialIntelligenceTransitionInterface: Option<Rc<RefCell<dyn ArtificialIntelligenceTransitionInterface>>> = panic!("ClassCastException: a Rust dyn ArtificialIntelligenceInterface cannot be cast to dyn ArtificialIntelligenceTransitionInterface");
            jcall!(artificialIntelligenceTransitionInterface).transition();
        }
    }
    

    pub fn getIndex(&mut self) -> i32 {
        return self.index.clone();
    }
    

    pub fn setIndex(&mut self, mut index: i32) {
        self.index = index.clone();
    }
    

    pub fn getSelectedArtificialIntelligenceInterface(&mut self) -> Option<Rc<RefCell<dyn ArtificialIntelligenceInterface>>> {
        return self.getArtificialIntelligenceInterface()[(self.index) as usize].clone();
    }
    

    pub fn getName(&mut self) -> String {
        return SequenceAI_SEQUENCE_AI.with(Clone::clone).clone();
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(self.base.toString());
        jcall!(stringBuffer).append(String::from(" Selected AI: "));
        jcall!(stringBuffer).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(self.getSelectedArtificialIntelligenceInterface())));
        return jcall!(stringBuffer).toString();
    }
}
impl std::ops::Deref for SequenceAI {
    type Target = ArrayAI;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for SequenceAI {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait SequenceAIDyn: crate::org::allbinary::game::ag::ai::ArrayAI::ArrayAIDyn {
    fn as_SequenceAI(&self) -> &SequenceAI;
    fn as_SequenceAI_mut(&mut self) -> &mut SequenceAI;
    fn next(&mut self) { SequenceAI::next(self.as_SequenceAI_mut()) }
    fn getIndex(&mut self) -> i32 { SequenceAI::getIndex(self.as_SequenceAI_mut()) }
    fn setIndex(&mut self, mut index: i32) { SequenceAI::setIndex(self.as_SequenceAI_mut(), index) }
    fn getSelectedArtificialIntelligenceInterface(&mut self) -> Option<Rc<RefCell<dyn ArtificialIntelligenceInterface>>> { SequenceAI::getSelectedArtificialIntelligenceInterface(self.as_SequenceAI_mut()) }
}
impl SequenceAIDyn for SequenceAI {
    fn as_SequenceAI(&self) -> &SequenceAI { self }
    fn as_SequenceAI_mut(&mut self) -> &mut SequenceAI { self }
}

impl crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligenceDyn for SequenceAI {
    fn as_ArtificialIntelligence(&self) -> &crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligence { &self.base.base.base }
    fn as_ArtificialIntelligence_mut(&mut self) -> &mut crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligence { &mut self.base.base.base }
    fn processAI(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { Self::processAI(self, allBinaryLayerManager) }
}
impl crate::org::allbinary::game::ag::ai::ArrayAI::ArrayAIDyn for SequenceAI {
    fn as_ArrayAI(&self) -> &crate::org::allbinary::game::ag::ai::ArrayAI::ArrayAI { &self.base }
    fn as_ArrayAI_mut(&mut self) -> &mut crate::org::allbinary::game::ag::ai::ArrayAI::ArrayAI { &mut self.base }
}
impl crate::org::allbinary::game::ai::BasicAI::BasicAIDyn for SequenceAI {
    fn as_BasicAI(&self) -> &crate::org::allbinary::game::ai::BasicAI::BasicAI { &self.base.base }
    fn as_BasicAI_mut(&mut self) -> &mut crate::org::allbinary::game::ai::BasicAI::BasicAI { &mut self.base.base }
    fn getName(&mut self) -> String { Self::getName(self) }
}
impl crate::org::allbinary::game::input::GameKeyEventSourceInterface::GameKeyEventSourceInterface for SequenceAI {
    fn getSourceId(&mut self) -> i32 { crate::org::allbinary::game::ai::BasicAI::BasicAI::getSourceId(crate::org::allbinary::game::ai::BasicAI::BasicAIDyn::as_BasicAI_mut(self)) }
}


