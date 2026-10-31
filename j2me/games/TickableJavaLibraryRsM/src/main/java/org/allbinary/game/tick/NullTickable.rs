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
// package org.allbinary.game.tick
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager;
use super::TickableInterface::TickableInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NullTickable {
    pub NAME: String,
}

thread_local! { pub static NullTickable_SINGLETON: Option<Rc<RefCell<NullTickable>>> = NullTickable::new(); }
impl TickableInterface for NullTickable {
    fn processTick(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { Self::processTick(self, allBinaryLayerManager) }
}
impl crate::org::allbinary::layer::NamedInterface::NamedInterface for NullTickable {
    fn getName(&mut self) -> String { Self::getName(self) }
}

impl NullTickable {

    pub fn new_struct() -> Self {
        let mut this = Self {
            NAME: String::new(),
        };
        this.NAME = String::from("NulLTickable");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<NullTickable>>> {
        return NullTickable_SINGLETON.with(Clone::clone).clone();
    }
    

    // throws Exception
    pub fn processTick(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) {
    }
    

    pub fn getName(&mut self) -> String {
        return self.NAME.clone();
    }
}

