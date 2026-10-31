// Generated Code Do Not Modify
// package org.allbinary.game
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicGameInfoFactory {
    pub currentReleaseIndex: i32,
    pub currentDestroyIndex: i32,
    pub total: i32,
}

thread_local! { pub static BasicGameInfoFactory_SINGLETON: Option<Rc<RefCell<BasicGameInfoFactory>>> = BasicGameInfoFactory::new(); }

impl BasicGameInfoFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<BasicGameInfoFactory>>> {
        return BasicGameInfoFactory_SINGLETON.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            currentReleaseIndex: 0,
            currentDestroyIndex: 0,
            total: 0,
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn init(&mut self, mut total: i32) {
        self.currentReleaseIndex = 0;
        self.currentDestroyIndex = 0;
        self.total = total.clone();
    }
    

    pub fn isPeaked(&mut self) -> bool {
        if self.currentDestroyIndex >= self.total {
            return true;
        } else {
            return false;
        }
    }
    

    pub fn howManyLeft(&mut self) -> i32 {
        return self.total - self.currentDestroyIndex;
    }
}

