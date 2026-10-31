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
use crate::org::allbinary::game::identification::GroupCommonFactory::GroupCommonFactory;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::direction::Direction::Direction;
use crate::org::allbinary::direction::DirectionFactory::DirectionFactory;
use crate::org::allbinary::game::identification::BasicGroupFactory::BasicGroupFactory;
use crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManager;
use crate::org::allbinary::game::layer::geographic::map::LayerCoveringCellPositionsUtil::LayerCoveringCellPositionsUtil;
use crate::org::allbinary::graphics::GPoint::GPoint;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMap;
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMapCellPositionFactory::BasicGeographicMapCellPositionFactory;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPosition;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellType;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapDirectionUtil::GeographicMapDirectionUtil;
use crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory;
use crate::org::allbinary::media::graphics::geography::map::drop::DropCellPositionGeneratorInterface::DropCellPositionGeneratorInterface;
use crate::org::allbinary::media::graphics::geography::map::drop::DropCellPositionHistory::DropCellPositionHistory;
use crate::org::allbinary::media::graphics::geography::map::racetrack::BaseRaceTrackGeographicMap::BaseRaceTrackGeographicMap;
use crate::org::allbinary::media::graphics::geography::map::racetrack::RaceTrackGeographicMapCellType::RaceTrackGeographicMapCellType;
use crate::org::allbinary::media::graphics::geography::map::racetrack::RaceTrackGeographicMapCellTypeFactory::RaceTrackGeographicMapCellTypeFactory;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use super::RaceTrackDropCellPositionGenerator::RaceTrackDropCellPositionGenerator;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RaceTrackAdjacentDropCellPositionGenerator {
    pub base: RaceTrackDropCellPositionGenerator,
    pub layerCoveringCellPositionsUtil: Option<Rc<RefCell<LayerCoveringCellPositionsUtil>>>,
    pub groupCommonFactory: Option<Rc<RefCell<GroupCommonFactory>>>,
    pub surroundingCellPositions: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>>,
    pub hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>,
    pub geographicMapDirectionUtil: Option<Rc<RefCell<GeographicMapDirectionUtil>>>,
}

pub static mut RaceTrackAdjacentDropCellPositionGenerator_SINGLETON: std::cell::LazyCell<Option<Rc<RefCell<RaceTrackAdjacentDropCellPositionGenerator>>>> = std::cell::LazyCell::new(|| RaceTrackAdjacentDropCellPositionGenerator::new());

impl RaceTrackAdjacentDropCellPositionGenerator {
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn DropCellPositionGeneratorInterface>>> {
        return unsafe { RaceTrackAdjacentDropCellPositionGenerator_SINGLETON.clone() }.clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: RaceTrackDropCellPositionGenerator::new_struct(),
            layerCoveringCellPositionsUtil: None,
            groupCommonFactory: None,
            surroundingCellPositions: Vec::new(),
            hashtable: None,
            geographicMapDirectionUtil: None,
        };
        this.layerCoveringCellPositionsUtil = LayerCoveringCellPositionsUtil::getInstance();
        this.groupCommonFactory = GroupCommonFactory::getInstance();
        this.surroundingCellPositions = vec![None; (4) as usize];
        this.hashtable = jcall!(StdUtil::getInstance()).createHashtable();
        this.geographicMapDirectionUtil = GeographicMapDirectionUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn getFirstNonRoadAdjacentCellPosition(&mut self, mut column: i32, mut row: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> {
        let mut nonRoadGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.with(Clone::clone).clone();
        let mut geographicMapCellPositionFactory: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMapCellPositionFactory::BasicGeographicMapCellPositionFactoryDyn>>> = jcall!(self.raceTrackGeographicMap).getGeographicMapCellPositionFactory();
        self.surroundingCellPositions[(0) as usize] = jcall!(geographicMapCellPositionFactory).getAt(column.clone(), row - 1);
        self.surroundingCellPositions[(1) as usize] = jcall!(geographicMapCellPositionFactory).getAt(column.clone(), row + 1);
        self.surroundingCellPositions[(2) as usize] = jcall!(geographicMapCellPositionFactory).getAt(column - 1, row.clone());
        self.surroundingCellPositions[(3) as usize] = jcall!(geographicMapCellPositionFactory).getAt(column + 1, row.clone());
        let mut baseRaceTrackGeographicMap: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::racetrack::BaseRaceTrackGeographicMap::BaseRaceTrackGeographicMapDyn>>> = (self.raceTrackGeographicMap).clone() /* as BaseRaceTrackGeographicMap */;
        let mut raceTrackGeographicMapCellTypeFactory: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::racetrack::RaceTrackGeographicMapCellTypeFactory::RaceTrackGeographicMapCellTypeFactoryDyn>>> = (jcall!(baseRaceTrackGeographicMap).getGeographicMapCellTypeFactory()).clone() /* as RaceTrackGeographicMapCellTypeFactory */;
        let mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = None;
        let mut raceTrackGeographicMapCellType: Option<Rc<RefCell<RaceTrackGeographicMapCellType>>> = None;
        {
            let mut index: i32 = (self.surroundingCellPositions.len() as i32).clone();
            while { index -= 1; index } >= 0 {
            geographicMapCellPosition = self.surroundingCellPositions[(index) as usize].clone();
            raceTrackGeographicMapCellType = (jcall!(self.raceTrackGeographicMap).getCellTypeAt(geographicMapCellPosition.clone())).clone() /* as RaceTrackGeographicMapCellType */;
            if !jcall!(raceTrackGeographicMapCellTypeFactory).isPath(raceTrackGeographicMapCellType.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellTypeDyn>>)) {
                nonRoadGeographicMapCellPosition = geographicMapCellPosition.clone();
                break;
            }
            }
        }
        return nonRoadGeographicMapCellPosition.clone();
    }
    

