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
// package org.allbinary.game.layer.special
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::input::GameInputProcessor::GameInputProcessor;
use super::SpecialGameInputInterface::SpecialGameInputInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SpecialGameInputInterfaceProcessor {
    pub base: GameInputProcessor,
    pub specialGameInputInterface: Option<Rc<RefCell<dyn SpecialGameInputInterface>>>,
}


impl SpecialGameInputInterfaceProcessor {
    

    pub fn new_struct(mut collidableDestroyableDamageableLayer: Option<Rc<RefCell<dyn SpecialGameInputInterface>>>) -> Self {
        let mut this = Self {
            base: GameInputProcessor::new_struct(),
            specialGameInputInterface: None,
        };
        this.specialGameInputInterface = collidableDestroyableDamageableLayer.clone();
        this
    }

    pub fn new(mut collidableDestroyableDamageableLayer: Option<Rc<RefCell<dyn SpecialGameInputInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(collidableDestroyableDamageableLayer))))
    }
}
impl std::ops::Deref for SpecialGameInputInterfaceProcessor {
    type Target = GameInputProcessor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for SpecialGameInputInterfaceProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait SpecialGameInputInterfaceProcessorDyn: crate::org::allbinary::game::input::GameInputProcessor::GameInputProcessorDyn {
    fn as_SpecialGameInputInterfaceProcessor(&self) -> &SpecialGameInputInterfaceProcessor;
    fn as_SpecialGameInputInterfaceProcessor_mut(&mut self) -> &mut SpecialGameInputInterfaceProcessor;
}
impl SpecialGameInputInterfaceProcessorDyn for SpecialGameInputInterfaceProcessor {
    fn as_SpecialGameInputInterfaceProcessor(&self) -> &SpecialGameInputInterfaceProcessor { self }
    fn as_SpecialGameInputInterfaceProcessor_mut(&mut self) -> &mut SpecialGameInputInterfaceProcessor { self }
}

impl crate::org::allbinary::game::input::GameInputProcessor::GameInputProcessorDyn for SpecialGameInputInterfaceProcessor {
    fn as_GameInputProcessor(&self) -> &crate::org::allbinary::game::input::GameInputProcessor::GameInputProcessor { &self.base }
    fn as_GameInputProcessor_mut(&mut self) -> &mut crate::org::allbinary::game::input::GameInputProcessor::GameInputProcessor { &mut self.base }
}


