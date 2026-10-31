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
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
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

//1.4.2
pub struct BasePathFinderGraphVisitor {
    pub base: PathFinderGraphVisitorBase,
    pub geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>,
    pub edgeMinimum: i32,
    pub minPathWeight: i32,
    pub maxPathWeight: i32,
}


impl BasePathFinderGraphVisitor {
    

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
    pub fn visit(&mut self, mut graph: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut startPathFindingNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut endPathFindingNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
    }
    

    // throws Exception
    pub fn fixPath(&mut self, mut startPathFindingNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut endPathFindingNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut pathList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        let mut startPathFindingNode: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::pathfinding::PathFindingNode::PathFindingNodeDyn>>> = None;
        for index in (0..=jcall!(startPathFindingNodeList).size() - 1).rev() {
            startPathFindingNode = { let java_object = (jcall!(startPathFindingNodeList).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::pathfinding::PathFindingNode::PathFindingNodeDyn>>>>().expect("ClassCastException").clone()) };
            jcall!(pathList).remove(crate::java_into_object(jcall!(startPathFindingNode).as_PathFindingNode_mut().geographicMapCellPosition.clone()));
            jcall!(pathList).addAt(0, crate::java_into_object(jcall!(startPathFindingNode).as_PathFindingNode_mut().geographicMapCellPosition.clone()));
        }
    }
    

    pub fn isValid(&mut self, mut graphPath: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> bool {
        return false;
    }
    

    pub fn getInvalidReason(&mut self, mut graphPath: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> String {
        return jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
    }
}
impl std::ops::Deref for BasePathFinderGraphVisitor {
    type Target = PathFinderGraphVisitorBase;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for BasePathFinderGraphVisitor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait BasePathFinderGraphVisitorDyn: crate::org::allbinary::media::graphics::geography::pathfinding::PathFinderGraphVisitorBase::PathFinderGraphVisitorBaseDyn {
    fn as_BasePathFinderGraphVisitor(&self) -> &BasePathFinderGraphVisitor;
    fn as_BasePathFinderGraphVisitor_mut(&mut self) -> &mut BasePathFinderGraphVisitor;
}
impl BasePathFinderGraphVisitorDyn for BasePathFinderGraphVisitor {
    fn as_BasePathFinderGraphVisitor(&self) -> &BasePathFinderGraphVisitor { self }
    fn as_BasePathFinderGraphVisitor_mut(&mut self) -> &mut BasePathFinderGraphVisitor { self }
}

impl crate::org::allbinary::media::graphics::geography::pathfinding::PathFinderGraphVisitorBase::PathFinderGraphVisitorBaseDyn for BasePathFinderGraphVisitor {
    fn as_PathFinderGraphVisitorBase(&self) -> &crate::org::allbinary::media::graphics::geography::pathfinding::PathFinderGraphVisitorBase::PathFinderGraphVisitorBase { &self.base }
    fn as_PathFinderGraphVisitorBase_mut(&mut self) -> &mut crate::org::allbinary::media::graphics::geography::pathfinding::PathFinderGraphVisitorBase::PathFinderGraphVisitorBase { &mut self.base }
    fn visit(&mut self, mut graph: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut startPathFindingNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut endPathFindingNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { Self::visit(self, graph, startPathFindingNodeList, endPathFindingNodeList) }
    fn fixPath(&mut self, mut startPathFindingNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut endPathFindingNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut pathList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { Self::fixPath(self, startPathFindingNodeList, endPathFindingNodeList, pathList) }
    fn isValid(&mut self, mut graphPath: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> bool { Self::isValid(self, graphPath) }
    fn getInvalidReason(&mut self, mut graphPath: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> String { Self::getInvalidReason(self, graphPath) }
}


