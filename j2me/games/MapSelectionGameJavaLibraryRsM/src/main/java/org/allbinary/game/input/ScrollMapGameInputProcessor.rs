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
// package org.allbinary.game.input
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::GameInputProcessor::GameInputProcessor;
use super::ScrollMapPlayerGameInput::ScrollMapPlayerGameInput;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ScrollMapGameInputProcessor {
    pub base: GameInputProcessor,
    pub scrollMapPlayerGameInput: Option<Rc<RefCell<ScrollMapPlayerGameInput>>>,
}


impl ScrollMapGameInputProcessor {
    

    pub fn new_struct(mut scrollMapPlayerGameInput: Option<Rc<RefCell<ScrollMapPlayerGameInput>>>) -> Self {
        let mut this = Self {
            base: GameInputProcessor::new_struct(),
            scrollMapPlayerGameInput: None,
        };
        this.scrollMapPlayerGameInput = scrollMapPlayerGameInput.clone();
        this
    }

    pub fn new(mut scrollMapPlayerGameInput: Option<Rc<RefCell<ScrollMapPlayerGameInput>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(scrollMapPlayerGameInput))))
    }
}
impl std::ops::Deref for ScrollMapGameInputProcessor {
    type Target = GameInputProcessor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ScrollMapGameInputProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait ScrollMapGameInputProcessorDyn: crate::org::allbinary::game::input::GameInputProcessor::GameInputProcessorDyn {
    fn as_ScrollMapGameInputProcessor(&self) -> &ScrollMapGameInputProcessor;
    fn as_ScrollMapGameInputProcessor_mut(&mut self) -> &mut ScrollMapGameInputProcessor;
}
impl ScrollMapGameInputProcessorDyn for ScrollMapGameInputProcessor {
    fn as_ScrollMapGameInputProcessor(&self) -> &ScrollMapGameInputProcessor { self }
    fn as_ScrollMapGameInputProcessor_mut(&mut self) -> &mut ScrollMapGameInputProcessor { self }
}

impl crate::org::allbinary::game::input::GameInputProcessor::GameInputProcessorDyn for ScrollMapGameInputProcessor {
    fn as_GameInputProcessor(&self) -> &crate::org::allbinary::game::input::GameInputProcessor::GameInputProcessor { &self.base }
    fn as_GameInputProcessor_mut(&mut self) -> &mut crate::org::allbinary::game::input::GameInputProcessor::GameInputProcessor { &mut self.base }
}


