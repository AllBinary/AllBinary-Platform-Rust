// 
// AllBinary Open License Version 1 
// Copyright (c) 2003 AllBinary 
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
// package org.allbinary.game.input
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::GameInputProcessor::GameInputProcessor;
use super::RTSPlayerGameInput::RTSPlayerGameInput;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RTSPlayerGameInputProcessor {
    pub base: GameInputProcessor,
    pub rtsPlayerGameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::RTSPlayerGameInput::RTSPlayerGameInputDyn>>>,
}


impl RTSPlayerGameInputProcessor {
    

    pub fn new_struct(mut rtsPlayerGameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::RTSPlayerGameInput::RTSPlayerGameInputDyn>>>) -> Self {
        let mut this = Self {
            base: GameInputProcessor::new_struct(),
            rtsPlayerGameInput: None,
        };
        this.rtsPlayerGameInput = rtsPlayerGameInput.clone();
        this
    }

    pub fn new(mut rtsPlayerGameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::RTSPlayerGameInput::RTSPlayerGameInputDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(rtsPlayerGameInput))))
    }
}
impl std::ops::Deref for RTSPlayerGameInputProcessor {
    type Target = GameInputProcessor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for RTSPlayerGameInputProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait RTSPlayerGameInputProcessorDyn: crate::org::allbinary::game::input::GameInputProcessor::GameInputProcessorDyn {
    fn as_RTSPlayerGameInputProcessor(&self) -> &RTSPlayerGameInputProcessor;
    fn as_RTSPlayerGameInputProcessor_mut(&mut self) -> &mut RTSPlayerGameInputProcessor;
}
impl RTSPlayerGameInputProcessorDyn for RTSPlayerGameInputProcessor {
    fn as_RTSPlayerGameInputProcessor(&self) -> &RTSPlayerGameInputProcessor { self }
    fn as_RTSPlayerGameInputProcessor_mut(&mut self) -> &mut RTSPlayerGameInputProcessor { self }
}

impl crate::org::allbinary::game::input::GameInputProcessor::GameInputProcessorDyn for RTSPlayerGameInputProcessor {
    fn as_GameInputProcessor(&self) -> &crate::org::allbinary::game::input::GameInputProcessor::GameInputProcessor { &self.base }
    fn as_GameInputProcessor_mut(&mut self) -> &mut crate::org::allbinary::game::input::GameInputProcessor::GameInputProcessor { &mut self.base }
}


