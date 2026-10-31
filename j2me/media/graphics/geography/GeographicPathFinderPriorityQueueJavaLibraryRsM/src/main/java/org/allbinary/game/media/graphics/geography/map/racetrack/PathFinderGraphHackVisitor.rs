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
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMap;
use super::BasePathFinderGraphVisitor::BasePathFinderGraphVisitor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//1.4.2
pub struct PathFinderGraphHackVisitor {
    pub base: BasePathFinderGraphVisitor,
}


impl PathFinderGraphHackVisitor {
    

    pub fn new_struct(mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>, mut edgeMinimum: i32, mut minPathWeight: i32, mut maxPathWeight: i32) -> Self {
        let mut this = Self {
            base: BasePathFinderGraphVisitor::new_struct(geographicMapInterface.clone(), edgeMinimum.clone(), minPathWeight.clone(), maxPathWeight.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>, mut edgeMinimum: i32, mut minPathWeight: i32, mut maxPathWeight: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(geographicMapInterface, edgeMinimum, minPathWeight, maxPathWeight))))
    }
    

    // throws Exception
    pub fn visit(&mut self, mut graph: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut startPathFindingNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut endPathFindingNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        self.fixStart(graph.clone(), startPathFindingNodeList.clone());
        self.fixEnd(graph.clone(), endPathFindingNodeList.clone());
        self.fixOverPassEdges(graph.clone());
    }
    

    // throws Exception
    pub fn fixStart(&mut self, mut graph: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut startPathFindingNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
    }
    

    // throws Exception
    pub fn fixEnd(&mut self, mut graph: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut endPathFindingNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
    }
    

    // throws Exception
    pub fn fixOverPassEdges(&mut self, mut graph: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) {
    }
    

    // throws Exception
    pub fn fixPath(&mut self, mut startPathFindingNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut endPathFindingNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut pathList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
    }
    

    // throws Exception
    pub fn removeOverPassEdges(&mut self, mut pathList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
    }
    

    pub fn isValid(&mut self, mut graphPath: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> bool {
        return false;
    }
}
impl std::ops::Deref for PathFinderGraphHackVisitor {
    type Target = BasePathFinderGraphVisitor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for PathFinderGraphHackVisitor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::media::graphics::geography::map::racetrack::BasePathFinderGraphVisitor::BasePathFinderGraphVisitorDyn for PathFinderGraphHackVisitor {
    fn as_BasePathFinderGraphVisitor(&self) -> &crate::org::allbinary::game::media::graphics::geography::map::racetrack::BasePathFinderGraphVisitor::BasePathFinderGraphVisitor { &self.base }
    fn as_BasePathFinderGraphVisitor_mut(&mut self) -> &mut crate::org::allbinary::game::media::graphics::geography::map::racetrack::BasePathFinderGraphVisitor::BasePathFinderGraphVisitor { &mut self.base }
}
impl crate::org::allbinary::media::graphics::geography::pathfinding::PathFinderGraphVisitorBase::PathFinderGraphVisitorBaseDyn for PathFinderGraphHackVisitor {
    fn as_PathFinderGraphVisitorBase(&self) -> &crate::org::allbinary::media::graphics::geography::pathfinding::PathFinderGraphVisitorBase::PathFinderGraphVisitorBase { &self.base.base }
    fn as_PathFinderGraphVisitorBase_mut(&mut self) -> &mut crate::org::allbinary::media::graphics::geography::pathfinding::PathFinderGraphVisitorBase::PathFinderGraphVisitorBase { &mut self.base.base }
    fn visit(&mut self, mut graph: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut startPathFindingNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut endPathFindingNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { Self::visit(self, graph, startPathFindingNodeList, endPathFindingNodeList) }
    fn fixPath(&mut self, mut startPathFindingNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut endPathFindingNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut pathList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { Self::fixPath(self, startPathFindingNodeList, endPathFindingNodeList, pathList) }
    fn isValid(&mut self, mut graphPath: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> bool { Self::isValid(self, graphPath) }
    fn getInvalidReason(&mut self, p1: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> String { crate::org::allbinary::game::media::graphics::geography::map::racetrack::BasePathFinderGraphVisitor::BasePathFinderGraphVisitor::getInvalidReason(crate::org::allbinary::game::media::graphics::geography::map::racetrack::BasePathFinderGraphVisitor::BasePathFinderGraphVisitorDyn::as_BasePathFinderGraphVisitor_mut(self), p1) }
}


