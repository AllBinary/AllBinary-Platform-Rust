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
use crate::org::allbinary::game::ag::ai::tactical::BasicRandomAIFactory::BasicRandomAIFactory;
use crate::org::allbinary::game::ai::ArtificialIntelligenceInterfaceFactoryInterface::ArtificialIntelligenceInterfaceFactoryInterface;
use crate::org::allbinary::game::ai::BasicAI::BasicAI;
use crate::org::allbinary::game::input::GameInput::GameInput;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::logic::util::visitor::Visitor::Visitor;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use super::BasicProbabilityAIDataFactory::BasicProbabilityAIDataFactory;
use super::DiveAndDirectionalTrackingAI::DiveAndDirectionalTrackingAI;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct DiveAndDirectionalTrackingAIFactory {
}

impl ArtificialIntelligenceInterfaceFactoryInterface for DiveAndDirectionalTrackingAIFactory {
    fn getInstance(&mut self, mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>, mut ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut gameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInput::GameInputDyn>>>) -> Option<Rc<RefCell<dyn ArtificialIntelligenceInterface>>> { Self::getInstance(self, hashtable, ownerLayerInterface, gameInput) }
}

impl DiveAndDirectionalTrackingAIFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn getInstance(&mut self, mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>, mut ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut gameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInput::GameInputDyn>>>) -> Option<Rc<RefCell<dyn ArtificialIntelligenceInterface>>> {
        let mut visitorCanBeNull: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(hashtable).get(crate::java_into_object((crate::org::allbinary::game::ai::BasicAI::BasicAI_AI_VISITOR.with(Clone::clone)).clone() /* as Any */));
        if visitorCanBeNull.is_none() {
            visitorCanBeNull = ThrustAIVisitorFactory::ThrustAIVisitorFactory::getInstance();
        }
        let mut hashtable2: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> = jcall!(BasicProbabilityAIDataFactory::new()).getInstance();
        jcall!(hashtable2).put(crate::java_into_object(crate::org::allbinary::game::ai::BasicAI::BasicAI_AI_VISITOR.with(Clone::clone).clone()), crate::java_into_object(LastKeyAIVisitorFactory::LastKeyAIVisitorFactory::getInstance()));
        let mut artificialIntelligenceInterface: Option<Rc<RefCell<dyn ArtificialIntelligenceInterface>>> = jcall!(BasicRandomAIFactory::new()).getInstance(hashtable2.clone(), ownerLayerInterface.clone(), gameInput.clone());
        return DiveAndDirectionalTrackingAI::new(ownerLayerInterface.clone(), artificialIntelligenceInterface.clone(), gameInput.clone(), (visitorCanBeNull).clone() /* as Visitor */).map(|reference| reference as Rc<RefCell<dyn ArtificialIntelligenceInterface>>);
    }
}

