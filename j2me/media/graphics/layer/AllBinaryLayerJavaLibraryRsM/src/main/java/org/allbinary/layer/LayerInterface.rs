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
// package org.allbinary.layer
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::identification::GroupInterfaceCompositeInterface::GroupInterfaceCompositeInterface;
use crate::org::allbinary::graphics::paint::PaintableInterface::PaintableInterface;
use super::NamedInterface::NamedInterface;
use super::PositionInterface::PositionInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait LayerInterface: NamedInterface + PaintableInterface + GroupInterfaceCompositeInterface + PositionInterface {
    

    fn getHeight(&mut self) -> i32;
    

    fn getWidth(&mut self) -> i32;
    

    fn getHalfHeight(&mut self) -> i32;
    

    fn getHalfWidth(&mut self) -> i32;
    

    fn getXP(&mut self) -> i32;
    

    fn getYP(&mut self) -> i32;
    

    fn getZP(&mut self) -> i32;
    

    fn getX2(&mut self) -> i32;
    

    fn getY2(&mut self) -> i32;
    

    fn getZ2(&mut self) -> i32;
    

    fn isVisible(&mut self) -> bool;
    

    fn moveDXYZ(&mut self, dx: i32, dy: i32, dz: i32);
    

    fn setVisible(&mut self, visible: bool);
    

    fn implmentsTickableInterface(&mut self) -> bool;
    

    fn implmentsCollidableInterface(&mut self) -> bool;
    

    fn implmentsGameInputInterface(&mut self) -> bool;
    

    fn implmentsArtificialIntelligenceCompositeInterface(&mut self) -> bool;
    

    fn getType(&mut self) -> i32;
}


