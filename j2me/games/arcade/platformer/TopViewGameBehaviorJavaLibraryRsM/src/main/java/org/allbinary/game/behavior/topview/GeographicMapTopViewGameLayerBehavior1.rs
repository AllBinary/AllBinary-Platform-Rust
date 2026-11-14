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
use crate::org::allbinary::media::graphics::geography::map::MultiGeographicMapBehavior::MultiGeographicMapBehavior;
use crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayer;
use crate::org::allbinary::game::physics::acceleration::GravityUtil::GravityUtil;
use crate::org::allbinary::game::physics::velocity::VelocityProperties::VelocityProperties;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMap;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPosition;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellType;
use crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory;
use crate::java::lang::Exception::*;
use super::GeographicMapTopViewLayerBehavior::GeographicMapTopViewLayerBehavior;
use super::TopViewCharacterInterface::TopViewCharacterInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GeographicMapTopViewGameLayerBehavior1 {
    pub base: GeographicMapTopViewLayerBehavior,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub gravityUtil: Option<Rc<RefCell<GravityUtil>>>,
    pub geographicMapBehavior: Option<Rc<RefCell<MultiGeographicMapBehavior>>>,
    pub autoStepBlocks: bool,
    pub offsetY: i32,
    pub previousGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>,
}


impl GeographicMapTopViewGameLayerBehavior1 {
    

