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
use crate::org::jgrapht::GraphPath::GraphPath;
use crate::org::jgrapht::graph::DefaultWeightedEdge::DefaultWeightedEdge;
use crate::org::jgrapht::graph::SimpleWeightedGraph::SimpleWeightedGraph;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayer;
use crate::org::allbinary::graphics::CellPosition::CellPosition;
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMap;
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMapCellPositionFactory::BasicGeographicMapCellPositionFactory;
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMapUtil::BasicGeographicMapUtil;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPosition;
use crate::org::allbinary::media::graphics::geography::map::racetrack::RaceTrackGeographicMap::RaceTrackGeographicMap;
use crate::org::allbinary::media::graphics::geography::pathfinding::PathFindingNode::PathFindingNode;
use super::BasePathFinderGraphVisitor::BasePathFinderGraphVisitor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PathFinderGraphHackVisitor<V, E> {
    pub base: BasePathFinderGraphVisitor<V, E>,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}


impl<V, E> PathFinderGraphHackVisitor<V, E> {
    

    pub fn new_struct(mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>, mut edgeMinimum: i32, mut minPathWeight: i32, mut maxPathWeight: i32) -> Self {
        let mut this = Self {
            base: BasePathFinderGraphVisitor::new_struct(geographicMapInterface.clone(), edgeMinimum.clone(), minPathWeight.clone(), maxPathWeight.clone()),
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>, mut edgeMinimum: i32, mut minPathWeight: i32, mut maxPathWeight: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(geographicMapInterface, edgeMinimum, minPathWeight, maxPathWeight))))
    }
    

    // throws Exception
    pub fn visit(&mut self, mut graph: Option<Rc<RefCell<SimpleWeightedGraph>>>, mut startPathFindingNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut endPathFindingNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        self.fixStart(graph.clone(), startPathFindingNodeList.clone());
        self.fixEnd(graph.clone(), endPathFindingNodeList.clone());
        self.fixOverPassEdges(graph.clone());
    }
    

    // throws Exception
    pub fn fixStart(&mut self, mut graph: Option<Rc<RefCell<SimpleWeightedGraph<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>>, Option<Rc<RefCell<DefaultWeightedEdge>>>>>>>, mut startPathFindingNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        let mut geographicMapCellPositionFactory: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMapCellPositionFactory::BasicGeographicMapCellPositionFactoryDyn>>> = jcall!(self.geographicMapInterface).getGeographicMapCellPositionFactory();
        let mut tiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>> = jcall!(self.geographicMapInterface).getAllBinaryTiledLayer();
        for index in (0..=jcall!(startPathFindingNodeList).size() - 1).rev() {
            let mut startPathFindingNode: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::pathfinding::PathFindingNode::PathFindingNodeDyn>>> = { let java_object = (jcall!(startPathFindingNodeList).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::pathfinding::PathFindingNode::PathFindingNodeDyn>>>>().expect("ClassCastException").clone()) };
            let mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = jcall!(startPathFindingNode).as_PathFindingNode_mut().geographicMapCellPosition.clone();
            let mut column: i32 = jcall!(geographicMapCellPosition).getColumn();
            let mut row: i32 = jcall!(geographicMapCellPosition).getRow();
            let mut nextRow: i32 = row + 1;
            if jcall!(tiledLayer).isOnTileLayer(column.clone(), nextRow.clone()) {
                let mut geographicMapCellPositionNeighbor: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = jcall!(geographicMapCellPositionFactory).getAt(column.clone(), nextRow.clone());
                jcall!(graph).removeEdge(geographicMapCellPosition.clone(), geographicMapCellPositionNeighbor.clone());
            }
        }
    }
    

    // throws Exception
    pub fn fixEnd(&mut self, mut graph: Option<Rc<RefCell<SimpleWeightedGraph<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>>, Option<Rc<RefCell<DefaultWeightedEdge>>>>>>>, mut endPathFindingNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        let mut geographicMapCellPositionFactory: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMapCellPositionFactory::BasicGeographicMapCellPositionFactoryDyn>>> = jcall!(self.geographicMapInterface).getGeographicMapCellPositionFactory();
        let mut tiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>> = jcall!(self.geographicMapInterface).getAllBinaryTiledLayer();
        for index in (0..=jcall!(endPathFindingNodeList).size() - 1).rev() {
            let mut endPathFindingNode: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::pathfinding::PathFindingNode::PathFindingNodeDyn>>> = { let java_object = (jcall!(endPathFindingNodeList).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::pathfinding::PathFindingNode::PathFindingNodeDyn>>>>().expect("ClassCastException").clone()) };
            let mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = jcall!(endPathFindingNode).as_PathFindingNode_mut().geographicMapCellPosition.clone();
            jcall!(graph).addVertex(geographicMapCellPosition.clone());
            let mut column: i32 = jcall!(geographicMapCellPosition).getColumn();
            let mut row: i32 = jcall!(geographicMapCellPosition).getRow();
            let mut nextRow: i32 = row + 1;
            if jcall!(tiledLayer).isOnTileLayer(column.clone(), nextRow.clone()) {
                let mut geographicMapCellPositionNeighbor: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = jcall!(geographicMapCellPositionFactory).getAt(column.clone(), nextRow.clone());
                jcall!(graph).addEdge(geographicMapCellPosition.clone(), geographicMapCellPositionNeighbor.clone());
            }
        }
    }
    

    // throws Exception
    pub fn fixOverPassEdges(&mut self, mut graph: Option<Rc<RefCell<SimpleWeightedGraph<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>>, Option<Rc<RefCell<DefaultWeightedEdge>>>>>>>) {
        let mut geographicMapCellPositionFactory: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMapCellPositionFactory::BasicGeographicMapCellPositionFactoryDyn>>> = jcall!(self.geographicMapInterface).getGeographicMapCellPositionFactory();
        let mut raceTrackGeographicMap: Option<Rc<RefCell<RaceTrackGeographicMap>>> = (self.geographicMapInterface).clone() /* as RaceTrackGeographicMap */;
        let mut CellPositionArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>>> = jcall!(jcall!(raceTrackGeographicMap).getRaceTrackData()).getOverPassGeographicMapCellPositionArray();
        for index in (0..=(CellPositionArray.len() as i32) - 1).rev() {
            let mut overPassGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>> = CellPositionArray[(index) as usize].clone();
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", format!("{}{}", format!("{}{}", String::from("Fixing Over Pass: "), jcall!(overPassGeographicMapCellPosition).getColumn()), String::from(", ")), jcall!(overPassGeographicMapCellPosition).getRow()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("fixOverPassEdges"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            let mut underPassGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = jcall!(geographicMapCellPositionFactory).getAt(jcall!(overPassGeographicMapCellPosition).getColumn(), jcall!(overPassGeographicMapCellPosition).getRow());
            let mut rightUnderPassGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = jcall!(geographicMapCellPositionFactory).getAt(jcall!(overPassGeographicMapCellPosition).getColumn() + 1, jcall!(overPassGeographicMapCellPosition).getRow());
            let mut leftUnderPassGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = jcall!(geographicMapCellPositionFactory).getAt(jcall!(overPassGeographicMapCellPosition).getColumn() - 1, jcall!(overPassGeographicMapCellPosition).getRow());
            jcall!(graph).removeEdge(underPassGeographicMapCellPosition.clone(), rightUnderPassGeographicMapCellPosition.clone());
            jcall!(graph).removeEdge(underPassGeographicMapCellPosition.clone(), leftUnderPassGeographicMapCellPosition.clone());
            jcall!(graph).addVertex(overPassGeographicMapCellPosition.clone());
            jcall!(graph).addEdge(overPassGeographicMapCellPosition.clone(), rightUnderPassGeographicMapCellPosition.clone());
            jcall!(graph).addEdge(overPassGeographicMapCellPosition.clone(), leftUnderPassGeographicMapCellPosition.clone());
        }
    }
    

    // throws Exception
    pub fn fixPath(&mut self, mut startPathFindingNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut endPathFindingNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut pathList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        for index in (0..=jcall!(startPathFindingNodeList).size() - 1).rev() {
            let mut endPathFindingNode: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::pathfinding::PathFindingNode::PathFindingNodeDyn>>> = { let java_object = (jcall!(endPathFindingNodeList).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::pathfinding::PathFindingNode::PathFindingNodeDyn>>>>().expect("ClassCastException").clone()) };
            let mut startPathFindingNode: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::pathfinding::PathFindingNode::PathFindingNodeDyn>>> = { let java_object = (jcall!(startPathFindingNodeList).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::pathfinding::PathFindingNode::PathFindingNodeDyn>>>>().expect("ClassCastException").clone()) };
            if jcall!(BasicGeographicMapUtil::getInstance()).isSameCellPosition(jcall!(startPathFindingNode).as_PathFindingNode_mut().geographicMapCellPosition.clone(), jcall!(endPathFindingNode).as_PathFindingNode_mut().geographicMapCellPosition.clone()) {
                if jcall!(pathList).remove(crate::java_into_object(jcall!(endPathFindingNode).as_PathFindingNode_mut().geographicMapCellPosition.clone())) {
                    jcall!(pathList).remove(crate::java_into_object(jcall!(startPathFindingNode).as_PathFindingNode_mut().geographicMapCellPosition.clone()));
                    jcall!(pathList).addAt(0, crate::java_into_object(jcall!(startPathFindingNode).as_PathFindingNode_mut().geographicMapCellPosition.clone()));
                }
            }
        }
        self.removeOverPassEdges(pathList.clone());
    }
    

    // throws Exception
    pub fn removeOverPassEdges(&mut self, mut pathList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        let mut geographicMapCellPositionFactory: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMapCellPositionFactory::BasicGeographicMapCellPositionFactoryDyn>>> = jcall!(self.geographicMapInterface).getGeographicMapCellPositionFactory();
        let mut raceTrackGeographicMap: Option<Rc<RefCell<RaceTrackGeographicMap>>> = (self.geographicMapInterface).clone() /* as RaceTrackGeographicMap */;
        let mut CellPositionArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>>> = jcall!(jcall!(raceTrackGeographicMap).getRaceTrackData()).getOverPassGeographicMapCellPositionArray();
        for index in (0..=(CellPositionArray.len() as i32) - 1).rev() {
            let mut overPassGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>> = CellPositionArray[(index) as usize].clone();
            let mut underPassGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = jcall!(geographicMapCellPositionFactory).getAt(jcall!(overPassGeographicMapCellPosition).getColumn(), jcall!(overPassGeographicMapCellPosition).getRow());
            let mut indexOf: i32 = jcall!(pathList).indexOf(crate::java_into_object(overPassGeographicMapCellPosition.clone()));
            if indexOf != -1 {
                jcall!(pathList).set(indexOf.clone(), crate::java_into_object(underPassGeographicMapCellPosition.clone()));
            }
        }
    }
    

    pub fn isValid(&mut self, mut graphPath: Option<Rc<RefCell<GraphPath>>>) -> bool {
        if jcall!(jcall!(graphPath).getEdgeList()).size() > self.base.edgeMinimum && jcall!(graphPath).getWeight() < self.base.maxPathWeight && jcall!(graphPath).getWeight() > self.base.minPathWeight {
            return true;
        } else {
            return false;
        }
    }
}
impl<V, E> std::ops::Deref for PathFinderGraphHackVisitor<V, E> {
    type Target = BasePathFinderGraphVisitor<V, E>;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl<V, E> std::ops::DerefMut for PathFinderGraphHackVisitor<V, E> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}


