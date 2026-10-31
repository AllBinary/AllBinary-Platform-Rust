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
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListUtil::BasicArrayListUtil;
use super::PathFindingInfoInterface::PathFindingInfoInterface;
use super::GeographicPathFinderBase::GeographicPathFinderBase;
use super::PathFindingNodeCostInfoFactoryBase::PathFindingNodeCostInfoFactoryBase;
use super::PathFindingNode::PathFindingNode;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PathFindingInfo {
    pub pathFinder: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::pathfinding::GeographicPathFinderBase::GeographicPathFinderBaseDyn>>>,
    pub startPathFindingNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub endPathFindingNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub pathFindingNodeCostInfoFactoryInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::pathfinding::PathFindingNodeCostInfoFactoryBase::PathFindingNodeCostInfoFactoryBaseDyn>>>,
}

thread_local! { pub static PathFindingInfo_NULL_PATH_FINDING_INFO: Option<Rc<RefCell<PathFindingInfo>>> = PathFindingInfo::new(PathFindingNodeCostInfoFactoryBase_NULL_PATH_FINDING_NODE_COST_INFO_FACTORY_BASE.with(Clone::clone).clone(), jcall!(BasicArrayListUtil::getInstance()).getImmutableInstance(), jcall!(BasicArrayListUtil::getInstance()).getImmutableInstance()); }
impl PathFindingInfoInterface for PathFindingInfo {
    fn getStartPathFindingNodeList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Self::getStartPathFindingNodeList(self) }
    fn getEndPathFindingNodeList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Self::getEndPathFindingNodeList(self) }
    fn getPathFinder(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::pathfinding::GeographicPathFinderBase::GeographicPathFinderBaseDyn>>> { Self::getPathFinder(self) }
}

impl PathFindingInfo {
    

    pub fn new_struct(mut pathFindingNodeCostInfoFactoryInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::pathfinding::PathFindingNodeCostInfoFactoryBase::PathFindingNodeCostInfoFactoryBaseDyn>>>, mut startPathFindingNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut endPathFindingNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Self {
        let mut this = Self {
            pathFinder: None,
            startPathFindingNodeList: None,
            endPathFindingNodeList: None,
            pathFindingNodeCostInfoFactoryInterface: None,
        };
        this.pathFinder = GeographicPathFinderBase_NULL_GEOGRAPHIC_PATH_FINDER_BASE.with(Clone::clone).clone();
        this.pathFindingNodeCostInfoFactoryInterface = pathFindingNodeCostInfoFactoryInterface.clone();
        this.startPathFindingNodeList = startPathFindingNodeList.clone();
        this.endPathFindingNodeList = endPathFindingNodeList.clone();
        this
    }

    pub fn new(mut pathFindingNodeCostInfoFactoryInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::pathfinding::PathFindingNodeCostInfoFactoryBase::PathFindingNodeCostInfoFactoryBaseDyn>>>, mut startPathFindingNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut endPathFindingNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(pathFindingNodeCostInfoFactoryInterface, startPathFindingNodeList, endPathFindingNodeList))))
    }
    

    pub fn getStartPathFindingNodeList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.startPathFindingNodeList.clone();
    }
    

    pub fn getEndPathFindingNodeList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.endPathFindingNodeList.clone();
    }
    

    pub fn getPathFinder(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::pathfinding::GeographicPathFinderBase::GeographicPathFinderBaseDyn>>> {
        return self.pathFinder.clone();
    }
    

    pub fn addStartPathFindingNode(&mut self, mut startPathFindingNode: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::pathfinding::PathFindingNode::PathFindingNodeDyn>>>) {
        jcall!(self.startPathFindingNodeList).add(crate::java_into_object(startPathFindingNode.clone()));
    }
    

    pub fn addEndPathFindingNode(&mut self, mut endPathFindingNode: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::pathfinding::PathFindingNode::PathFindingNodeDyn>>>) {
        jcall!(self.endPathFindingNodeList).add(crate::java_into_object(endPathFindingNode.clone()));
    }
    

    pub fn setPathFinder(&mut self, mut pathFinder: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::pathfinding::GeographicPathFinderBase::GeographicPathFinderBaseDyn>>>) {
        self.pathFinder = pathFinder.clone();
    }
    

    pub fn getPathFindingNodeCostInfoFactoryInterface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::pathfinding::PathFindingNodeCostInfoFactoryBase::PathFindingNodeCostInfoFactoryBaseDyn>>> {
        return self.pathFindingNodeCostInfoFactoryInterface.clone();
    }
}

