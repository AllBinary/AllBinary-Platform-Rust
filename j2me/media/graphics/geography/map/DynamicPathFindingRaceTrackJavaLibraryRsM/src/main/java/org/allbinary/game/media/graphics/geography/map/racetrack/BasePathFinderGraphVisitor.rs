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
use crate::org::jgrapht::graph::SimpleWeightedGraph::SimpleWeightedGraph;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMap;
use crate::org::allbinary::media::graphics::geography::pathfinding::PathFinderGraphVisitorBase::PathFinderGraphVisitorBase;
use crate::org::allbinary::media::graphics::geography::pathfinding::PathFindingNode::PathFindingNode;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasePathFinderGraphVisitor<V, E> {
    pub base: PathFinderGraphVisitorBase,
    pub geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>,
    pub edgeMinimum: i32,
    pub minPathWeight: i32,
    pub maxPathWeight: i32,
}


impl<V, E> BasePathFinderGraphVisitor<V, E> {
    

    pub fn new_struct(mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>, mut edgeMinimum: i32, mut minPathWeight: i32, mut maxPathWeight: i32) -> Self {
        let mut this = Self {
            base: PathFinderGraphVisitorBase::new_struct(),
            geographicMapInterface: None,
            edgeMinimum: 0,
            minPathWeight: 0,
            maxPathWeight: 0,
        };
        this.geographicMapInterface = geographicMapInterface.clone();
        this.edgeMinimum = edgeMinimum.clone();
        this.minPathWeight = minPathWeight.clone();
        this.maxPathWeight = maxPathWeight.clone();
        this
    }

    pub fn new(mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>, mut edgeMinimum: i32, mut minPathWeight: i32, mut maxPathWeight: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(geographicMapInterface, edgeMinimum, minPathWeight, maxPathWeight))))
    }
    

    // throws Exception
    pub fn visit(&mut self, mut graph: Option<Rc<RefCell<SimpleWeightedGraph>>>, mut startPathFindingNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut endPathFindingNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
    }
    

    // throws Exception
    pub fn fixPath(&mut self, mut startPathFindingNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut endPathFindingNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut pathList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        for index in (0..=jcall!(startPathFindingNodeList).size() - 1).rev() {
            let mut startPathFindingNode: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::pathfinding::PathFindingNode::PathFindingNodeDyn>>> = { let java_object = (jcall!(startPathFindingNodeList).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::pathfinding::PathFindingNode::PathFindingNodeDyn>>>>().expect("ClassCastException").clone()) };
            jcall!(pathList).remove(crate::java_into_object(jcall!(startPathFindingNode).as_PathFindingNode_mut().geographicMapCellPosition.clone()));
            jcall!(pathList).addAt(0, crate::java_into_object(jcall!(startPathFindingNode).as_PathFindingNode_mut().geographicMapCellPosition.clone()));
        }
    }
    

    pub fn isValid(&mut self, mut graphPath: Option<Rc<RefCell<GraphPath>>>) -> bool {
        if jcall!(jcall!(graphPath).getEdgeList()).size() > self.edgeMinimum && jcall!(graphPath).getWeight() < self.maxPathWeight && jcall!(graphPath).getWeight() > self.minPathWeight {
            return true;
        } else {
            return false;
        }
    }
    

    pub fn getInvalidReason(&mut self, mut graphPath: Option<Rc<RefCell<GraphPath>>>) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        if jcall!(jcall!(graphPath).getEdgeList()).size() < self.edgeMinimum {
            jcall!(stringBuffer).append(format!("{}{}", String::from("Ignoring Small: "), jcall!(jcall!(graphPath).getEdgeList()).size()));
        }
        if jcall!(graphPath).getWeight() <= self.minPathWeight {
            jcall!(stringBuffer).append(format!("{}{}", String::from(" Low Weighted Path: "), jcall!(graphPath).getWeight()));
        }
        if jcall!(graphPath).getWeight() >= self.maxPathWeight {
            jcall!(stringBuffer).append(format!("{}{}", String::from(" To High Weighted Path: "), jcall!(graphPath).getWeight()));
        }
        return jcall!(stringBuffer).toString();
    }
}
impl<V, E> std::ops::Deref for BasePathFinderGraphVisitor<V, E> {
    type Target = PathFinderGraphVisitorBase;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl<V, E> std::ops::DerefMut for BasePathFinderGraphVisitor<V, E> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}


