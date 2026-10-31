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
// package org.allbinary.media.graphics.geography.map.racetrack.drop
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::layer::pickup::PickedUpLayerInterfaceFactoryInterface::PickedUpLayerInterfaceFactoryInterface;
use crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManager;
use crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayer;
use crate::org::allbinary::game::layer::drop::DropLayerFactory::DropLayerFactory;
use crate::org::allbinary::game::layer::pickup::PickupLayerCircularStaticPool::PickupLayerCircularStaticPool;
use crate::org::allbinary::game::layer::pickup::RaceTrackPickupLayer::RaceTrackPickupLayer;
use crate::org::allbinary::game::rand::MyRandomFactory::MyRandomFactory;
use crate::org::allbinary::graphics::GPoint::GPoint;
use crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager;
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMap;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPosition;
use super::RaceTrackDropCellPositionGenerator::RaceTrackDropCellPositionGenerator;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RaceTrackRandomDropCellPositionGenerator {
    pub base: RaceTrackDropCellPositionGenerator,
    pub myRandomFactory: Option<Rc<RefCell<MyRandomFactory>>>,
    pub cellsPerRowOrColumn: i32,
    pub totalCells: i32,
    pub rowArray: Vec<i32>,
    pub columnArray: Vec<i32>,
    pub cellWidth: i32,
    pub cellHeight: i32,
}

pub static mut RaceTrackRandomDropCellPositionGenerator_SINGLETON: std::cell::LazyCell<Option<Rc<RefCell<RaceTrackRandomDropCellPositionGenerator>>>> = std::cell::LazyCell::new(|| RaceTrackRandomDropCellPositionGenerator::new());

impl RaceTrackRandomDropCellPositionGenerator {
    

    pub fn getInstance() -> Option<Rc<RefCell<RaceTrackRandomDropCellPositionGenerator>>> {
        return unsafe { RaceTrackRandomDropCellPositionGenerator_SINGLETON.clone() }.clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: RaceTrackDropCellPositionGenerator::new_struct(),
            myRandomFactory: None,
            cellsPerRowOrColumn: 0,
            totalCells: 0,
            rowArray: Vec::new(),
            columnArray: Vec::new(),
            cellWidth: 0,
            cellHeight: 0,
        };
        this.myRandomFactory = MyRandomFactory::getInstance();
        this.cellsPerRowOrColumn = 3;
        this.totalCells = this.cellsPerRowOrColumn * this.cellsPerRowOrColumn;
        this.rowArray = vec![0, 1, 2, 0, 1, 2, 0, 1, 2];
        this.columnArray = vec![0, 0, 0, 1, 1, 1, 2, 2, 2];
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn update(&mut self, mut allBinaryGameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>) {
        self.base.update(allBinaryGameLayerManager.clone(), geographicMapInterface.clone());
        let mut tiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>> = jcall!(geographicMapInterface).getAllBinaryTiledLayer();
        self.cellWidth = jcall!(tiledLayer).getCellWidth() / self.cellsPerRowOrColumn;
        self.cellHeight = jcall!(tiledLayer).getCellHeight() / self.cellsPerRowOrColumn;
    }
    

