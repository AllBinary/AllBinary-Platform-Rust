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
// package org.allbinary.game.layer.waypoint
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManager;
use crate::org::allbinary::game::layer::PathFindingLayerInterface::PathFindingLayerInterface;
use crate::org::allbinary::game::layer::RTSLayerEvent::RTSLayerEvent;
use crate::org::allbinary::game::layer::building::event::BuildingEventListenerInterface::BuildingEventListenerInterface;
use crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer;
use crate::org::allbinary::game::layer::unit::UnitWaypointBehavior::UnitWaypointBehavior;
use crate::org::allbinary::game::media::graphics::geography::map::racetrack::PathFindingInfoFactory::PathFindingInfoFactory;
use crate::org::allbinary::logic::communication::log::ForcedLogUtil::ForcedLogUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject;
use crate::org::allbinary::logic::util::event::EventStrings::EventStrings;
use crate::org::allbinary::media::audio::Sound::Sound;
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMap;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPosition;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellTypeFactory::GeographicMapCellTypeFactory;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCompositeInterface::GeographicMapCompositeInterface;
use crate::org::allbinary::media::graphics::geography::map::racetrack::CustomMapGenerator::CustomMapGenerator;
use crate::org::allbinary::media::graphics::geography::map::racetrack::RaceTrackGeographicMap::RaceTrackGeographicMap;
use crate::org::allbinary::media::graphics::geography::pathfinding::MultipassState::MultipassState;
use crate::org::allbinary::media::graphics::geography::pathfinding::PathFindingInfo::PathFindingInfo;
use crate::org::allbinary::media::graphics::geography::pathfinding::PathGenerator::PathGenerator;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListUtil::BasicArrayListUtil;
use super::WaypointBase::WaypointBase;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MultipassNoCacheWaypoint {
    pub base: WaypointBase,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub ownerLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>,
}

impl BuildingEventListenerInterface for MultipassNoCacheWaypoint {
    fn onBuildingEvent(&mut self, mut event: Option<Rc<RefCell<RTSLayerEvent>>>) { Self::onBuildingEvent(self, event) }
}
impl crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface for MultipassNoCacheWaypoint {
    fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { Self::onEvent(self, eventObject) }
}

impl MultipassNoCacheWaypoint {
    

