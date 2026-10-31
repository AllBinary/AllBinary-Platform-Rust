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
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::util::BasicArrayListUtil::BasicArrayListUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManager;
use crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager;
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMap;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellHistory::GeographicMapCellHistory;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPosition;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellTypeFactory::GeographicMapCellTypeFactory;
use crate::org::allbinary::media::graphics::geography::map::drop::BaseDropCellPositionGenerator::BaseDropCellPositionGenerator;
use crate::org::allbinary::media::graphics::geography::map::racetrack::BaseRaceTrackGeographicMap::BaseRaceTrackGeographicMap;
use crate::org::allbinary::media::graphics::geography::map::racetrack::NullRaceTrackGeographicMap::NullRaceTrackGeographicMap;
use crate::org::allbinary::media::graphics::geography::map::racetrack::RaceTrackGeographicMapCellType::RaceTrackGeographicMapCellType;
use crate::org::allbinary::media::graphics::geography::map::racetrack::RaceTrackGeographicMapCellTypeFactory::RaceTrackGeographicMapCellTypeFactory;
use crate::org::allbinary::media::graphics::geography::map::racetrack::RaceTrackGeographicMapInterface::RaceTrackGeographicMapInterface;
use crate::org::allbinary::media::graphics::geography::map::racetrack::RaceTrackRoadsGeographicMapCellHistoryFactory::RaceTrackRoadsGeographicMapCellHistoryFactory;
use crate::org::allbinary::time::GameTickTimeDelayHelperFactory::GameTickTimeDelayHelperFactory;
use crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelper;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RaceTrackDropCellPositionGenerator {
    pub base: BaseDropCellPositionGenerator,
    pub list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub timeDelayHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>>,
    pub STRAIGHTAWAY: i32,
    pub raceTrackGeographicMap: Option<Rc<RefCell<dyn RaceTrackGeographicMapInterface>>>,
    pub raceTrackGeographicMapCellTypeFactory: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellTypeFactory::GeographicMapCellTypeFactoryDyn>>>,
    pub basicArrayListUtil: Option<Rc<RefCell<BasicArrayListUtil>>>,
}


