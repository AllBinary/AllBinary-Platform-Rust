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
// package org.allbinary.game.displayable.canvas
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use crate::Runnable;
use super::*;
use crate::org::allbinary::graphics::displayable::DisplayInfoSingleton::DisplayInfoSingleton;
use crate::java::lang::Thread::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GameRunnable {
    pub WAIT: i32,
    pub FAST: i32,
    pub waitInMillis: i64,
}

impl Runnable for GameRunnable {
    fn run(&mut self) { Self::run(self) }
}

impl GameRunnable {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            WAIT: 0,
            FAST: 0,
            waitInMillis: 0_i64,
        };
        this.WAIT = 240;
        this.FAST = 60;
        this.waitInMillis = ((this.WAIT) as i64);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn run(&mut self) {
        jcall!(DisplayInfoSingleton::getInstance()).process();
    }
    

    // throws Exception
    pub fn processLoopSleep(&mut self) {
        std::thread::sleep(std::time::Duration::from_millis((self.waitInMillis.clone()) as u64));
    }
}
pub trait GameRunnableDyn: crate::Runnable {
    fn as_GameRunnable(&self) -> &GameRunnable;
    fn as_GameRunnable_mut(&mut self) -> &mut GameRunnable;
    fn processLoopSleep(&mut self) { GameRunnable::processLoopSleep(self.as_GameRunnable_mut()) }
}
impl GameRunnableDyn for GameRunnable {
    fn as_GameRunnable(&self) -> &GameRunnable { self }
    fn as_GameRunnable_mut(&mut self) -> &mut GameRunnable { self }
}


