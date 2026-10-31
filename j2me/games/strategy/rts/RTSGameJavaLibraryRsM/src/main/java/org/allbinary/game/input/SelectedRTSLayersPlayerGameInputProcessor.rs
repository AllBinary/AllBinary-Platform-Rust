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
use super::SelectedRTSLayersPlayerGameInput::SelectedRTSLayersPlayerGameInput;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SelectedRTSLayersPlayerGameInputProcessor {
    pub base: GameInputProcessor,
    pub selectedRTSLayersPlayerGameInput: Option<Rc<RefCell<SelectedRTSLayersPlayerGameInput>>>,
}


impl SelectedRTSLayersPlayerGameInputProcessor {
    

    pub fn new_struct(mut selectedRTSLayersPlayerGameInput: Option<Rc<RefCell<SelectedRTSLayersPlayerGameInput>>>) -> Self {
        let mut this = Self {
            base: GameInputProcessor::new_struct(),
            selectedRTSLayersPlayerGameInput: None,
        };
        this.selectedRTSLayersPlayerGameInput = selectedRTSLayersPlayerGameInput.clone();
        this
    }

    pub fn new(mut selectedRTSLayersPlayerGameInput: Option<Rc<RefCell<SelectedRTSLayersPlayerGameInput>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(selectedRTSLayersPlayerGameInput))))
    }
}
impl std::ops::Deref for SelectedRTSLayersPlayerGameInputProcessor {
    type Target = GameInputProcessor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for SelectedRTSLayersPlayerGameInputProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait SelectedRTSLayersPlayerGameInputProcessorDyn: crate::org::allbinary::game::input::GameInputProcessor::GameInputProcessorDyn {
    fn as_SelectedRTSLayersPlayerGameInputProcessor(&self) -> &SelectedRTSLayersPlayerGameInputProcessor;
    fn as_SelectedRTSLayersPlayerGameInputProcessor_mut(&mut self) -> &mut SelectedRTSLayersPlayerGameInputProcessor;
}
impl SelectedRTSLayersPlayerGameInputProcessorDyn for SelectedRTSLayersPlayerGameInputProcessor {
    fn as_SelectedRTSLayersPlayerGameInputProcessor(&self) -> &SelectedRTSLayersPlayerGameInputProcessor { self }
    fn as_SelectedRTSLayersPlayerGameInputProcessor_mut(&mut self) -> &mut SelectedRTSLayersPlayerGameInputProcessor { self }
}

impl crate::org::allbinary::game::input::GameInputProcessor::GameInputProcessorDyn for SelectedRTSLayersPlayerGameInputProcessor {
    fn as_GameInputProcessor(&self) -> &crate::org::allbinary::game::input::GameInputProcessor::GameInputProcessor { &self.base }
    fn as_GameInputProcessor_mut(&mut self) -> &mut crate::org::allbinary::game::input::GameInputProcessor::GameInputProcessor { &mut self.base }
}


