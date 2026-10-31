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
// package org.allbinary.game.media.graphics.geography.map.racetrack
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayer;
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMap;
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMapCellPositionFactory::BasicGeographicMapCellPositionFactory;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPosition;
use crate::org::allbinary::media::graphics::geography::map::racetrack::BasePathFindingInfoFactory::BasePathFindingInfoFactory;
use crate::org::allbinary::media::graphics::geography::map::racetrack::RaceTrackRoadsGeographicMapCellHistoryFactory::RaceTrackRoadsGeographicMapCellHistoryFactory;
use crate::org::allbinary::media::graphics::geography::pathfinding::PathFinder::PathFinder;
use crate::org::allbinary::media::graphics::geography::pathfinding::PathFindingInfo::PathFindingInfo;
use crate::org::allbinary::media::graphics::geography::pathfinding::PathFindingNodeCostInfoFactory::PathFindingNodeCostInfoFactory;
use crate::org::allbinary::util::BasicArrayListS::BasicArrayListS;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//1.4.2
pub struct PathFindingInfoFactory {
    pub base: BasePathFindingInfoFactory,
    pub pathFindingInfo: Option<Rc<RefCell<PathFindingInfo>>>,
    pub pathFinder: Option<Rc<RefCell<PathFinder>>>,
}

thread_local! { pub static PathFindingInfoFactory_instance: Option<Rc<RefCell<PathFindingInfoFactory>>> = PathFindingInfoFactory::new(); }
pub static mut PathFindingInfoFactory_MAX: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| 32768);

impl PathFindingInfoFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<PathFindingInfoFactory>>> {
        return PathFindingInfoFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn initMax(mut max: i32) {
        unsafe { *PathFindingInfoFactory_MAX = max.clone() };
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: BasePathFindingInfoFactory::new_struct(),
            pathFindingInfo: None,
            pathFinder: None,
        };
        this.pathFindingInfo = PathFindingInfo::new(PathFindingNodeCostInfoFactory::new(unsafe { PathFindingInfoFactory_MAX.clone() }.clone()), BasicArrayListS::new(1).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayListS::BasicArrayListSDyn>>), BasicArrayListS::new(1).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayListS::BasicArrayListSDyn>>));
        this.pathFinder = PathFinder::new();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn getInstancePathFindingInfo(&mut self, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>, mut mapArray: Vec<Vec<i32>>) -> Option<Rc<RefCell<PathFindingInfo>>> {
        let mut pathFindingInfo: Option<Rc<RefCell<PathFindingInfo>>> = PathFindingInfo::new(jcall!(self.pathFindingInfo).getPathFindingNodeCostInfoFactoryInterface(), BasicArrayListS::new(1).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayListS::BasicArrayListSDyn>>), BasicArrayListS::new(1).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayListS::BasicArrayListSDyn>>));
        jcall!(RaceTrackRoadsGeographicMapCellHistoryFactory::getInstance()).init();
        self.init(geographicMapInterface.clone(), pathFindingInfo.clone(), mapArray.clone());
        jcall!(self.pathFinder).init(geographicMapInterface.clone());
        jcall!(pathFindingInfo).setPathFinder(self.pathFinder.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::pathfinding::GeographicPathFinderBase::GeographicPathFinderBaseDyn>>));
        return pathFindingInfo.clone();
    }
    

    // throws Exception
    pub fn init(&mut self, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>, mut pathFindingInfo: Option<Rc<RefCell<PathFindingInfo>>>, mut mapArray: Vec<Vec<i32>>) {
        self.base.init(geographicMapInterface.clone(), pathFindingInfo.clone(), mapArray.clone());
        self.buildPathFindingNodes(geographicMapInterface.clone(), pathFindingInfo.clone(), mapArray.clone());
    }
    

    // throws Exception
    pub fn buildPathFindingNodesForCellPosition(&mut self, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>, mut pathFindingInfo: Option<Rc<RefCell<PathFindingInfo>>>, mut mapArray: Vec<Vec<i32>>, mut cellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) {
    }
    

    // throws Exception
    pub fn buildPathFindingNodes(&mut self, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>, mut pathFindingInfo: Option<Rc<RefCell<PathFindingInfo>>>, mut mapArray: Vec<Vec<i32>>) {
        let mut geographicMapCellPositionFactory: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMapCellPositionFactory::BasicGeographicMapCellPositionFactoryDyn>>> = jcall!(geographicMapInterface).getGeographicMapCellPositionFactory();
        let mut allBinaryTiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>> = jcall!(geographicMapInterface).getAllBinaryTiledLayer();
        let mut totalColumns: i32 = jcall!(allBinaryTiledLayer).getColumns();
        let mut totalRows: i32 = jcall!(allBinaryTiledLayer).getRows();
        for column in (0..totalColumns) {
            for row in (0..totalRows) {
                { let java_arguments = (geographicMapInterface.clone(), pathFindingInfo.clone(), mapArray.clone(), jcall!(geographicMapCellPositionFactory).getAt(column.clone(), row.clone()),); self.buildPathFindingNodesForCellPosition(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3) };
            }
        }
    }
}
impl std::ops::Deref for PathFindingInfoFactory {
    type Target = BasePathFindingInfoFactory;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for PathFindingInfoFactory {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::media::graphics::geography::map::racetrack::BasePathFindingInfoFactory::BasePathFindingInfoFactoryDyn for PathFindingInfoFactory {
    fn as_BasePathFindingInfoFactory(&self) -> &crate::org::allbinary::media::graphics::geography::map::racetrack::BasePathFindingInfoFactory::BasePathFindingInfoFactory { &self.base }
    fn as_BasePathFindingInfoFactory_mut(&mut self) -> &mut crate::org::allbinary::media::graphics::geography::map::racetrack::BasePathFindingInfoFactory::BasePathFindingInfoFactory { &mut self.base }
    fn init(&mut self, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>, mut pathFindingInfo: Option<Rc<RefCell<PathFindingInfo>>>, mut mapArray: Vec<Vec<i32>>) { Self::init(self, geographicMapInterface, pathFindingInfo, mapArray) }
    fn getInstancePathFindingInfo(&mut self, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>, mut mapArray: Vec<Vec<i32>>) -> Option<Rc<RefCell<PathFindingInfo>>> { Self::getInstancePathFindingInfo(self, geographicMapInterface, mapArray) }
}


