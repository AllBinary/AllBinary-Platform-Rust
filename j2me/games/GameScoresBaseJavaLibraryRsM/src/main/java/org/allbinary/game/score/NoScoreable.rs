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
use super::Scoreable::Scoreable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NoScoreable {
    pub base: Scoreable,
}

thread_local! { pub static NoScoreable_SINGLETON: Option<Rc<RefCell<NoScoreable>>> = NoScoreable::new(); }

impl NoScoreable {
    

    pub fn getInstance() -> Option<Rc<RefCell<NoScoreable>>> {
        return NoScoreable_SINGLETON.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: Scoreable::new_struct(0),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn addPoints(&mut self, mut points: i32) {
    }
    

    pub fn removePoints(&mut self, mut points: i32) {
    }
}
impl std::ops::Deref for NoScoreable {
    type Target = Scoreable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for NoScoreable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::score::Scoreable::ScoreableDyn for NoScoreable {
    fn as_Scoreable(&self) -> &crate::org::allbinary::game::score::Scoreable::Scoreable { &self.base }
    fn as_Scoreable_mut(&mut self) -> &mut crate::org::allbinary::game::score::Scoreable::Scoreable { &mut self.base }
}
impl crate::org::allbinary::game::score::ScoreableInterface::ScoreableInterface for NoScoreable {
    fn getPoints(&mut self) -> i32 { crate::org::allbinary::game::score::Scoreable::Scoreable::getPoints(crate::org::allbinary::game::score::Scoreable::ScoreableDyn::as_Scoreable_mut(self)) }
    fn addPoints(&mut self, mut points: i32) { Self::addPoints(self, points) }
    fn removePoints(&mut self, mut points: i32) { Self::removePoints(self, points) }
}


