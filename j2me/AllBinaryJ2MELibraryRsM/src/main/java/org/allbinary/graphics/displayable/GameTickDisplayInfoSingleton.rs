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
// package org.allbinary.graphics.displayable
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::DisplayInfoSingleton::DisplayInfoSingleton;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GameTickDisplayInfoSingleton {
    pub displayInfoSingleton: Option<Rc<RefCell<DisplayInfoSingleton>>>,
    pub lastWidth: i32,
    pub lastHeight: i32,
    pub lastHalfWidth: i32,
    pub lastHalfHeight: i32,
}

thread_local! { pub static GameTickDisplayInfoSingleton_instance: Option<Rc<RefCell<GameTickDisplayInfoSingleton>>> = GameTickDisplayInfoSingleton::new(); }

impl GameTickDisplayInfoSingleton {

    pub fn new_struct() -> Self {
        let mut this = Self {
            displayInfoSingleton: None,
            lastWidth: 0,
            lastHeight: 0,
            lastHalfWidth: 0,
            lastHalfHeight: 0,
        };
        this.displayInfoSingleton = DisplayInfoSingleton::getInstance();
        this.lastWidth = jcall!(this.displayInfoSingleton).getLastWidth();
        this.lastHeight = jcall!(this.displayInfoSingleton).getLastHeight();
        this.lastHalfWidth = jcall!(this.displayInfoSingleton).getLastHalfWidth();
        this.lastHalfHeight = jcall!(this.displayInfoSingleton).getLastHalfHeight();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<GameTickDisplayInfoSingleton>>> {
        return GameTickDisplayInfoSingleton_instance.with(Clone::clone).clone();
    }
    

    pub fn update(&mut self) {
        self.lastWidth = jcall!(self.displayInfoSingleton).getLastWidth();
        self.lastHeight = jcall!(self.displayInfoSingleton).getLastHeight();
        self.lastHalfWidth = jcall!(self.displayInfoSingleton).getLastHalfWidth();
        self.lastHalfHeight = jcall!(self.displayInfoSingleton).getLastHalfHeight();
    }
    

    pub fn getLastWidth(&mut self) -> i32 {
        return self.lastWidth.clone();
    }
    

    pub fn getLastHeight(&mut self) -> i32 {
        return self.lastHeight.clone();
    }
    

    pub fn getLastHalfWidth(&mut self) -> i32 {
        return self.lastHalfWidth.clone();
    }
    

    pub fn getLastHalfHeight(&mut self) -> i32 {
        return self.lastHalfHeight.clone();
    }
}

