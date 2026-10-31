//
// AllBinary Open License Version 1
// Copyright (c) 2006 AllBinary
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
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPosition;
use super::PathFindingNodeCostInfoFactoryBase::PathFindingNodeCostInfoFactoryBase;
use super::PathFindingNodeCostInfo::PathFindingNodeCostInfo;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PathFindingNodeCostInfoFactory {
    pub base: PathFindingNodeCostInfoFactoryBase,
    pub pathFindingNodeCostInfoAdjacencyList: Vec<Vec<Option<Rc<RefCell<PathFindingNodeCostInfo>>>>>,
}


impl PathFindingNodeCostInfoFactory {
    

    pub fn new_struct(mut max: i32) -> Self {
        let mut this = Self {
            base: PathFindingNodeCostInfoFactoryBase::new_struct(),
            pathFindingNodeCostInfoAdjacencyList: Vec::new(),
        };
        this.pathFindingNodeCostInfoAdjacencyList = vec![vec![None; (max) as usize]; (max) as usize];
        this
    }

    pub fn new(mut max: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(max))))
    }
    

    // throws Exception
    pub fn create(&mut self, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>, mut goingToGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>, mut costFromStart: i64, mut costToEnd: i64) {
        self.getInstance4(goingToGeographicMapCellPosition.clone(), geographicMapCellPosition.clone(), costFromStart.clone(), costToEnd.clone());
    }
    

    // throws Exception
    pub fn getInstance4(&mut self, mut goingToGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>, mut costFromStart: i64, mut costToEnd: i64) -> Option<Rc<RefCell<PathFindingNodeCostInfo>>> {
        let mut pathFindingNodeCostInfo: Option<Rc<RefCell<PathFindingNodeCostInfo>>> = self.getInstance(goingToGeographicMapCellPosition.clone(), geographicMapCellPosition.clone());
        if pathFindingNodeCostInfo.is_none() {
            pathFindingNodeCostInfo = PathFindingNodeCostInfo::new(costFromStart.clone(), costToEnd.clone());
            jcall!(pathFindingNodeCostInfo).setCalculatedTotalCost();
            self.pathFindingNodeCostInfoAdjacencyList[(jcall!(geographicMapCellPosition).getId()) as usize][(jcall!(goingToGeographicMapCellPosition).getId()) as usize] = pathFindingNodeCostInfo.clone();
        } else {
            jcall!(pathFindingNodeCostInfo).setCostFromStart(costFromStart.clone());
            jcall!(pathFindingNodeCostInfo).setCostToEnd(costToEnd.clone());
            jcall!(pathFindingNodeCostInfo).setCalculatedTotalCost();
        }
        return pathFindingNodeCostInfo.clone();
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn getInstance(&mut self, mut goingToFromGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> Option<Rc<RefCell<PathFindingNodeCostInfo>>> {
        return self.pathFindingNodeCostInfoAdjacencyList[(jcall!(geographicMapCellPosition).getId()) as usize][(jcall!(goingToFromGeographicMapCellPosition).getId()) as usize].clone();
    }
    

    pub fn getTotalCost(&mut self, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>, mut comingFromGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> i64 {
        return jcall!(self.getInstance(comingFromGeographicMapCellPosition.clone(), geographicMapCellPosition.clone())).totalCostP.clone();
    }
}
impl std::ops::Deref for PathFindingNodeCostInfoFactory {
    type Target = PathFindingNodeCostInfoFactoryBase;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for PathFindingNodeCostInfoFactory {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::media::graphics::geography::pathfinding::PathFindingNodeCostInfoFactoryBase::PathFindingNodeCostInfoFactoryBaseDyn for PathFindingNodeCostInfoFactory {
    fn as_PathFindingNodeCostInfoFactoryBase(&self) -> &crate::org::allbinary::media::graphics::geography::pathfinding::PathFindingNodeCostInfoFactoryBase::PathFindingNodeCostInfoFactoryBase { &self.base }
    fn as_PathFindingNodeCostInfoFactoryBase_mut(&mut self) -> &mut crate::org::allbinary::media::graphics::geography::pathfinding::PathFindingNodeCostInfoFactoryBase::PathFindingNodeCostInfoFactoryBase { &mut self.base }
}
impl crate::org::allbinary::media::graphics::geography::pathfinding::PathFindingNodeCostInfoFactoryBaseInterface::PathFindingNodeCostInfoFactoryBaseInterface for PathFindingNodeCostInfoFactory {
    fn create(&mut self, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>, mut goingToGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>, mut costFromStart: i64, mut costToEnd: i64) { Self::create(self, geographicMapInterface, goingToGeographicMapCellPosition, geographicMapCellPosition, costFromStart, costToEnd) }
    fn getTotalCost(&mut self, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>, mut comingFromGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> i64 { Self::getTotalCost(self, geographicMapInterface, comingFromGeographicMapCellPosition, geographicMapCellPosition) }
}


