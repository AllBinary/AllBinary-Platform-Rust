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
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayer;
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMap;
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMapCellPositionFactory::BasicGeographicMapCellPositionFactory;
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMapUtil::BasicGeographicMapUtil;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPosition;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellType;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellTypeFactory::GeographicMapCellTypeFactory;
use crate::org::allbinary::media::graphics::geography::map::racetrack::BasePathFindingInfoFactory::BasePathFindingInfoFactory;
use crate::org::allbinary::media::graphics::geography::map::racetrack::RaceTrackGeographicMapCellType::RaceTrackGeographicMapCellType;
use crate::org::allbinary::media::graphics::geography::map::racetrack::RaceTrackRoadsGeographicMapCellHistoryFactory::RaceTrackRoadsGeographicMapCellHistoryFactory;
use crate::org::allbinary::media::graphics::geography::pathfinding::PathFindingNodeCostInfoFactoryBase::PathFindingNodeCostInfoFactoryBase;
use crate::org::allbinary::media::graphics::geography::pathfinding::BasicGeographicMapGraph::BasicGeographicMapGraph;
use crate::org::allbinary::media::graphics::geography::pathfinding::BasicGeographicMapPathFinder::BasicGeographicMapPathFinder;
use crate::org::allbinary::media::graphics::geography::pathfinding::PathFinderGraphVisitorFactoryBase::PathFinderGraphVisitorFactoryBase;
use crate::org::allbinary::media::graphics::geography::pathfinding::PathFindingInfo::PathFindingInfo;
use crate::org::allbinary::media::graphics::geography::pathfinding::PathFindingNodeCostInfoFactory::PathFindingNodeCostInfoFactory;
use crate::org::allbinary::util::BasicArrayListS::BasicArrayListS;
use super::SimplePathFinderGraphVisitorFactory::SimplePathFinderGraphVisitorFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PathFindingInfoFactory {
    pub base: BasePathFindingInfoFactory,
    pub MAX_DIRECTIONS: i32,
    pub pathFindingInfo: Option<Rc<RefCell<PathFindingInfo>>>,
    pub basicGeographicMapGraph: Option<Rc<RefCell<BasicGeographicMapGraph>>>,
    pub basicGeographicMapPathFinder: Option<Rc<RefCell<BasicGeographicMapPathFinder>>>,
    pub pathFinderGraphVisitorFactoryInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::pathfinding::PathFinderGraphVisitorFactoryBase::PathFinderGraphVisitorFactoryBaseDyn>>>,
}

pub static mut PathFindingInfoFactory_instance: std::cell::LazyCell<Option<Rc<RefCell<PathFindingInfoFactory>>>> = std::cell::LazyCell::new(|| None);

