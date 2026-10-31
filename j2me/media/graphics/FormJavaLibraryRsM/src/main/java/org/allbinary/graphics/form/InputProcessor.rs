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
// package org.allbinary.graphics.form
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::canvas::Processor::Processor;
use crate::org::allbinary::game::input::PlayerGameInputCompositeInterface::PlayerGameInputCompositeInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InputProcessor {
    pub base: Processor,
    pub playerGameInputCompositeInterface: Option<Rc<RefCell<dyn PlayerGameInputCompositeInterface>>>,
}


impl InputProcessor {
    

    pub fn new_struct(mut playerGameInputCompositeInterface: Option<Rc<RefCell<dyn PlayerGameInputCompositeInterface>>>) -> Self {
        let mut this = Self {
            base: Processor::new_struct(),
            playerGameInputCompositeInterface: None,
        };
        this.playerGameInputCompositeInterface = playerGameInputCompositeInterface.clone();
        this
    }

    pub fn new(mut playerGameInputCompositeInterface: Option<Rc<RefCell<dyn PlayerGameInputCompositeInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(playerGameInputCompositeInterface))))
    }
    

    // throws Exception
    pub fn process(&mut self) {
        jcall!(jcall!(self.playerGameInputCompositeInterface).getPlayerGameInput()).update();
    }
}
impl std::ops::Deref for InputProcessor {
    type Target = Processor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for InputProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait InputProcessorDyn: crate::org::allbinary::canvas::Processor::ProcessorDyn {
    fn as_InputProcessor(&self) -> &InputProcessor;
    fn as_InputProcessor_mut(&mut self) -> &mut InputProcessor;
}
impl InputProcessorDyn for InputProcessor {
    fn as_InputProcessor(&self) -> &InputProcessor { self }
    fn as_InputProcessor_mut(&mut self) -> &mut InputProcessor { self }
}

impl crate::org::allbinary::canvas::Processor::ProcessorDyn for InputProcessor {
    fn as_Processor(&self) -> &crate::org::allbinary::canvas::Processor::Processor { &self.base }
    fn as_Processor_mut(&mut self) -> &mut crate::org::allbinary::canvas::Processor::Processor { &mut self.base }
    fn process(&mut self) { Self::process(self) }
}


