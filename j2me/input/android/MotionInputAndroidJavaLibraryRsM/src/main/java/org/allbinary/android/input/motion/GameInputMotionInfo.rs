//
// AllBinary Open License Version 1
// Copyright (c) 2009 AllBinary
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
// package org.allbinary.android.input.motion
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

pub struct GameInputMotionInfo {
    pub MAX_API: i32,
    pub MAX_POINTERS: i32,
}


impl GameInputMotionInfo {
    

    pub fn new_struct(mut maxApi: i32, mut maxPointers: i32) -> Self {
        let mut this = Self {
            MAX_API: 0,
            MAX_POINTERS: 0,
        };
        this.MAX_API = maxApi.clone();
        this.MAX_POINTERS = maxPointers.clone();
        this
    }

    pub fn new(mut maxApi: i32, mut maxPointers: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(maxApi, maxPointers))))
    }
}
pub trait GameInputMotionInfoDyn {
    fn as_GameInputMotionInfo(&self) -> &GameInputMotionInfo;
    fn as_GameInputMotionInfo_mut(&mut self) -> &mut GameInputMotionInfo;
}
impl GameInputMotionInfoDyn for GameInputMotionInfo {
    fn as_GameInputMotionInfo(&self) -> &GameInputMotionInfo { self }
    fn as_GameInputMotionInfo_mut(&mut self) -> &mut GameInputMotionInfo { self }
}


