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
use crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayer;
use crate::org::allbinary::game::layer::PathFindingLayerInterface::PathFindingLayerInterface;
use crate::org::allbinary::game::layer::RTSLayerEvent::RTSLayerEvent;
use crate::org::allbinary::game::layer::building::event::BuildingEventListenerInterface::BuildingEventListenerInterface;
use crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer;
use crate::org::allbinary::game::layer::unit::UnitWaypointBehavior::UnitWaypointBehavior;
use crate::org::allbinary::game::media::graphics::geography::map::racetrack::PathFindingInfoFactory::PathFindingInfoFactory;
use crate::org::allbinary::logic::communication::log::ForcedLogUtil::ForcedLogUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject;
use crate::org::allbinary::logic::util::event::EventStrings::EventStrings;
use crate::org::allbinary::media::audio::Sound::Sound;
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMap;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPosition;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellTypeFactory::GeographicMapCellTypeFactory;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCompositeInterface::GeographicMapCompositeInterface;
use crate::org::allbinary::media::graphics::geography::map::racetrack::CustomMapGenerator::CustomMapGenerator;
use crate::org::allbinary::media::graphics::geography::map::racetrack::RaceTrackGeographicMap::RaceTrackGeographicMap;
use crate::org::allbinary::media::graphics::geography::pathfinding::BasicGeographicMapExtractedPathsCacheFactory::BasicGeographicMapExtractedPathsCacheFactory;
use crate::org::allbinary::media::graphics::geography::pathfinding::PathFindingInfo::PathFindingInfo;
use crate::org::allbinary::media::graphics::geography::pathfinding::PathGenerator::PathGenerator;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListUtil::BasicArrayListUtil;
use crate::java::lang::Exception::*;
use crate::java::lang::RuntimeException::*;
use super::WaypointBase::WaypointBase;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct Waypoint {
    pub base: WaypointBase,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub basicArrayListUtil: Option<Rc<RefCell<BasicArrayListUtil>>>,
    pub ownerLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>,
    pub endList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub paths: Vec<Vec<Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>>>,
}

impl BuildingEventListenerInterface for Waypoint {
    fn onBuildingEvent(&mut self, mut event: Option<Rc<RefCell<RTSLayerEvent>>>) { Self::onBuildingEvent(self, event) }
}
impl crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface for Waypoint {
    fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { Self::onEvent(self, eventObject) }
}

impl Waypoint {
    

