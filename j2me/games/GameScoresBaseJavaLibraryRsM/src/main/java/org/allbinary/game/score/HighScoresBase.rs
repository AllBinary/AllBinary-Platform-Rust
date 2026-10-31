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
use crate::org::allbinary::game::GameInfo::GameInfo;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::HighScoresFactoryInterface::HighScoresFactoryInterface;
use super::HighScoresResultsListener::HighScoresResultsListener;
use super::HighScoresHelperBaseInterface::HighScoresHelperBaseInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct HighScoresBase {
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
}

impl HighScoresFactoryInterface for HighScoresBase {
    fn fetchHighScores(&mut self, mut gameInfo: Option<Rc<RefCell<GameInfo>>>, mut highScoresResultsListener: Option<Rc<RefCell<dyn HighScoresResultsListener>>>) { Self::fetchHighScores(self, gameInfo, highScoresResultsListener) }
    fn fetchHighScoresPreload(&mut self, mut gameInfo: Option<Rc<RefCell<GameInfo>>>, mut highScoresResultsListener: Option<Rc<RefCell<dyn HighScoresResultsListener>>>, mut preload: bool) { Self::fetchHighScoresPreload(self, gameInfo, highScoresResultsListener, preload) }
    fn createHighScoresHelper(&mut self) -> Option<Rc<RefCell<dyn HighScoresHelperBaseInterface>>> { Self::createHighScoresHelper(self) }
}

impl HighScoresBase {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            commonStrings: None,
        };
        this.commonStrings = CommonStrings::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn fetchHighScores(&mut self, mut gameInfo: Option<Rc<RefCell<GameInfo>>>, mut highScoresResultsListener: Option<Rc<RefCell<dyn HighScoresResultsListener>>>) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn fetchHighScoresPreload(&mut self, mut gameInfo: Option<Rc<RefCell<GameInfo>>>, mut highScoresResultsListener: Option<Rc<RefCell<dyn HighScoresResultsListener>>>, mut preload: bool) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn createHighScoresHelper(&mut self) -> Option<Rc<RefCell<dyn HighScoresHelperBaseInterface>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
}
pub trait HighScoresBaseDyn: crate::org::allbinary::game::score::HighScoresFactoryInterface::HighScoresFactoryInterface {
    fn as_HighScoresBase(&self) -> &HighScoresBase;
    fn as_HighScoresBase_mut(&mut self) -> &mut HighScoresBase;
}
impl HighScoresBaseDyn for HighScoresBase {
    fn as_HighScoresBase(&self) -> &HighScoresBase { self }
    fn as_HighScoresBase_mut(&mut self) -> &mut HighScoresBase { self }
}


