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
// package org.allbinary.game.multiplayer.layer
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MultiplayerBehavior {
}

thread_local! { pub static MultiplayerBehavior_NULL_MULTIPLAYER_BEHAVIOR: Option<Rc<RefCell<dyn MultiplayerBehaviorDyn>>> = MultiplayerBehavior::new(jcall!(StringUtil::getInstance()).EMPTY_STRING.clone()).map(|reference| reference as Rc<RefCell<dyn MultiplayerBehaviorDyn>>); }

impl MultiplayerBehavior {
    

    pub fn new_struct(mut username: String) -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new(mut username: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(username))))
    }
    

    pub fn startTick(&mut self) {
    }
    

    pub fn endTick(&mut self) {
    }
    

    pub fn getApproximateElapsed(&mut self) -> i64 {
        return 0_i64;
    }
}
pub trait MultiplayerBehaviorDyn {
    fn as_MultiplayerBehavior(&self) -> &MultiplayerBehavior;
    fn as_MultiplayerBehavior_mut(&mut self) -> &mut MultiplayerBehavior;
    fn startTick(&mut self) { MultiplayerBehavior::startTick(self.as_MultiplayerBehavior_mut()) }
    fn endTick(&mut self) { MultiplayerBehavior::endTick(self.as_MultiplayerBehavior_mut()) }
    fn getApproximateElapsed(&mut self) -> i64 { MultiplayerBehavior::getApproximateElapsed(self.as_MultiplayerBehavior_mut()) }
}
impl MultiplayerBehaviorDyn for MultiplayerBehavior {
    fn as_MultiplayerBehavior(&self) -> &MultiplayerBehavior { self }
    fn as_MultiplayerBehavior_mut(&mut self) -> &mut MultiplayerBehavior { self }
}


