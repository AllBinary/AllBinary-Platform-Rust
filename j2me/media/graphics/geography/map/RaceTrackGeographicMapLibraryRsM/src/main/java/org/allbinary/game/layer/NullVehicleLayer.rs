// 
// AllBinary Open License Version 1 
// Copyright (c) 2025 AllBinary 
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
// package org.allbinary.game.layer
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::animation::RotationAnimation::RotationAnimation;
use crate::org::allbinary::game::identification::Group::Group;
use crate::org::allbinary::logic::math::BasicDecimal::BasicDecimal;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellHistory::GeographicMapCellHistory;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPositionTracking::GeographicMapCellPositionTracking;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use super::VehicleLayerInterface::VehicleLayerInterface;
use super::VehicleProperties::VehicleProperties;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NullVehicleLayer {
}

thread_local! { pub static NullVehicleLayer_NULL_VEHICLE_LAYER: Option<Rc<RefCell<NullVehicleLayer>>> = NullVehicleLayer::new(); }
// impl VehicleLayerInterface for NullVehicleLayer with the methods of the trait

impl NullVehicleLayer {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getName(&mut self) -> String {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getGroupInterface(&mut self) -> Vec<Option<Rc<RefCell<Group>>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn setPosition(&mut self, mut x: i32, mut y: i32, mut z: i32) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getHeight(&mut self) -> i32 {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getWidth(&mut self) -> i32 {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getHalfHeight(&mut self) -> i32 {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getHalfWidth(&mut self) -> i32 {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getXP(&mut self) -> i32 {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getYP(&mut self) -> i32 {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getZP(&mut self) -> i32 {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getX2(&mut self) -> i32 {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getY2(&mut self) -> i32 {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getZ2(&mut self) -> i32 {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn isVisible(&mut self) -> bool {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn moveDXYZ(&mut self, mut dx: i32, mut dy: i32, mut dz: i32) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn setVisible(&mut self, mut visible: bool) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn implmentsTickableInterface(&mut self) -> bool {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn implmentsCollidableInterface(&mut self) -> bool {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn implmentsGameInputInterface(&mut self) -> bool {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn implmentsArtificialIntelligenceCompositeInterface(&mut self) -> bool {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getType(&mut self) -> i32 {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getVehicleProperties(&mut self) -> Option<Rc<RefCell<VehicleProperties>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getGameKeyEventList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getSpeedBasicDecimal(&mut self) -> Option<Rc<RefCell<BasicDecimal>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn isReadyForExplosion(&mut self) -> bool {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getRotationAnimationInterface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn setRotationAnimationInterface(&mut self, mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn>>>) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn isDestroyed(&mut self) -> bool {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn isFinish(&mut self) -> bool {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getFinalPosition(&mut self) -> i32 {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // throws Exception
    pub fn handleFinish(&mut self) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getGeographicMapCellPositionTracking(&mut self) -> Option<Rc<RefCell<GeographicMapCellPositionTracking>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getGeographicMapCellHistoryArray(&mut self) -> Vec<Option<Rc<RefCell<GeographicMapCellHistory>>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn setGeographicMapCellHistoryArray(&mut self, mut geographicMapCellHistory: Vec<Option<Rc<RefCell<GeographicMapCellHistory>>>>) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
    }
    

    pub fn paintThreed(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
    }
}

