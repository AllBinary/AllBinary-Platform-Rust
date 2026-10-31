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
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use super::HighScoresBase::HighScoresBase;
use super::HighScores::HighScores;
use super::HighScoresResultsListener::HighScoresResultsListener;
use super::HighScoresHelperBase::HighScoresHelperBase;
use super::HighScoresHelperBaseInterface::HighScoresHelperBaseInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NoHighScoresFactory {
    pub base: HighScoresBase,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub NO_HIGH_SCORES: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::game::score::HighScores::HighScoresDyn>>>>,
}

thread_local! { pub static NoHighScoresFactory_instance: Option<Rc<RefCell<NoHighScoresFactory>>> = NoHighScoresFactory::new(); }

impl NoHighScoresFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<NoHighScoresFactory>>> {
        return NoHighScoresFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: HighScoresBase::new_struct(),
            logUtil: None,
            NO_HIGH_SCORES: Vec::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.NO_HIGH_SCORES = vec![None; (0) as usize];
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn fetchHighScores(&mut self, mut gameInfo: Option<Rc<RefCell<GameInfo>>>, mut highScoresResultsListener: Option<Rc<RefCell<dyn HighScoresResultsListener>>>) {
        jcall!(self.logUtil).putF(String::from("Getting No HighScores"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("fetchHighScores"));
        jcall!(highScoresResultsListener).setHighScoresArray(self.NO_HIGH_SCORES.clone());
    }
    

    pub fn createHighScoresHelper(&mut self) -> Option<Rc<RefCell<dyn HighScoresHelperBaseInterface>>> {
        return HighScoresHelperBase::new().map(|reference| reference as Rc<RefCell<dyn HighScoresHelperBaseInterface>>);
    }
}
impl std::ops::Deref for NoHighScoresFactory {
    type Target = HighScoresBase;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for NoHighScoresFactory {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::score::HighScoresBase::HighScoresBaseDyn for NoHighScoresFactory {
    fn as_HighScoresBase(&self) -> &crate::org::allbinary::game::score::HighScoresBase::HighScoresBase { &self.base }
    fn as_HighScoresBase_mut(&mut self) -> &mut crate::org::allbinary::game::score::HighScoresBase::HighScoresBase { &mut self.base }
}
impl crate::org::allbinary::game::score::HighScoresFactoryInterface::HighScoresFactoryInterface for NoHighScoresFactory {
    fn fetchHighScores(&mut self, mut gameInfo: Option<Rc<RefCell<GameInfo>>>, mut highScoresResultsListener: Option<Rc<RefCell<dyn HighScoresResultsListener>>>) { Self::fetchHighScores(self, gameInfo, highScoresResultsListener) }
    fn fetchHighScoresPreload(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::game::GameInfo::GameInfo>>>, p2: Option<Rc<RefCell<dyn crate::org::allbinary::game::score::HighScoresResultsListener::HighScoresResultsListener>>>, p3: bool) { crate::org::allbinary::game::score::HighScoresBase::HighScoresBase::fetchHighScoresPreload(crate::org::allbinary::game::score::HighScoresBase::HighScoresBaseDyn::as_HighScoresBase_mut(self), p1, p2, p3) }
    fn createHighScoresHelper(&mut self) -> Option<Rc<RefCell<dyn HighScoresHelperBaseInterface>>> { Self::createHighScoresHelper(self) }
}


