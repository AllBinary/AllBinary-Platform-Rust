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
// package org.allbinary.canvas
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::graphics::GPoint::GPoint;
use crate::org::allbinary::graphics::PointFactory::PointFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GameGlobalsFactory {
    pub newCanvas: bool,
    pub newDisplaybleTime: i64,
    pub point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>,
}

thread_local! { pub static GameGlobalsFactory_instance: Option<Rc<RefCell<GameGlobalsFactory>>> = GameGlobalsFactory::new(); }

impl GameGlobalsFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            newCanvas: false,
            newDisplaybleTime: 0_i64,
            point: None,
        };
        this.newCanvas = true;
        this.newDisplaybleTime = ((i32::MIN) as i64);
        this.point = jcall!(PointFactory::getInstance()).ZERO_ZERO.clone();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<GameGlobalsFactory>>> {
        return GameGlobalsFactory_instance.with(Clone::clone).clone();
    }
}

