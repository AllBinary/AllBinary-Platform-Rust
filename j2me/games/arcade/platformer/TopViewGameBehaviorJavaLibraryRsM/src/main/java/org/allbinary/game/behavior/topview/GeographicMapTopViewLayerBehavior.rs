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
// package org.allbinary.game.behavior.topview
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::physics::velocity::VelocityProperties::VelocityProperties;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMap;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPosition;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellType;
use crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory;
use crate::org::allbinary::media::graphics::geography::map::topview::BasicTopViewGeographicMapCellTypeFactory::BasicTopViewGeographicMapCellTypeFactory;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use super::TopViewGameLayerBehavior::TopViewGameLayerBehavior;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GeographicMapTopViewLayerBehavior {
    pub base: TopViewGameLayerBehavior,
}


impl GeographicMapTopViewLayerBehavior {
    

    pub fn new_struct(mut maxGravityActionIndex: i32) -> Self {
        let mut this = Self {
            base: TopViewGameLayerBehavior::new_struct(maxGravityActionIndex.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut maxGravityActionIndex: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(maxGravityActionIndex))))
    }
    

    // throws Exception
    pub fn getGeographicMapCellPositionIfNotSolidBlockOrOffMapLocation(&mut self, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<BasicGeographicMap>>>>, mut geographicMapCellTypeArray: Vec<Option<Rc<RefCell<GeographicMapCellType>>>>, mut velocityProperties: Option<Rc<RefCell<VelocityProperties>>>, mut layer: Option<Rc<RefCell<AllBinaryLayer>>>, mut x: i32, mut y: i32) -> Option<Rc<RefCell<GeographicMapCellPosition>>> {
        return unsafe { crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.clone() }.clone();
    }
    

    // throws Exception
    pub fn getGeographicMapCellPositionIfNotSolidBlockOrOffMap(&mut self, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<BasicGeographicMap>>>>, mut geographicMapCellTypeArray: Vec<Option<Rc<RefCell<GeographicMapCellType>>>>, mut geographicMapCellPosition: Option<Rc<RefCell<GeographicMapCellPosition>>>, mut velocityProperties: Option<Rc<RefCell<VelocityProperties>>>, mut layer: Option<Rc<RefCell<AllBinaryLayer>>>) -> Option<Rc<RefCell<GeographicMapCellPosition>>> {
        return unsafe { crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.clone() }.clone();
    }
    

    // throws Exception
    pub fn getGeographicMapCellPositionFromListIfNotSolidBlockOrOffMap(&mut self, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<BasicGeographicMap>>>>, mut geographicMapCellTypeArray: Vec<Option<Rc<RefCell<GeographicMapCellType>>>>, mut geographicMapCellPositionList: Option<Rc<RefCell<BasicArrayList>>>, mut velocityProperties: Option<Rc<RefCell<VelocityProperties>>>, mut layer: Option<Rc<RefCell<AllBinaryLayer>>>) -> Option<Rc<RefCell<GeographicMapCellPosition>>> {
        return unsafe { crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.clone() }.clone();
    }
    

    // throws Exception
    pub fn gravity(&mut self, mut velocityProperties: Option<Rc<RefCell<VelocityProperties>>>, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<BasicGeographicMap>>>>, mut geographicMapCellTypeArray: Vec<Option<Rc<RefCell<GeographicMapCellType>>>>, mut geographicMapCellPosition: Option<Rc<RefCell<GeographicMapCellPosition>>>) {
    }
    

    // throws Exception
    pub fn left(&mut self, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<BasicGeographicMap>>>>, mut geographicMapCellTypeArray: Vec<Option<Rc<RefCell<GeographicMapCellType>>>>, mut velocityProperties: Option<Rc<RefCell<VelocityProperties>>>, mut layer: Option<Rc<RefCell<AllBinaryLayer>>>) {
    }
    

    // throws Exception
    pub fn r#move(&mut self, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<BasicGeographicMap>>>>, mut geographicMapCellTypeArray: Vec<Option<Rc<RefCell<GeographicMapCellType>>>>, mut velocityProperties: Option<Rc<RefCell<VelocityProperties>>>, mut layer: Option<Rc<RefCell<AllBinaryLayer>>>, mut x: i32, mut y: i32) -> bool {
        return false;
    }
    

    // throws Exception
    pub fn moveAndLand(&mut self, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<BasicGeographicMap>>>>, mut geographicMapCellTypeArray: Vec<Option<Rc<RefCell<GeographicMapCellType>>>>, mut geographicMapCellPosition: Option<Rc<RefCell<GeographicMapCellPosition>>>, mut velocityProperties: Option<Rc<RefCell<VelocityProperties>>>, mut layer: Option<Rc<RefCell<AllBinaryLayer>>>, mut x: i32, mut y: i32) {
    }
    

    // throws Exception
    pub fn right(&mut self, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<BasicGeographicMap>>>>, mut geographicMapCellTypeArray: Vec<Option<Rc<RefCell<GeographicMapCellType>>>>, mut velocityProperties: Option<Rc<RefCell<VelocityProperties>>>, mut layer: Option<Rc<RefCell<AllBinaryLayer>>>) {
    }
    

    pub fn hasSolidBlock(&mut self, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<BasicGeographicMap>>>>, mut geographicMapCellTypeArray: Vec<Option<Rc<RefCell<GeographicMapCellType>>>>) -> bool {
        let mut size: i32 = (geographicMapInterfaceArray.len() as i32).clone();
        let mut basicTopViewGeographicMapCellTypeFactory: Option<Rc<RefCell<BasicTopViewGeographicMapCellTypeFactory>>> = None;
        for index in (0..size) {
            basicTopViewGeographicMapCellTypeFactory = (jcall!(geographicMapInterfaceArray[(index) as usize]).getGeographicMapCellTypeFactory()).clone() /* as BasicTopViewGeographicMapCellTypeFactory */;
            if jcall!(jcall!(basicTopViewGeographicMapCellTypeFactory).BLOCK_CELL_TYPE).isType(geographicMapCellTypeArray[(index) as usize].clone()) {
                return true;
            }
        }
        return false;
    }
    

    pub fn isOffMap(&mut self, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<BasicGeographicMap>>>>, mut geographicMapCellTypeArray: Vec<Option<Rc<RefCell<GeographicMapCellType>>>>) -> bool {
        let mut size: i32 = (geographicMapInterfaceArray.len() as i32).clone();
        let mut basicTopViewGeographicMapCellTypeFactory: Option<Rc<RefCell<BasicTopViewGeographicMapCellTypeFactory>>> = None;
        for index in (0..size) {
            basicTopViewGeographicMapCellTypeFactory = (jcall!(geographicMapInterfaceArray[(index) as usize]).getGeographicMapCellTypeFactory()).clone() /* as BasicTopViewGeographicMapCellTypeFactory */;
            if jcall!(jcall!(basicTopViewGeographicMapCellTypeFactory).OFF_MAP_CELL_TYPE).isType(geographicMapCellTypeArray[(index) as usize].clone()) {
                return true;
            }
        }
        return false;
    }
}
impl std::ops::Deref for GeographicMapTopViewLayerBehavior {
    type Target = TopViewGameLayerBehavior;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for GeographicMapTopViewLayerBehavior {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait GeographicMapTopViewLayerBehaviorDyn: crate::org::allbinary::game::behavior::topview::TopViewGameLayerBehavior::TopViewGameLayerBehaviorDyn {
    fn as_GeographicMapTopViewLayerBehavior(&self) -> &GeographicMapTopViewLayerBehavior;
    fn as_GeographicMapTopViewLayerBehavior_mut(&mut self) -> &mut GeographicMapTopViewLayerBehavior;
    fn getGeographicMapCellPositionIfNotSolidBlockOrOffMapLocation(&mut self, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<BasicGeographicMap>>>>, mut geographicMapCellTypeArray: Vec<Option<Rc<RefCell<GeographicMapCellType>>>>, mut velocityProperties: Option<Rc<RefCell<VelocityProperties>>>, mut layer: Option<Rc<RefCell<AllBinaryLayer>>>, mut x: i32, mut y: i32) -> Option<Rc<RefCell<GeographicMapCellPosition>>> { GeographicMapTopViewLayerBehavior::getGeographicMapCellPositionIfNotSolidBlockOrOffMapLocation(self.as_GeographicMapTopViewLayerBehavior_mut(), geographicMapInterfaceArray, geographicMapCellTypeArray, velocityProperties, layer, x, y) }
    fn getGeographicMapCellPositionIfNotSolidBlockOrOffMap(&mut self, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<BasicGeographicMap>>>>, mut geographicMapCellTypeArray: Vec<Option<Rc<RefCell<GeographicMapCellType>>>>, mut geographicMapCellPosition: Option<Rc<RefCell<GeographicMapCellPosition>>>, mut velocityProperties: Option<Rc<RefCell<VelocityProperties>>>, mut layer: Option<Rc<RefCell<AllBinaryLayer>>>) -> Option<Rc<RefCell<GeographicMapCellPosition>>> { GeographicMapTopViewLayerBehavior::getGeographicMapCellPositionIfNotSolidBlockOrOffMap(self.as_GeographicMapTopViewLayerBehavior_mut(), geographicMapInterfaceArray, geographicMapCellTypeArray, geographicMapCellPosition, velocityProperties, layer) }
    fn getGeographicMapCellPositionFromListIfNotSolidBlockOrOffMap(&mut self, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<BasicGeographicMap>>>>, mut geographicMapCellTypeArray: Vec<Option<Rc<RefCell<GeographicMapCellType>>>>, mut geographicMapCellPositionList: Option<Rc<RefCell<BasicArrayList>>>, mut velocityProperties: Option<Rc<RefCell<VelocityProperties>>>, mut layer: Option<Rc<RefCell<AllBinaryLayer>>>) -> Option<Rc<RefCell<GeographicMapCellPosition>>> { GeographicMapTopViewLayerBehavior::getGeographicMapCellPositionFromListIfNotSolidBlockOrOffMap(self.as_GeographicMapTopViewLayerBehavior_mut(), geographicMapInterfaceArray, geographicMapCellTypeArray, geographicMapCellPositionList, velocityProperties, layer) }
    fn gravity(&mut self, mut velocityProperties: Option<Rc<RefCell<VelocityProperties>>>, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<BasicGeographicMap>>>>, mut geographicMapCellTypeArray: Vec<Option<Rc<RefCell<GeographicMapCellType>>>>, mut geographicMapCellPosition: Option<Rc<RefCell<GeographicMapCellPosition>>>) { GeographicMapTopViewLayerBehavior::gravity(self.as_GeographicMapTopViewLayerBehavior_mut(), velocityProperties, geographicMapInterfaceArray, geographicMapCellTypeArray, geographicMapCellPosition) }
    fn left(&mut self, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<BasicGeographicMap>>>>, mut geographicMapCellTypeArray: Vec<Option<Rc<RefCell<GeographicMapCellType>>>>, mut velocityProperties: Option<Rc<RefCell<VelocityProperties>>>, mut layer: Option<Rc<RefCell<AllBinaryLayer>>>) { GeographicMapTopViewLayerBehavior::left(self.as_GeographicMapTopViewLayerBehavior_mut(), geographicMapInterfaceArray, geographicMapCellTypeArray, velocityProperties, layer) }
    fn r#move(&mut self, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<BasicGeographicMap>>>>, mut geographicMapCellTypeArray: Vec<Option<Rc<RefCell<GeographicMapCellType>>>>, mut velocityProperties: Option<Rc<RefCell<VelocityProperties>>>, mut layer: Option<Rc<RefCell<AllBinaryLayer>>>, mut x: i32, mut y: i32) -> bool { GeographicMapTopViewLayerBehavior::r#move(self.as_GeographicMapTopViewLayerBehavior_mut(), geographicMapInterfaceArray, geographicMapCellTypeArray, velocityProperties, layer, x, y) }
    fn moveAndLand(&mut self, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<BasicGeographicMap>>>>, mut geographicMapCellTypeArray: Vec<Option<Rc<RefCell<GeographicMapCellType>>>>, mut geographicMapCellPosition: Option<Rc<RefCell<GeographicMapCellPosition>>>, mut velocityProperties: Option<Rc<RefCell<VelocityProperties>>>, mut layer: Option<Rc<RefCell<AllBinaryLayer>>>, mut x: i32, mut y: i32) { GeographicMapTopViewLayerBehavior::moveAndLand(self.as_GeographicMapTopViewLayerBehavior_mut(), geographicMapInterfaceArray, geographicMapCellTypeArray, geographicMapCellPosition, velocityProperties, layer, x, y) }
    fn right(&mut self, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<BasicGeographicMap>>>>, mut geographicMapCellTypeArray: Vec<Option<Rc<RefCell<GeographicMapCellType>>>>, mut velocityProperties: Option<Rc<RefCell<VelocityProperties>>>, mut layer: Option<Rc<RefCell<AllBinaryLayer>>>) { GeographicMapTopViewLayerBehavior::right(self.as_GeographicMapTopViewLayerBehavior_mut(), geographicMapInterfaceArray, geographicMapCellTypeArray, velocityProperties, layer) }
    fn hasSolidBlock(&mut self, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<BasicGeographicMap>>>>, mut geographicMapCellTypeArray: Vec<Option<Rc<RefCell<GeographicMapCellType>>>>) -> bool { GeographicMapTopViewLayerBehavior::hasSolidBlock(self.as_GeographicMapTopViewLayerBehavior_mut(), geographicMapInterfaceArray, geographicMapCellTypeArray) }
    fn isOffMap(&mut self, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<BasicGeographicMap>>>>, mut geographicMapCellTypeArray: Vec<Option<Rc<RefCell<GeographicMapCellType>>>>) -> bool { GeographicMapTopViewLayerBehavior::isOffMap(self.as_GeographicMapTopViewLayerBehavior_mut(), geographicMapInterfaceArray, geographicMapCellTypeArray) }
}
impl GeographicMapTopViewLayerBehaviorDyn for GeographicMapTopViewLayerBehavior {
    fn as_GeographicMapTopViewLayerBehavior(&self) -> &GeographicMapTopViewLayerBehavior { self }
    fn as_GeographicMapTopViewLayerBehavior_mut(&mut self) -> &mut GeographicMapTopViewLayerBehavior { self }
}

impl crate::org::allbinary::game::behavior::topview::TopViewGameLayerBehavior::TopViewGameLayerBehaviorDyn for GeographicMapTopViewLayerBehavior {
    fn as_TopViewGameLayerBehavior(&self) -> &crate::org::allbinary::game::behavior::topview::TopViewGameLayerBehavior::TopViewGameLayerBehavior { &self.base }
    fn as_TopViewGameLayerBehavior_mut(&mut self) -> &mut crate::org::allbinary::game::behavior::topview::TopViewGameLayerBehavior::TopViewGameLayerBehavior { &mut self.base }
}


