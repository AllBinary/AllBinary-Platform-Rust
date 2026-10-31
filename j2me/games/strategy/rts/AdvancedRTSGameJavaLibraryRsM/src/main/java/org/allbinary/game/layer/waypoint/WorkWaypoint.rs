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
use crate::org::allbinary::game::layer::PathFindingLayerInterface::PathFindingLayerInterface;
use crate::org::allbinary::game::layer::RTSLayer::RTSLayer;
use crate::org::allbinary::game::layer::geological::resources::GeologicalGeographicMapCellPosition::GeologicalGeographicMapCellPosition;
use crate::org::allbinary::game::layer::geological::resources::GeologicalResource::GeologicalResource;
use crate::org::allbinary::game::layer::unit::UnitLayer::UnitLayer;
use crate::org::allbinary::game::layer::unit::UnitWaypointBehavior::UnitWaypointBehavior;
use crate::org::allbinary::media::audio::WorkSound::WorkSound;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::logic::math::SmallIntegerSingletonFactory::SmallIntegerSingletonFactory;
use super::Waypoint::Waypoint;
use super::WaypointInfoHudPaintable::WaypointInfoHudPaintable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct WorkWaypoint {
    pub base: Waypoint,
}

thread_local! { pub static WorkWaypoint_ID: Option<i32> = jcall!(SmallIntegerSingletonFactory::getInstance()).getAt(25); }

impl WorkWaypoint {
    

    pub fn new_struct(mut ownerLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) -> Self {
        let mut this = Self {
            base: Waypoint::new_struct(ownerLayer.clone(), WorkSound::getInstance()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut ownerLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(ownerLayer))))
    }
    

    // throws Exception
    pub fn visit(&mut self, mut unitLayer: Option<Rc<RefCell<UnitLayer>>>) {
        if !match (&jcall!(unitLayer).getGroupInterface()[(0) as usize], &jcall!(self.ownerLayer).getGroupInterface()[(0) as usize]) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return;
        }
        let mut size: i32 = jcall!(self.getConnectedWaypointList()).size();
        if size > 0 {
            let mut occupyList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(jcall!(self.ownerLayer).getGeographicMapCellPositionArea()).getOccupyingGeographicMapCellPositionList();
            let mut outOfResources: bool = true;
            let mut geologicalGeographicMapCellPosition: Option<Rc<RefCell<GeologicalGeographicMapCellPosition>>> = None;
            let mut geologicalResource: Option<Rc<RefCell<GeologicalResource>>> = None;
            {
                let mut index: i32 = jcall!(occupyList).size();
                while { index -= 1; index } >= 0 {
                geologicalGeographicMapCellPosition = { let java_object = (jcall!(occupyList).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<GeologicalGeographicMapCellPosition>>>>().expect("ClassCastException").clone()) };
                geologicalResource = jcall!(geologicalGeographicMapCellPosition).getGeologicalResource();
                if jcall!(geologicalResource).getTotal() >= 1 {
                    outOfResources = false;
                    let mut maxResourceLoad: i32 = ((jcall!(unitLayer).getMaxResourceLoad()) as i32);
                    jcall!(geologicalResource).remove(maxResourceLoad.clone());
                    jcall!(unitLayer).setLoad(maxResourceLoad.clone());
                }
                }
            }
            let mut waypointInfoHudPaintable: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::waypoint::WaypointInfoHudPaintable::WaypointInfoHudPaintableDyn>>> = (jcall!(self.ownerLayer).getHudPaintable()).clone() /* as WaypointInfoHudPaintable */;
            if match (&self.ownerLayer, &jcall!(waypointInfoHudPaintable).getRtsLayer()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                jcall!(waypointInfoHudPaintable).updateSelectionInfo();
            }
            if !outOfResources {
                let mut unitWaypointBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::unit::UnitWaypointBehavior::UnitWaypointBehaviorDyn>>> = (jcall!(unitLayer).getWaypointBehavior()).clone() /* as UnitWaypointBehavior */;
                let mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>> = None;
                while jcall!(self.getConnectedWaypointList()).size() > 0 {
                    rtsLayer = (jcall!(self.getConnectedWaypointList()).get(0)).clone() /* as RTSLayer */;
                    if jcall!(rtsLayer).isDestroyed() {
                        jcall!(self.getConnectedWaypointList()).remove(rtsLayer.clone());
                    } else {
                        jcall!(unitWaypointBehavior).insertWaypoint(0, rtsLayer.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>));
                        break;
                    }
                }
            }
        }
    }
    

    pub fn getType(&mut self) -> i32 {
        return 2;
    }
}
impl std::ops::Deref for WorkWaypoint {
    type Target = Waypoint;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for WorkWaypoint {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::layer::waypoint::Waypoint::WaypointDyn for WorkWaypoint {
    fn as_Waypoint(&self) -> &crate::org::allbinary::game::layer::waypoint::Waypoint::Waypoint { &self.base }
    fn as_Waypoint_mut(&mut self) -> &mut crate::org::allbinary::game::layer::waypoint::Waypoint::Waypoint { &mut self.base }
}
impl crate::org::allbinary::game::layer::waypoint::WaypointBase::WaypointBaseDyn for WorkWaypoint {
    fn as_WaypointBase(&self) -> &crate::org::allbinary::game::layer::waypoint::WaypointBase::WaypointBase { &self.base.base }
    fn as_WaypointBase_mut(&mut self) -> &mut crate::org::allbinary::game::layer::waypoint::WaypointBase::WaypointBase { &mut self.base.base }
    fn setAllBinaryGameLayerManager(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>) { crate::org::allbinary::game::layer::waypoint::Waypoint::Waypoint::setAllBinaryGameLayerManager(crate::org::allbinary::game::layer::waypoint::Waypoint::WaypointDyn::as_Waypoint_mut(self), p1) }
    fn getPathsListRunnable(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { crate::org::allbinary::game::layer::waypoint::Waypoint::Waypoint::getPathsListRunnable(crate::org::allbinary::game::layer::waypoint::Waypoint::WaypointDyn::as_Waypoint_mut(self), p1) }
    fn getPathsListFromCacheOnly(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { crate::org::allbinary::game::layer::waypoint::Waypoint::Waypoint::getPathsListFromCacheOnly(crate::org::allbinary::game::layer::waypoint::Waypoint::WaypointDyn::as_Waypoint_mut(self), p1) }
    fn visit(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::PathFindingLayerInterface::PathFindingLayerInterface>>>) { crate::org::allbinary::game::layer::waypoint::Waypoint::Waypoint::visit(crate::org::allbinary::game::layer::waypoint::Waypoint::WaypointDyn::as_Waypoint_mut(self), p1) }
    fn reset(&mut self) { crate::org::allbinary::game::layer::waypoint::Waypoint::Waypoint::reset(crate::org::allbinary::game::layer::waypoint::Waypoint::WaypointDyn::as_Waypoint_mut(self)) }
    fn getType(&mut self) -> i32 { Self::getType(self) }
}
impl crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface for WorkWaypoint {
    fn onEvent(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { crate::org::allbinary::game::layer::waypoint::Waypoint::Waypoint::onEvent(crate::org::allbinary::game::layer::waypoint::Waypoint::WaypointDyn::as_Waypoint_mut(self), p1) }
}


