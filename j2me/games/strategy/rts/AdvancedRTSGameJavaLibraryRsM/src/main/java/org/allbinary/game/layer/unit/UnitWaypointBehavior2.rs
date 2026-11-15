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
// package org.allbinary.game.layer.unit
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::J2MEUtil::J2MEUtil;
use crate::org::allbinary::game::configuration::feature::Features::Features;
use crate::org::allbinary::game::layer::SteeringVisitor::SteeringVisitor;
use crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer;
use crate::org::allbinary::game::layer::MultipassWaypointPathRunnable::MultipassWaypointPathRunnable;
use crate::org::allbinary::game::layer::RTSLayer::RTSLayer;
use crate::org::allbinary::game::layer::WaypointPathRunnable::WaypointPathRunnable;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::game::layer::PathFindingLayerInterface::PathFindingLayerInterface;
use crate::org::allbinary::game::layer::WaypointPathRunnableBase::WaypointPathRunnableBase;
use crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer;
use crate::org::allbinary::game::layer::waypoint::WaypointBase::WaypointBase;
use crate::org::allbinary::game::tracking::TrackingEventHandler::TrackingEventHandler;
use crate::org::allbinary::graphics::GPoint::GPoint;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::java::bool::BooleanFactory::BooleanFactory;
use crate::org::allbinary::math::LayerDistanceUtil::LayerDistanceUtil;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPosition;
use crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory;
use crate::org::allbinary::thread::PathFindingThreadPool::PathFindingThreadPool;
use crate::org::allbinary::thread::ThreadPool::ThreadPool;
use crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelper;
use crate::java::lang::Exception::*;
use super::UnitWaypointBehavior::UnitWaypointBehavior;
use super::UnitWaypointStrings::UnitWaypointStrings;
use super::UnitLayer::UnitLayer;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct UnitWaypointBehavior2 {
    pub base: UnitWaypointBehavior,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub unitWaypointStrings: Option<Rc<RefCell<UnitWaypointStrings>>>,
    pub basicColorFactory: Option<Rc<RefCell<BasicColorFactory>>>,
    pub layerDistanceUtil: Option<Rc<RefCell<LayerDistanceUtil>>>,
    pub pathFindingThreadPool: Option<Rc<RefCell<dyn crate::org::allbinary::thread::ThreadPool::ThreadPoolDyn>>>,
    pub progressTimeDelayHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>>,
    pub wanderPathsList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub waypointPathRunnable: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointPathRunnableBase::WaypointPathRunnableBaseDyn>>>,
    pub sensorRange: i32,
    pub closeRange: i32,
    pub nextUnvisitedPathGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>,
    pub afterNextUnvisitedPathGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>,
    pub waitingOnTargetPath: bool,
    pub waitingOnWaypointPath: bool,
    pub targetWithoutCachedPathLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>,
    pub buildingSteeringVisitor: Option<Rc<RefCell<BuildingSteeringVisitor>>>,
}

pub static UnitWaypointBehavior2_runningWaypointPathList: crate::JavaStatic<Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>> = crate::JavaStatic::new(|| BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>));
pub static UnitWaypointBehavior2_TARGET_DISTANCE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Target Distance"));
pub static UnitWaypointBehavior2_TARGET_LAYER: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Target Layer"));
impl crate::org::allbinary::layer::NamedInterface::NamedInterface for UnitWaypointBehavior2 {
    fn getName(&mut self) -> String { crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBase::getName(crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn::as_WaypointBehaviorBase_mut(self)) }
}
impl crate::org::allbinary::game::layer::waypoint::event::WaypointEventListenerInterface::WaypointEventListenerInterface for UnitWaypointBehavior2 {
    fn onWaypointEvent(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::game::layer::RTSLayerEvent::RTSLayerEvent>>>) { crate::org::allbinary::game::layer::unit::UnitWaypointBehavior::UnitWaypointBehavior::onWaypointEvent(crate::org::allbinary::game::layer::unit::UnitWaypointBehavior::UnitWaypointBehaviorDyn::as_UnitWaypointBehavior_mut(self), p1) }
}
impl crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface for UnitWaypointBehavior2 {
    fn onEvent(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { crate::org::allbinary::game::layer::unit::UnitWaypointBehavior::UnitWaypointBehavior::onEvent(crate::org::allbinary::game::layer::unit::UnitWaypointBehavior::UnitWaypointBehaviorDyn::as_UnitWaypointBehavior_mut(self), p1) }
}
impl crate::org::allbinary::game::tick::TickableInterface::TickableInterface for UnitWaypointBehavior2 {
    fn processTick(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { Self::processTick(self, allBinaryLayerManager) }
}

impl UnitWaypointBehavior2 {
    

    pub fn new_struct(mut ownerAdvancedRTSGameLayer: Option<Rc<RefCell<UnitLayer>>>, mut fakeWaypoint: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn>>>) -> Self {
        let mut this = Self {
            base: UnitWaypointBehavior::new_struct(ownerAdvancedRTSGameLayer.clone(), fakeWaypoint.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>)),
            logUtil: None,
            unitWaypointStrings: None,
            basicColorFactory: None,
            layerDistanceUtil: None,
            pathFindingThreadPool: None,
            progressTimeDelayHelper: None,
            wanderPathsList: None,
            waypointPathRunnable: None,
            sensorRange: 0,
            closeRange: 0,
            nextUnvisitedPathGeographicMapCellPosition: None,
            afterNextUnvisitedPathGeographicMapCellPosition: None,
            waitingOnTargetPath: false,
            waitingOnWaypointPath: false,
            targetWithoutCachedPathLayerInterface: None,
            buildingSteeringVisitor: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.unitWaypointStrings = UnitWaypointStrings::getInstance();
        this.basicColorFactory = BasicColorFactory::getInstance();
        this.layerDistanceUtil = LayerDistanceUtil::getInstance();
        this.pathFindingThreadPool = PathFindingThreadPool::getInstance();
        this.sensorRange = 0;
        this.closeRange = 0;
        this.nextUnvisitedPathGeographicMapCellPosition = crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.with(Clone::clone).clone();
        this.afterNextUnvisitedPathGeographicMapCellPosition = crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.with(Clone::clone).clone();
        this.targetWithoutCachedPathLayerInterface = CollidableDestroyableDamageableLayer::getNullInstance();
        this.buildingSteeringVisitor = BuildingSteeringVisitor::new();
        // super(...) is in the struct literal above
        this.progressTimeDelayHelper = TimeDelayHelper::new(5000).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>);
        this.wanderPathsList = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut features: Option<Rc<RefCell<Features>>> = Features::getInstance();
        this.waypointPathRunnable = { let java_value = ((if J2MEUtil::isHTML() { MultipassWaypointPathRunnable::new() } else { WaypointPathRunnable::new() })).clone(); java_value };
        this
    }

