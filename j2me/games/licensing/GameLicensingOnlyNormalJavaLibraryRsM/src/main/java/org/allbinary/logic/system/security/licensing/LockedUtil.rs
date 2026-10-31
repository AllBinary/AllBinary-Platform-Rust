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
// package org.allbinary.logic.system.security.licensing
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::LockedUtilInterface::LockedUtilInterface;
use super::LockableFeature::LockableFeature;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LockedUtil {
}

thread_local! { pub static LockedUtil_instance: Option<Rc<RefCell<LockedUtil>>> = LockedUtil::new(); }
impl LockedUtilInterface for LockedUtil {
    fn isLocked(&mut self) -> bool { Self::isLocked(self) }
    fn isLockedFeature(&mut self, mut lockableFeatrue: Option<Rc<RefCell<LockableFeature>>>) -> bool { Self::isLockedFeature(self, lockableFeatrue) }
}

impl LockedUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<LockedUtil>>> {
        return LockedUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn isLocked(&mut self) -> bool {
        return false;
    }
    

    pub fn isLockedFeature(&mut self, mut lockableFeatrue: Option<Rc<RefCell<LockableFeature>>>) -> bool {
        return false;
    }
}

