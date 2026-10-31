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
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use super::HighScoresInterface::HighScoresInterface;
use super::HighScore::HighScore;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct HighScores {
    pub orderedHighScoresList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub name: String,
    pub heading: String,
    pub columnOneHeading: String,
    pub columnTwoHeading: String,
}

impl HighScoresInterface for HighScores {
    fn getList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Self::getList(self) }
    fn addHighScore(&mut self, mut newHighScore: Option<Rc<RefCell<HighScore>>>) { Self::addHighScore(self, newHighScore) }
    fn getTotal(&mut self) -> i32 { Self::getTotal(self) }
    fn isBestScore(&mut self, mut newHighScore: Option<Rc<RefCell<HighScore>>>) -> bool { Self::isBestScore(self, newHighScore) }
}

impl HighScores {
    

    pub fn new_struct(mut name: String, mut heading: String, mut columnTwoHeading: String) -> Self {
        let mut this = Self {
            orderedHighScoresList: None,
            name: String::new(),
            heading: String::new(),
            columnOneHeading: String::new(),
            columnTwoHeading: String::new(),
        };
        this.orderedHighScoresList = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.columnOneHeading = String::from("Name");
        this.name = name.clone();
        this.heading = heading.clone();
        this.columnTwoHeading = columnTwoHeading.clone();
        this
    }

    pub fn new(mut name: String, mut heading: String, mut columnTwoHeading: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(name, heading, columnTwoHeading))))
    }
    

    pub fn setList(&mut self, mut orderedHighScoresList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        self.orderedHighScoresList = orderedHighScoresList.clone();
    }
    

    pub fn getList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.orderedHighScoresList.clone();
    }
    

    pub fn getName(&mut self) -> String {
        return self.name.clone();
    }
    

    pub fn getTotal(&mut self) -> i32 {
        return jcall!(self.getList()).size();
    }
    

    pub fn addHighScore(&mut self, mut newHighScore: Option<Rc<RefCell<HighScore>>>) {
        jcall!(self.orderedHighScoresList).add(crate::java_into_object(newHighScore.clone()));
    }
    

    pub fn getHeading(&mut self) -> String {
        return self.heading.clone();
    }
    

    pub fn getColumnOneHeading(&mut self) -> String {
        return self.columnOneHeading.clone();
    }
    

    pub fn getColumnTwoHeading(&mut self) -> String {
        return self.columnTwoHeading.clone();
    }
    

    // throws Exception
    pub fn isBestScore(&mut self, mut newHighScore: Option<Rc<RefCell<HighScore>>>) -> bool {
        return false;
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(String::from("HighScores: "));
        jcall!(stringBuffer).append(self.heading.clone());
        jcall!(stringBuffer).append(jcall!(CommonSeps::getInstance()).SPACE.clone());
        jcall!(stringBuffer).append(jcall!(CommonLabels::getInstance()).TOTAL_LABEL.clone());
        jcall!(stringBuffer).appendint(jcall!(self.orderedHighScoresList).size());
        return jcall!(stringBuffer).toString();
    }
}
pub trait HighScoresDyn: crate::org::allbinary::game::score::HighScoresInterface::HighScoresInterface {
    fn as_HighScores(&self) -> &HighScores;
    fn as_HighScores_mut(&mut self) -> &mut HighScores;
    fn setList(&mut self, mut orderedHighScoresList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { HighScores::setList(self.as_HighScores_mut(), orderedHighScoresList) }
    fn getName(&mut self) -> String { HighScores::getName(self.as_HighScores_mut()) }
    fn getHeading(&mut self) -> String { HighScores::getHeading(self.as_HighScores_mut()) }
    fn getColumnOneHeading(&mut self) -> String { HighScores::getColumnOneHeading(self.as_HighScores_mut()) }
    fn getColumnTwoHeading(&mut self) -> String { HighScores::getColumnTwoHeading(self.as_HighScores_mut()) }
}
impl HighScoresDyn for HighScores {
    fn as_HighScores(&self) -> &HighScores { self }
    fn as_HighScores_mut(&mut self) -> &mut HighScores { self }
}


