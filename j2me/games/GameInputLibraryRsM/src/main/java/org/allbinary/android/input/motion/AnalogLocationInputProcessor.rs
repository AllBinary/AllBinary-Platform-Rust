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
// package org.allbinary.android.input.motion
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::input::analog::AnalogLocationInput::AnalogLocationInput;
use crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AnalogLocationInputProcessor {
}


impl AnalogLocationInputProcessor {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn process(&mut self, mut allbinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut analogLocationInput: Option<Rc<RefCell<AnalogLocationInput>>>) {
    }
}
pub trait AnalogLocationInputProcessorDyn {
    fn as_AnalogLocationInputProcessor(&self) -> &AnalogLocationInputProcessor;
    fn as_AnalogLocationInputProcessor_mut(&mut self) -> &mut AnalogLocationInputProcessor;
    fn process(&mut self, mut allbinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut analogLocationInput: Option<Rc<RefCell<AnalogLocationInput>>>) { AnalogLocationInputProcessor::process(self.as_AnalogLocationInputProcessor_mut(), allbinaryLayerManager, analogLocationInput) }
}
impl AnalogLocationInputProcessorDyn for AnalogLocationInputProcessor {
    fn as_AnalogLocationInputProcessor(&self) -> &AnalogLocationInputProcessor { self }
    fn as_AnalogLocationInputProcessor_mut(&mut self) -> &mut AnalogLocationInputProcessor { self }
}


