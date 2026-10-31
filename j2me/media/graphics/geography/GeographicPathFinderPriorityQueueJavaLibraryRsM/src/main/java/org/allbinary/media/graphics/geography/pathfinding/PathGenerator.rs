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
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMap;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellHistory::GeographicMapCellHistory;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use super::PathGeneratorInterface::PathGeneratorInterface;
use super::PathFindingInfo::PathFindingInfo;
use super::GeographicPathFinderBase::GeographicPathFinderBase;
use super::MultipassState::MultipassState;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PathGenerator {
}

thread_local! { pub static PathGenerator_SINGLETON: Option<Rc<RefCell<PathGenerator>>> = PathGenerator::new(); }
impl PathGeneratorInterface for PathGenerator {
    fn createList(&mut self, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>, mut geographicMapCellHistory: Option<Rc<RefCell<GeographicMapCellHistory>>>, mut pathFindingInfo: Option<Rc<RefCell<PathFindingInfo>>>, mut totalPaths: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Self::createList(self, geographicMapInterface, geographicMapCellHistory, pathFindingInfo, totalPaths) }
}

impl PathGenerator {
    

    pub fn getInstance() -> Option<Rc<RefCell<PathGenerator>>> {
        return PathGenerator_SINGLETON.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn init(&mut self, mut geographicMapInterface: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut totalPaths: i32) {
    }
    

    // throws Exception
    pub fn getInstanceNoCache(&mut self, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>, mut pathFindingInfo: Option<Rc<RefCell<PathFindingInfo>>>, mut totalPaths: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut geographicMapCellPositionBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.create(geographicMapInterface.clone(), pathFindingInfo.clone(), totalPaths.clone());
        return geographicMapCellPositionBasicArrayList.clone();
    }
    

    // throws Exception
    pub fn createList(&mut self, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>, mut geographicMapCellHistory: Option<Rc<RefCell<GeographicMapCellHistory>>>, mut pathFindingInfo: Option<Rc<RefCell<PathFindingInfo>>>, mut totalPaths: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // throws Exception
    pub fn create(&mut self, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>, mut pathFindingInfo: Option<Rc<RefCell<PathFindingInfo>>>, mut totalPaths: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut startPathFindingNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(pathFindingInfo).getStartPathFindingNodeList();
        let mut endPathFindingNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(pathFindingInfo).getEndPathFindingNodeList();
        let mut geographicPathFinderInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::pathfinding::GeographicPathFinderBase::GeographicPathFinderBaseDyn>>> = jcall!(pathFindingInfo).getPathFinder();
        let mut geographicMapCellPositionBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(geographicPathFinderInterface).searchTotalPath(startPathFindingNodeList.clone(), endPathFindingNodeList.clone(), totalPaths.clone());
        return geographicMapCellPositionBasicArrayList.clone();
    }
    

    // throws Exception
    pub fn createN(&mut self, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>, mut pathFindingInfo: Option<Rc<RefCell<PathFindingInfo>>>, mut totalPaths: i32, mut multipassState: Option<Rc<RefCell<MultipassState>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut startPathFindingNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(pathFindingInfo).getStartPathFindingNodeList();
        let mut endPathFindingNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(pathFindingInfo).getEndPathFindingNodeList();
        let mut geographicPathFinderInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::pathfinding::GeographicPathFinderBase::GeographicPathFinderBaseDyn>>> = jcall!(pathFindingInfo).getPathFinder();
        let mut geographicMapCellPositionBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(geographicPathFinderInterface).searchTotalPathN(startPathFindingNodeList.clone(), endPathFindingNodeList.clone(), totalPaths.clone(), multipassState.clone());
        return geographicMapCellPositionBasicArrayList.clone();
    }
    

    // throws Exception
    pub fn isValid(&mut self, mut pathList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut geographicMapCellHistory: Option<Rc<RefCell<GeographicMapCellHistory>>>) -> bool {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
}

