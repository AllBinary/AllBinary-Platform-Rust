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
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMap;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPosition;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellType;
use crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory;
use super::GeographicMapTopViewLayerBehavior::GeographicMapTopViewLayerBehavior;
use super::TopViewCharacterInterface::TopViewCharacterInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GeographicMapTopViewGameLayerBehavior2 {
    pub base: GeographicMapTopViewLayerBehavior,
    pub gravityUtil: Option<Rc<RefCell<GravityUtil>>>,
    pub geographicMapBehavior: Option<Rc<RefCell<MultiGeographicMapBehavior>>>,
    pub autoStepBlocks: bool,
    pub offsetY: i32,
    pub previousGeographicMapCellPosition: Option<Rc<RefCell<GeographicMapCellPosition>>>,
}


impl GeographicMapTopViewGameLayerBehavior2 {
    

    pub fn new_struct(mut maxGravityActionIndex: i32, mut autoStepBlocks: bool, mut offsetY: i32) -> Self {
        let mut this = Self {
            base: GeographicMapTopViewLayerBehavior::new_struct(maxGravityActionIndex.clone()),
            gravityUtil: None,
            geographicMapBehavior: None,
            autoStepBlocks: false,
            offsetY: 0,
            previousGeographicMapCellPosition: None,
        };
        this.gravityUtil = GravityUtil::getInstance();
        this.geographicMapBehavior = MultiGeographicMapBehavior::new();
        this.previousGeographicMapCellPosition = unsafe { crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.clone() }.clone();
        // super(...) is in the struct literal above
        this.autoStepBlocks = autoStepBlocks.clone();
        this.offsetY = offsetY.clone();
        this
    }

