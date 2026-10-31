// 
// AllBinary Open License Version 1 
// Copyright (c) 2022 AllBinary 
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
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::HighScoresHelperBaseInterface::HighScoresHelperBaseInterface;
use super::HighScores::HighScores;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct HighScoresHelperBase {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub highScoresArrayP: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::game::score::HighScores::HighScoresDyn>>>>,
}

impl HighScoresHelperBaseInterface for HighScoresHelperBase {
    fn setSelectedHighScores(&mut self, mut selectedHighScores: Option<Rc<RefCell<dyn crate::org::allbinary::game::score::HighScores::HighScoresDyn>>>) { Self::setSelectedHighScores(self, selectedHighScores) }
    fn getHighScoresArray(&mut self) -> Vec<Option<Rc<RefCell<dyn crate::org::allbinary::game::score::HighScores::HighScoresDyn>>>> { Self::getHighScoresArray(self) }
    fn getSelectedHighScores(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::score::HighScores::HighScoresDyn>>> { Self::getSelectedHighScores(self) }
}
impl crate::org::allbinary::game::score::HighScoresResultsListener::HighScoresResultsListener for HighScoresHelperBase {
    fn setHighScoresArray(&mut self, mut highScoresArrayP: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::game::score::HighScores::HighScoresDyn>>>>) { Self::setHighScoresArray(self, highScoresArrayP) }
}

impl HighScoresHelperBase {

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            highScoresArrayP: Vec::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.highScoresArrayP = jcall!(LastFetchHighScoresFactory::LastFetchHighScoresFactory::getInstance()).highScoresArray.clone();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn setHighScoresArray(&mut self, mut highScoresArrayP: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::game::score::HighScores::HighScoresDyn>>>>) {
        if highScoresArrayP.is_some() {
            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(commonStrings).START.clone()).appendint((highScoresArrayP.len() as i32).clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setHighScoresArray"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        } else {
            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(self.logUtil).putF(jcall!(commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setHighScoresArray"));
        }
        self.highScoresArrayP = highScoresArrayP.clone();
    }
    

    pub fn getNextHighScores(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::score::HighScores::HighScoresDyn>>> {
        return NullHighScoresSingletonFactory::NullHighScoresSingletonFactory::getInstance();
    }
    

    pub fn isAnyHighScores(&mut self) -> bool {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn setSelectedHighScores(&mut self, mut selectedHighScores: Option<Rc<RefCell<dyn crate::org::allbinary::game::score::HighScores::HighScoresDyn>>>) {
    }
    

    pub fn getSelectedHighScores(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::score::HighScores::HighScoresDyn>>> {
        return NullHighScoresSingletonFactory::NullHighScoresSingletonFactory::getInstance();
    }
    

    pub fn getHighScoresArray(&mut self) -> Vec<Option<Rc<RefCell<dyn crate::org::allbinary::game::score::HighScores::HighScoresDyn>>>> {
        return self.highScoresArrayP.clone();
    }
    

    pub fn selectHighScores(&mut self) {
    }
}
pub trait HighScoresHelperBaseDyn {
    fn as_HighScoresHelperBase(&self) -> &HighScoresHelperBase;
    fn as_HighScoresHelperBase_mut(&mut self) -> &mut HighScoresHelperBase;
    fn setHighScoresArray(&mut self, mut highScoresArrayP: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::game::score::HighScores::HighScoresDyn>>>>) { HighScoresHelperBase::setHighScoresArray(self.as_HighScoresHelperBase_mut(), highScoresArrayP) }
    fn getNextHighScores(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::score::HighScores::HighScoresDyn>>> { HighScoresHelperBase::getNextHighScores(self.as_HighScoresHelperBase_mut()) }
    fn isAnyHighScores(&mut self) -> bool { HighScoresHelperBase::isAnyHighScores(self.as_HighScoresHelperBase_mut()) }
    fn setSelectedHighScores(&mut self, mut selectedHighScores: Option<Rc<RefCell<dyn crate::org::allbinary::game::score::HighScores::HighScoresDyn>>>) { HighScoresHelperBase::setSelectedHighScores(self.as_HighScoresHelperBase_mut(), selectedHighScores) }
    fn getSelectedHighScores(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::score::HighScores::HighScoresDyn>>> { HighScoresHelperBase::getSelectedHighScores(self.as_HighScoresHelperBase_mut()) }
    fn getHighScoresArray(&mut self) -> Vec<Option<Rc<RefCell<dyn crate::org::allbinary::game::score::HighScores::HighScoresDyn>>>> { HighScoresHelperBase::getHighScoresArray(self.as_HighScoresHelperBase_mut()) }
    fn selectHighScores(&mut self) { HighScoresHelperBase::selectHighScores(self.as_HighScoresHelperBase_mut()) }
}
impl HighScoresHelperBaseDyn for HighScoresHelperBase {
    fn as_HighScoresHelperBase(&self) -> &HighScoresHelperBase { self }
    fn as_HighScoresHelperBase_mut(&mut self) -> &mut HighScoresHelperBase { self }
}


