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
// package org.allbinary.ai
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager;
use super::ArtificialIntelligenceInterface::ArtificialIntelligenceInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ArtificialIntelligence {
}

thread_local! { pub static ArtificialIntelligence_AI_ID: i32 = 2; }
thread_local! { pub static ArtificialIntelligence_instance: Option<Rc<RefCell<dyn ArtificialIntelligenceDyn>>> = ArtificialIntelligence::new().map(|reference| reference as Rc<RefCell<dyn ArtificialIntelligenceDyn>>); }
impl ArtificialIntelligenceInterface for ArtificialIntelligence {
    fn processAI(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { Self::processAI(self, layerManager) }
}
impl crate::org::allbinary::graphics::IdentifierInterface::IdentifierInterface for ArtificialIntelligence {
    fn getId(&mut self) -> i32 { Self::getId(self) }
}

impl ArtificialIntelligence {
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn ArtificialIntelligenceDyn>>> {
        return ArtificialIntelligence_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn processAI(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) {
    }
    

    pub fn getId(&mut self) -> i32 {
        return 1;
    }
}
pub trait ArtificialIntelligenceDyn {
    fn as_ArtificialIntelligence(&self) -> &ArtificialIntelligence;
    fn as_ArtificialIntelligence_mut(&mut self) -> &mut ArtificialIntelligence;
    fn processAI(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { ArtificialIntelligence::processAI(self.as_ArtificialIntelligence_mut(), layerManager) }
    fn getId(&mut self) -> i32 { ArtificialIntelligence::getId(self.as_ArtificialIntelligence_mut()) }
}
impl ArtificialIntelligenceDyn for ArtificialIntelligence {
    fn as_ArtificialIntelligence(&self) -> &ArtificialIntelligence { self }
    fn as_ArtificialIntelligence_mut(&mut self) -> &mut ArtificialIntelligence { self }
}


