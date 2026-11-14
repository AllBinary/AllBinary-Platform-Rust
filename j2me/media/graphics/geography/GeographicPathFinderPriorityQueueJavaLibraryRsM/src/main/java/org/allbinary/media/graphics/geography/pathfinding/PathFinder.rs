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
// package org.allbinary.media.graphics.geography.pathfinding
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::java::util::HashSet::HashSet;
use crate::java::util::PriorityQueue::PriorityQueue;
use crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayer;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::math::MathUtil::MathUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMap;
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMapCellPositionFactory::BasicGeographicMapCellPositionFactory;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPosition;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellType;
use crate::org::allbinary::media::graphics::geography::map::racetrack::RaceTrackGeographicMapCellType::RaceTrackGeographicMapCellType;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::util::BasicArrayListUtil::BasicArrayListUtil;
use crate::java::lang::Exception::*;
use crate::java::lang::RuntimeException::*;
use super::GeographicPathFinderBase::GeographicPathFinderBase;
use super::PathFindingNodeCost::PathFindingNodeCost;
use super::PathFindingNodeCostInfo::PathFindingNodeCostInfo;
use super::PathFindingNode::PathFindingNode;
use super::MultipassState::MultipassState;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PathFinder {
    pub base: GeographicPathFinderBase,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub basicArrayListUtil: Option<Rc<RefCell<BasicArrayListUtil>>>,
    pub mathUtil: Option<Rc<RefCell<MathUtil>>>,
    pub openPriorityQueue: Option<Rc<RefCell<PriorityQueue<Option<Rc<RefCell<PathFindingNodeCost>>>>>>>,
    pub closedSet: Option<Rc<RefCell<HashSet<crate::JavaObject>>>>,
    pub geographicMapInterface: Option<Rc<RefCell<Box<dyn std::any::Any>>>>,
    pub costArray: Vec<Vec<Option<Rc<RefCell<PathFindingNodeCost>>>>>,
}


impl PathFinder {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: GeographicPathFinderBase::new_struct(),
            logUtil: None,
            basicArrayListUtil: None,
            mathUtil: None,
            openPriorityQueue: None,
            closedSet: None,
            geographicMapInterface: None,
            costArray: Vec::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.basicArrayListUtil = BasicArrayListUtil::getInstance();
        this.mathUtil = MathUtil::getInstance();
        this.openPriorityQueue = PriorityQueue::new();
        this.closedSet = HashSet::new();
        this.geographicMapInterface = { let java_field = jcall!(NullUtil::getInstance()).NULL_OBJECT.clone(); java_field }.clone();
        this.costArray = PathFindingNodeCost_NULL_PATH_FINDING_NODE_COST_ARRAY_ARRAY.with(Clone::clone).clone();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn init(&mut self, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>) {
        self.geographicMapInterface = crate::java_into_object(geographicMapInterface.clone());
        let mut tiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>> = jcall!(geographicMapInterface).getAllBinaryTiledLayer();
        self.costArray = { let java_value = (vec![vec![None; (jcall!(tiledLayer).getRows()) as usize]; (jcall!(tiledLayer).getColumns()) as usize]).clone(); java_value };
        let mut basicGeographicMapCellPositionFactory: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMapCellPositionFactory::BasicGeographicMapCellPositionFactoryDyn>>> = jcall!(geographicMapInterface).getGeographicMapCellPositionFactory();
        let mut node: Option<Rc<RefCell<PathFindingNodeCost>>> = None;
        let mut sizeX: i32 = (self.costArray.len() as i32).clone();
        let mut sizeY: i32 = (self.costArray[(0) as usize].len() as i32).clone();
        for column in (0..sizeX) {
            for row in (0..sizeY) {
                let mut geographicMapCellType: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellTypeDyn>>> = jcall!(geographicMapInterface).getCellTypeAt(jcall!(basicGeographicMapCellPositionFactory).getAt(column.clone(), row.clone()));
                let mut raceTrackGeographicMapCellType: Option<Rc<RefCell<RaceTrackGeographicMapCellType>>> = (geographicMapCellType).clone().map(|reference| { assert!({ let java_borrowed = reference.borrow(); let java_any: &dyn std::any::Any = &*java_borrowed; java_any.is::<RaceTrackGeographicMapCellType>() }, "ClassCastException: not a RaceTrackGeographicMapCellType"); unsafe { Rc::from_raw(Rc::into_raw(reference) as *const RefCell<RaceTrackGeographicMapCellType>) } });
                node = { let java_value = (PathFindingNodeCost::new(crate::java_into_object({ let java_field = jcall!(NullUtil::getInstance()).NULL_OBJECT.clone(); java_field }.clone()), jcall!(basicGeographicMapCellPositionFactory).getAt(column.clone(), row.clone()), PathFindingNodeCostInfo::new(((jcall!(raceTrackGeographicMapCellType).getTravelCost()) as i64), ((-1) as i64)))).clone(); java_value };
                { let java_index = (row) as usize; self.costArray[(column) as usize][java_index] = node.clone(); };
            }
        }
    }
    

