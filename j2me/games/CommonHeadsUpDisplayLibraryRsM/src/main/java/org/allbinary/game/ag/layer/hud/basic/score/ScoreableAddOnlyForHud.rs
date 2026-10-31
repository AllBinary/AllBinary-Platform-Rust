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
// package org.allbinary.game.ag.layer.hud.basic.score
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::score::Scoreable::Scoreable;
use super::ScoreHudWidget::ScoreHudWidget;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ScoreableAddOnlyForHud {
    pub base: Scoreable,
    pub scoreHudWidget: Option<Rc<RefCell<ScoreHudWidget>>>,
}


impl ScoreableAddOnlyForHud {
    

    pub fn new_struct(mut scoreVectorGraphic: Option<Rc<RefCell<ScoreHudWidget>>>) -> Self {
        let mut this = Self {
            base: Scoreable::new_struct(0),
            scoreHudWidget: None,
        };
        // super(...) is in the struct literal above
        this.scoreHudWidget = scoreVectorGraphic.clone();
        jcall!(this.getScoreHudWidget()).set(0);
        this
    }

    pub fn new(mut scoreVectorGraphic: Option<Rc<RefCell<ScoreHudWidget>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(scoreVectorGraphic))))
    }
    

    pub fn removePoints(&mut self, mut points: i32) {
    }
    

    pub fn addPoints(&mut self, mut points: i32) {
        self.base.addPoints(points.clone());
        jcall!(self.getScoreHudWidget()).add(points.clone());
    }
    

    pub fn getScoreHudWidget(&mut self) -> Option<Rc<RefCell<ScoreHudWidget>>> {
        return self.scoreHudWidget.clone();
    }
}
impl std::ops::Deref for ScoreableAddOnlyForHud {
    type Target = Scoreable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ScoreableAddOnlyForHud {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::score::Scoreable::ScoreableDyn for ScoreableAddOnlyForHud {
    fn as_Scoreable(&self) -> &crate::org::allbinary::game::score::Scoreable::Scoreable { &self.base }
    fn as_Scoreable_mut(&mut self) -> &mut crate::org::allbinary::game::score::Scoreable::Scoreable { &mut self.base }
}
impl crate::org::allbinary::game::score::ScoreableInterface::ScoreableInterface for ScoreableAddOnlyForHud {
    fn getPoints(&mut self) -> i32 { crate::org::allbinary::game::score::Scoreable::Scoreable::getPoints(crate::org::allbinary::game::score::Scoreable::ScoreableDyn::as_Scoreable_mut(self)) }
    fn addPoints(&mut self, mut points: i32) { Self::addPoints(self, points) }
    fn removePoints(&mut self, mut points: i32) { Self::removePoints(self, points) }
}