    pub fn new_struct(mut ownerLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut sound: Option<Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>>) -> Self {
        let mut this = Self {
            base: WaypointBase::new_struct(sound.clone()),
            logUtil: None,
            ownerLayer: None,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        this.ownerLayer = ownerLayer.clone();
        this
    }

    pub fn new(mut ownerLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut sound: Option<Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(ownerLayer, sound))))
    }
    

    // throws Exception
    pub fn setAllBinaryGameLayerManager(&mut self, mut allBinaryGameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>) {
        self.base.setAllBinaryGameLayerManager(allBinaryGameLayerManager.clone());
    }
    

    pub fn releaseCachedPaths(&mut self) {
    }
    

    // throws Exception
    pub fn getPathFindingInfo(&mut self, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> Option<Rc<RefCell<PathFindingInfo>>> {
        let mut startGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = geographicMapCellPosition.clone();
        let mut endGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = self.getEndGeographicMapCellPosition(startGeographicMapCellPosition.clone());
        if endGeographicMapCellPosition.is_none() {
            return crate::org::allbinary::media::graphics::geography::pathfinding::PathFindingInfo::PathFindingInfo_NULL_PATH_FINDING_INFO.with(Clone::clone).clone();
        }
        if match (&startGeographicMapCellPosition, &endGeographicMapCellPosition) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            if jcall!(self.ownerLayer).shouldHandleStartSameAsEnd() {
                return crate::org::allbinary::media::graphics::geography::pathfinding::PathFindingInfo::PathFindingInfo_NULL_PATH_FINDING_INFO.with(Clone::clone).clone();
            } else {
                panic!("{:?}", format!("{}: {:?}", "Exception", (format!("{}{}", String::from("Start should not be End: "), endGeographicMapCellPosition))));
            }
        }
        let mut geographicMapCompositeInterface: Option<Rc<RefCell<dyn GeographicMapCompositeInterface>>> = (self.allBinaryGameLayerManagerP).clone() /* as GeographicMapCompositeInterface */;
        let mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>> = jcall!(geographicMapCompositeInterface).getGeographicMapInterface()[(0) as usize].clone();
        let mut raceTrackGeographicMap: Option<Rc<RefCell<RaceTrackGeographicMap>>> = (geographicMapInterface).clone() /* as RaceTrackGeographicMap */;
        let mut raceTrackGeographicMapCellTypeFactory: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellTypeFactory::GeographicMapCellTypeFactoryDyn>>> = jcall!(raceTrackGeographicMap).getGeographicMapCellTypeFactory();
        let mut customMapGenerator: Option<Rc<RefCell<CustomMapGenerator>>> = (jcall!(raceTrackGeographicMap).getCustomMapGenerator()).clone() /* as CustomMapGenerator */;
        jcall!(customMapGenerator).copyMapIntoCustomMap();
        let mut customMapArray: Vec<Vec<i32>> = jcall!(customMapGenerator).getCustomMapArray();
        customMapArray[(jcall!(startGeographicMapCellPosition).getRow()) as usize][(jcall!(startGeographicMapCellPosition).getColumn()) as usize] = jcall!(raceTrackGeographicMapCellTypeFactory).getStartType();
        customMapArray[(jcall!(endGeographicMapCellPosition).getRow()) as usize][(jcall!(endGeographicMapCellPosition).getColumn()) as usize] = jcall!(raceTrackGeographicMapCellTypeFactory).getEndType();
        let mut pathFindingInfo: Option<Rc<RefCell<PathFindingInfo>>> = jcall!(PathFindingInfoFactory::getInstance()).getInstancePathFindingInfo(raceTrackGeographicMap.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>), customMapArray.clone());
        return pathFindingInfo.clone();
    }
    

    // throws Exception
    pub fn getPathsList(&mut self, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>, mut pathFindingInfo: Option<Rc<RefCell<PathFindingInfo>>>, mut multipassState: Option<Rc<RefCell<MultipassState>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if pathFindingInfo.is_none() {
                jcall!(self.logUtil).putF(format!("{}{}", String::from("pathFindingInfo: "), pathFindingInfo), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getPathsList"));
                return jcall!(BasicArrayListUtil::getInstance()).getImmutableInstance();
            }
            let mut geographicMapCompositeInterface: Option<Rc<RefCell<dyn GeographicMapCompositeInterface>>> = (self.allBinaryGameLayerManagerP).clone() /* as GeographicMapCompositeInterface */;
            let mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>> = jcall!(geographicMapCompositeInterface).getGeographicMapInterface()[(0) as usize].clone();
            let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(PathGenerator::getInstance()).createN(geographicMapInterface.clone(), pathFindingInfo.clone(), 2, multipassState.clone());
            if list.is_some() && !match (&list, &jcall!(BasicArrayListUtil::getInstance()).getImmutableInstance()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                if jcall!(list).size() < 1 {
                    panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Path may have been cleared by clearing caching"))));
                }
                return list.clone();
            }
            return jcall!(BasicArrayListUtil::getInstance()).getImmutableInstance();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(self.logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getPathsList"), crate::java_into_object(e.clone()));
            return jcall!(BasicArrayListUtil::getInstance()).getImmutableInstance();
            }
        }
    }
    

    // throws Exception
    pub fn getEndGeographicMapCellPosition(&mut self, mut startGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> {
        let mut endList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(self.ownerLayer).getEndGeographicMapCellPositionList();
        let mut endGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = { let java_object = (jcall!(BasicArrayListUtil::getInstance()).getRandom(endList.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>>().expect("ClassCastException").clone()) };
        if match (&startGeographicMapCellPosition, &endGeographicMapCellPosition) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            for index in (0..=jcall!(endList).size() - 1).rev() {
                endGeographicMapCellPosition = { let java_object = (jcall!(endList).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>>().expect("ClassCastException").clone()) };
                if !match (&startGeographicMapCellPosition, &endGeographicMapCellPosition) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                    break;
                }
            }
        }
        return endGeographicMapCellPosition.clone();
    }
    

    pub fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) {
        ForcedLogUtil::log(jcall!(EventStrings::getInstance()).PERFORMANCE_MESSAGE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())));
    }
    

    // throws Exception
    pub fn onBuildingEvent(&mut self, mut event: Option<Rc<RefCell<RTSLayerEvent>>>) {
    }
    

    pub fn reset(&mut self) {
        jcall!(self.getConnectedWaypointList()).clear();
    }
    

    // throws Exception
    pub fn visit(&mut self, mut unitLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
        if !match (&jcall!(unitLayer).getGroupInterface()[(0) as usize], &jcall!(self.ownerLayer).getGroupInterface()[(0) as usize]) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return;
        }
        let mut size: i32 = jcall!(self.getConnectedWaypointList()).size();
        if size > 0 {
            let mut unitWaypointBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::unit::UnitWaypointBehavior::UnitWaypointBehaviorDyn>>> = (jcall!(unitLayer).getWaypointBehavior()).clone() /* as UnitWaypointBehavior */;
            while jcall!(self.getConnectedWaypointList()).size() > 0 {
                let mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>> = (jcall!(self.getConnectedWaypointList()).get(0)).clone() /* as CollidableDestroyableDamageableLayer */;
                if jcall!(rtsLayer).isDestroyed() {
                    jcall!(self.getConnectedWaypointList()).remove(rtsLayer.clone());
                } else {
                    jcall!(unitLayer).handleCost(self.ownerLayer.clone());
                    jcall!(unitWaypointBehavior).insertWaypoint(0, rtsLayer.clone());
                    break;
                }
            }
        }
    }
}
impl std::ops::Deref for MultipassNoCacheWaypoint {
    type Target = WaypointBase;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for MultipassNoCacheWaypoint {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::layer::waypoint::WaypointBase::WaypointBaseDyn for MultipassNoCacheWaypoint {
    fn as_WaypointBase(&self) -> &crate::org::allbinary::game::layer::waypoint::WaypointBase::WaypointBase { &self.base }
    fn as_WaypointBase_mut(&mut self) -> &mut crate::org::allbinary::game::layer::waypoint::WaypointBase::WaypointBase { &mut self.base }
    fn setAllBinaryGameLayerManager(&mut self, mut allBinaryGameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>) { Self::setAllBinaryGameLayerManager(self, allBinaryGameLayerManager) }
    fn getPathFindingInfo(&mut self, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> Option<Rc<RefCell<PathFindingInfo>>> { Self::getPathFindingInfo(self, geographicMapCellPosition) }
    fn getPathsList(&mut self, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>, mut pathFindingInfo: Option<Rc<RefCell<PathFindingInfo>>>, mut multipassState: Option<Rc<RefCell<MultipassState>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Self::getPathsList(self, geographicMapCellPosition, pathFindingInfo, multipassState) }
    fn visit(&mut self, mut unitLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { Self::visit(self, unitLayer) }
    fn reset(&mut self) { Self::reset(self) }
}