    pub fn searchTotalPath(&mut self, mut startPathFindingNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut endPathFindingNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut totalPaths: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            return Some({ let java_arguments = ({ let java_object = (jcall!(startPathFindingNodeList).get(0)).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::pathfinding::PathFindingNode::PathFindingNodeDyn>>>>().expect("ClassCastException").clone()) }, { let java_object = (jcall!(endPathFindingNodeList).get(0)).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::pathfinding::PathFindingNode::PathFindingNodeDyn>>>>().expect("ClassCastException").clone()) },); self.search(java_arguments.0, java_arguments.1) });
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(self.logUtil).put({ let java_field = jcall!(commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("search"), crate::java_into_object(e.clone()));
            return jcall!(self.basicArrayListUtil).getImmutableInstance();
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    // throws Exception
    pub fn searchTotalPathN(&mut self, mut startPathFindingNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut endPathFindingNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut totalPaths: i32, mut multipassState: Option<Rc<RefCell<MultipassState>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        if { let java_field = jcall!(multipassState).step.clone(); java_field } == 0 {
            jcall!(multipassState).step += 1;
        } else if { let java_field = jcall!(multipassState).step.clone(); java_field } == 1 {
            { let java_arguments = ({ let java_object = (jcall!(startPathFindingNodeList).get(0)).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::pathfinding::PathFindingNode::PathFindingNodeDyn>>>>().expect("ClassCastException").clone()) }, { let java_object = (jcall!(endPathFindingNodeList).get(0)).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::pathfinding::PathFindingNode::PathFindingNodeDyn>>>>().expect("ClassCastException").clone()) }, multipassState.clone(),); self.searchStart(java_arguments.0, java_arguments.1, java_arguments.2) };
        } else if { let java_field = jcall!(multipassState).step.clone(); java_field } == 2 {
            return { let java_arguments = ({ let java_object = (jcall!(startPathFindingNodeList).get(0)).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::pathfinding::PathFindingNode::PathFindingNodeDyn>>>>().expect("ClassCastException").clone()) }, { let java_object = (jcall!(endPathFindingNodeList).get(0)).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::pathfinding::PathFindingNode::PathFindingNodeDyn>>>>().expect("ClassCastException").clone()) }, multipassState.clone(),); self.searchN(java_arguments.0, java_arguments.1, java_arguments.2) };
        }
        return jcall!(self.basicArrayListUtil).getImmutableInstance();
    }
    

    // throws Exception
    pub fn search(&mut self, mut startPathFindingNode: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::pathfinding::PathFindingNode::PathFindingNodeDyn>>>, mut endPathFindingNode: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::pathfinding::PathFindingNode::PathFindingNodeDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = { let java_arguments = ({ let java_field = jcall!(startPathFindingNode).as_PathFindingNode_mut().geographicMapCellPosition.clone(); java_field }.clone(), { let java_field = jcall!(endPathFindingNode).as_PathFindingNode_mut().geographicMapCellPosition.clone(); java_field }.clone(),); self.findPath(java_arguments.0, java_arguments.1) };
        let mut pathList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        jcall!(pathList).add(crate::java_into_object(list.clone()));
        return pathList.clone();
    }
    

    // throws Exception
    pub fn searchStart(&mut self, mut startPathFindingNode: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::pathfinding::PathFindingNode::PathFindingNodeDyn>>>, mut endPathFindingNode: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::pathfinding::PathFindingNode::PathFindingNodeDyn>>>, mut multipassState: Option<Rc<RefCell<MultipassState>>>) {
        { let java_arguments = ({ let java_field = jcall!(startPathFindingNode).as_PathFindingNode_mut().geographicMapCellPosition.clone(); java_field }.clone(), { let java_field = jcall!(endPathFindingNode).as_PathFindingNode_mut().geographicMapCellPosition.clone(); java_field }.clone(), multipassState.clone(),); self.findPathStart(java_arguments.0, java_arguments.1, java_arguments.2) };
    }
    

    // throws Exception
    pub fn searchN(&mut self, mut startPathFindingNode: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::pathfinding::PathFindingNode::PathFindingNodeDyn>>>, mut endPathFindingNode: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::pathfinding::PathFindingNode::PathFindingNodeDyn>>>, mut multipassState: Option<Rc<RefCell<MultipassState>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = { let java_arguments = ({ let java_field = jcall!(startPathFindingNode).as_PathFindingNode_mut().geographicMapCellPosition.clone(); java_field }.clone(), { let java_field = jcall!(endPathFindingNode).as_PathFindingNode_mut().geographicMapCellPosition.clone(); java_field }.clone(), multipassState.clone(),); self.findPathEnd(java_arguments.0, java_arguments.1, java_arguments.2) };
        if list.is_none() {
            return jcall!(self.basicArrayListUtil).getImmutableInstance();
        }
        let mut pathList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        jcall!(pathList).add(crate::java_into_object(list.clone()));
        jcall!(multipassState).iteration = 0;
        jcall!(multipassState).step = 0;
        return pathList.clone();
    }
    

    // throws Exception
    pub fn findPath(&mut self, mut start: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>, mut target: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        jcall!(self.openPriorityQueue).clear();
        jcall!(self.closedSet).clear();
        let mut discoveryCalculation: i64 = 0_i64;
        let mut node: Option<Rc<RefCell<PathFindingNodeCost>>> = None;
        let mut targetColumn: i32 = jcall!(target).getColumn();
        let mut targetRow: i32 = jcall!(target).getRow();
        let mut sizeX: i32 = (self.costArray.len() as i32).clone();
        let mut sizeY: i32 = (self.costArray[(0) as usize].len() as i32).clone();
        for column in (0..sizeX) {
            for row in (0..sizeY) {
                discoveryCalculation = { let java_value = (((jcall!(self.mathUtil).abs(column - targetColumn)) as i64) + ((jcall!(self.mathUtil).abs(row - targetRow)) as i64)).clone(); java_value };
                node = { let java_value = (self.costArray[(column) as usize][(row) as usize]).clone(); java_value }.clone();
                jcall!({ let java_field = jcall!(node).pathFindingNodeCostInfoP.clone(); java_field }).totalCostP = 0_i64;
                jcall!({ let java_field = jcall!(node).pathFindingNodeCostInfoP.clone(); java_field }).costToEndP = discoveryCalculation.clone();
            }
        }
        let mut startNode: Option<Rc<RefCell<PathFindingNodeCost>>> = self.costArray[(jcall!(start).getColumn()) as usize][(jcall!(start).getRow()) as usize].clone();
        jcall!(self.openPriorityQueue).add(startNode.clone());
        let mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>> = { let java_object = (self.geographicMapInterface).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>>().expect("ClassCastException").clone()) };
        let mut basicGeographicMapCellPositionFactory: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMapCellPositionFactory::BasicGeographicMapCellPositionFactoryDyn>>> = jcall!(geographicMapInterface).getGeographicMapCellPositionFactory();
        let mut allBinaryTiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>> = jcall!(geographicMapInterface).getAllBinaryTiledLayer();
        let mut targetNode: Option<Rc<RefCell<PathFindingNodeCost>>> = self.costArray[(jcall!(target).getColumn()) as usize][(jcall!(target).getRow()) as usize].clone();
        let mut current: Option<Rc<RefCell<PathFindingNodeCost>>> = None;
        loop {
            current = { let java_value = (jcall!(self.openPriorityQueue).poll()).clone(); java_value };
            jcall!(self.closedSet).add(current.clone());
            if jcall!(current).equals(targetNode.clone()) {
                return self.extractPath(start.clone(), current.clone());
            }
            let mut neighbor: Option<Rc<RefCell<PathFindingNodeCost>>> = None;
            let mut neighborInfo: Option<Rc<RefCell<PathFindingNodeCostInfo>>> = None;
            let mut calculatedCost: i64 = 0_i64;
            for column in ({ let java_bound = jcall!({ let java_field = jcall!(current).geographicMapCellPosition.clone(); java_field }).getColumn() - 1; java_bound }..{ let java_bound = jcall!({ let java_field = jcall!(current).geographicMapCellPosition.clone(); java_field }).getColumn() + 2; java_bound }) {
                for row in ({ let java_bound = jcall!({ let java_field = jcall!(current).geographicMapCellPosition.clone(); java_field }).getRow() - 1; java_bound }..{ let java_bound = jcall!({ let java_field = jcall!(current).geographicMapCellPosition.clone(); java_field }).getRow() + 2; java_bound }) {
                    if column > 0 && row > 0 && column < jcall!(allBinaryTiledLayer).getColumns() && row < jcall!(allBinaryTiledLayer).getRows() && jcall!(geographicMapInterface).isOnMap(jcall!(basicGeographicMapCellPositionFactory).getAt(column.clone(), row.clone())) {
                        neighbor = { let java_value = (self.costArray[(column) as usize][(row) as usize]).clone(); java_value }.clone();
                        if jcall!(self.closedSet).contains(neighbor.clone()) {
                            continue;
                        }
                        neighborInfo = { let java_value = ({ let java_field = jcall!(neighbor).pathFindingNodeCostInfoP.clone(); java_field }).clone(); java_value }.clone();
                        calculatedCost = { let java_value = ({ let java_field = jcall!(neighborInfo).costToEndP.clone(); java_field } + { let java_field = jcall!(neighborInfo).costFromStartP.clone(); java_field } + { let java_field = jcall!({ let java_field = jcall!(current).pathFindingNodeCostInfoP.clone(); java_field }).totalCostP.clone(); java_field }).clone(); java_value };
                        if calculatedCost < { let java_field = jcall!(neighborInfo).totalCostP.clone(); java_field } || !jcall!(self.openPriorityQueue).contains(neighbor.clone()) {
                            jcall!(neighborInfo).totalCostP = calculatedCost.clone();
                            jcall!(neighbor).parent = crate::java_into_object(current.clone());
                            if !jcall!(self.openPriorityQueue).contains(neighbor.clone()) {
                                if jcall!(geographicMapInterface).isOfFourDirections({ let java_field = jcall!(current).geographicMapCellPosition.clone(); java_field }.clone(), { let java_field = jcall!(neighbor).geographicMapCellPosition.clone(); java_field }.clone()) {
                                    jcall!(self.openPriorityQueue).add(neighbor.clone());
                                }
                            }
                        }
                    }
                }
            }
            if !(!jcall!(self.openPriorityQueue).isEmpty()) { break; }
        }
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // throws Exception
    pub fn findPathStart(&mut self, mut start: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>, mut target: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>, mut multipassState: Option<Rc<RefCell<MultipassState>>>) {
        jcall!(self.openPriorityQueue).clear();
        jcall!(self.closedSet).clear();
        let mut discoveryCalculation: i64 = 0_i64;
        let mut node: Option<Rc<RefCell<PathFindingNodeCost>>> = None;
        let mut targetColumn: i32 = jcall!(target).getColumn();
        let mut targetRow: i32 = jcall!(target).getRow();
        let mut sizeX: i32 = (self.costArray.len() as i32).clone();
        let mut sizeY: i32 = (self.costArray[(0) as usize].len() as i32).clone();
        for column in (0..sizeX) {
            for row in (0..sizeY) {
                discoveryCalculation = { let java_value = (((jcall!(self.mathUtil).abs(column - targetColumn)) as i64) + ((jcall!(self.mathUtil).abs(row - targetRow)) as i64)).clone(); java_value };
                node = { let java_value = (self.costArray[(column) as usize][(row) as usize]).clone(); java_value }.clone();
                jcall!({ let java_field = jcall!(node).pathFindingNodeCostInfoP.clone(); java_field }).totalCostP = 0_i64;
                jcall!({ let java_field = jcall!(node).pathFindingNodeCostInfoP.clone(); java_field }).costToEndP = discoveryCalculation.clone();
            }
        }
        let mut startNode: Option<Rc<RefCell<PathFindingNodeCost>>> = self.costArray[(jcall!(start).getColumn()) as usize][(jcall!(start).getRow()) as usize].clone();
        jcall!(self.openPriorityQueue).add(startNode.clone());
        jcall!(multipassState).step += 1;
    }
    

    // throws Exception
    pub fn findPathEnd(&mut self, mut start: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>, mut target: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>, mut multipassState: Option<Rc<RefCell<MultipassState>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>> = { let java_object = (self.geographicMapInterface).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>>().expect("ClassCastException").clone()) };
        let mut basicGeographicMapCellPositionFactory: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMapCellPositionFactory::BasicGeographicMapCellPositionFactoryDyn>>> = jcall!(geographicMapInterface).getGeographicMapCellPositionFactory();
        let mut allBinaryTiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>> = jcall!(geographicMapInterface).getAllBinaryTiledLayer();
        let mut targetNode: Option<Rc<RefCell<PathFindingNodeCost>>> = self.costArray[(jcall!(target).getColumn()) as usize][(jcall!(target).getRow()) as usize].clone();
        let mut current: Option<Rc<RefCell<PathFindingNodeCost>>> = None;
        let mut total: i32 = 0;
        loop {
            current = { let java_value = (jcall!(self.openPriorityQueue).poll()).clone(); java_value };
            jcall!(self.closedSet).add(current.clone());
            if jcall!(current).equals(targetNode.clone()) {
                return self.extractPath(start.clone(), current.clone());
            }
            let mut neighbor: Option<Rc<RefCell<PathFindingNodeCost>>> = None;
            let mut neighborInfo: Option<Rc<RefCell<PathFindingNodeCostInfo>>> = None;
            let mut calculatedCost: i64 = 0_i64;
            for column in ({ let java_bound = jcall!({ let java_field = jcall!(current).geographicMapCellPosition.clone(); java_field }).getColumn() - 1; java_bound }..{ let java_bound = jcall!({ let java_field = jcall!(current).geographicMapCellPosition.clone(); java_field }).getColumn() + 2; java_bound }) {
                for row in ({ let java_bound = jcall!({ let java_field = jcall!(current).geographicMapCellPosition.clone(); java_field }).getRow() - 1; java_bound }..{ let java_bound = jcall!({ let java_field = jcall!(current).geographicMapCellPosition.clone(); java_field }).getRow() + 2; java_bound }) {
                    if column > 0 && row > 0 && column < jcall!(allBinaryTiledLayer).getColumns() && row < jcall!(allBinaryTiledLayer).getRows() && jcall!(geographicMapInterface).isOnMap(jcall!(basicGeographicMapCellPositionFactory).getAt(column.clone(), row.clone())) {
                        neighbor = { let java_value = (self.costArray[(column) as usize][(row) as usize]).clone(); java_value }.clone();
                        if jcall!(self.closedSet).contains(neighbor.clone()) {
                            continue;
                        }
                        neighborInfo = { let java_value = ({ let java_field = jcall!(neighbor).pathFindingNodeCostInfoP.clone(); java_field }).clone(); java_value }.clone();
                        calculatedCost = { let java_value = ({ let java_field = jcall!(neighborInfo).costToEndP.clone(); java_field } + { let java_field = jcall!(neighborInfo).costFromStartP.clone(); java_field } + { let java_field = jcall!({ let java_field = jcall!(current).pathFindingNodeCostInfoP.clone(); java_field }).totalCostP.clone(); java_field }).clone(); java_value };
                        if calculatedCost < { let java_field = jcall!(neighborInfo).totalCostP.clone(); java_field } || !jcall!(self.openPriorityQueue).contains(neighbor.clone()) {
                            jcall!(neighborInfo).totalCostP = calculatedCost.clone();
                            jcall!(neighbor).parent = crate::java_into_object(current.clone());
                            if !jcall!(self.openPriorityQueue).contains(neighbor.clone()) {
                                if jcall!(geographicMapInterface).isOfFourDirections({ let java_field = jcall!(current).geographicMapCellPosition.clone(); java_field }.clone(), { let java_field = jcall!(neighbor).geographicMapCellPosition.clone(); java_field }.clone()) {
                                    jcall!(self.openPriorityQueue).add(neighbor.clone());
                                }
                            }
                        }
                    }
                }
            }
            total += 1;
            if total > 10 {
                return jcall!(self.basicArrayListUtil).getImmutableInstance();
            }
            if !(!jcall!(self.openPriorityQueue).isEmpty()) { break; }
        }
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn extractPath(&mut self, mut start: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>, mut current: Option<Rc<RefCell<PathFindingNodeCost>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut path: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        while !match (&{ let java_field = jcall!(current).parent.clone(); java_field }, &{ let java_field = jcall!(NullUtil::getInstance()).NULL_OBJECT.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            jcall!(path).add(crate::java_into_object({ let java_field = jcall!(current).geographicMapCellPosition.clone(); java_field }.clone()));
            current = { let java_value = ({ let java_object = ({ let java_field = jcall!(current).parent.clone(); java_field }).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<PathFindingNodeCost>>>>().expect("ClassCastException").clone()) }).clone(); java_value };
        }
        jcall!(path).add(crate::java_into_object(start.clone()));
        jcall!(self.basicArrayListUtil).reverse(path.clone());
        return path.clone();
    }
}
impl std::ops::Deref for PathFinder {
    type Target = GeographicPathFinderBase;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for PathFinder {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::media::graphics::geography::pathfinding::GeographicPathFinderBase::GeographicPathFinderBaseDyn for PathFinder {
    fn as_GeographicPathFinderBase(&self) -> &crate::org::allbinary::media::graphics::geography::pathfinding::GeographicPathFinderBase::GeographicPathFinderBase { &self.base }
    fn as_GeographicPathFinderBase_mut(&mut self) -> &mut crate::org::allbinary::media::graphics::geography::pathfinding::GeographicPathFinderBase::GeographicPathFinderBase { &mut self.base }
    fn searchTotalPath(&mut self, mut startPathFindingNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut endPathFindingNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut totalPaths: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Self::searchTotalPath(self, startPathFindingNodeList, endPathFindingNodeList, totalPaths) }
    fn searchTotalPathN(&mut self, mut startPathFindingNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut endPathFindingNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut totalPaths: i32, mut multipassState: Option<Rc<RefCell<MultipassState>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Self::searchTotalPathN(self, startPathFindingNodeList, endPathFindingNodeList, totalPaths, multipassState) }
}