    pub fn new_struct(mut ownerLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut sound: Option<Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>>) -> Self {
        let mut this = Self {
            base: WaypointBase::new_struct(sound.clone()),
            logUtil: None,
            basicArrayListUtil: None,
            ownerLayer: None,
            endList: None,
            paths: Vec::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.basicArrayListUtil = BasicArrayListUtil::getInstance();
        this.endList = jcall!(this.basicArrayListUtil).getImmutableInstance();
        this.paths = { let java_field = jcall!(this.basicArrayListUtil).NULL_ARRAY_OF_ARRAY.clone(); java_field }.clone();
        // super(...) is in the struct literal above
        this.ownerLayer = ownerLayer.clone();
        this
    }

    pub fn new(mut ownerLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut sound: Option<Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(ownerLayer, sound))))
    }
    

    // throws Exception
    pub fn setAllBinaryGameLayerManager(&mut self, mut allBinaryGameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>) {
        { let java_arguments = (allBinaryGameLayerManager.clone(),); self.base.setAllBinaryGameLayerManager(java_arguments.0) };
        let mut geographicMapCompositeInterface: Option<Rc<RefCell<dyn GeographicMapCompositeInterface>>> = (|| -> Option<Rc<RefCell<dyn GeographicMapCompositeInterface>>> { panic!("ClassCastException: a Rust dyn AllBinaryGameLayerManager cannot be cast to GeographicMapCompositeInterface") })();
        let mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>> = jcall!(geographicMapCompositeInterface).getGeographicMapInterface()[(0) as usize].clone();
        let mut tiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>> = jcall!(geographicMapInterface).getAllBinaryTiledLayer();
        if !match (&self.paths, &{ let java_field = jcall!(self.basicArrayListUtil).NULL_ARRAY_OF_ARRAY.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
        }
        self.endList = { let java_value = (jcall!(self.ownerLayer).getEndGeographicMapCellPositionList()).clone(); java_value };
        self.paths = { let java_value = (vec![vec![None; (jcall!(tiledLayer).getRows()) as usize]; (jcall!(tiledLayer).getColumns()) as usize]).clone(); java_value };
        self.clearPaths();
    }
    

    pub fn releaseCachedPaths(&mut self) {
        {
            let mut columnIndex: i32 = (self.paths.len() as i32).clone();
            while { columnIndex -= 1; columnIndex } >= 0 {
            {
                let mut rowIndex: i32 = (self.paths.len() as i32).clone();
                while { rowIndex -= 1; rowIndex } >= 0 {
                let mut pathsList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.paths[(columnIndex) as usize][(rowIndex) as usize].clone();
                if !match (&pathsList, &jcall!(self.basicArrayListUtil).getImmutableInstance()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                    jcall!(BasicGeographicMapExtractedPathsCacheFactory::getInstance()).release(pathsList.clone());
                }
                }
            }
            }
        }
    }
    

    // throws Exception
    pub fn getPathsListFromCacheOnly(&mut self, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.paths[(jcall!(geographicMapCellPosition).getColumn()) as usize][(jcall!(geographicMapCellPosition).getRow()) as usize].clone();
    }
    

    // throws Exception
    pub fn getPathsListRunnable(&mut self, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut pathsList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.paths[(jcall!(geographicMapCellPosition).getColumn()) as usize][(jcall!(geographicMapCellPosition).getRow()) as usize].clone();
        if match (&pathsList, &jcall!(self.basicArrayListUtil).getImmutableInstance()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            pathsList = self.createPaths(geographicMapCellPosition.clone());
            { let java_index = (jcall!(geographicMapCellPosition).getRow()) as usize; self.paths[(jcall!(geographicMapCellPosition).getColumn()) as usize][java_index] = pathsList.clone(); };
        }
        return pathsList.clone();
    }
    

    pub fn getEndGeographicMapCellPosition(&mut self, mut startGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> {
        let mut endGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = { let java_object = (jcall!(self.basicArrayListUtil).getRandom(self.endList.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>>().expect("ClassCastException").clone()) };
        if match (&startGeographicMapCellPosition, &endGeographicMapCellPosition) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            for index in (0..={ let java_bound = jcall!(self.endList).size() - 1; java_bound }).rev() {
                endGeographicMapCellPosition = { let java_value = ({ let java_object = (jcall!(self.endList).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>>().expect("ClassCastException").clone()) }).clone(); java_value };
                if !match (&startGeographicMapCellPosition, &endGeographicMapCellPosition) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                    break;
                }
            }
        }
        return endGeographicMapCellPosition.clone();
    }
    

    // throws Exception
    pub fn createPaths(&mut self, mut startGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        if jcall!(self.endList).size() == 0 {
            return jcall!(self.basicArrayListUtil).getImmutableInstance();
        }
        let mut endGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = self.getEndGeographicMapCellPosition(startGeographicMapCellPosition.clone());
        if endGeographicMapCellPosition.is_none() {
            return jcall!(self.basicArrayListUtil).getImmutableInstance();
        }
        if match (&startGeographicMapCellPosition, &endGeographicMapCellPosition) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            if jcall!(self.ownerLayer).shouldHandleStartSameAsEnd() {
                return jcall!(self.basicArrayListUtil).getImmutableInstance();
            } else {
                panic!("{:?}", format!("{}: {:?}", "Exception", (format!("{}{}", String::from("Start should not be End: "), jcall!(self.endList).size()))));
            }
        }
        let mut geographicMapCompositeInterface: Option<Rc<RefCell<dyn GeographicMapCompositeInterface>>> = (|| -> Option<Rc<RefCell<dyn GeographicMapCompositeInterface>>> { panic!("ClassCastException: a Rust dyn AllBinaryGameLayerManager cannot be cast to GeographicMapCompositeInterface") })();
        let mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>> = jcall!(geographicMapCompositeInterface).getGeographicMapInterface()[(0) as usize].clone();
        let mut raceTrackGeographicMap: Option<Rc<RefCell<RaceTrackGeographicMap>>> = (geographicMapInterface).clone().map(|reference| { assert!({ let java_borrowed = reference.borrow(); let java_any: &dyn std::any::Any = &*java_borrowed; java_any.is::<RaceTrackGeographicMap>() }, "ClassCastException: not a RaceTrackGeographicMap"); unsafe { Rc::from_raw(Rc::into_raw(reference) as *const RefCell<RaceTrackGeographicMap>) } });
        let mut raceTrackGeographicMapCellTypeFactory: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellTypeFactory::GeographicMapCellTypeFactoryDyn>>> = jcall!(raceTrackGeographicMap).getGeographicMapCellTypeFactory();
        let mut customMapGenerator: Option<Rc<RefCell<CustomMapGenerator>>> = (jcall!(raceTrackGeographicMap).getCustomMapGenerator()).clone().map(|reference| { assert!({ let java_borrowed = reference.borrow(); let java_any: &dyn std::any::Any = &*java_borrowed; java_any.is::<CustomMapGenerator>() }, "ClassCastException: not a CustomMapGenerator"); unsafe { Rc::from_raw(Rc::into_raw(reference) as *const RefCell<CustomMapGenerator>) } });
        jcall!(customMapGenerator).copyMapIntoCustomMap();
        let mut customMapArray: Vec<Vec<i32>> = jcall!(customMapGenerator).getCustomMapArray();
        customMapArray[(jcall!(startGeographicMapCellPosition).getRow()) as usize][(jcall!(startGeographicMapCellPosition).getColumn()) as usize] = jcall!(raceTrackGeographicMapCellTypeFactory).getStartType();
        customMapArray[(jcall!(endGeographicMapCellPosition).getRow()) as usize][(jcall!(endGeographicMapCellPosition).getColumn()) as usize] = jcall!(raceTrackGeographicMapCellTypeFactory).getEndType();
        let mut pathFindingInfo: Option<Rc<RefCell<PathFindingInfo>>> = jcall!(PathFindingInfoFactory::getInstance()).getInstancePathFindingInfo(raceTrackGeographicMap.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>), customMapArray.clone());
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(PathGenerator::getInstance()).getInstanceNoCache(geographicMapInterface.clone(), pathFindingInfo.clone(), 2);
        if jcall!(list).size() < 1 {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Path may have been cleared by clearing caching"))));
        }
        return list.clone();
    }
    

    pub fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) {
        ForcedLogUtil::log({ let java_field = jcall!(EventStrings::getInstance()).PERFORMANCE_MESSAGE.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())));
    }
    

    // throws Exception
    pub fn onBuildingEvent(&mut self, mut event: Option<Rc<RefCell<RTSLayerEvent>>>) {
        self.clearPaths();
    }
    

    pub fn clearPaths(&mut self) {
        {
            let mut columnIndex: i32 = (self.paths.len() as i32).clone();
            while { columnIndex -= 1; columnIndex } >= 0 {
            {
                let mut rowIndex: i32 = (self.paths.len() as i32).clone();
                while { rowIndex -= 1; rowIndex } >= 0 {
                { let java_index = (rowIndex) as usize; self.paths[(columnIndex) as usize][java_index] = jcall!(self.basicArrayListUtil).getImmutableInstance(); };
                }
            }
            }
        }
    }
    

    pub fn reset(&mut self) {
        jcall!(self.getConnectedWaypointList()).clear();
        self.releaseCachedPaths();
    }
    

    // throws Exception
    pub fn visit(&mut self, mut unitLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
        if !match (&jcall!(unitLayer).getGroupInterface()[(0) as usize], &jcall!(self.ownerLayer).getGroupInterface()[(0) as usize]) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return;
        }
        let mut size: i32 = jcall!(self.getConnectedWaypointList()).size();
        if size > 0 {
            let mut unitWaypointBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::unit::UnitWaypointBehavior::UnitWaypointBehaviorDyn>>> = (jcall!(unitLayer).getWaypointBehavior()).clone();
            while { let java_value = jcall!(self.getConnectedWaypointList()).size(); java_value } > 0 {
                let mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>> = { let java_object = (jcall!(self.getConnectedWaypointList()).get(0)).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>>().expect("ClassCastException").clone()) };
                if jcall!(rtsLayer).isDestroyed() {
                    jcall!(self.getConnectedWaypointList()).remove(crate::java_into_object(rtsLayer.clone()));
                } else {
                    jcall!(unitLayer).handleCost(self.ownerLayer.clone());
                    jcall!(unitWaypointBehavior).insertWaypoint(0, rtsLayer.clone());
                    break;
                }
            }
        }
    }
}
impl std::ops::Deref for Waypoint {
    type Target = WaypointBase;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for Waypoint {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait WaypointDyn: crate::org::allbinary::game::layer::waypoint::WaypointBase::WaypointBaseDyn {
    fn as_Waypoint(&self) -> &Waypoint;
    fn as_Waypoint_mut(&mut self) -> &mut Waypoint;
    fn releaseCachedPaths(&mut self) { Waypoint::releaseCachedPaths(self.as_Waypoint_mut()) }
    fn getEndGeographicMapCellPosition(&mut self, mut startGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> { Waypoint::getEndGeographicMapCellPosition(self.as_Waypoint_mut(), startGeographicMapCellPosition) }
    fn createPaths(&mut self, mut startGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Waypoint::createPaths(self.as_Waypoint_mut(), startGeographicMapCellPosition) }
    fn onBuildingEvent(&mut self, mut event: Option<Rc<RefCell<RTSLayerEvent>>>) { Waypoint::onBuildingEvent(self.as_Waypoint_mut(), event) }
    fn clearPaths(&mut self) { Waypoint::clearPaths(self.as_Waypoint_mut()) }
}
impl WaypointDyn for Waypoint {
    fn as_Waypoint(&self) -> &Waypoint { self }
    fn as_Waypoint_mut(&mut self) -> &mut Waypoint { self }
}

impl crate::org::allbinary::game::layer::waypoint::WaypointBase::WaypointBaseDyn for Waypoint {
    fn as_WaypointBase(&self) -> &crate::org::allbinary::game::layer::waypoint::WaypointBase::WaypointBase { &self.base }
    fn as_WaypointBase_mut(&mut self) -> &mut crate::org::allbinary::game::layer::waypoint::WaypointBase::WaypointBase { &mut self.base }
    fn setAllBinaryGameLayerManager(&mut self, mut allBinaryGameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>) { Self::setAllBinaryGameLayerManager(self, allBinaryGameLayerManager) }
    fn getPathsListRunnable(&mut self, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Self::getPathsListRunnable(self, geographicMapCellPosition) }
    fn getPathsListFromCacheOnly(&mut self, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Self::getPathsListFromCacheOnly(self, geographicMapCellPosition) }
    fn visit(&mut self, mut unitLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { Self::visit(self, unitLayer) }
    fn reset(&mut self) { Self::reset(self) }
}


