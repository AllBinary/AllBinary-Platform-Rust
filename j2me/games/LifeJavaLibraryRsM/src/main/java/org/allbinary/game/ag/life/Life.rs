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
// package org.allbinary.game.ag.life
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::LifeInterface::LifeInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct Life {
    pub lives: i16,
    pub startLives: i16,
    pub maxlives: i16,
}

thread_local! { pub static Life_NO_LIFE: Option<Rc<RefCell<Life>>> = Life::createLife(); }
impl LifeInterface for Life {
    fn get(&mut self) -> i16 { Self::get(self) }
    fn set(&mut self, mut lives: i16) { Self::set(self, lives) }
    fn reset(&mut self) { Self::reset(self) }
    fn add(&mut self, mut lives: i16) { Self::add(self, lives) }
    fn deaths(&mut self, mut lives: i16) { Self::deaths(self, lives) }
    fn getMaxlives(&mut self) -> i16 { Self::getMaxlives(self) }
    fn isAlive(&mut self) -> bool { Self::isAlive(self) }
}

impl Life {
    

    pub fn createLife() -> Option<Rc<RefCell<Life>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            return Life::new(((0) as i16), ((0) as i16));
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
            }
        }
    }
    

    pub fn new_struct(mut lives: i16, mut maxlives: i16) -> Self {
        let mut this = Self {
            lives: 0,
            startLives: 0,
            maxlives: 0,
        };
        this.maxlives = maxlives.clone();
        if ((lives) as i32) > ((this.maxlives) as i32) {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("To Many Lives"))));
        }
        this.lives = lives.clone();
        this.startLives = lives.clone();
        this
    }

    pub fn new(mut lives: i16, mut maxlives: i16) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(lives, maxlives))))
    }
    

    pub fn get(&mut self) -> i16 {
        return self.lives.clone();
    }
    

    pub fn set(&mut self, mut lives: i16) {
        self.lives = lives.clone();
    }
    

    pub fn reset(&mut self) {
        self.lives = self.startLives.clone();
    }
    

    pub fn add(&mut self, mut lives: i16) {
        if !self.reachedLimit() {
            if ((self.lives) as i32) + ((lives) as i32) <= ((self.getMaxlives()) as i32) {
                self.lives = (((((self.lives) as i32) + ((lives) as i32))) as i16);
            } else {
                self.lives = self.getMaxlives();
            }
        }
    }
    

    pub fn isAlive(&mut self) -> bool {
        if ((self.lives) as i32) <= 0 {
            return false;
        } else {
            return true;
        }
    }
    

    pub fn getMaxlives(&mut self) -> i16 {
        return self.maxlives.clone();
    }
    

    pub fn reachedLimit(&mut self) -> bool {
        if ((self.lives) as i32) > ((self.maxlives) as i32) || ((self.lives) as i32) < 0 {
            return true;
        } else {
            return false;
        }
    }
    

    pub fn deaths(&mut self, mut lives: i16) {
        if !self.reachedLimit() {
            if ((self.lives) as i32) - ((lives) as i32) >= 0 {
                self.lives = (((((self.lives) as i32) - ((lives) as i32))) as i16);
            } else {
                self.lives = 0;
            }
        }
    }
    

    pub fn getStartLives(&mut self) -> i16 {
        return self.startLives.clone();
    }
    

    pub fn setStartLives(&mut self, mut startLives: i16) {
        self.startLives = startLives.clone();
    }
}