impl RaceTrackDropCellPositionGenerator {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: BaseDropCellPositionGenerator::new_struct(),
            list: None,
            timeDelayHelper: None,
            STRAIGHTAWAY: 0,
            raceTrackGeographicMap: None,
            raceTrackGeographicMapCellTypeFactory: None,
            basicArrayListUtil: None,
        };
        this.list = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.STRAIGHTAWAY = 4;
        this.raceTrackGeographicMap = crate::org::allbinary::media::graphics::geography::map::racetrack::NullRaceTrackGeographicMap::NullRaceTrackGeographicMap_NULL_RACE_TRACK_GEOGRAPHIC_MAP.with(Clone::clone).clone();
        this.raceTrackGeographicMapCellTypeFactory = GeographicMapCellTypeFactory::getInstance();
        this.basicArrayListUtil = BasicArrayListUtil::getInstance();
        this.timeDelayHelper = TimeDelayHelper::new(10000).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn init(&mut self) {
        jcall!(self.list).clear();
    }
    

    // throws Exception
    pub fn isDropAllowedAt(&mut self, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> bool {
        let mut raceTrackGeographicMapCellType: Option<Rc<RefCell<RaceTrackGeographicMapCellType>>> = (jcall!(self.raceTrackGeographicMap).getCellTypeAt(geographicMapCellPosition.clone())).clone() /* as RaceTrackGeographicMapCellType */;
        let mut raceTrackGeographicMapCellTypeFactory: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::racetrack::RaceTrackGeographicMapCellTypeFactory::RaceTrackGeographicMapCellTypeFactoryDyn>>> = (self.raceTrackGeographicMapCellTypeFactory).clone() /* as RaceTrackGeographicMapCellTypeFactory */;
        if match (&raceTrackGeographicMapCellType, &jcall!(raceTrackGeographicMapCellTypeFactory).as_GeographicMapCellTypeFactory_mut().HORIZONTAL_STRAIGHT_ROAD_CELL_TYPE) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } || match (&raceTrackGeographicMapCellType, &jcall!(raceTrackGeographicMapCellTypeFactory).as_GeographicMapCellTypeFactory_mut().VERTICAL_STRAIGHT_ROAD_CELL_TYPE) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return true;
        }
        return false;
    }
    

    // throws Exception
    pub fn update(&mut self, mut allBinaryGameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>) {
        self.init();
        let mut baseRaceTrackGeographicMap: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::racetrack::BaseRaceTrackGeographicMap::BaseRaceTrackGeographicMapDyn>>> = (geographicMapInterface).clone() /* as BaseRaceTrackGeographicMap */;
        self.raceTrackGeographicMap = baseRaceTrackGeographicMap.clone().map(|reference| reference as Rc<RefCell<dyn RaceTrackGeographicMapInterface>>);
        self.raceTrackGeographicMapCellTypeFactory = (jcall!(baseRaceTrackGeographicMap).getGeographicMapCellTypeFactory()).clone() /* as RaceTrackGeographicMapCellTypeFactory */;
        let mut roadGeographicMapCellHistory: Option<Rc<RefCell<GeographicMapCellHistory>>> = RaceTrackRoadsGeographicMapCellHistoryFactory::getInstance();
        let mut trackedList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(roadGeographicMapCellHistory).getTracked();
        let mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = None;
        let mut lastIndex: i32 = jcall!(trackedList).size() - 1;
        let mut total: i32 = 0;
        let mut ahead: i32 = 0;
        for index in ((0) + 1..=lastIndex).rev() {
            geographicMapCellPosition = { let java_object = (jcall!(trackedList).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>>().expect("ClassCastException").clone()) };
            total = 0;
            ahead = index.clone();
            while { let java_arguments = ({ let java_object = (jcall!(trackedList).get(ahead.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>>().expect("ClassCastException").clone()) },); self.isDropAllowedAt(java_arguments.0) } {
                total += 1;
                if total > self.STRAIGHTAWAY {
                    jcall!(self.list).add(crate::java_into_object(geographicMapCellPosition.clone()));
                    break;
                }
                ahead += 1;
                if ahead > lastIndex {
                    break;
                }
            }
        }
        let mut size: i32 = jcall!(self.list).size();
        for index in (0..=size - 1).rev() {
            self.drop(allBinaryGameLayerManager.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>), index.clone());
        }
    }
    

    // throws Exception
    pub fn processTick(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) {
        if { let java_receiver = self.timeDelayHelper.clone(); let java_arguments = (jcall!(GameTickTimeDelayHelperFactory::getInstance()).startTime.clone(),); let java_result = jcall!(java_receiver).isTime(java_arguments.0); java_result } {
            let mut index: i32 = jcall!(self.basicArrayListUtil).getRandomIndex(self.list.clone());
            self.drop(allBinaryLayerManager.clone(), index.clone());
        }
    }
    

    // throws Exception
    pub fn drop(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut index: i32) {
        panic!("{:?}", format!("{}: {:?}", "Exception", (jcall!(CommonStrings::getInstance()).NOT_IMPLEMENTED.clone())));
    }
}
impl std::ops::Deref for RaceTrackDropCellPositionGenerator {
    type Target = BaseDropCellPositionGenerator;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for RaceTrackDropCellPositionGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait RaceTrackDropCellPositionGeneratorDyn: crate::org::allbinary::media::graphics::geography::map::drop::BaseDropCellPositionGenerator::BaseDropCellPositionGeneratorDyn {
    fn as_RaceTrackDropCellPositionGenerator(&self) -> &RaceTrackDropCellPositionGenerator;
    fn as_RaceTrackDropCellPositionGenerator_mut(&mut self) -> &mut RaceTrackDropCellPositionGenerator;
    fn init(&mut self) { RaceTrackDropCellPositionGenerator::init(self.as_RaceTrackDropCellPositionGenerator_mut()) }
    fn drop(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut index: i32) { RaceTrackDropCellPositionGenerator::drop(self.as_RaceTrackDropCellPositionGenerator_mut(), allBinaryLayerManager, index) }
}
impl RaceTrackDropCellPositionGeneratorDyn for RaceTrackDropCellPositionGenerator {
    fn as_RaceTrackDropCellPositionGenerator(&self) -> &RaceTrackDropCellPositionGenerator { self }
    fn as_RaceTrackDropCellPositionGenerator_mut(&mut self) -> &mut RaceTrackDropCellPositionGenerator { self }
}

impl crate::org::allbinary::media::graphics::geography::map::drop::BaseDropCellPositionGenerator::BaseDropCellPositionGeneratorDyn for RaceTrackDropCellPositionGenerator {
    fn as_BaseDropCellPositionGenerator(&self) -> &crate::org::allbinary::media::graphics::geography::map::drop::BaseDropCellPositionGenerator::BaseDropCellPositionGenerator { &self.base }
    fn as_BaseDropCellPositionGenerator_mut(&mut self) -> &mut crate::org::allbinary::media::graphics::geography::map::drop::BaseDropCellPositionGenerator::BaseDropCellPositionGenerator { &mut self.base }
}
impl crate::org::allbinary::media::graphics::geography::map::drop::DropCellPositionGeneratorInterface::DropCellPositionGeneratorInterface for RaceTrackDropCellPositionGenerator {
    fn getName(&mut self) -> String { crate::org::allbinary::media::graphics::geography::map::drop::BaseDropCellPositionGenerator::BaseDropCellPositionGenerator::getName(crate::org::allbinary::media::graphics::geography::map::drop::BaseDropCellPositionGenerator::BaseDropCellPositionGeneratorDyn::as_BaseDropCellPositionGenerator_mut(self)) }
    fn update(&mut self, mut allBinaryGameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>) { Self::update(self, allBinaryGameLayerManager, geographicMapInterface) }
    fn processTick(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { Self::processTick(self, allBinaryLayerManager) }
    fn isDropAllowedAt(&mut self, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> bool { Self::isDropAllowedAt(self, geographicMapCellPosition) }
}


