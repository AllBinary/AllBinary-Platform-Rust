// 
// AllBinary Open License Version 1 
// Copyright (c) 2006 AllBinary 
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
// package org.allbinary.game.layer.unit
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::input::GameInputProcessor::GameInputProcessor;
use super::UnitLayer::UnitLayer;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct UnitGameInputProcessor {
    pub base: GameInputProcessor,
    pub unitLayer: Option<Rc<RefCell<UnitLayer>>>,
}


impl UnitGameInputProcessor {
    

    pub fn new_struct(mut unitLayer: Option<Rc<RefCell<UnitLayer>>>) -> Self {
        let mut this = Self {
            base: GameInputProcessor::new_struct(),
            unitLayer: None,
        };
        this.unitLayer = unitLayer.clone();
        this
    }

    pub fn new(mut unitLayer: Option<Rc<RefCell<UnitLayer>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(unitLayer))))
    }
}
impl std::ops::Deref for UnitGameInputProcessor {
    type Target = GameInputProcessor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for UnitGameInputProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::input::GameInputProcessor::GameInputProcessorDyn for UnitGameInputProcessor {
    fn as_GameInputProcessor(&self) -> &crate::org::allbinary::game::input::GameInputProcessor::GameInputProcessor { &self.base }
    fn as_GameInputProcessor_mut(&mut self) -> &mut crate::org::allbinary::game::input::GameInputProcessor::GameInputProcessor { &mut self.base }
}