    pub fn new(mut maxGravityActionIndex: i32, mut autoStepBlocks: bool, mut offsetY: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(maxGravityActionIndex, autoStepBlocks, offsetY))))
    }
    

    // throws Exception
    pub fn gravity(&mut self, mut velocityProperties: Option<Rc<RefCell<VelocityProperties>>>, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<BasicGeographicMap>>>>, mut geographicMapCellTypeArray: Vec<Option<Rc<RefCell<GeographicMapCellType>>>>, mut geographicMapCellPosition: Option<Rc<RefCell<GeographicMapCellPosition>>>) {
        if !match (&geographicMapCellPosition, &unsafe { crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.clone() }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            jcall!(self.geographicMapBehavior).getCellTypeAt(geographicMapInterfaceArray.clone(), geographicMapCellTypeArray.clone(), geographicMapCellPosition.clone());
            let mut hasSolidBlock: bool = self.hasSolidBlock(geographicMapInterfaceArray.clone(), geographicMapCellTypeArray.clone());
            if !hasSolidBlock {
                jcall!(self.gravityUtil).process(velocityProperties.clone(), jcall!(self.gravityUtil).GAME_GRAVITY_VELOCITY.clone());
                jcall!(velocityProperties).limitXYToForwardAndReverseMaxVelocity();
                self.gravity();
            } else {
            }
        }
    }
    

    // throws Exception
    pub fn getPositionTopLeft(&mut self, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<BasicGeographicMap>>>>, mut layer: Option<Rc<RefCell<AllBinaryLayer>>>, mut x: i32, mut y: i32) -> Option<Rc<RefCell<GeographicMapCellPosition>>> {
        let mut xCellPosition: i32 = jcall!(layer).getXP() + -x;
        let mut yCellPosition: i32 = jcall!(layer).getYP() + -y;
        return jcall!(geographicMapInterfaceArray[(0) as usize]).getCellPositionAtXYNoThrow(xCellPosition.clone(), yCellPosition.clone());
    }
    

    // throws Exception
    pub fn getPositionTopRight(&mut self, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<BasicGeographicMap>>>>, mut layer: Option<Rc<RefCell<AllBinaryLayer>>>, mut x: i32, mut y: i32) -> Option<Rc<RefCell<GeographicMapCellPosition>>> {
        let mut xCellPosition: i32 = jcall!(layer).getXP() + -x + jcall!(layer).getWidth();
        let mut yCellPosition: i32 = jcall!(layer).getYP() + -y;
        return jcall!(geographicMapInterfaceArray[(0) as usize]).getCellPositionAtXYNoThrow(xCellPosition.clone(), yCellPosition.clone());
    }
    

    // throws Exception
    pub fn getPositionBottomLeft(&mut self, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<BasicGeographicMap>>>>, mut layer: Option<Rc<RefCell<AllBinaryLayer>>>, mut x: i32, mut y: i32) -> Option<Rc<RefCell<GeographicMapCellPosition>>> {
        let mut xCellPosition: i32 = jcall!(layer).getXP() + -x;
        let mut yCellPosition: i32 = jcall!(layer).getYP() + -y + jcall!(layer).getHeight();
        return jcall!(geographicMapInterfaceArray[(0) as usize]).getCellPositionAtXYNoThrow(xCellPosition.clone(), yCellPosition.clone());
    }
    

    // throws Exception
    pub fn getPositionBottomRight(&mut self, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<BasicGeographicMap>>>>, mut layer: Option<Rc<RefCell<AllBinaryLayer>>>, mut x: i32, mut y: i32) -> Option<Rc<RefCell<GeographicMapCellPosition>>> {
        let mut xCellPosition: i32 = jcall!(layer).getXP() + -x + jcall!(layer).getWidth();
        let mut yCellPosition: i32 = jcall!(layer).getYP() + -y + jcall!(layer).getHeight();
        return jcall!(geographicMapInterfaceArray[(0) as usize]).getCellPositionAtXYNoThrow(xCellPosition.clone(), yCellPosition.clone());
    }
    

    // throws Exception
    pub fn getLeftPosition(&mut self, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<BasicGeographicMap>>>>, mut layer: Option<Rc<RefCell<AllBinaryLayer>>>) -> Option<Rc<RefCell<GeographicMapCellPosition>>> {
        let mut xCellPosition: i32 = jcall!(layer).getXP();
        let mut yCellPosition: i32 = jcall!(layer).getYP() + jcall!(layer).getHeight();
        return jcall!(geographicMapInterfaceArray[(0) as usize]).getCellPositionAtXYNoThrow(xCellPosition.clone(), yCellPosition.clone());
    }
    

    // throws Exception
    pub fn getRightPosition(&mut self, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<BasicGeographicMap>>>>, mut layer: Option<Rc<RefCell<AllBinaryLayer>>>) -> Option<Rc<RefCell<GeographicMapCellPosition>>> {
        let mut xCellPosition: i32 = jcall!(layer).getXP() + jcall!(layer).getWidth();
        let mut yCellPosition: i32 = jcall!(layer).getYP() + jcall!(layer).getHeight();
        return jcall!(geographicMapInterfaceArray[(0) as usize]).getCellPositionAtXYNoThrow(xCellPosition.clone(), yCellPosition.clone());
    }
    

    // throws Exception
    pub fn getGeographicMapCellPositionIfNotSolidBlockOrOffMapLocation(&mut self, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<BasicGeographicMap>>>>, mut geographicMapCellTypeArray: Vec<Option<Rc<RefCell<GeographicMapCellType>>>>, mut velocityProperties: Option<Rc<RefCell<VelocityProperties>>>, mut layer: Option<Rc<RefCell<AllBinaryLayer>>>, mut x: i32, mut y: i32) -> Option<Rc<RefCell<GeographicMapCellPosition>>> {
        let mut geographicMapCellPosition: Option<Rc<RefCell<GeographicMapCellPosition>>> = unsafe { crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.clone() }.clone();
        let mut topLeftGeographicMapCellPosition: Option<Rc<RefCell<GeographicMapCellPosition>>> = self.getPositionTopLeft(geographicMapInterfaceArray.clone(), layer.clone(), x.clone(), y.clone());
        geographicMapCellPosition = self.getGeographicMapCellPositionIfNotSolidBlockOrOffMap(geographicMapInterfaceArray.clone(), geographicMapCellTypeArray.clone(), topLeftGeographicMapCellPosition.clone(), velocityProperties.clone(), layer.clone());
        if match (&geographicMapCellPosition, &unsafe { crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.clone() }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return unsafe { crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.clone() }.clone();
        }
        let mut topRightGeographicMapCellPosition: Option<Rc<RefCell<GeographicMapCellPosition>>> = self.getPositionTopRight(geographicMapInterfaceArray.clone(), layer.clone(), x.clone(), y.clone());
        geographicMapCellPosition = self.getGeographicMapCellPositionIfNotSolidBlockOrOffMap(geographicMapInterfaceArray.clone(), geographicMapCellTypeArray.clone(), topRightGeographicMapCellPosition.clone(), velocityProperties.clone(), layer.clone());
        if match (&geographicMapCellPosition, &unsafe { crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.clone() }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return unsafe { crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.clone() }.clone();
        }
        let mut bottomLeftGeographicMapCellPosition: Option<Rc<RefCell<GeographicMapCellPosition>>> = self.getPositionBottomLeft(geographicMapInterfaceArray.clone(), layer.clone(), x.clone(), y.clone());
        geographicMapCellPosition = self.getGeographicMapCellPositionIfNotSolidBlockOrOffMap(geographicMapInterfaceArray.clone(), geographicMapCellTypeArray.clone(), bottomLeftGeographicMapCellPosition.clone(), velocityProperties.clone(), layer.clone());
        if match (&geographicMapCellPosition, &unsafe { crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.clone() }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return unsafe { crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.clone() }.clone();
        }
        let mut bottomRightGeographicMapCellPosition: Option<Rc<RefCell<GeographicMapCellPosition>>> = self.getPositionBottomRight(geographicMapInterfaceArray.clone(), layer.clone(), x.clone(), y.clone());
        geographicMapCellPosition = self.getGeographicMapCellPositionIfNotSolidBlockOrOffMap(geographicMapInterfaceArray.clone(), geographicMapCellTypeArray.clone(), bottomRightGeographicMapCellPosition.clone(), velocityProperties.clone(), layer.clone());
        if match (&geographicMapCellPosition, &unsafe { crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.clone() }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
        } else {
        }
        return geographicMapCellPosition.clone();
    }
    

    // throws Exception
    pub fn getGeographicMapCellPositionIfNotSolidBlockOrOffMap(&mut self, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<BasicGeographicMap>>>>, mut geographicMapCellTypeArray: Vec<Option<Rc<RefCell<GeographicMapCellType>>>>, mut geographicMapCellPosition: Option<Rc<RefCell<GeographicMapCellPosition>>>, mut velocityProperties: Option<Rc<RefCell<VelocityProperties>>>, mut layer: Option<Rc<RefCell<AllBinaryLayer>>>) -> Option<Rc<RefCell<GeographicMapCellPosition>>> {
        if !match (&geographicMapCellPosition, &unsafe { crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.clone() }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            let mut possibleStepGeographicMapCellPosition: Option<Rc<RefCell<GeographicMapCellPosition>>> = geographicMapCellPosition.clone();
            let mut tiledLayer: Option<Rc<RefCell<AllBinaryTiledLayer>>> = jcall!(geographicMapInterfaceArray[(0) as usize]).getAllBinaryTiledLayer();
            if jcall!(possibleStepGeographicMapCellPosition).getColumn() > 0 && jcall!(possibleStepGeographicMapCellPosition).getRow() > 0 && jcall!(possibleStepGeographicMapCellPosition).getColumn() < jcall!(tiledLayer).getColumns() && jcall!(possibleStepGeographicMapCellPosition).getRow() < jcall!(tiledLayer).getRows() {
                jcall!(self.geographicMapBehavior).getCellTypeAt(geographicMapInterfaceArray.clone(), geographicMapCellTypeArray.clone(), possibleStepGeographicMapCellPosition.clone());
                let mut hasSolidBlock: bool = self.hasSolidBlock(geographicMapInterfaceArray.clone(), geographicMapCellTypeArray.clone());
                let mut hasOffMap: bool = self.isOffMap(geographicMapInterfaceArray.clone(), geographicMapCellTypeArray.clone());
                if hasSolidBlock || hasOffMap {
                    self.previousGeographicMapCellPosition = unsafe { crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.clone() }.clone();
                    return unsafe { crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.clone() }.clone();
                } else {
                    self.previousGeographicMapCellPosition = possibleStepGeographicMapCellPosition.clone();
                    return possibleStepGeographicMapCellPosition.clone();
                }
            }
        } else {
        }
        self.previousGeographicMapCellPosition = unsafe { crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.clone() }.clone();
        return unsafe { crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.clone() }.clone();
    }
    

    // throws Exception
    pub fn moveAndLand(&mut self, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<BasicGeographicMap>>>>, mut geographicMapCellTypeArray: Vec<Option<Rc<RefCell<GeographicMapCellType>>>>, mut geographicMapCellPosition: Option<Rc<RefCell<GeographicMapCellPosition>>>, mut velocityProperties: Option<Rc<RefCell<VelocityProperties>>>, mut layer: Option<Rc<RefCell<AllBinaryLayer>>>, mut x: i32, mut y: i32) {
        if !match (&geographicMapCellPosition, &unsafe { crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.clone() }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            let mut topViewCharacterInterface: Option<Rc<RefCell<dyn TopViewCharacterInterface>>> = ((layer).clone() /* as TopViewCharacterInterface */);
            jcall!(topViewCharacterInterface).terrainMove(geographicMapInterfaceArray.clone(), geographicMapCellTypeArray.clone(), x.clone(), y.clone());
        } else {
        }
    }
    

    // throws Exception
    pub fn r#move(&mut self, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<BasicGeographicMap>>>>, mut geographicMapCellTypeArray: Vec<Option<Rc<RefCell<GeographicMapCellType>>>>, mut velocityProperties: Option<Rc<RefCell<VelocityProperties>>>, mut layer: Option<Rc<RefCell<AllBinaryLayer>>>, mut x: i32, mut y: i32) -> bool {
        let mut geographicMapCellPosition: Option<Rc<RefCell<GeographicMapCellPosition>>> = self.getGeographicMapCellPositionIfNotSolidBlockOrOffMapLocation(geographicMapInterfaceArray.clone(), geographicMapCellTypeArray.clone(), velocityProperties.clone(), layer.clone(), x.clone(), y.clone());
        self.moveAndLand(geographicMapInterfaceArray.clone(), geographicMapCellTypeArray.clone(), geographicMapCellPosition.clone(), velocityProperties.clone(), layer.clone(), x.clone(), y.clone());
        if match (&geographicMapCellPosition, &unsafe { crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.clone() }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return false;
        } else {
            return true;
        }
    }
    

    // throws Exception
    pub fn left(&mut self, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<BasicGeographicMap>>>>, mut geographicMapCellTypeArray: Vec<Option<Rc<RefCell<GeographicMapCellType>>>>, mut velocityProperties: Option<Rc<RefCell<VelocityProperties>>>, mut layer: Option<Rc<RefCell<AllBinaryLayer>>>) {
        let mut geographicMapCellPosition: Option<Rc<RefCell<GeographicMapCellPosition>>> = self.getLeftPosition(geographicMapInterfaceArray.clone(), layer.clone());
        if !match (&geographicMapCellPosition, &unsafe { crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.clone() }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            let mut possibleStepGeographicMapCellPosition: Option<Rc<RefCell<GeographicMapCellPosition>>> = jcall!(jcall!(geographicMapInterfaceArray[(0) as usize]).getGeographicMapCellPositionFactory()).getAt(jcall!(geographicMapCellPosition).getColumn(), jcall!(geographicMapCellPosition).getRow() - 1);
            jcall!(self.geographicMapBehavior).getCellTypeAt(geographicMapInterfaceArray.clone(), geographicMapCellTypeArray.clone(), possibleStepGeographicMapCellPosition.clone());
            let mut hasSolidBlock: bool = self.hasSolidBlock(geographicMapInterfaceArray.clone(), geographicMapCellTypeArray.clone());
            if hasSolidBlock {
                if self.autoStepBlocks {
                    let mut topViewCharacterInterface: Option<Rc<RefCell<dyn TopViewCharacterInterface>>> = ((layer).clone() /* as TopViewCharacterInterface */);
                    jcall!(topViewCharacterInterface).leftp();
                } else {
                    jcall!(jcall!(velocityProperties).getVelocityXBasicDecimalP()).setint(0);
                }
            } else {
                let mut topViewCharacterInterface: Option<Rc<RefCell<dyn TopViewCharacterInterface>>> = ((layer).clone() /* as TopViewCharacterInterface */);
                jcall!(topViewCharacterInterface).leftp();
            }
        }
    }
    

    // throws Exception
    pub fn right(&mut self, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<BasicGeographicMap>>>>, mut geographicMapCellTypeArray: Vec<Option<Rc<RefCell<GeographicMapCellType>>>>, mut velocityProperties: Option<Rc<RefCell<VelocityProperties>>>, mut layer: Option<Rc<RefCell<AllBinaryLayer>>>) {
        let mut geographicMapCellPosition: Option<Rc<RefCell<GeographicMapCellPosition>>> = self.getRightPosition(geographicMapInterfaceArray.clone(), layer.clone());
        if !match (&geographicMapCellPosition, &unsafe { crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.clone() }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            let mut possibleStepGeographicMapCellPosition: Option<Rc<RefCell<GeographicMapCellPosition>>> = jcall!(jcall!(geographicMapInterfaceArray[(0) as usize]).getGeographicMapCellPositionFactory()).getAt(jcall!(geographicMapCellPosition).getColumn(), jcall!(geographicMapCellPosition).getRow() - 1);
            jcall!(self.geographicMapBehavior).getCellTypeAt(geographicMapInterfaceArray.clone(), geographicMapCellTypeArray.clone(), possibleStepGeographicMapCellPosition.clone());
            let mut hasSolidBlock: bool = self.hasSolidBlock(geographicMapInterfaceArray.clone(), geographicMapCellTypeArray.clone());
            if hasSolidBlock {
                if self.autoStepBlocks {
                    let mut topViewCharacterInterface: Option<Rc<RefCell<dyn TopViewCharacterInterface>>> = ((layer).clone() /* as TopViewCharacterInterface */);
                    jcall!(topViewCharacterInterface).rightp();
                } else {
                    jcall!(jcall!(velocityProperties).getVelocityXBasicDecimalP()).setint(0);
                }
            } else {
                let mut topViewCharacterInterface: Option<Rc<RefCell<dyn TopViewCharacterInterface>>> = ((layer).clone() /* as TopViewCharacterInterface */);
                jcall!(topViewCharacterInterface).rightp();
            }
        }
    }
}
impl std::ops::Deref for GeographicMapTopViewGameLayerBehavior2 {
    type Target = GeographicMapTopViewLayerBehavior;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for GeographicMapTopViewGameLayerBehavior2 {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::behavior::topview::GeographicMapTopViewLayerBehavior::GeographicMapTopViewLayerBehaviorDyn for GeographicMapTopViewGameLayerBehavior2 {
    fn as_GeographicMapTopViewLayerBehavior(&self) -> &crate::org::allbinary::game::behavior::topview::GeographicMapTopViewLayerBehavior::GeographicMapTopViewLayerBehavior { &self.base }
    fn as_GeographicMapTopViewLayerBehavior_mut(&mut self) -> &mut crate::org::allbinary::game::behavior::topview::GeographicMapTopViewLayerBehavior::GeographicMapTopViewLayerBehavior { &mut self.base }
    fn getGeographicMapCellPositionIfNotSolidBlockOrOffMapLocation(&mut self, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<BasicGeographicMap>>>>, mut geographicMapCellTypeArray: Vec<Option<Rc<RefCell<GeographicMapCellType>>>>, mut velocityProperties: Option<Rc<RefCell<VelocityProperties>>>, mut layer: Option<Rc<RefCell<AllBinaryLayer>>>, mut x: i32, mut y: i32) -> Option<Rc<RefCell<GeographicMapCellPosition>>> { Self::getGeographicMapCellPositionIfNotSolidBlockOrOffMapLocation(self, geographicMapInterfaceArray, geographicMapCellTypeArray, velocityProperties, layer, x, y) }
    fn getGeographicMapCellPositionIfNotSolidBlockOrOffMap(&mut self, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<BasicGeographicMap>>>>, mut geographicMapCellTypeArray: Vec<Option<Rc<RefCell<GeographicMapCellType>>>>, mut geographicMapCellPosition: Option<Rc<RefCell<GeographicMapCellPosition>>>, mut velocityProperties: Option<Rc<RefCell<VelocityProperties>>>, mut layer: Option<Rc<RefCell<AllBinaryLayer>>>) -> Option<Rc<RefCell<GeographicMapCellPosition>>> { Self::getGeographicMapCellPositionIfNotSolidBlockOrOffMap(self, geographicMapInterfaceArray, geographicMapCellTypeArray, geographicMapCellPosition, velocityProperties, layer) }
    fn gravity(&mut self, mut velocityProperties: Option<Rc<RefCell<VelocityProperties>>>, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<BasicGeographicMap>>>>, mut geographicMapCellTypeArray: Vec<Option<Rc<RefCell<GeographicMapCellType>>>>, mut geographicMapCellPosition: Option<Rc<RefCell<GeographicMapCellPosition>>>) { Self::gravity(self, velocityProperties, geographicMapInterfaceArray, geographicMapCellTypeArray, geographicMapCellPosition) }
    fn left(&mut self, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<BasicGeographicMap>>>>, mut geographicMapCellTypeArray: Vec<Option<Rc<RefCell<GeographicMapCellType>>>>, mut velocityProperties: Option<Rc<RefCell<VelocityProperties>>>, mut layer: Option<Rc<RefCell<AllBinaryLayer>>>) { Self::left(self, geographicMapInterfaceArray, geographicMapCellTypeArray, velocityProperties, layer) }
    fn r#move(&mut self, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<BasicGeographicMap>>>>, mut geographicMapCellTypeArray: Vec<Option<Rc<RefCell<GeographicMapCellType>>>>, mut velocityProperties: Option<Rc<RefCell<VelocityProperties>>>, mut layer: Option<Rc<RefCell<AllBinaryLayer>>>, mut x: i32, mut y: i32) -> bool { Self::r#move(self, geographicMapInterfaceArray, geographicMapCellTypeArray, velocityProperties, layer, x, y) }
    fn moveAndLand(&mut self, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<BasicGeographicMap>>>>, mut geographicMapCellTypeArray: Vec<Option<Rc<RefCell<GeographicMapCellType>>>>, mut geographicMapCellPosition: Option<Rc<RefCell<GeographicMapCellPosition>>>, mut velocityProperties: Option<Rc<RefCell<VelocityProperties>>>, mut layer: Option<Rc<RefCell<AllBinaryLayer>>>, mut x: i32, mut y: i32) { Self::moveAndLand(self, geographicMapInterfaceArray, geographicMapCellTypeArray, geographicMapCellPosition, velocityProperties, layer, x, y) }
    fn right(&mut self, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<BasicGeographicMap>>>>, mut geographicMapCellTypeArray: Vec<Option<Rc<RefCell<GeographicMapCellType>>>>, mut velocityProperties: Option<Rc<RefCell<VelocityProperties>>>, mut layer: Option<Rc<RefCell<AllBinaryLayer>>>) { Self::right(self, geographicMapInterfaceArray, geographicMapCellTypeArray, velocityProperties, layer) }
}
impl crate::org::allbinary::game::behavior::topview::TopViewGameLayerBehavior::TopViewGameLayerBehaviorDyn for GeographicMapTopViewGameLayerBehavior2 {
    fn as_TopViewGameLayerBehavior(&self) -> &crate::org::allbinary::game::behavior::topview::TopViewGameLayerBehavior::TopViewGameLayerBehavior { &self.base.base }
    fn as_TopViewGameLayerBehavior_mut(&mut self) -> &mut crate::org::allbinary::game::behavior::topview::TopViewGameLayerBehavior::TopViewGameLayerBehavior { &mut self.base.base }
}


