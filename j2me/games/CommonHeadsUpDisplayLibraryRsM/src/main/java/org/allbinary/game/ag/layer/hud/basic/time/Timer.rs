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
// package org.allbinary.game.ag.layer.hud.basic.time
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::math::PrimitiveLongSingleton::PrimitiveLongSingleton;
use crate::org::allbinary::logic::math::PrimitiveLongUtil::PrimitiveLongUtil;
use crate::org::allbinary::time::GameTickTimeDelayHelper::GameTickTimeDelayHelper;
use crate::org::allbinary::time::GameTickTimeDelayHelperFactory::GameTickTimeDelayHelperFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct Timer {
    pub startTime: i64,
    pub hundredthTime: i64,
    pub maxTime: i64,
    pub modifier: i64,
    pub hundrethsString: Vec<char>,
    pub totalDigits: i32,
    pub primitiveLongUtil: Option<Rc<RefCell<PrimitiveLongUtil>>>,
    pub MAX: Vec<char>,
    pub gameTickTimeDelayHelper: Option<Rc<RefCell<GameTickTimeDelayHelper>>>,
}


impl Timer {
    

    pub fn new_struct(mut maxTime: i32) -> Self {
        let mut this = Self {
            startTime: 0_i64,
            hundredthTime: 0_i64,
            maxTime: 0_i64,
            modifier: 0_i64,
            hundrethsString: Vec::new(),
            totalDigits: 0,
            primitiveLongUtil: None,
            MAX: Vec::new(),
            gameTickTimeDelayHelper: None,
        };
        this.startTime = ((-1) as i64);
        this.hundrethsString = jcall!(PrimitiveLongSingleton::getInstance()).ZERO.clone();
        this.totalDigits = 1;
        this.MAX = vec!['L', 'O', 'L'];
        this.gameTickTimeDelayHelper = GameTickTimeDelayHelperFactory::getInstance();
        this.primitiveLongUtil = PrimitiveLongUtil::createPowerOfTen(maxTime.clone());
        this.maxTime = ((maxTime) as i64);
        this
    }

    pub fn new(mut maxTime: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(maxTime))))
    }
    

    pub fn start(&mut self) {
        self.hundredthTime = 0_i64;
        self.startTime = jcall!(self.gameTickTimeDelayHelper).startTime.clone();
    }
    

    pub fn get(&mut self) -> i64 {
        return self.hundredthTime.clone();
    }
    

    pub fn update(&mut self) {
        let mut lastLowerTime: i64 = self.hundredthTime.clone();
        let mut currentTime: i64 = ((self.modifier + jcall!(self.gameTickTimeDelayHelper).startTime) - self.startTime);
        if currentTime < 0_i64 {
            currentTime = 0_i64;
        }
        self.hundredthTime = currentTime / 100_i64;
        if self.hundredthTime >= self.maxTime {
            if self.startTime > 0_i64 {
                self.hundrethsString = self.MAX.clone();
            }
        } else if lastLowerTime != self.hundredthTime {
            self.hundrethsString = jcall!(self.primitiveLongUtil).getCharArray(((self.hundredthTime) as i32));
            self.totalDigits = jcall!(self.primitiveLongUtil).getCurrentTotalDigits();
        }
    }
    

    pub fn reduce(&mut self, mut value: i32) {
        self.modifier -= ((value) as i64).clone();
    }
    

    pub fn add(&mut self, mut value: i32) {
        self.modifier += ((value) as i64).clone();
    }
    

    pub fn getTimeChars(&mut self) -> Vec<char> {
        return self.hundrethsString.clone();
    }
    

    pub fn getCurrentTotalDigits(&mut self) -> i32 {
        return self.totalDigits.clone();
    }
}
pub trait TimerDyn {
    fn as_Timer(&self) -> &Timer;
    fn as_Timer_mut(&mut self) -> &mut Timer;
    fn start(&mut self) { Timer::start(self.as_Timer_mut()) }
    fn get(&mut self) -> i64 { Timer::get(self.as_Timer_mut()) }
    fn update(&mut self) { Timer::update(self.as_Timer_mut()) }
    fn reduce(&mut self, mut value: i32) { Timer::reduce(self.as_Timer_mut(), value) }
    fn add(&mut self, mut value: i32) { Timer::add(self.as_Timer_mut(), value) }
    fn getTimeChars(&mut self) -> Vec<char> { Timer::getTimeChars(self.as_Timer_mut()) }
    fn getCurrentTotalDigits(&mut self) -> i32 { Timer::getCurrentTotalDigits(self.as_Timer_mut()) }
}
impl TimerDyn for Timer {
    fn as_Timer(&self) -> &Timer { self }
    fn as_Timer_mut(&mut self) -> &mut Timer { self }
}