impl PathFindingInfoFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<PathFindingInfoFactory>>> {
        return unsafe { PathFindingInfoFactory_instance.clone() }.clone();
    }
    

    pub fn initMax(mut max: i32) {
        unsafe { *PathFindingInfoFactory_instance = PathFindingInfoFactory::new(SimplePathFinderGraphVisitorFactory::new(1, 1, i32::MAX.clone()), max.clone()) };
    }
    

    pub fn new_struct(mut pathFinderGraphVisitorFactoryInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::pathfinding::PathFinderGraphVisitorFactoryBase::PathFinderGraphVisitorFactoryBaseDyn>>>, mut max: i32) -> Self {
        let mut this = Self {
            base: BasePathFindingInfoFactory::new_struct(),
            MAX_DIRECTIONS: 0,
            pathFindingInfo: None,
            basicGeographicMapGraph: None,
            basicGeographicMapPathFinder: None,
            pathFinderGraphVisitorFactoryInterface: None,
        };
        this.MAX_DIRECTIONS = 4;
        let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
        PreLogUtil::put(String::from("Using Dynamic Path Finding"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).GET_INSTANCE.clone());
        this.basicGeographicMapPathFinder = BasicGeographicMapPathFinder::new(max.clone());
        this.pathFindingInfo = PathFindingInfo::new(PathFindingNodeCostInfoFactory::new(max.clone()), BasicArrayListS::new(1).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayListS::BasicArrayListSDyn>>), BasicArrayListS::new(1).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayListS::BasicArrayListSDyn>>));
        this.basicGeographicMapGraph = BasicGeographicMapGraph::new((jcall!(this.pathFindingInfo).getPathFindingNodeCostInfoFactoryInterface()).clone() /* as PathFindingNodeCostInfoFactory */);
        this.pathFinderGraphVisitorFactoryInterface = pathFinderGraphVisitorFactoryInterface.clone();
        this
    }

    pub fn new(mut pathFinderGraphVisitorFactoryInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::pathfinding::PathFinderGraphVisitorFactoryBase::PathFinderGraphVisitorFactoryBaseDyn>>>, mut max: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(pathFinderGraphVisitorFactoryInterface, max))))
    }
    

    // throws Exception
    pub fn getInstancePathFindingInfo(&mut self, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>, mut mapArray: Vec<Vec<i32>>) -> Option<Rc<RefCell<PathFindingInfo>>> {
        let mut pathFindingInfo: Option<Rc<RefCell<PathFindingInfo>>> = PathFindingInfo::new(jcall!(self.pathFindingInfo).getPathFindingNodeCostInfoFactoryInterface(), BasicArrayListS::new(1).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayListS::BasicArrayListSDyn>>), BasicArrayListS::new(1).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayListS::BasicArrayListSDyn>>));
        jcall!(RaceTrackRoadsGeographicMapCellHistoryFactory::getInstance()).init();
        self.init(geographicMapInterface.clone(), pathFindingInfo.clone(), mapArray.clone());
        { let java_receiver = self.basicGeographicMapPathFinder.clone(); let java_arguments = (geographicMapInterface.clone(), self.basicGeographicMapGraph.clone(), jcall!(self.pathFinderGraphVisitorFactoryInterface).getInstance(geographicMapInterface.clone()),); jcall!(java_receiver).init(java_arguments.0, java_arguments.1, java_arguments.2); };
        jcall!(pathFindingInfo).setPathFinder(self.basicGeographicMapPathFinder.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::pathfinding::GeographicPathFinderBase::GeographicPathFinderBaseDyn>>));
        return pathFindingInfo.clone();
    }
    

    // throws Exception
    pub fn init(&mut self, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>, mut pathFindingInfo: Option<Rc<RefCell<PathFindingInfo>>>, mut mapArray: Vec<Vec<i32>>) {
        self.base.init(geographicMapInterface.clone(), pathFindingInfo.clone(), mapArray.clone());
        self.buildPathFindingNodes(geographicMapInterface.clone(), pathFindingInfo.clone(), mapArray.clone());
    }
    

    // throws Exception
    pub fn buildPathFindingNodesForCellPosition(&mut self, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>, mut pathFindingInfo: Option<Rc<RefCell<PathFindingInfo>>>, mut mapArray: Vec<Vec<i32>>, mut cellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) {
        let mut geographicMapCellPositionFactory: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMapCellPositionFactory::BasicGeographicMapCellPositionFactoryDyn>>> = jcall!(geographicMapInterface).getGeographicMapCellPositionFactory();
        let mut cellTypeId: i32 = mapArray[(jcall!(cellPosition).getRow()) as usize][(jcall!(cellPosition).getColumn()) as usize].clone();
        let mut geographicMapCellType: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellTypeDyn>>> = jcall!(GeographicMapCellTypeFactory::getInstance()).get(jcall!(geographicMapInterface).getCellTypeFromMapCellTypeInt(cellTypeId.clone()));
        let mut raceTrackGeographicMapCellType: Option<Rc<RefCell<RaceTrackGeographicMapCellType>>> = (geographicMapCellType).clone() /* as RaceTrackGeographicMapCellType */;
        let mut allBinaryTiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>> = jcall!(geographicMapInterface).getAllBinaryTiledLayer();
        let mut pathFindingNodeCostInfoFactoryInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::pathfinding::PathFindingNodeCostInfoFactoryBase::PathFindingNodeCostInfoFactoryBaseDyn>>> = jcall!(pathFindingInfo).getPathFindingNodeCostInfoFactoryInterface();
        let mut basicGeographicMapUtil: Option<Rc<RefCell<BasicGeographicMapUtil>>> = BasicGeographicMapUtil::getInstance();
        let mut row: i32 = 0;
        let mut column: i32 = 0;
        let mut goingToGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = None;
        let mut costFromStart: i32 = 0;
        let mut costToEnd: i32 = 0;
        for index in (0..self.MAX_DIRECTIONS) {
            row = jcall!(basicGeographicMapUtil).getBorderingRow(index.clone(), cellPosition.clone());
            column = jcall!(basicGeographicMapUtil).getBorderingColumn(index.clone(), cellPosition.clone());
            if row < jcall!(allBinaryTiledLayer).getRows() && column < jcall!(allBinaryTiledLayer).getColumns() && row >= 0 && column >= 0 {
                goingToGeographicMapCellPosition = jcall!(geographicMapCellPositionFactory).getAt(column.clone(), row.clone());
                costFromStart = jcall!(raceTrackGeographicMapCellType).getTravelCost();
                costToEnd = costFromStart.clone();
                jcall!(pathFindingNodeCostInfoFactoryInterface).create(geographicMapInterface.clone(), goingToGeographicMapCellPosition.clone(), cellPosition.clone(), costFromStart.clone(), costToEnd.clone());
            }
        }
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


