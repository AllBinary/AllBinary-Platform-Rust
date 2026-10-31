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
// package org.allbinary.game.displayable.canvas
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::GameRunnable::GameRunnable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NullWaitGameRunnable {
    pub base: GameRunnable,
}

thread_local! { pub static NullWaitGameRunnable_instance: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::GameRunnable::GameRunnableDyn>>> = NullWaitGameRunnable::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::GameRunnable::GameRunnableDyn>>); }

impl NullWaitGameRunnable {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: GameRunnable::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::GameRunnable::GameRunnableDyn>>> {
        return NullWaitGameRunnable_instance.with(Clone::clone).clone();
    }
}
impl std::ops::Deref for NullWaitGameRunnable {
    type Target = GameRunnable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for NullWaitGameRunnable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::displayable::canvas::GameRunnable::GameRunnableDyn for NullWaitGameRunnable {
    fn as_GameRunnable(&self) -> &crate::org::allbinary::game::displayable::canvas::GameRunnable::GameRunnable { &self.base }
    fn as_GameRunnable_mut(&mut self) -> &mut crate::org::allbinary::game::displayable::canvas::GameRunnable::GameRunnable { &mut self.base }
}
impl crate::Runnable for NullWaitGameRunnable {
    fn run(&mut self) { crate::org::allbinary::game::displayable::canvas::GameRunnable::GameRunnable::run(crate::org::allbinary::game::displayable::canvas::GameRunnable::GameRunnableDyn::as_GameRunnable_mut(self)) }
}


