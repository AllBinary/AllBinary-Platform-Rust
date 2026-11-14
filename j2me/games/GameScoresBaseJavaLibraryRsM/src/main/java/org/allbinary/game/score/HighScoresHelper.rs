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

pub struct HighScoresHelper {
    pub base: HighScoresHelperBase,
    pub circularIndexUtil: Option<Rc<RefCell<CircularIndexUtil>>>,
    pub lastIndex: i32,
}

impl crate::org::allbinary::game::score::HighScoresHelperBaseInterface::HighScoresHelperBaseInterface for HighScoresHelper {
    fn setSelectedHighScores(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::game::score::HighScores::HighScoresDyn>>>) { crate::org::allbinary::game::score::HighScoresHelperBase::HighScoresHelperBase::setSelectedHighScores(crate::org::allbinary::game::score::HighScoresHelperBase::HighScoresHelperBaseDyn::as_HighScoresHelperBase_mut(self), p1) }
    fn getHighScoresArray(&mut self) -> Vec<Option<Rc<RefCell<dyn crate::org::allbinary::game::score::HighScores::HighScoresDyn>>>> { crate::org::allbinary::game::score::HighScoresHelperBase::HighScoresHelperBase::getHighScoresArray(crate::org::allbinary::game::score::HighScoresHelperBase::HighScoresHelperBaseDyn::as_HighScoresHelperBase_mut(self)) }
    fn getSelectedHighScores(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::score::HighScores::HighScoresDyn>>> { crate::org::allbinary::game::score::HighScoresHelperBase::HighScoresHelperBase::getSelectedHighScores(crate::org::allbinary::game::score::HighScoresHelperBase::HighScoresHelperBaseDyn::as_HighScoresHelperBase_mut(self)) }
}
impl crate::org::allbinary::game::score::HighScoresResultsListener::HighScoresResultsListener for HighScoresHelper {
    fn setHighScoresArray(&mut self, mut highScoresArrayP: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::game::score::HighScores::HighScoresDyn>>>>) { Self::setHighScoresArray(self, highScoresArrayP) }
}

impl HighScoresHelper {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: HighScoresHelperBase::new_struct(),
            circularIndexUtil: None,
            lastIndex: 0,
        };
        this.circularIndexUtil = CircularIndexUtil::createInstanceAt(0, 0);
        this.lastIndex = -1;
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn setHighScoresArray(&mut self, mut highScoresArrayP: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::game::score::HighScores::HighScoresDyn>>>>) {
        { let java_arguments = (highScoresArrayP.clone(),); self.base.setHighScoresArray(java_arguments.0) };
        jcall!(self.circularIndexUtil).setSize((self.highScoresArrayP.len() as i32).clone());
    }
    

    pub fn getNextHighScores(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::score::HighScores::HighScoresDyn>>> {
        let mut highScores: Option<Rc<RefCell<dyn crate::org::allbinary::game::score::HighScores::HighScoresDyn>>> = self.highScoresArrayP[(jcall!(self.circularIndexUtil).getIndex()) as usize].clone();
        let mut index: i32 = 0;
        while (jcall!(highScores).as_HighScores_mut().getTotal() < 1 || self.lastIndex == jcall!(self.circularIndexUtil).getIndex()) && index < (self.highScoresArrayP.len() as i32) {
            jcall!(self.circularIndexUtil).next();
            highScores = { let java_value = (self.highScoresArrayP[(jcall!(self.circularIndexUtil).getIndex()) as usize]).clone(); java_value }.clone();
            index += 1;
        }
        self.lastIndex = { let java_value = (jcall!(self.circularIndexUtil).getIndex()).clone(); java_value };
        return highScores.clone();
    }
    

    pub fn isAnyHighScores(&mut self) -> bool {
        if (self.highScoresArrayP.len() as i32) < 1 {
            return false;
        }
        let mut anyScores: bool = false;
        let mut highScores: Option<Rc<RefCell<dyn crate::org::allbinary::game::score::HighScores::HighScoresDyn>>> = None;
        for index in (0..=(self.highScoresArrayP.len() as i32) - 1).rev() {
            highScores = { let java_value = (self.highScoresArrayP[(index) as usize]).clone(); java_value }.clone();
            if jcall!(highScores).as_HighScores_mut().getTotal() > 0 {
                anyScores = true;
            }
        }
        return anyScores.clone();
    }
}
impl std::ops::Deref for HighScoresHelper {
    type Target = HighScoresHelperBase;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for HighScoresHelper {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::score::HighScoresHelperBase::HighScoresHelperBaseDyn for HighScoresHelper {
    fn as_HighScoresHelperBase(&self) -> &crate::org::allbinary::game::score::HighScoresHelperBase::HighScoresHelperBase { &self.base }
    fn as_HighScoresHelperBase_mut(&mut self) -> &mut crate::org::allbinary::game::score::HighScoresHelperBase::HighScoresHelperBase { &mut self.base }
    fn setHighScoresArray(&mut self, mut highScoresArrayP: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::game::score::HighScores::HighScoresDyn>>>>) { Self::setHighScoresArray(self, highScoresArrayP) }
    fn getNextHighScores(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::score::HighScores::HighScoresDyn>>> { Self::getNextHighScores(self) }
    fn isAnyHighScores(&mut self) -> bool { Self::isAnyHighScores(self) }
}


