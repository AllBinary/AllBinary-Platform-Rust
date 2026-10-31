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
use crate::org::allbinary::ai::ArtificialIntelligenceInterface::ArtificialIntelligenceInterface;
use crate::org::allbinary::game::ag::ai::ArrayAI::ArrayAI;
use crate::org::allbinary::game::ai::ArtificialIntelligenceInterfaceFactoryInterface::ArtificialIntelligenceInterfaceFactoryInterface;
use crate::org::allbinary::game::input::GameInput::GameInput;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use super::RandomPatrolAI::RandomPatrolAI;
use super::TimedFireAI::TimedFireAI;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RandomPatrolFlockingFiringAIFactory {
}

impl ArtificialIntelligenceInterfaceFactoryInterface for RandomPatrolFlockingFiringAIFactory {
    fn getInstance(&mut self, mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>, mut ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut gameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInput::GameInputDyn>>>) -> Option<Rc<RefCell<dyn ArtificialIntelligenceInterface>>> { Self::getInstance(self, hashtable, ownerLayerInterface, gameInput) }
}

impl RandomPatrolFlockingFiringAIFactory {

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
        let mut artificialIntelligenceInterface: Vec<Option<Rc<RefCell<dyn ArtificialIntelligenceInterface>>>> = vec![None; (2) as usize];
        artificialIntelligenceInterface[(0) as usize] = RandomPatrolAI::new(hashtable.clone(), ownerLayerInterface.clone(), gameInput.clone());
        artificialIntelligenceInterface[(1) as usize] = TimedFireAI::new(1800, ownerLayerInterface.clone(), gameInput.clone());
        return ArrayAI::new(artificialIntelligenceInterface.clone(), ownerLayerInterface.clone(), gameInput.clone()).map(|reference| reference as Rc<RefCell<dyn ArtificialIntelligenceInterface>>);
    }
}

