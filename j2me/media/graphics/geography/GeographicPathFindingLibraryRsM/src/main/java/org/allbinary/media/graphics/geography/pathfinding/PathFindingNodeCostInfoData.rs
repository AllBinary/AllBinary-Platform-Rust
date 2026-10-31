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
// package org.allbinary.media.graphics.geography.pathfinding
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

pub struct PathFindingNodeCostInfoData {
    pub MAX_TOTAL_COST: i32,
    pub MAX_NODE_COST: i32,
}

thread_local! { pub static PathFindingNodeCostInfoData_instance: Option<Rc<RefCell<PathFindingNodeCostInfoData>>> = PathFindingNodeCostInfoData::new(); }

impl PathFindingNodeCostInfoData {
    

    pub fn getInstance() -> Option<Rc<RefCell<PathFindingNodeCostInfoData>>> {
        return PathFindingNodeCostInfoData_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            MAX_TOTAL_COST: 0,
            MAX_NODE_COST: 0,
        };
        this.MAX_TOTAL_COST = i32::MAX.clone();
        this.MAX_NODE_COST = i32::MAX / 10000;
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

