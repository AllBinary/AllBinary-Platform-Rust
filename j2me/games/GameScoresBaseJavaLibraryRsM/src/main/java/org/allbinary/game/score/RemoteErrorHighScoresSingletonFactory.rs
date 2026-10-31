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
// package org.allbinary.game.score
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::HighScores::HighScores;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RemoteErrorHighScoresSingletonFactory {
}

thread_local! { pub static RemoteErrorHighScoresSingletonFactory_SINGLETON: Option<Rc<RefCell<dyn crate::org::allbinary::game::score::HighScores::HighScoresDyn>>> = HighScores::new(jcall!(CommonStrings::getInstance()).LOADING.clone(), String::from("Remote High Scores Did Not Load"), jcall!(CommonStrings::getInstance()).PERIODS.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::score::HighScores::HighScoresDyn>>); }

impl RemoteErrorHighScoresSingletonFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn crate::org::allbinary::game::score::HighScores::HighScoresDyn>>> {
        return RemoteErrorHighScoresSingletonFactory_SINGLETON.with(Clone::clone).clone();
    }
}

