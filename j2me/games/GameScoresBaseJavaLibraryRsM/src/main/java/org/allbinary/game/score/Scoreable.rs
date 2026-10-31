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
use crate::org::allbinary::logic::math::SmallIntegerSingletonFactory::SmallIntegerSingletonFactory;
use super::ScoreableInterface::ScoreableInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct Scoreable {
    pub points: i32,
}

pub static mut Scoreable_ID: std::cell::LazyCell<Option<i32>> = std::cell::LazyCell::new(|| jcall!(SmallIntegerSingletonFactory::getInstance()).getAt(3));
impl ScoreableInterface for Scoreable {
    fn getPoints(&mut self) -> i32 { Self::getPoints(self) }
    fn addPoints(&mut self, mut points: i32) { Self::addPoints(self, points) }
    fn removePoints(&mut self, mut points: i32) { Self::removePoints(self, points) }
}

impl Scoreable {
    

    pub fn new_struct(mut points: i32) -> Self {
        let mut this = Self {
            points: 0,
        };
        this.points = points.clone();
        this
    }

    pub fn new(mut points: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(points))))
    }
    

    pub fn addPoints(&mut self, mut points: i32) {
        self.points += points.clone();
    }
    

    pub fn removePoints(&mut self, mut points: i32) {
        self.points -= points.clone();
    }
    

    pub fn setPoints(&mut self, mut points: i32) {
        self.points = points.clone();
    }
    

    pub fn getPoints(&mut self) -> i32 {
        return self.points.clone();
    }
}
pub trait ScoreableDyn: crate::org::allbinary::game::score::ScoreableInterface::ScoreableInterface {
    fn as_Scoreable(&self) -> &Scoreable;
    fn as_Scoreable_mut(&mut self) -> &mut Scoreable;
    fn setPoints(&mut self, mut points: i32) { Scoreable::setPoints(self.as_Scoreable_mut(), points) }
}
impl ScoreableDyn for Scoreable {
    fn as_Scoreable(&self) -> &Scoreable { self }
    fn as_Scoreable_mut(&mut self) -> &mut Scoreable { self }
}


