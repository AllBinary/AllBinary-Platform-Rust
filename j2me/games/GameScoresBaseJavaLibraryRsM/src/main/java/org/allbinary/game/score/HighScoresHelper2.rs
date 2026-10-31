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
use crate::org::allbinary::util::CircularIndexUtil::CircularIndexUtil;
use super::HighScoresHelperBase::HighScoresHelperBase;
use super::HighScores::HighScores;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct HighScoresHelper2 {
    pub base: HighScoresHelperBase,
    pub circularIndexUtil: Option<Rc<RefCell<CircularIndexUtil>>>,
    pub selectedHighScores: Option<Rc<RefCell<dyn crate::org::allbinary::game::score::HighScores::HighScoresDyn>>>,
}

impl crate::org::allbinary::game::score::HighScoresResultsListener::HighScoresResultsListener for HighScoresHelper2 {
    fn setHighScoresArray(&mut self, mut highScoresArrayP: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::game::score::HighScores::HighScoresDyn>>>>) { Self::setHighScoresArray(self, highScoresArrayP) }
}

impl HighScoresHelper2 {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: HighScoresHelperBase::new_struct(),
            circularIndexUtil: None,
            selectedHighScores: None,
        };
        this.circularIndexUtil = CircularIndexUtil::createInstanceAt(0, 0);
        this.selectedHighScores = NullHighScoresSingletonFactory::NullHighScoresSingletonFactory::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn setSelectedHighScores(&mut self, mut selectedHighScores: Option<Rc<RefCell<dyn crate::org::allbinary::game::score::HighScores::HighScoresDyn>>>) {
        self.selectedHighScores = selectedHighScores.clone();
    }
    

    pub fn getSelectedHighScores(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::score::HighScores::HighScoresDyn>>> {
        return self.selectedHighScores.clone();
    }
    

    pub fn setHighScoresArray(&mut self, mut highScoresArrayP: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::game::score::HighScores::HighScoresDyn>>>>) {
        self.base.setHighScoresArray(highScoresArrayP.clone());
        jcall!(self.circularIndexUtil).setSize((self.highScoresArrayP.len() as i32).clone());
        self.selectHighScores();
    }
    

    pub fn selectHighScores(&mut self) {
        if (self.highScoresArrayP.len() as i32) == 0 {
            { let java_arguments = (NullHighScoresSingletonFactory::NullHighScoresSingletonFactory::getInstance(),); self.setSelectedHighScores(java_arguments.0) };
            return;
        }
        jcall!(self.circularIndexUtil).next();
        let mut highScores: Option<Rc<RefCell<dyn crate::org::allbinary::game::score::HighScores::HighScoresDyn>>> = self.highScoresArrayP[(jcall!(self.circularIndexUtil).getIndex()) as usize].clone();
        let mut index: i32 = 0;
        while jcall!(highScores).getTotal() < 1 && index < (self.highScoresArrayP.len() as i32) {
            highScores = self.highScoresArrayP[(jcall!(self.circularIndexUtil).next()) as usize].clone();
            index += 1;
        }
        self.setSelectedHighScores(highScores.clone());
    }
}
impl std::ops::Deref for HighScoresHelper2 {
    type Target = HighScoresHelperBase;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for HighScoresHelper2 {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::score::HighScoresHelperBase::HighScoresHelperBaseDyn for HighScoresHelper2 {
    fn as_HighScoresHelperBase(&self) -> &crate::org::allbinary::game::score::HighScoresHelperBase::HighScoresHelperBase { &self.base }
    fn as_HighScoresHelperBase_mut(&mut self) -> &mut crate::org::allbinary::game::score::HighScoresHelperBase::HighScoresHelperBase { &mut self.base }
    fn setHighScoresArray(&mut self, mut highScoresArrayP: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::game::score::HighScores::HighScoresDyn>>>>) { Self::setHighScoresArray(self, highScoresArrayP) }
    fn setSelectedHighScores(&mut self, mut selectedHighScores: Option<Rc<RefCell<dyn crate::org::allbinary::game::score::HighScores::HighScoresDyn>>>) { Self::setSelectedHighScores(self, selectedHighScores) }
    fn getSelectedHighScores(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::score::HighScores::HighScoresDyn>>> { Self::getSelectedHighScores(self) }
    fn selectHighScores(&mut self) { Self::selectHighScores(self) }
}


