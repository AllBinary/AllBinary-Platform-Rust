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
use crate::org::allbinary::game::ai::ArtificialIntelligenceInterfaceFactoryInterface::ArtificialIntelligenceInterfaceFactoryInterface;
use crate::org::allbinary::game::input::GameInput::GameInput;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use super::NullAI::NullAI;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NullAIFactory {
}

pub static mut NullAIFactory_NULL_AI: std::cell::LazyCell<Option<Rc<RefCell<NullAI>>>> = std::cell::LazyCell::new(|| NullAI::new());
impl ArtificialIntelligenceInterfaceFactoryInterface for NullAIFactory {
    fn getInstance(&mut self, mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>, mut ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut gameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInput::GameInputDyn>>>) -> Option<Rc<RefCell<dyn ArtificialIntelligenceInterface>>> { Self::getInstance3(self, hashtable, ownerLayerInterface, gameInput) }
}

impl NullAIFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance3(&mut self, mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>, mut ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut gameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInput::GameInputDyn>>>) -> Option<Rc<RefCell<dyn ArtificialIntelligenceInterface>>> {
        return unsafe { NullAIFactory_NULL_AI.clone() }.clone();
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn getInstance() -> Option<Rc<RefCell<dyn ArtificialIntelligenceInterface>>> {
        return unsafe { NullAIFactory_NULL_AI.clone() }.clone();
    }
}