    // throws Exception
    pub fn drop(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut index: i32) {
        let mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = { let java_object = (jcall!(self.list).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>>().expect("ClassCastException").clone()) };
        let mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = jcall!(geographicMapCellPosition).getPoint();
        let mut randomCell: i32 = jcall!(self.myRandomFactory).getAbsoluteNextInt(self.totalCells.clone());
        let mut row: i32 = self.rowArray[(randomCell) as usize].clone();
        let mut column: i32 = self.columnArray[(randomCell) as usize].clone();
        let mut x: i32 = jcall!(point).getX() + (row * self.cellWidth);
        let mut y: i32 = jcall!(point).getY() + (column * self.cellHeight);
        let mut pickedUpLayerInterfaceFactory: Option<Rc<RefCell<dyn PickedUpLayerInterfaceFactoryInterface>>> = jcall!(DropLayerFactory::getInstance()).getRandomInstance();
        let mut tiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>> = jcall!(self.raceTrackGeographicMap).getAllBinaryTiledLayer();
        let mut pickupLayer: Option<Rc<RefCell<RaceTrackPickupLayer>>> = (jcall!(PickupLayerCircularStaticPool::getInstance()).getInstanceXYZ(pickedUpLayerInterfaceFactory.clone(), x.clone(), y.clone(), jcall!(tiledLayer).getZP() + 3)).clone() /* as RaceTrackPickupLayer */;
        jcall!(pickupLayer).setTiledLayer(tiledLayer.clone());
        jcall!(allBinaryLayerManager).append(pickupLayer.clone());
    }
}
impl std::ops::Deref for RaceTrackRandomDropCellPositionGenerator {
    type Target = RaceTrackDropCellPositionGenerator;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for RaceTrackRandomDropCellPositionGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::media::graphics::geography::map::racetrack::drop::RaceTrackDropCellPositionGenerator::RaceTrackDropCellPositionGeneratorDyn for RaceTrackRandomDropCellPositionGenerator {
    fn as_RaceTrackDropCellPositionGenerator(&self) -> &crate::org::allbinary::media::graphics::geography::map::racetrack::drop::RaceTrackDropCellPositionGenerator::RaceTrackDropCellPositionGenerator { &self.base }
    fn as_RaceTrackDropCellPositionGenerator_mut(&mut self) -> &mut crate::org::allbinary::media::graphics::geography::map::racetrack::drop::RaceTrackDropCellPositionGenerator::RaceTrackDropCellPositionGenerator { &mut self.base }
    fn drop(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut index: i32) { Self::drop(self, allBinaryLayerManager, index) }
}
impl crate::org::allbinary::media::graphics::geography::map::drop::BaseDropCellPositionGenerator::BaseDropCellPositionGeneratorDyn for RaceTrackRandomDropCellPositionGenerator {
    fn as_BaseDropCellPositionGenerator(&self) -> &crate::org::allbinary::media::graphics::geography::map::drop::BaseDropCellPositionGenerator::BaseDropCellPositionGenerator { &self.base.base }
    fn as_BaseDropCellPositionGenerator_mut(&mut self) -> &mut crate::org::allbinary::media::graphics::geography::map::drop::BaseDropCellPositionGenerator::BaseDropCellPositionGenerator { &mut self.base.base }
}
impl crate::org::allbinary::media::graphics::geography::map::drop::DropCellPositionGeneratorInterface::DropCellPositionGeneratorInterface for RaceTrackRandomDropCellPositionGenerator {
    fn getName(&mut self) -> String { crate::org::allbinary::media::graphics::geography::map::drop::BaseDropCellPositionGenerator::BaseDropCellPositionGenerator::getName(crate::org::allbinary::media::graphics::geography::map::drop::BaseDropCellPositionGenerator::BaseDropCellPositionGeneratorDyn::as_BaseDropCellPositionGenerator_mut(self)) }
    fn update(&mut self, mut allBinaryGameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>) { Self::update(self, allBinaryGameLayerManager, geographicMapInterface) }
    fn processTick(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { crate::org::allbinary::media::graphics::geography::map::racetrack::drop::RaceTrackDropCellPositionGenerator::RaceTrackDropCellPositionGenerator::processTick(crate::org::allbinary::media::graphics::geography::map::racetrack::drop::RaceTrackDropCellPositionGenerator::RaceTrackDropCellPositionGeneratorDyn::as_RaceTrackDropCellPositionGenerator_mut(self), p1) }
    fn isDropAllowedAt(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> bool { crate::org::allbinary::media::graphics::geography::map::racetrack::drop::RaceTrackDropCellPositionGenerator::RaceTrackDropCellPositionGenerator::isDropAllowedAt(crate::org::allbinary::media::graphics::geography::map::racetrack::drop::RaceTrackDropCellPositionGenerator::RaceTrackDropCellPositionGeneratorDyn::as_RaceTrackDropCellPositionGenerator_mut(self), p1) }
}