    // throws Exception
    pub fn update(&mut self, mut allBinaryGameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>) {
        jcall!(self.hashtable).put(crate::java_into_object(crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManager_ID.with(Clone::clone).clone()), crate::java_into_object(allBinaryGameLayerManager.clone()));
        self.base.update(allBinaryGameLayerManager.clone(), geographicMapInterface.clone());
    }
    

    // throws Exception
    pub fn drop(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut index: i32) {
        let mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = { let java_object = (jcall!(self.list).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>>().expect("ClassCastException").clone()) };
        let mut randomGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = { let java_arguments = (jcall!(geographicMapCellPosition).getColumn(), jcall!(geographicMapCellPosition).getRow(),); self.getFirstNonRoadAdjacentCellPosition(java_arguments.0, java_arguments.1) };
        if randomGeographicMapCellPosition.is_none() {
            return;
        }
        let mut dropCellPositionHistory: Option<Rc<RefCell<DropCellPositionHistory>>> = DropCellPositionHistory::getInstance();
        if !jcall!(dropCellPositionHistory).isCellPositionWithDrop(geographicMapCellPosition.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>)) {
            let mut direction: Option<Rc<RefCell<Direction>>> = jcall!(self.geographicMapDirectionUtil).getDirectionFromCellPositionToAdjacentCellPosition(randomGeographicMapCellPosition.clone(), geographicMapCellPosition.clone());
            let mut objectCanBeNull: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(self.hashtable).get(crate::java_into_object(jcall!(self.groupCommonFactory).ID.clone()));
            if objectCanBeNull.is_some() {
            } else {
                { let java_receiver = self.hashtable.clone(); let java_arguments = (crate::java_into_object(jcall!(self.groupCommonFactory).ID.clone()), crate::java_into_object(jcall!(BasicGroupFactory::getInstance()).NONE_ARRAY.clone()),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1); };
            }
            { let java_receiver = self.hashtable.clone(); let java_arguments = (crate::java_into_object(jcall!(DirectionFactory::getInstance()).NAME.clone()), crate::java_into_object(direction.clone()),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1); };
            let mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = jcall!(randomGeographicMapCellPosition).getPoint();
            let mut x: i32 = jcall!(point).getX();
            let mut y: i32 = jcall!(point).getY();
            let mut z: i32 = jcall!(point).getZ();
            let mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> = jcall!(jcall!(RaceTrackAdjacentDropLayerFactory::RaceTrackAdjacentDropLayerFactory::getInstance()).getRandomInstance()).getNextInstance(self.hashtable.clone(), x.clone(), y.clone(), z.clone());
            let mut baseRaceTrackGeographicMap: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::racetrack::BaseRaceTrackGeographicMap::BaseRaceTrackGeographicMapDyn>>> = (self.raceTrackGeographicMap).clone() /* as BaseRaceTrackGeographicMap */;
            let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(self.layerCoveringCellPositionsUtil).getAll(baseRaceTrackGeographicMap.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>), randomGeographicMapCellPosition.clone(), layerInterface.clone(), BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayListD::BasicArrayListDDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>));
            if jcall!(dropCellPositionHistory).anyCellPositionWithDrop(list.clone()) {
                return;
            }
            let mut geographicMapCellType: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellTypeDyn>>> = None;
            {
                let mut index2: i32 = jcall!(list).size();
                while { index2 -= 1; index2 } >= 0 {
                geographicMapCellType = { let java_receiver = self.raceTrackGeographicMap.clone(); let java_arguments = ({ let java_object = (jcall!(list).get(index2.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>>().expect("ClassCastException").clone()) },); let java_result = jcall!(java_receiver).getCellTypeAt(java_arguments.0); java_result };
                if jcall!(self.raceTrackGeographicMapCellTypeFactory).isPath(geographicMapCellType.clone()) {
                    return;
                }
                }
            }
            jcall!(dropCellPositionHistory).addAll(list.clone(), layerInterface.clone());
            jcall!(allBinaryLayerManager).append(layerInterface.clone());
        }
    }
}
impl std::ops::Deref for RaceTrackAdjacentDropCellPositionGenerator {
    type Target = RaceTrackDropCellPositionGenerator;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for RaceTrackAdjacentDropCellPositionGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::media::graphics::geography::map::racetrack::drop::RaceTrackDropCellPositionGenerator::RaceTrackDropCellPositionGeneratorDyn for RaceTrackAdjacentDropCellPositionGenerator {
    fn as_RaceTrackDropCellPositionGenerator(&self) -> &crate::org::allbinary::media::graphics::geography::map::racetrack::drop::RaceTrackDropCellPositionGenerator::RaceTrackDropCellPositionGenerator { &self.base }
    fn as_RaceTrackDropCellPositionGenerator_mut(&mut self) -> &mut crate::org::allbinary::media::graphics::geography::map::racetrack::drop::RaceTrackDropCellPositionGenerator::RaceTrackDropCellPositionGenerator { &mut self.base }
    fn drop(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut index: i32) { Self::drop(self, allBinaryLayerManager, index) }
}
impl crate::org::allbinary::media::graphics::geography::map::drop::BaseDropCellPositionGenerator::BaseDropCellPositionGeneratorDyn for RaceTrackAdjacentDropCellPositionGenerator {
    fn as_BaseDropCellPositionGenerator(&self) -> &crate::org::allbinary::media::graphics::geography::map::drop::BaseDropCellPositionGenerator::BaseDropCellPositionGenerator { &self.base.base }
    fn as_BaseDropCellPositionGenerator_mut(&mut self) -> &mut crate::org::allbinary::media::graphics::geography::map::drop::BaseDropCellPositionGenerator::BaseDropCellPositionGenerator { &mut self.base.base }
}
impl crate::org::allbinary::media::graphics::geography::map::drop::DropCellPositionGeneratorInterface::DropCellPositionGeneratorInterface for RaceTrackAdjacentDropCellPositionGenerator {
    fn getName(&mut self) -> String { crate::org::allbinary::media::graphics::geography::map::drop::BaseDropCellPositionGenerator::BaseDropCellPositionGenerator::getName(crate::org::allbinary::media::graphics::geography::map::drop::BaseDropCellPositionGenerator::BaseDropCellPositionGeneratorDyn::as_BaseDropCellPositionGenerator_mut(self)) }
    fn update(&mut self, mut allBinaryGameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>) { Self::update(self, allBinaryGameLayerManager, geographicMapInterface) }
    fn processTick(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { crate::org::allbinary::media::graphics::geography::map::racetrack::drop::RaceTrackDropCellPositionGenerator::RaceTrackDropCellPositionGenerator::processTick(crate::org::allbinary::media::graphics::geography::map::racetrack::drop::RaceTrackDropCellPositionGenerator::RaceTrackDropCellPositionGeneratorDyn::as_RaceTrackDropCellPositionGenerator_mut(self), p1) }
    fn isDropAllowedAt(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> bool { crate::org::allbinary::media::graphics::geography::map::racetrack::drop::RaceTrackDropCellPositionGenerator::RaceTrackDropCellPositionGenerator::isDropAllowedAt(crate::org::allbinary::media::graphics::geography::map::racetrack::drop::RaceTrackDropCellPositionGenerator::RaceTrackDropCellPositionGeneratorDyn::as_RaceTrackDropCellPositionGenerator_mut(self), p1) }
}


