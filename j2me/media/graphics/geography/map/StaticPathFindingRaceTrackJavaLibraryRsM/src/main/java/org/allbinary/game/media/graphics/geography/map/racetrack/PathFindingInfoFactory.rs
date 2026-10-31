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
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMap;
use crate::org::allbinary::media::graphics::geography::map::racetrack::BasePathFindingInfoFactory::BasePathFindingInfoFactory;
use crate::org::allbinary::media::graphics::geography::map::racetrack::RaceTrackRoadsGeographicMapCellHistoryFactory::RaceTrackRoadsGeographicMapCellHistoryFactory;
use crate::org::allbinary::media::graphics::geography::pathfinding::PathFindingInfo::PathFindingInfo;
use crate::org::allbinary::media::graphics::geography::pathfinding::PathFindingNodeCostInfoFactoryBase::PathFindingNodeCostInfoFactoryBase;
use crate::org::allbinary::util::BasicArrayListS::BasicArrayListS;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PathFindingInfoFactory {
    pub base: BasePathFindingInfoFactory,
}

thread_local! { pub static PathFindingInfoFactory_instance: Option<Rc<RefCell<PathFindingInfoFactory>>> = PathFindingInfoFactory::new(); }

impl PathFindingInfoFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<PathFindingInfoFactory>>> {
        return PathFindingInfoFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: BasePathFindingInfoFactory::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn getInstancePathFindingInfo(&mut self, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>, mut graphArray: Vec<Vec<i32>>) -> Option<Rc<RefCell<PathFindingInfo>>> {
        let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
        PreLogUtil::put(jcall!(commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).GET_INSTANCE.clone());
        let mut pathFindingInfo: Option<Rc<RefCell<PathFindingInfo>>> = PathFindingInfo::new(PathFindingNodeCostInfoFactoryBase::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::pathfinding::PathFindingNodeCostInfoFactoryBase::PathFindingNodeCostInfoFactoryBaseDyn>>), BasicArrayListS::new(1).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayListS::BasicArrayListSDyn>>), BasicArrayListS::new(1).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayListS::BasicArrayListSDyn>>));
        jcall!(RaceTrackRoadsGeographicMapCellHistoryFactory::getInstance()).init();
        self.init(geographicMapInterface.clone(), pathFindingInfo.clone(), graphArray.clone());
        return pathFindingInfo.clone();
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
    fn getInstancePathFindingInfo(&mut self, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>, mut graphArray: Vec<Vec<i32>>) -> Option<Rc<RefCell<PathFindingInfo>>> { Self::getInstancePathFindingInfo(self, geographicMapInterface, graphArray) }
}


