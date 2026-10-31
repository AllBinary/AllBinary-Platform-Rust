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
// package org.allbinary.android.input.motion.api1
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::android::input::motion::GameInputMotionInfo::GameInputMotionInfo;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GameInputMotionInfoAPI1 {
    pub base: GameInputMotionInfo,
}

thread_local! { pub static GameInputMotionInfoAPI1_instance: Option<Rc<RefCell<GameInputMotionInfoAPI1>>> = GameInputMotionInfoAPI1::new(); }

impl GameInputMotionInfoAPI1 {
    

    pub fn getInstance() -> Option<Rc<RefCell<GameInputMotionInfoAPI1>>> {
        return GameInputMotionInfoAPI1_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: GameInputMotionInfo::new_struct(4, 1),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}
impl std::ops::Deref for GameInputMotionInfoAPI1 {
    type Target = GameInputMotionInfo;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for GameInputMotionInfoAPI1 {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::android::input::motion::GameInputMotionInfo::GameInputMotionInfoDyn for GameInputMotionInfoAPI1 {
    fn as_GameInputMotionInfo(&self) -> &crate::org::allbinary::android::input::motion::GameInputMotionInfo::GameInputMotionInfo { &self.base }
    fn as_GameInputMotionInfo_mut(&mut self) -> &mut crate::org::allbinary::android::input::motion::GameInputMotionInfo::GameInputMotionInfo { &mut self.base }
}


