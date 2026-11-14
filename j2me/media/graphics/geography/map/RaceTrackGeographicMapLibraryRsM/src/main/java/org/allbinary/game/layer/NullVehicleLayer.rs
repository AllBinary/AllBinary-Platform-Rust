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
use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::animation::RotationAnimation::RotationAnimation;
use crate::org::allbinary::game::identification::Group::Group;
use crate::org::allbinary::logic::math::BasicDecimal::BasicDecimal;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellHistory::GeographicMapCellHistory;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPositionTracking::GeographicMapCellPositionTracking;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::java::lang::Exception::*;
use crate::java::lang::RuntimeException::*;
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
impl VehicleLayerInterface for NullVehicleLayer {
    fn getGameKeyEventList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Self::getGameKeyEventList(self) }
    fn getSpeedBasicDecimal(&mut self) -> Option<Rc<RefCell<BasicDecimal>>> { Self::getSpeedBasicDecimal(self) }
    fn isReadyForExplosion(&mut self) -> bool { Self::isReadyForExplosion(self) }
    fn getRotationAnimationInterface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn>>> { Self::getRotationAnimationInterface(self) }
    fn setRotationAnimationInterface(&mut self, mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn>>>) { Self::setRotationAnimationInterface(self, animationInterface) }
    fn isDestroyed(&mut self) -> bool { Self::isDestroyed(self) }
    fn isFinish(&mut self) -> bool { Self::isFinish(self) }
    fn getFinalPosition(&mut self) -> i32 { Self::getFinalPosition(self) }
    fn handleFinish(&mut self) { Self::handleFinish(self) }
    fn getGeographicMapCellPositionTracking(&mut self) -> Option<Rc<RefCell<GeographicMapCellPositionTracking>>> { Self::getGeographicMapCellPositionTracking(self) }
    fn getGeographicMapCellHistoryArray(&mut self) -> Vec<Option<Rc<RefCell<GeographicMapCellHistory>>>> { Self::getGeographicMapCellHistoryArray(self) }
    fn setGeographicMapCellHistoryArray(&mut self, mut geographicMapCellHistory: Vec<Option<Rc<RefCell<GeographicMapCellHistory>>>>) { Self::setGeographicMapCellHistoryArray(self, geographicMapCellHistory) }
}
impl crate::org::allbinary::game::layer::VehiclePropertiesCompositeInterface::VehiclePropertiesCompositeInterface for NullVehicleLayer {
    fn getVehicleProperties(&mut self) -> Option<Rc<RefCell<VehicleProperties>>> { Self::getVehicleProperties(self) }
}
impl crate::org::allbinary::game::identification::GroupInterfaceCompositeInterface::GroupInterfaceCompositeInterface for NullVehicleLayer {
    fn getGroupInterface(&mut self) -> Vec<Option<Rc<RefCell<Group>>>> { Self::getGroupInterface(self) }
}
impl crate::org::allbinary::layer::LayerInterface::LayerInterface for NullVehicleLayer {
    fn getHeight(&mut self) -> i32 { Self::getHeight(self) }
    fn getWidth(&mut self) -> i32 { Self::getWidth(self) }
    fn getHalfHeight(&mut self) -> i32 { Self::getHalfHeight(self) }
    fn getHalfWidth(&mut self) -> i32 { Self::getHalfWidth(self) }
    fn getXP(&mut self) -> i32 { Self::getXP(self) }
    fn getYP(&mut self) -> i32 { Self::getYP(self) }
    fn getZP(&mut self) -> i32 { Self::getZP(self) }
    fn getX2(&mut self) -> i32 { Self::getX2(self) }
    fn getY2(&mut self) -> i32 { Self::getY2(self) }
    fn getZ2(&mut self) -> i32 { Self::getZ2(self) }
    fn isVisible(&mut self) -> bool { Self::isVisible(self) }
    fn moveDXYZ(&mut self, mut dx: i32, mut dy: i32, mut dz: i32) { Self::moveDXYZ(self, dx, dy, dz) }
    fn setVisible(&mut self, mut visible: bool) { Self::setVisible(self, visible) }
    fn implmentsTickableInterface(&mut self) -> bool { Self::implmentsTickableInterface(self) }
    fn implmentsCollidableInterface(&mut self) -> bool { Self::implmentsCollidableInterface(self) }
    fn implmentsGameInputInterface(&mut self) -> bool { Self::implmentsGameInputInterface(self) }
    fn implmentsArtificialIntelligenceCompositeInterface(&mut self) -> bool { Self::implmentsArtificialIntelligenceCompositeInterface(self) }
    fn getType(&mut self) -> i32 { Self::getType(self) }
}
impl crate::org::allbinary::layer::NamedInterface::NamedInterface for NullVehicleLayer {
    fn getName(&mut self) -> String { Self::getName(self) }
}
impl crate::org::allbinary::layer::PositionInterface::PositionInterface for NullVehicleLayer {
    fn setPosition(&mut self, mut x: i32, mut y: i32, mut z: i32) { Self::setPosition(self, x, y, z) }
}
impl crate::org::allbinary::graphics::paint::PaintableInterface::PaintableInterface for NullVehicleLayer {
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::paint(self, graphics) }
    fn paintThreed(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::paintThreed(self, graphics) }
}

impl NullVehicleLayer {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }

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
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
    }
    

    pub fn paintThreed(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
    }
}