    pub fn new(mut ownerAdvancedRTSGameLayer: Option<Rc<RefCell<UnitLayer>>>, mut fakeWaypoint: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(ownerAdvancedRTSGameLayer, fakeWaypoint))))
    }
    

    pub fn initRange(&mut self, mut weaponRange: i32) {
        { let java_arguments = (weaponRange.clone(),); self.base.initRange(java_arguments.0) };
        self.closeRange = weaponRange.clone();
        self.sensorRange = weaponRange * 4;
        jcall!({ let java_field = jcall!(self.associatedAdvancedRTSGameLayer).waypoint2LogHelperP.clone(); java_field }).initRange(self.associatedAdvancedRTSGameLayer.clone().map(|reference| reference as Rc<RefCell<dyn PathFindingLayerInterface>>), self.closeRange.clone(), self.sensorRange.clone());
    }
    

    pub fn getNextUnvisitedPathGeographicMapCellPosition(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> {
        return self.nextUnvisitedPathGeographicMapCellPosition.clone();
    }
    

    pub fn isRunning(&mut self) -> bool {
        if jcall!(self.waypointPathRunnable).isRunning() {
            return true;
        } else {
            return false;
        }
    }
    

    // throws Exception
    pub fn processTick(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) {
        if jcall!(self.waypointPathRunnable).isRunning() {
            if !match (&self.waypointPathsListP, &UnitWaypointBehavior2_runningWaypointPathList.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                jcall!(self.waypointPathRunnable).setRunning(false);
                if self.waitingOnTargetPath {
                    self.setTargetPath();
                } else if self.waitingOnWaypointPath {
                    { let java_arguments = ((|| -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn>>> { panic!("ClassCastException: a Rust dyn PathFindingLayerInterface cannot be cast to AdvancedRTSGameLayer") })(),); self.setWaypointPath(java_arguments.0) };
                } else {
                    panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Should not happen"))));
                }
            } else {
                return;
            }
        }
        self.processTargetList();
        if !jcall!(self.waypointPathRunnable).isRunning() {
            self.processWaypoint();
        } else {
            return;
        }
        if !jcall!(self.waypointPathRunnable).isRunning() {
            self.processTargeting();
        } else {
            return;
        }
        if !jcall!(self.waypointPathRunnable).isRunning() {
            self.teleportIfNoProgress();
        } else {
            return;
        }
    }
    

    // throws Exception
    pub fn onEnemyMovement(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn>>>) {
        let mut anotherTargetDistance: i32 = jcall!(self.layerDistanceUtil).getDistance(self.associatedAdvancedRTSGameLayer.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>), layerInterface.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>));
        if match (&layerInterface, &self.currentTargetLayerInterfaceP) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            self.setCurrentTargetDistance(anotherTargetDistance.clone());
        } else {
            self.processPossibleTarget(layerInterface.clone(), anotherTargetDistance.clone());
        }
    }
    

    // throws Exception
    pub fn processPossibleTarget(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn>>>, mut anotherTargetDistance: i32) {
        let mut isShorterThanCurrentTargetDistance: bool = self.getCurrentTargetDistance() > anotherTargetDistance;
        let mut isCurrentTargetDestroyed: bool = !match (&self.currentTargetLayerInterfaceP, &CollidableDestroyableDamageableLayer::getNullInstance()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } && jcall!(self.currentTargetLayerInterfaceP).isDestroyed();
        jcall!({ let java_field = jcall!(self.associatedAdvancedRTSGameLayer).waypoint2LogHelperP.clone(); java_field }).processPossibleTarget(self.associatedAdvancedRTSGameLayer.clone().map(|reference| reference as Rc<RefCell<dyn PathFindingLayerInterface>>), None /* Java this: a &mut self method has no Rc of itself */, layerInterface.clone().map(|_java_reference| -> Rc<RefCell<dyn PathFindingLayerInterface>> { panic!("ClassCastException: the Rust trait of AdvancedRTSGameLayer does not have PathFindingLayerInterface as a base trait") }), anotherTargetDistance.clone(), isShorterThanCurrentTargetDistance.clone(), isCurrentTargetDestroyed.clone());
        if self.isWaypointListEmptyOrOnlyTargets() && self.isInSensorRange(layerInterface.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>), anotherTargetDistance.clone()) && (isShorterThanCurrentTargetDistance || isCurrentTargetDestroyed) {
            jcall!({ let java_field = jcall!(self.associatedAdvancedRTSGameLayer).waypoint2LogHelperP.clone(); java_field }).processSetTarget(self.associatedAdvancedRTSGameLayer.clone().map(|reference| reference as Rc<RefCell<dyn PathFindingLayerInterface>>), None /* Java this: a &mut self method has no Rc of itself */, layerInterface.clone().map(|_java_reference| -> Rc<RefCell<dyn PathFindingLayerInterface>> { panic!("ClassCastException: the Rust trait of AdvancedRTSGameLayer does not have PathFindingLayerInterface as a base trait") }), anotherTargetDistance.clone());
            self.setTarget(layerInterface.clone(), anotherTargetDistance.clone());
        } else if self.isCloseRange(layerInterface.clone(), anotherTargetDistance.clone()) && (isShorterThanCurrentTargetDistance || isCurrentTargetDestroyed) {
            jcall!({ let java_field = jcall!(self.associatedAdvancedRTSGameLayer).waypoint2LogHelperP.clone(); java_field }).processPossibleTargetCloser(self.associatedAdvancedRTSGameLayer.clone().map(|reference| reference as Rc<RefCell<dyn PathFindingLayerInterface>>), None /* Java this: a &mut self method has no Rc of itself */, layerInterface.clone().map(|_java_reference| -> Rc<RefCell<dyn PathFindingLayerInterface>> { panic!("ClassCastException: the Rust trait of AdvancedRTSGameLayer does not have PathFindingLayerInterface as a base trait") }), anotherTargetDistance.clone());
            self.setTarget(layerInterface.clone(), anotherTargetDistance.clone());
        }
    }
    

    // throws Exception
    pub fn teleportIfNoProgress(&mut self) {
        if self.isTrackingWaypoint() || { let java_value = jcall!(jcall!(self.associatedAdvancedRTSGameLayer).getParentLayer()).isDestroyed(); java_value } {
            if jcall!(self.progressTimeDelayHelper).isTimeTNT() && !match (&self.nextUnvisitedPathGeographicMapCellPosition, &crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                jcall!(self.associatedAdvancedRTSGameLayer).teleportTo(self.nextUnvisitedPathGeographicMapCellPosition.clone());
            }
            if jcall!(self.getCompleteTimeDelayHelper()).isTimeTNT() {
                let mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = { let java_object = (jcall!(jcall!(self.currentGeographicMapCellHistoryP).getTracked()).get(jcall!(self.currentGeographicMapCellHistoryP).getSize() - 1)).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>>().expect("ClassCastException").clone()) };
                jcall!(self.associatedAdvancedRTSGameLayer).teleportTo(geographicMapCellPosition.clone());
            }
        }
    }
    

    // throws Exception
    pub fn setTarget(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn>>>, mut anotherTargetDistance: i32) {
        jcall!({ let java_field = jcall!(self.associatedAdvancedRTSGameLayer).waypoint2LogHelperP.clone(); java_field }).setTarget(self.associatedAdvancedRTSGameLayer.clone().map(|reference| reference as Rc<RefCell<dyn PathFindingLayerInterface>>), None /* Java this: a &mut self method has no Rc of itself */, layerInterface.clone().map(|_java_reference| -> Rc<RefCell<dyn PathFindingLayerInterface>> { panic!("ClassCastException: the Rust trait of AdvancedRTSGameLayer does not have PathFindingLayerInterface as a base trait") }), anotherTargetDistance.clone());
        jcall!(jcall!(self.associatedAdvancedRTSGameLayer).getCaptionAnimationHelper()).update({ let java_field = jcall!(self.unitWaypointStrings).TARGET.clone(); java_field }.clone(), { let java_field = jcall!(BasicColorFactory::getInstance()).GREEN.clone(); java_field }.clone());
        jcall!(self.associatedAdvancedRTSGameLayer).setLoad(0);
        self.setCurrentTargetDistance(anotherTargetDistance.clone());
        self.setCurrentTargetLayerInterface(layerInterface.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>));
        self.setTrackingWaypoint(false);
        jcall!(self.targetList).clear();
        if !self.isCloseRange(layerInterface.clone(), anotherTargetDistance.clone()) && { let java_arguments = (0, self.currentTargetLayerInterfaceP.clone(),); self.canInsertWaypoint(java_arguments.0, java_arguments.1) } {
            let mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = jcall!(self.associatedAdvancedRTSGameLayer).getCurrentGeographicMapCellPosition();
            let mut pathFindingLayerInterface: Option<Rc<RefCell<dyn PathFindingLayerInterface>>> = ((|| -> Option<Rc<RefCell<dyn PathFindingLayerInterface>>> { panic!("ClassCastException: a Rust dyn CollidableDestroyableDamageableLayer cannot be cast to PathFindingLayerInterface") })());
            let mut waypoint: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::waypoint::WaypointBase::WaypointBaseDyn>>> = jcall!(jcall!(pathFindingLayerInterface).getWaypointBehavior()).getWaypoint();
            let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(waypoint).getPathsListFromCacheOnly(geographicMapCellPosition.clone());
            self.setWaypointPathsList(list.clone());
            if self.waypointPathsListP.is_none() {
                self.targetWithoutCachedPathLayerInterface = self.currentTargetLayerInterfaceP.clone();
            } else if jcall!(self.waypointPathsListP).size() != 0 {
                self.setTargetPath();
            }
        }
    }
    

    // throws Exception
    pub fn setTargetPath(&mut self) {
        if !match (&self.currentTargetLayerInterfaceP, &CollidableDestroyableDamageableLayer::getNullInstance()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            if jcall!(self.currentTargetLayerInterfaceP).isDestroyed() {
                jcall!({ let java_field = jcall!(self.associatedAdvancedRTSGameLayer).waypoint2LogHelperP.clone(); java_field }).setTargetPath(self.associatedAdvancedRTSGameLayer.clone().map(|reference| reference as Rc<RefCell<dyn PathFindingLayerInterface>>));
                jcall!(jcall!(self.associatedAdvancedRTSGameLayer).getCaptionAnimationHelper()).update({ let java_field = jcall!(self.unitWaypointStrings).KILL.clone(); java_field }.clone(), { let java_field = jcall!(self.basicColorFactory).ORANGE.clone(); java_field }.clone());
                self.clearTarget();
                return;
            }
            if match (&self.currentTargetLayerInterfaceP, &jcall!(self.waypointPathRunnable).getTargetLayer()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                jcall!({ let java_field = jcall!(self.associatedAdvancedRTSGameLayer).waypoint2LogHelperP.clone(); java_field }).setTargetPathIgnoreNewPath(self.associatedAdvancedRTSGameLayer.clone().map(|reference| reference as Rc<RefCell<dyn PathFindingLayerInterface>>), None /* Java this: a &mut self method has no Rc of itself */);
                { let java_arguments = (0, self.currentTargetLayerInterfaceP.clone(),); self.insertWaypoint(java_arguments.0, java_arguments.1) };
                { let java_arguments = (self.waypointPathsListP.clone(),); self.setRandomGeographicMapCellHistory(java_arguments.0) };
            }
        }
    }
    

    // throws Exception
    pub fn setGeographicMapCellHistoryPath(&mut self, mut geographicMapCellPositionBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        { let java_arguments = (crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.with(Clone::clone).clone(),); self.setCurrentPathGeographicMapCellPosition(java_arguments.0) };
        self.nextUnvisitedPathGeographicMapCellPosition = { let java_value = (crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.with(Clone::clone)).clone(); java_value }.clone();
        { let java_arguments = (geographicMapCellPositionBasicArrayList.clone(),); self.base.setGeographicMapCellHistoryPath(java_arguments.0) };
    }
    

    // throws Exception
    pub fn processWaypoint(&mut self) {
        let mut size: i32 = jcall!(self.targetList).size();
        if size > 0 {
            let mut targetLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn>>> = { let java_object = (jcall!(self.targetList).get(0)).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn>>>>().expect("ClassCastException").clone()) };
            jcall!({ let java_field = jcall!(self.associatedAdvancedRTSGameLayer).waypoint2LogHelperP.clone(); java_field }).processWaypoint(self.associatedAdvancedRTSGameLayer.clone().map(|reference| reference as Rc<RefCell<dyn PathFindingLayerInterface>>), None /* Java this: a &mut self method has no Rc of itself */, targetLayer.clone().map(|_java_reference| -> Rc<RefCell<dyn PathFindingLayerInterface>> { panic!("ClassCastException: the Rust trait of AdvancedRTSGameLayer does not have PathFindingLayerInterface as a base trait") }), size.clone());
            if jcall!(targetLayer).isDestroyed() {
                jcall!(jcall!(self.associatedAdvancedRTSGameLayer).getCaptionAnimationHelper()).update({ let java_field = jcall!(self.unitWaypointStrings).WAYPOINT_DESTROYED_SHORT.clone(); java_field }.clone(), { let java_field = jcall!(self.basicColorFactory).YELLOW.clone(); java_field }.clone());
                { let java_arguments = (targetLayer.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>), { let java_field = jcall!(self.unitWaypointStrings).WAYPOINT_DESTROYED.clone(); java_field }.clone(),); self.removeWaypoint(java_arguments.0, java_arguments.1) };
            } else {
                let mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = jcall!(self.associatedAdvancedRTSGameLayer).getCurrentGeographicMapCellPosition();
                if self.isTrackingWaypoint() {
                    jcall!({ let java_field = jcall!(self.associatedAdvancedRTSGameLayer).waypoint2LogHelperP.clone(); java_field }).processWaypointTracked(self.associatedAdvancedRTSGameLayer.clone().map(|reference| reference as Rc<RefCell<dyn PathFindingLayerInterface>>), None /* Java this: a &mut self method has no Rc of itself */);
                    if self.visitIfAtMidPoint(geographicMapCellPosition.clone()) {
                        jcall!({ let java_field = jcall!(self.associatedAdvancedRTSGameLayer).waypoint2LogHelperP.clone(); java_field }).processWaypointTrackedVisit(self.associatedAdvancedRTSGameLayer.clone().map(|reference| reference as Rc<RefCell<dyn PathFindingLayerInterface>>), geographicMapCellPosition.clone());
                    }
                    if jcall!(self.currentGeographicMapCellHistoryP).isAllVisited2() && !match (&self.currentTargetLayerInterfaceP, &CollidableDestroyableDamageableLayer::getNullInstance()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                        let mut oldWaypointLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn>>> = (|| -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn>>> { panic!("ClassCastException: a Rust dyn CollidableDestroyableDamageableLayer cannot be cast to AdvancedRTSGameLayer") })();
                        jcall!(jcall!(jcall!(oldWaypointLayer).getWaypointBehavior()).getWaypoint()).visit(self.associatedAdvancedRTSGameLayer.clone().map(|reference| reference as Rc<RefCell<dyn PathFindingLayerInterface>>));
                        jcall!(jcall!(self.associatedAdvancedRTSGameLayer).getCaptionAnimationHelper()).update({ let java_field = jcall!(self.unitWaypointStrings).ALL_VISITED_SHORT.clone(); java_field }.clone(), { let java_field = jcall!(self.basicColorFactory).GREEN.clone(); java_field }.clone());
                        { let java_arguments = ((|| -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn>>> { panic!("ClassCastException: a Rust dyn CollidableDestroyableDamageableLayer cannot be cast to AdvancedRTSGameLayer") })(), { let java_field = jcall!(self.unitWaypointStrings).ALL_VISITED.clone(); java_field }.clone(),); self.removeWaypoint(java_arguments.0, java_arguments.1) };
                    }
                } else if self.currentTargetLayerInterfaceP.is_none() || self.waypointOverridesAttacking {
                    let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(jcall!(jcall!(targetLayer).getWaypointBehavior()).getWaypoint()).getPathsListFromCacheOnly(geographicMapCellPosition.clone());
                    self.setWaypointPathsList(list.clone());
                    if self.waypointPathsListP.is_none() {
                        self.waitingOnWaypointPath = true;
                        jcall!(jcall!(self.associatedAdvancedRTSGameLayer).getCaptionAnimationHelper()).update({ let java_field = jcall!(self.unitWaypointStrings).THINKING.clone(); java_field }.clone(), { let java_field = jcall!(self.basicColorFactory).GREEN.clone(); java_field }.clone());
                        self.runWaypointPathTask(targetLayer.clone());
                        return;
                    }
                    self.setWaypointPath(targetLayer.clone());
                }
            }
        }
    }
    

    // throws Exception
    pub fn wander(&mut self) {
        if jcall!(self.currentGeographicMapCellHistoryP).isAllVisited2() {
            jcall!({ let java_field = jcall!(self.associatedAdvancedRTSGameLayer).waypoint2LogHelperP.clone(); java_field }).wander(self.associatedAdvancedRTSGameLayer.clone().map(|reference| reference as Rc<RefCell<dyn PathFindingLayerInterface>>));
            jcall!(jcall!(self.associatedAdvancedRTSGameLayer).getCaptionAnimationHelper()).update({ let java_field = jcall!(self.unitWaypointStrings).WANDERING.clone(); java_field }.clone(), { let java_field = jcall!(self.basicColorFactory).RED.clone(); java_field }.clone());
            jcall!(self.wanderPathsList).clear();
            { let java_receiver = self.wanderPathsList.clone(); let java_arguments = (crate::java_into_object(jcall!(self.associatedAdvancedRTSGameLayer).getSurroundingGeographicMapCellPositionList()),); jcall!(java_receiver).add(java_arguments.0); };
            { let java_arguments = (self.wanderPathsList.clone(),); self.setRandomGeographicMapCellHistory(java_arguments.0) };
        }
        { let java_arguments = (self.getCurrentPathGeographicMapCellPosition(),); self.visitIfAtMidPoint(java_arguments.0) };
        self.updateCurrentPathGeographicMapCellPosition();
        jcall!(self.associatedAdvancedRTSGameLayer).trackTo({ let java_field = jcall!(self.unitWaypointStrings).NEXT_PATH_NODE.clone(); java_field }.clone());
    }
    

    pub fn visitIfAtMidPoint(&mut self, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> bool {
        let mut unitLayer: Option<Rc<RefCell<UnitLayer>>> = self.associatedAdvancedRTSGameLayer.clone();
        if !match (&geographicMapCellPosition, &crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } && match (&self.nextUnvisitedPathGeographicMapCellPosition, &geographicMapCellPosition) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            let mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = jcall!(geographicMapCellPosition).getMidPoint();
            let mut afterNextPoint: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = jcall!(self.afterNextUnvisitedPathGeographicMapCellPosition).getMidPoint();
            let mut beyondMidPoint: bool = true;
            if jcall!(geographicMapCellPosition).getColumn() == jcall!(self.afterNextUnvisitedPathGeographicMapCellPosition).getColumn() {
            } else if jcall!(point).getX() < jcall!(afterNextPoint).getX() {
                if jcall!(unitLayer).getXP() + jcall!(unitLayer).getHalfWidth() < jcall!(point).getX() {
                    beyondMidPoint = false;
                }
            } else {
                if jcall!(unitLayer).getXP() + jcall!(unitLayer).getHalfWidth() > jcall!(point).getX() {
                    beyondMidPoint = false;
                }
            }
            if jcall!(geographicMapCellPosition).getRow() == jcall!(self.afterNextUnvisitedPathGeographicMapCellPosition).getRow() {
            } else if jcall!(point).getY() < jcall!(afterNextPoint).getY() {
                if jcall!(unitLayer).getYP() + jcall!(unitLayer).getHalfHeight() < jcall!(point).getY() {
                    beyondMidPoint = false;
                }
            } else {
                if jcall!(unitLayer).getYP() + jcall!(unitLayer).getHalfHeight() > jcall!(point).getY() {
                    beyondMidPoint = false;
                }
            }
            if beyondMidPoint {
                return jcall!(self.currentGeographicMapCellHistoryP).visit(geographicMapCellPosition.clone());
            }
        }
        return false;
    }
    

    // throws Exception
    pub fn processTargetList(&mut self) {
        self.targetWithoutCachedPathLayerInterface = { let java_value = (CollidableDestroyableDamageableLayer::getNullInstance()).clone(); java_value };
        for index in (0..={ let java_bound = { let java_value = jcall!(self.getPossibleTargetList()).size(); java_value } - 1; java_bound }).rev() {
            let mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn>>> = { let java_object = (jcall!(self.getPossibleTargetList()).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn>>>>().expect("ClassCastException").clone()) };
            if jcall!(layerInterface).isDestroyed() {
                jcall!(self.getPossibleTargetList()).remove(crate::java_into_object(layerInterface.clone()));
            } else {
                self.onEnemyMovement(layerInterface.clone());
            }
        }
        if !match (&self.targetWithoutCachedPathLayerInterface, &CollidableDestroyableDamageableLayer::getNullInstance()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            self.waitingOnTargetPath = true;
            jcall!(jcall!(self.associatedAdvancedRTSGameLayer).getCaptionAnimationHelper()).update({ let java_field = jcall!(self.unitWaypointStrings).THINKING_ABOUT_TARGET.clone(); java_field }.clone(), { let java_field = jcall!(self.basicColorFactory).GREEN.clone(); java_field }.clone());
            { let java_arguments = ((|| -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn>>> { panic!("ClassCastException: a Rust dyn CollidableDestroyableDamageableLayer cannot be cast to AdvancedRTSGameLayer") })(),); self.runWaypointPathTask(java_arguments.0) };
        }
        jcall!(self.getPossibleTargetList()).clear();
    }
    

    // throws Exception
    pub fn processTargeting(&mut self) {
        if !match (&self.currentTargetLayerInterfaceP, &CollidableDestroyableDamageableLayer::getNullInstance()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } && ({ let java_arguments = (self.currentTargetLayerInterfaceP.clone(), self.getCurrentTargetDistance(),); self.isInSensorRange(java_arguments.0, java_arguments.1) } || self.isTrackingWaypoint()) {
            if jcall!(self.currentTargetLayerInterfaceP).isDestroyed() {
                jcall!({ let java_field = jcall!(self.associatedAdvancedRTSGameLayer).waypoint2LogHelperP.clone(); java_field }).targetDestroyed(self.associatedAdvancedRTSGameLayer.clone().map(|reference| reference as Rc<RefCell<dyn PathFindingLayerInterface>>));
                jcall!(jcall!(self.associatedAdvancedRTSGameLayer).getCaptionAnimationHelper()).update({ let java_field = jcall!(self.unitWaypointStrings).KILL.clone(); java_field }.clone(), { let java_field = jcall!(self.basicColorFactory).ORANGE.clone(); java_field }.clone());
                self.clearTarget();
                return;
            }
            let mut dx: i32 = 0;
            let mut dy: i32 = 0;
            if self.isTrackingWaypoint() {
                self.updateCurrentPathGeographicMapCellPosition();
                let mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = jcall!(self.nextUnvisitedPathGeographicMapCellPosition).getMidPoint();
                dx = { let java_value = (jcall!(self.associatedAdvancedRTSGameLayer).getXP() + jcall!(self.associatedAdvancedRTSGameLayer).getHalfWidth() - jcall!(point).getX()).clone(); java_value };
                dy = { let java_value = (jcall!(self.associatedAdvancedRTSGameLayer).getYP() + jcall!(self.associatedAdvancedRTSGameLayer).getHalfHeight() - jcall!(point).getY()).clone(); java_value };
                jcall!({ let java_field = jcall!(self.associatedAdvancedRTSGameLayer).waypoint2LogHelperP.clone(); java_field }).processTargeting(self.associatedAdvancedRTSGameLayer.clone().map(|reference| reference as Rc<RefCell<dyn PathFindingLayerInterface>>), dx.clone(), dy.clone());
            } else {
                jcall!({ let java_field = jcall!(self.associatedAdvancedRTSGameLayer).waypoint2LogHelperP.clone(); java_field }).processTargetingNonWayPoint(self.associatedAdvancedRTSGameLayer.clone().map(|reference| reference as Rc<RefCell<dyn PathFindingLayerInterface>>), dx.clone(), dy.clone());
                dx = { let java_value = ((jcall!(self.associatedAdvancedRTSGameLayer).getXP() + jcall!(self.associatedAdvancedRTSGameLayer).getHalfWidth()) - (jcall!(self.currentTargetLayerInterfaceP).getXP() + jcall!(self.currentTargetLayerInterfaceP).getHalfWidth())).clone(); java_value };
                dy = { let java_value = ((jcall!(self.associatedAdvancedRTSGameLayer).getYP() + jcall!(self.associatedAdvancedRTSGameLayer).getHalfHeight()) - (jcall!(self.currentTargetLayerInterfaceP).getYP() + jcall!(self.currentTargetLayerInterfaceP).getHalfHeight())).clone(); java_value };
            }
            jcall!(self.associatedAdvancedRTSGameLayer).trackToDXY(dx.clone(), dy.clone());
        } else {
            if jcall!(jcall!(self.associatedAdvancedRTSGameLayer).getParentLayer()).isDestroyed() {
                self.wander();
            } else {
                if { let java_field = jcall!(self.associatedAdvancedRTSGameLayer).showMoreCaptionStates.clone(); java_field } {
                    jcall!(jcall!(self.associatedAdvancedRTSGameLayer).getCaptionAnimationHelper()).update({ let java_field = jcall!(self.unitWaypointStrings).STOP.clone(); java_field }.clone(), { let java_field = jcall!(self.basicColorFactory).YELLOW.clone(); java_field }.clone());
                }
                jcall!(self.associatedAdvancedRTSGameLayer).allStop();
            }
        }
    }
    

    pub fn updateCurrentPathGeographicMapCellPosition(&mut self) {
        { let java_arguments = (self.getCurrentPathGeographicMapCellPosition(),); self.setLastPathGeographicMapCellPosition(java_arguments.0) };
        { let java_arguments = (self.nextUnvisitedPathGeographicMapCellPosition.clone(),); self.setCurrentPathGeographicMapCellPosition(java_arguments.0) };
        self.nextUnvisitedPathGeographicMapCellPosition = { let java_value = (jcall!(self.currentGeographicMapCellHistoryP).getFirstUnvisited()).clone(); java_value };
        self.afterNextUnvisitedPathGeographicMapCellPosition = { let java_value = (jcall!(self.currentGeographicMapCellHistoryP).getAfterIfNotLast(self.nextUnvisitedPathGeographicMapCellPosition.clone())).clone(); java_value };
        if !match (&self.getCurrentPathGeographicMapCellPosition(), &self.nextUnvisitedPathGeographicMapCellPosition) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            jcall!(self.progressTimeDelayHelper).setStartTimeTNT();
        }
    }
    

    // throws Exception
    pub fn setWaypointPath(&mut self, mut waypointLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn>>>) {
        if jcall!(self.waypointPathsListP).size() != 0 {
            self.setCurrentTargetLayerInterface(waypointLayer.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>));
            let mut MAX: i32 = i32::MAX.clone();
            self.setCurrentTargetDistance(MAX.clone());
            { let java_arguments = (self.waypointPathsListP.clone(),); self.setRandomGeographicMapCellHistory(java_arguments.0) };
        } else {
            jcall!(jcall!(jcall!(waypointLayer).getWaypointBehavior()).getWaypoint()).visit(self.associatedAdvancedRTSGameLayer.clone().map(|reference| reference as Rc<RefCell<dyn PathFindingLayerInterface>>));
            jcall!(jcall!(self.associatedAdvancedRTSGameLayer).getCaptionAnimationHelper()).update({ let java_field = jcall!(self.unitWaypointStrings).ALREADY_THERE_SHORT.clone(); java_field }.clone(), { let java_field = jcall!(self.basicColorFactory).YELLOW.clone(); java_field }.clone());
            { let java_arguments = (waypointLayer.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>), { let java_field = jcall!(self.unitWaypointStrings).ALREADY_THERE.clone(); java_field }.clone(),); self.removeWaypoint(java_arguments.0, java_arguments.1) };
        }
    }
    

    // throws Exception
    pub fn runWaypointPathTask(&mut self, mut waypointLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn>>>) {
        { let java_arguments = (UnitWaypointBehavior2_runningWaypointPathList.with(Clone::clone).clone(),); self.setWaypointPathsList(java_arguments.0) };
        if jcall!(self.waypointPathRunnable).isRunning() {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Should never be running here"))));
        }
        jcall!(self.waypointPathRunnable).setRunning(true);
        jcall!(self.waypointPathRunnable).setUnitLayer(self.associatedAdvancedRTSGameLayer.clone().map(|reference| reference as Rc<RefCell<dyn PathFindingLayerInterface>>));
        jcall!(self.waypointPathRunnable).setTargetLayer(waypointLayer.clone().map(|_java_reference| -> Rc<RefCell<dyn PathFindingLayerInterface>> { panic!("ClassCastException: the Rust trait of AdvancedRTSGameLayer does not have PathFindingLayerInterface as a base trait") }));
        jcall!(self.pathFindingThreadPool).runTask(self.waypointPathRunnable.clone().map(|_java_reference| -> Rc<RefCell<dyn crate::Runnable>> { panic!("ClassCastException: the Rust trait of WaypointPathRunnableBase does not have Runnable as a base trait") }));
    }
    

    // throws Exception
    pub fn removeWaypoint(&mut self, mut waypointLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>>, mut reason: String) {
        jcall!({ let java_field = jcall!(self.associatedAdvancedRTSGameLayer).waypoint2LogHelperP.clone(); java_field }).removeWaypoint(self.associatedAdvancedRTSGameLayer.clone().map(|reference| reference as Rc<RefCell<dyn PathFindingLayerInterface>>), None /* Java this: a &mut self method has no Rc of itself */, waypointLayer.clone().map(|_java_reference| -> Rc<RefCell<dyn PathFindingLayerInterface>> { panic!("ClassCastException: the Rust trait of RTSLayer does not have PathFindingLayerInterface as a base trait") }), reason.clone());
        jcall!(self.targetList).remove(crate::java_into_object(waypointLayer.clone()));
        jcall!({ let java_field = jcall!(self.associatedAdvancedRTSGameLayer).waypoint2LogHelperP.clone(); java_field }).removeWaypointList(self.associatedAdvancedRTSGameLayer.clone().map(|reference| reference as Rc<RefCell<dyn PathFindingLayerInterface>>), None /* Java this: a &mut self method has no Rc of itself */, self.targetList.clone());
        if match (&self.currentTargetLayerInterfaceP, &waypointLayer) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            jcall!({ let java_field = jcall!(self.associatedAdvancedRTSGameLayer).waypoint2LogHelperP.clone(); java_field }).removeWaypointClear(self.associatedAdvancedRTSGameLayer.clone().map(|reference| reference as Rc<RefCell<dyn PathFindingLayerInterface>>));
            self.clearTarget();
        }
    }
    

    // throws Exception
    pub fn clearTarget(&mut self) {
        jcall!({ let java_field = jcall!(self.associatedAdvancedRTSGameLayer).waypoint2LogHelperP.clone(); java_field }).clearTarget(self.associatedAdvancedRTSGameLayer.clone().map(|reference| reference as Rc<RefCell<dyn PathFindingLayerInterface>>));
        { let java_arguments = (CollidableDestroyableDamageableLayer::getNullInstance(),); self.setCurrentTargetLayerInterface(java_arguments.0) };
        self.setTrackingWaypoint(false);
        let mut MAX: i32 = i32::MAX.clone();
        self.setCurrentTargetDistance(MAX.clone());
        jcall!(TrackingEventHandler::getInstance()).fireEvent(jcall!(self.associatedAdvancedRTSGameLayer).getTrackingEvent().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>));
    }
    

    pub fn isWaypointListEmptyOrOnlyTargets(&mut self) -> bool {
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.targetList.clone();
        if jcall!(list).size() == 0 {
            return true;
        }
        for index in (0..={ let java_bound = jcall!(list).size() - 1; java_bound }).rev() {
            let mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn>>> = { let java_object = (jcall!(list).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn>>>>().expect("ClassCastException").clone()) };
            if jcall!(layerInterface).isWaypointListEmptyOrOnlyTargets() {
                return false;
            }
        }
        return true;
    }
    

    pub fn isCloseRange(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn>>>, mut targetDistance: i32) -> bool {
        return targetDistance < self.closeRange + jcall!(layerInterface).getHalfHeight();
    }
    

    pub fn isInSensorRange(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>, mut targetDistance: i32) -> bool {
        return targetDistance < self.sensorRange + jcall!(layerInterface).getHalfHeight();
    }
    

    pub fn getCurrentTargetingStateString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        if !match (&self.currentTargetLayerInterfaceP, &CollidableDestroyableDamageableLayer::getNullInstance()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            jcall!(stringBuffer).append(UnitWaypointBehavior2_TARGET_LAYER.with(Clone::clone).clone());
            jcall!(stringBuffer).append({ let java_field = jcall!(self.commonSeps).SPACE.clone(); java_field }.clone());
            jcall!(stringBuffer).append(jcall!(self.currentTargetLayerInterfaceP).getName());
            jcall!(stringBuffer).append(String::from(" with "));
            jcall!(stringBuffer).append(UnitWaypointBehavior2_TARGET_DISTANCE.with(Clone::clone).clone());
            jcall!(stringBuffer).append({ let java_field = jcall!(self.commonSeps).SPACE.clone(); java_field }.clone());
            jcall!(stringBuffer).appendint(self.getCurrentTargetDistance());
        }
        return jcall!(stringBuffer).toString();
    }
    

    // throws Exception
    pub fn addWaypointFromUser(&mut self, mut advancedRTSGameLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn>>>) {
        if jcall!(advancedRTSGameLayer).isDestroyed() {
            panic!("{:?}", format!("{}: {:?}", "Exception", (format!("{}{}", String::from("Trying to add a dead: "), advancedRTSGameLayer))));
        }
        jcall!({ let java_field = jcall!(self.associatedAdvancedRTSGameLayer).waypoint2LogHelperP.clone(); java_field }).addWaypointFromUser(self.associatedAdvancedRTSGameLayer.clone().map(|reference| reference as Rc<RefCell<dyn PathFindingLayerInterface>>), advancedRTSGameLayer.clone().map(|_java_reference| -> Rc<RefCell<dyn PathFindingLayerInterface>> { panic!("ClassCastException: the Rust trait of AdvancedRTSGameLayer does not have PathFindingLayerInterface as a base trait") }));
        jcall!(self.targetList).clear();
        jcall!(self.targetList).add(crate::java_into_object(advancedRTSGameLayer.clone()));
        self.clearTarget();
    }
    

    // throws Exception
    pub fn addBuildingChase(&mut self, mut allbinaryLayer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut cellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) {
        if !jcall!(jcall!(self.buildingSteeringVisitor).getList()).contains(crate::java_into_object(allbinaryLayer.clone())) {
            jcall!(jcall!(self.buildingSteeringVisitor).getList()).add(crate::java_into_object(allbinaryLayer.clone()));
            jcall!(jcall!(self.buildingSteeringVisitor).getPositionList()).add(cellPosition.clone());
        }
        if !jcall!(self.getSteeringVisitorList()).contains(crate::java_into_object(self.buildingSteeringVisitor.clone())) {
            jcall!(self.getSteeringVisitorList()).add(crate::java_into_object(self.buildingSteeringVisitor.clone()));
        }
    }
    

    // throws Exception
    pub fn buildingChase(&mut self, mut allbinaryLayer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut cellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> bool {
        return jcall!(self.associatedAdvancedRTSGameLayer).buildingChase(allbinaryLayer.clone(), cellPosition.clone());
    }
}
impl std::ops::Deref for UnitWaypointBehavior2 {
    type Target = UnitWaypointBehavior;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for UnitWaypointBehavior2 {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::layer::unit::UnitWaypointBehavior::UnitWaypointBehaviorDyn for UnitWaypointBehavior2 {
    fn as_UnitWaypointBehavior(&self) -> &crate::org::allbinary::game::layer::unit::UnitWaypointBehavior::UnitWaypointBehavior { &self.base }
    fn as_UnitWaypointBehavior_mut(&mut self) -> &mut crate::org::allbinary::game::layer::unit::UnitWaypointBehavior::UnitWaypointBehavior { &mut self.base }
    fn initRange(&mut self, mut weaponRange: i32) { Self::initRange(self, weaponRange) }
    fn addWaypointFromUser(&mut self, mut advancedRTSGameLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn>>>) { Self::addWaypointFromUser(self, advancedRTSGameLayer) }
    fn setGeographicMapCellHistoryPath(&mut self, mut geographicMapCellPositionBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { Self::setGeographicMapCellHistoryPath(self, geographicMapCellPositionBasicArrayList) }
    fn addBuildingChase(&mut self, mut allbinaryLayer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut cellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) { Self::addBuildingChase(self, allbinaryLayer, cellPosition) }
}
impl crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn for UnitWaypointBehavior2 {
    fn as_WaypointBehaviorBase(&self) -> &crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBase { &self.base.base }
    fn as_WaypointBehaviorBase_mut(&mut self) -> &mut crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBase { &mut self.base.base }
    fn isRunning(&mut self) -> bool { Self::isRunning(self) }
    fn processTick(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { Self::processTick(self, allBinaryLayerManager) }
    fn getMovementLogicAsString(&mut self) -> String { crate::org::allbinary::game::layer::unit::UnitWaypointBehavior::UnitWaypointBehavior::getMovementLogicAsString(crate::org::allbinary::game::layer::unit::UnitWaypointBehavior::UnitWaypointBehaviorDyn::as_UnitWaypointBehavior_mut(self)) }
    fn isWaypointListEmptyOrOnlyTargets(&mut self) -> bool { Self::isWaypointListEmptyOrOnlyTargets(self) }
    fn isInSensorRange(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>, mut targetDistance: i32) -> bool { Self::isInSensorRange(self, layerInterface, targetDistance) }
    fn getCurrentTargetLayerInterface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>> { crate::org::allbinary::game::layer::unit::UnitWaypointBehavior::UnitWaypointBehavior::getCurrentTargetLayerInterface(crate::org::allbinary::game::layer::unit::UnitWaypointBehavior::UnitWaypointBehaviorDyn::as_UnitWaypointBehavior_mut(self)) }
    fn getCurrentTargetingStateString(&mut self) -> String { Self::getCurrentTargetingStateString(self) }
    fn isTrackingWaypoint(&mut self) -> bool { crate::org::allbinary::game::layer::unit::UnitWaypointBehavior::UnitWaypointBehavior::isTrackingWaypoint(crate::org::allbinary::game::layer::unit::UnitWaypointBehavior::UnitWaypointBehaviorDyn::as_UnitWaypointBehavior_mut(self)) }
    fn getCurrentPathGeographicMapCellPosition(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> { crate::org::allbinary::game::layer::unit::UnitWaypointBehavior::UnitWaypointBehavior::getCurrentPathGeographicMapCellPosition(crate::org::allbinary::game::layer::unit::UnitWaypointBehavior::UnitWaypointBehaviorDyn::as_UnitWaypointBehavior_mut(self)) }
    fn getCurrentGeographicMapCellHistory(&mut self) -> Option<Rc<RefCell<crate::org::allbinary::media::graphics::geography::map::GeographicMapCellHistory::GeographicMapCellHistory>>> { crate::org::allbinary::game::layer::unit::UnitWaypointBehavior::UnitWaypointBehavior::getCurrentGeographicMapCellHistory(crate::org::allbinary::game::layer::unit::UnitWaypointBehavior::UnitWaypointBehaviorDyn::as_UnitWaypointBehavior_mut(self)) }
    fn getNextUnvisitedPathGeographicMapCellPosition(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> { Self::getNextUnvisitedPathGeographicMapCellPosition(self) }
    fn needToMove(&mut self) -> bool { crate::org::allbinary::game::layer::unit::UnitWaypointBehavior::UnitWaypointBehavior::needToMove(crate::org::allbinary::game::layer::unit::UnitWaypointBehavior::UnitWaypointBehaviorDyn::as_UnitWaypointBehavior_mut(self)) }
    fn setWaypointPathsList(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { crate::org::allbinary::game::layer::unit::UnitWaypointBehavior::UnitWaypointBehavior::setWaypointPathsList(crate::org::allbinary::game::layer::unit::UnitWaypointBehavior::UnitWaypointBehaviorDyn::as_UnitWaypointBehavior_mut(self), p1) }
    fn getWaypointPathsList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { crate::org::allbinary::game::layer::unit::UnitWaypointBehavior::UnitWaypointBehavior::getWaypointPathsList(crate::org::allbinary::game::layer::unit::UnitWaypointBehavior::UnitWaypointBehaviorDyn::as_UnitWaypointBehavior_mut(self)) }
    fn clearTarget(&mut self) { Self::clearTarget(self) }
}


pub struct BuildingSteeringVisitor {
    pub base: SteeringVisitor,
    pub positionList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}


impl BuildingSteeringVisitor {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: SteeringVisitor::new_struct(),
            positionList: None,
        };
        this.positionList = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
        

        pub fn visit(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
            let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
            let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                if { let java_value = jcall!(self.getList()).size(); java_value } > 0 {
                    let mut allbinaryLayer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> = (jcall!(self.getList()).get(0)).clone();
                    let mut cellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = (jcall!(self.getPositionList()).get(0)).clone();
                    let mut clear: bool = self.buildingChase(allbinaryLayer.clone(), cellPosition.clone());
                    if clear {
                        jcall!(self.getList()).clear();
                        jcall!(self.positionList).clear();
                        return Some({ let java_field = jcall!(NullUtil::getInstance()).NULL_OBJECT.clone(); java_field }.clone());
                    }
                    return Some(crate::java_into_object({ let java_field = jcall!(BooleanFactory::getInstance()).FALSE.clone(); java_field }.clone()));
                }
                return Some({ let java_field = jcall!(NullUtil::getInstance()).NULL_OBJECT.clone(); java_field }.clone());
            })) {
                Ok(java_return) => java_return,
                Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

                let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
                jcall!(logUtil).put({ let java_field = jcall!(commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("visit"), crate::java_into_object(e.clone()));
                return { let java_field = jcall!(NullUtil::getInstance()).NULL_OBJECT.clone(); java_field }.clone();
                None
                }
            };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
        }
        

        pub fn getPositionList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
            return self.positionList.clone();
        }
}
impl std::ops::Deref for BuildingSteeringVisitor {
    type Target = SteeringVisitor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for BuildingSteeringVisitor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::layer::SteeringVisitor::SteeringVisitorDyn for BuildingSteeringVisitor {
    fn as_SteeringVisitor(&self) -> &crate::org::allbinary::game::layer::SteeringVisitor::SteeringVisitor { &self.base }
    fn as_SteeringVisitor_mut(&mut self) -> &mut crate::org::allbinary::game::layer::SteeringVisitor::SteeringVisitor { &mut self.base }
}
impl crate::org::allbinary::logic::util::visitor::Visitor::VisitorDyn for BuildingSteeringVisitor {
    fn as_Visitor(&self) -> &crate::org::allbinary::logic::util::visitor::Visitor::Visitor { &self.base.base }
    fn as_Visitor_mut(&mut self) -> &mut crate::org::allbinary::logic::util::visitor::Visitor::Visitor { &mut self.base.base }
}
impl crate::org::allbinary::logic::util::visitor::VisitorInterface::VisitorInterface for BuildingSteeringVisitor {
    fn visit(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { Self::visit(self, anyType) }
}