    pub fn new_struct(mut maxGravityActionIndex: i32, mut autoStepBlocks: bool, mut offsetY: i32) -> Self {
        let mut this = Self {
            base: GeographicMapTopViewLayerBehavior::new_struct(maxGravityActionIndex.clone()),
            logUtil: None,
            gravityUtil: None,
            geographicMapBehavior: None,
            autoStepBlocks: false,
            offsetY: 0,
            previousGeographicMapCellPosition: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.gravityUtil = GravityUtil::getInstance();
        this.geographicMapBehavior = MultiGeographicMapBehavior::new();
        this.previousGeographicMapCellPosition = crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.with(Clone::clone).clone();
        // super(...) is in the struct literal above
        this.autoStepBlocks = autoStepBlocks.clone();
        this.offsetY = offsetY.clone();
        this
    }

    pub fn new(mut maxGravityActionIndex: i32, mut autoStepBlocks: bool, mut offsetY: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(maxGravityActionIndex, autoStepBlocks, offsetY))))
    }
    

    // throws Exception
    pub fn gravity(&mut self, mut velocityProperties: Option<Rc<RefCell<VelocityProperties>>>, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>>, mut geographicMapCellTypeArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellTypeDyn>>>>, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) {
        if !match (&geographicMapCellPosition, &crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            jcall!(self.geographicMapBehavior).getCellTypeAt(geographicMapInterfaceArray.clone(), geographicMapCellTypeArray.clone(), geographicMapCellPosition.clone());
            let mut hasSolidBlock: bool = self.hasSolidBlock(geographicMapInterfaceArray.clone(), geographicMapCellTypeArray.clone());
            if !hasSolidBlock {
                jcall!(self.gravityUtil).process(velocityProperties.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::physics::velocity::BasicVelocityProperties::BasicVelocityPropertiesDyn>>), { let java_field = jcall!(self.gravityUtil).GAME_GRAVITY_VELOCITY.clone(); java_field }.clone());
                jcall!(velocityProperties).limitXYToForwardAndReverseMaxVelocity();
                self.gravity();
            } else {
            }
        }
    }
    

    // throws Exception
    pub fn getPosition(&mut self, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>>, mut layer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut x: i32, mut y: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> {
        let mut xCellPosition: i32 = jcall!(layer).getXP() + -x + jcall!(layer).getWidth();
        let mut yCellPosition: i32 = jcall!(layer).getYP() + -y + jcall!(layer).getHeight();
        return jcall!(geographicMapInterfaceArray[(0) as usize]).getCellPositionAtXYNoThrow(xCellPosition.clone(), yCellPosition.clone());
    }
    

    // throws Exception
    pub fn getLeftPosition(&mut self, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>>, mut layer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> {
        let mut xCellPosition: i32 = jcall!(layer).getXP();
        let mut yCellPosition: i32 = jcall!(layer).getYP() + jcall!(layer).getHeight();
        return jcall!(geographicMapInterfaceArray[(0) as usize]).getCellPositionAtXYNoThrow(xCellPosition.clone(), yCellPosition.clone());
    }
    

    // throws Exception
    pub fn getRightPosition(&mut self, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>>, mut layer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> {
        let mut xCellPosition: i32 = jcall!(layer).getXP() + jcall!(layer).getWidth();
        let mut yCellPosition: i32 = jcall!(layer).getYP() + jcall!(layer).getHeight();
        return jcall!(geographicMapInterfaceArray[(0) as usize]).getCellPositionAtXYNoThrow(xCellPosition.clone(), yCellPosition.clone());
    }
    

    // throws Exception
    pub fn getGeographicMapCellPositionIfNotSolidBlockOrOffMapLocation(&mut self, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>>, mut geographicMapCellTypeArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellTypeDyn>>>>, mut velocityProperties: Option<Rc<RefCell<VelocityProperties>>>, mut layer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut x: i32, mut y: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> {
        let mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = self.getPosition(geographicMapInterfaceArray.clone(), layer.clone(), x.clone(), y.clone());
        if !match (&self.previousGeographicMapCellPosition, &geographicMapCellPosition) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } && !match (&geographicMapCellPosition, &crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            let mut possibleStepGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = geographicMapCellPosition.clone();
            let mut tiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>> = jcall!(geographicMapInterfaceArray[(0) as usize]).getAllBinaryTiledLayer();
            if jcall!(possibleStepGeographicMapCellPosition).getColumn() > 0 && jcall!(possibleStepGeographicMapCellPosition).getRow() > 0 && jcall!(possibleStepGeographicMapCellPosition).getColumn() < jcall!(tiledLayer).getColumns() && jcall!(possibleStepGeographicMapCellPosition).getRow() < jcall!(tiledLayer).getRows() {
                jcall!(self.geographicMapBehavior).getCellTypeAt(geographicMapInterfaceArray.clone(), geographicMapCellTypeArray.clone(), possibleStepGeographicMapCellPosition.clone());
                let mut hasSolidBlock: bool = self.hasSolidBlock(geographicMapInterfaceArray.clone(), geographicMapCellTypeArray.clone());
                if hasSolidBlock {
                    jcall!(jcall!(velocityProperties).getVelocityXBasicDecimalP()).setint(0);
                    self.previousGeographicMapCellPosition = { let java_value = (crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.with(Clone::clone)).clone(); java_value }.clone();
                    return crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.with(Clone::clone).clone();
                } else {
                    self.previousGeographicMapCellPosition = possibleStepGeographicMapCellPosition.clone();
                    return possibleStepGeographicMapCellPosition.clone();
                }
            }
        }
        self.previousGeographicMapCellPosition = { let java_value = (crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.with(Clone::clone)).clone(); java_value }.clone();
        return crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.with(Clone::clone).clone();
    }
    

    // throws Exception
    pub fn moveAndLand(&mut self, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>>, mut geographicMapCellTypeArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellTypeDyn>>>>, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>, mut velocityProperties: Option<Rc<RefCell<VelocityProperties>>>, mut layer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut x: i32, mut y: i32) {
        if !match (&geographicMapCellPosition, &crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            let mut topViewCharacterInterface: Option<Rc<RefCell<dyn TopViewCharacterInterface>>> = ((|| -> Option<Rc<RefCell<dyn TopViewCharacterInterface>>> { panic!("ClassCastException: a Rust dyn AllBinaryLayer cannot be cast to TopViewCharacterInterface") })());
            jcall!(topViewCharacterInterface).terrainMove(geographicMapInterfaceArray.clone(), geographicMapCellTypeArray.clone(), x.clone(), y.clone());
        } else {
        }
    }
    

    // throws Exception
    pub fn r#move(&mut self, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>>, mut geographicMapCellTypeArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellTypeDyn>>>>, mut velocityProperties: Option<Rc<RefCell<VelocityProperties>>>, mut layer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut x: i32, mut y: i32) -> bool {
        let mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = self.getGeographicMapCellPositionIfNotSolidBlockOrOffMapLocation(geographicMapInterfaceArray.clone(), geographicMapCellTypeArray.clone(), velocityProperties.clone(), layer.clone(), x.clone(), y.clone());
        self.moveAndLand(geographicMapInterfaceArray.clone(), geographicMapCellTypeArray.clone(), geographicMapCellPosition.clone(), velocityProperties.clone(), layer.clone(), x.clone(), y.clone());
        if match (&geographicMapCellPosition, &crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return false;
        } else {
            return true;
        }
    }
    

    // throws Exception
    pub fn left(&mut self, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>>, mut geographicMapCellTypeArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellTypeDyn>>>>, mut velocityProperties: Option<Rc<RefCell<VelocityProperties>>>, mut layer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) {
        let mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = self.getLeftPosition(geographicMapInterfaceArray.clone(), layer.clone());
        if !match (&geographicMapCellPosition, &crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            let mut possibleStepGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = jcall!(jcall!(geographicMapInterfaceArray[(0) as usize]).getGeographicMapCellPositionFactory()).getAt(jcall!(geographicMapCellPosition).getColumn(), jcall!(geographicMapCellPosition).getRow() - 1);
            jcall!(self.geographicMapBehavior).getCellTypeAt(geographicMapInterfaceArray.clone(), geographicMapCellTypeArray.clone(), possibleStepGeographicMapCellPosition.clone());
            let mut hasSolidBlock: bool = self.hasSolidBlock(geographicMapInterfaceArray.clone(), geographicMapCellTypeArray.clone());
            if hasSolidBlock {
                if self.autoStepBlocks {
                    let mut topViewCharacterInterface: Option<Rc<RefCell<dyn TopViewCharacterInterface>>> = ((|| -> Option<Rc<RefCell<dyn TopViewCharacterInterface>>> { panic!("ClassCastException: a Rust dyn AllBinaryLayer cannot be cast to TopViewCharacterInterface") })());
                    jcall!(topViewCharacterInterface).leftp();
                } else {
                    jcall!(jcall!(velocityProperties).getVelocityXBasicDecimalP()).setint(0);
                }
            } else {
                let mut topViewCharacterInterface: Option<Rc<RefCell<dyn TopViewCharacterInterface>>> = ((|| -> Option<Rc<RefCell<dyn TopViewCharacterInterface>>> { panic!("ClassCastException: a Rust dyn AllBinaryLayer cannot be cast to TopViewCharacterInterface") })());
                jcall!(topViewCharacterInterface).leftp();
            }
        }
    }
    

    // throws Exception
    pub fn right(&mut self, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>>, mut geographicMapCellTypeArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellTypeDyn>>>>, mut velocityProperties: Option<Rc<RefCell<VelocityProperties>>>, mut layer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) {
        let mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = self.getRightPosition(geographicMapInterfaceArray.clone(), layer.clone());
        if !match (&geographicMapCellPosition, &crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            let mut possibleStepGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = jcall!(jcall!(geographicMapInterfaceArray[(0) as usize]).getGeographicMapCellPositionFactory()).getAt(jcall!(geographicMapCellPosition).getColumn(), jcall!(geographicMapCellPosition).getRow() - 1);
            jcall!(self.geographicMapBehavior).getCellTypeAt(geographicMapInterfaceArray.clone(), geographicMapCellTypeArray.clone(), possibleStepGeographicMapCellPosition.clone());
            let mut hasSolidBlock: bool = self.hasSolidBlock(geographicMapInterfaceArray.clone(), geographicMapCellTypeArray.clone());
            if hasSolidBlock {
                if self.autoStepBlocks {
                    let mut topViewCharacterInterface: Option<Rc<RefCell<dyn TopViewCharacterInterface>>> = ((|| -> Option<Rc<RefCell<dyn TopViewCharacterInterface>>> { panic!("ClassCastException: a Rust dyn AllBinaryLayer cannot be cast to TopViewCharacterInterface") })());
                    jcall!(topViewCharacterInterface).rightp();
                } else {
                    jcall!(jcall!(velocityProperties).getVelocityXBasicDecimalP()).setint(0);
                }
            } else {
                let mut topViewCharacterInterface: Option<Rc<RefCell<dyn TopViewCharacterInterface>>> = ((|| -> Option<Rc<RefCell<dyn TopViewCharacterInterface>>> { panic!("ClassCastException: a Rust dyn AllBinaryLayer cannot be cast to TopViewCharacterInterface") })());
                jcall!(topViewCharacterInterface).rightp();
            }
        }
    }
}
impl std::ops::Deref for GeographicMapTopViewGameLayerBehavior1 {
    type Target = GeographicMapTopViewLayerBehavior;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for GeographicMapTopViewGameLayerBehavior1 {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::behavior::topview::GeographicMapTopViewLayerBehavior::GeographicMapTopViewLayerBehaviorDyn for GeographicMapTopViewGameLayerBehavior1 {
    fn as_GeographicMapTopViewLayerBehavior(&self) -> &crate::org::allbinary::game::behavior::topview::GeographicMapTopViewLayerBehavior::GeographicMapTopViewLayerBehavior { &self.base }
    fn as_GeographicMapTopViewLayerBehavior_mut(&mut self) -> &mut crate::org::allbinary::game::behavior::topview::GeographicMapTopViewLayerBehavior::GeographicMapTopViewLayerBehavior { &mut self.base }
    fn getGeographicMapCellPositionIfNotSolidBlockOrOffMapLocation(&mut self, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>>, mut geographicMapCellTypeArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellTypeDyn>>>>, mut velocityProperties: Option<Rc<RefCell<VelocityProperties>>>, mut layer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut x: i32, mut y: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> { Self::getGeographicMapCellPositionIfNotSolidBlockOrOffMapLocation(self, geographicMapInterfaceArray, geographicMapCellTypeArray, velocityProperties, layer, x, y) }
    fn gravity(&mut self, mut velocityProperties: Option<Rc<RefCell<VelocityProperties>>>, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>>, mut geographicMapCellTypeArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellTypeDyn>>>>, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) { Self::gravity(self, velocityProperties, geographicMapInterfaceArray, geographicMapCellTypeArray, geographicMapCellPosition) }
    fn left(&mut self, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>>, mut geographicMapCellTypeArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellTypeDyn>>>>, mut velocityProperties: Option<Rc<RefCell<VelocityProperties>>>, mut layer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) { Self::left(self, geographicMapInterfaceArray, geographicMapCellTypeArray, velocityProperties, layer) }
    fn r#move(&mut self, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>>, mut geographicMapCellTypeArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellTypeDyn>>>>, mut velocityProperties: Option<Rc<RefCell<VelocityProperties>>>, mut layer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut x: i32, mut y: i32) -> bool { Self::r#move(self, geographicMapInterfaceArray, geographicMapCellTypeArray, velocityProperties, layer, x, y) }
    fn moveAndLand(&mut self, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>>, mut geographicMapCellTypeArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellTypeDyn>>>>, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>, mut velocityProperties: Option<Rc<RefCell<VelocityProperties>>>, mut layer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut x: i32, mut y: i32) { Self::moveAndLand(self, geographicMapInterfaceArray, geographicMapCellTypeArray, geographicMapCellPosition, velocityProperties, layer, x, y) }
    fn right(&mut self, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>>, mut geographicMapCellTypeArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellTypeDyn>>>>, mut velocityProperties: Option<Rc<RefCell<VelocityProperties>>>, mut layer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) { Self::right(self, geographicMapInterfaceArray, geographicMapCellTypeArray, velocityProperties, layer) }
}
impl crate::org::allbinary::game::behavior::topview::TopViewGameLayerBehavior::TopViewGameLayerBehaviorDyn for GeographicMapTopViewGameLayerBehavior1 {
    fn as_TopViewGameLayerBehavior(&self) -> &crate::org::allbinary::game::behavior::topview::TopViewGameLayerBehavior::TopViewGameLayerBehavior { &self.base.base }
    fn as_TopViewGameLayerBehavior_mut(&mut self) -> &mut crate::org::allbinary::game::behavior::topview::TopViewGameLayerBehavior::TopViewGameLayerBehavior { &mut self.base.base }
}
impl crate::org::allbinary::game::layer::behavior::GameLayerBehavior::GameLayerBehaviorDyn for GeographicMapTopViewGameLayerBehavior1 {
    fn as_GameLayerBehavior(&self) -> &crate::org::allbinary::game::layer::behavior::GameLayerBehavior::GameLayerBehavior { &self.base.base.base }
    fn as_GameLayerBehavior_mut(&mut self) -> &mut crate::org::allbinary::game::layer::behavior::GameLayerBehavior::GameLayerBehavior { &mut self.base.base.base }
}


