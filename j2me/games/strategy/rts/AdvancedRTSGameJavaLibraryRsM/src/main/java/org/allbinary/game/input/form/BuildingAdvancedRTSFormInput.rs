// 
// AllBinary Open License Version 1 
// Copyright (c) 2002 AllBinary 
//  
// By agreeing to this license you and any business entity you represent are 
// legally bound to the AllBinary Open License Version 1 legal agreement. 
//  
// You may obtain the AllBinary Open License Version 1 legal agreement from 
// AllBinary or the root directory of AllBinary's AllBinary Platform repository. 
//  
// Created By: Travis Berthelot  
// Generated Code Do Not Modify
// package org.allbinary.game.input.form
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::layer::AdvancedPlayerOwnedRTSLayers::AdvancedPlayerOwnedRTSLayers;
use crate::org::allbinary::game::layer::AdvancedRTSPlayerLayerInterface::AdvancedRTSPlayerLayerInterface;
use crate::org::allbinary::game::layer::RTSLayer::RTSLayer;
use crate::org::allbinary::game::layer::RTSLayerEvent::RTSLayerEvent;
use crate::org::allbinary::game::layer::RTSPlayerLayerInterface::RTSPlayerLayerInterface;
use crate::org::allbinary::game::layer::building::event::BuildingEventHandler::BuildingEventHandler;
use crate::org::allbinary::game::layer::building::event::LocalPlayerBuildingEventHandler::LocalPlayerBuildingEventHandler;
use crate::org::allbinary::game::layer::waypoint::WaypointCellPositionHistory::WaypointCellPositionHistory;
use crate::org::allbinary::logic::java::bool::BooleanFactory::BooleanFactory;
use crate::org::allbinary::game::identification::Group::Group;
use crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManager;
use crate::org::allbinary::game::layer::NullPathFindingLayer::NullPathFindingLayer;
use crate::org::allbinary::game::layer::hud::event::GameNotificationEvent::GameNotificationEvent;
use crate::org::allbinary::game::layer::hud::event::GameNotificationEventHandler::GameNotificationEventHandler;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager;
use crate::org::allbinary::logic::math::SmallIntegerSingletonFactory::SmallIntegerSingletonFactory;
use crate::org::allbinary::media::audio::ErrorSound::ErrorSound;
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMap;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCompositeInterface::GeographicMapCompositeInterface;
use crate::org::allbinary::media::graphics::geography::map::drop::DropCellPositionHistory::DropCellPositionHistory;
use crate::java::lang::Exception::*;
use super::BuildingRTSFormInput::BuildingRTSFormInput;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BuildingAdvancedRTSFormInput {
    pub base: BuildingRTSFormInput,
    pub atLeastOneHouseGameNotificationEvent: Option<Rc<RefCell<GameNotificationEvent>>>,
    pub BUILD_BUILDING_RTS_LAYER_EVENT: Option<Rc<RefCell<RTSLayerEvent>>>,
}


impl BuildingAdvancedRTSFormInput {
    

    pub fn new_struct(mut groupInterface: Vec<Option<Rc<RefCell<Group>>>>, mut isUnitProducer: bool) -> Self {
        let mut this = Self {
            base: BuildingRTSFormInput::new_struct(groupInterface.clone(), isUnitProducer.clone()),
            atLeastOneHouseGameNotificationEvent: None,
            BUILD_BUILDING_RTS_LAYER_EVENT: None,
        };
        this.BUILD_BUILDING_RTS_LAYER_EVENT = RTSLayerEvent::new(crate::org::allbinary::game::layer::NullPathFindingLayer::NullPathFindingLayer_NULL_PATH_FINDING_LAYER.with(Clone::clone).clone().map(|_java_reference| -> Rc<RefCell<dyn crate::org::allbinary::game::layer::PathFindingLayerInterface::PathFindingLayerInterface>> { panic!("ClassCastException: the Rust trait of NullPathFindingLayer does not have PathFindingLayerInterface as a base trait") }));
        // super(...) is in the struct literal above
        let mut basicColorFactory: Option<Rc<RefCell<BasicColorFactory>>> = BasicColorFactory::getInstance();
        this
    }

    pub fn new(mut groupInterface: Vec<Option<Rc<RefCell<Group>>>>, mut isUnitProducer: bool) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(groupInterface.clone(), isUnitProducer.clone()))));
            let mut basicColorFactory: Option<Rc<RefCell<BasicColorFactory>>> = BasicColorFactory::getInstance();
            jcall!(this).atLeastOneHouseGameNotificationEvent = { let java_value = (GameNotificationEvent::new((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()), String::from("Build At Least One House First"), jcall!(SmallIntegerSingletonFactory::getInstance()).getAt(2), { let java_field = jcall!(basicColorFactory).WHITE.clone(); java_field }.clone(), { let java_field = jcall!(BooleanFactory::getInstance()).FALSE.clone(); java_field }.clone())).clone(); java_value };
        this
    }
    

    // throws Exception
    pub fn setAllBinaryGameLayerManager(&mut self, mut allBinaryGameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>) {
        { let java_arguments = (allBinaryGameLayerManager.clone(),); self.base.setAllBinaryGameLayerManager(java_arguments.0) };
        let mut geographicMapCompositeInterface: Option<Rc<RefCell<dyn GeographicMapCompositeInterface>>> = (|| -> Option<Rc<RefCell<dyn GeographicMapCompositeInterface>>> { panic!("ClassCastException: a Rust dyn AllBinaryGameLayerManager cannot be cast to GeographicMapCompositeInterface") })();
        let mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>> = jcall!(geographicMapCompositeInterface).getGeographicMapInterface()[(0) as usize].clone();
        { let java_receiver = self.atLeastOneHouseGameNotificationEvent.clone(); let java_arguments = (jcall!(geographicMapInterface).getForegroundBasicColor(),); jcall!(java_receiver).setBasicColorP(java_arguments.0); };
    }
    

    pub fn isPositionBlocked(&mut self) -> bool {
        if { let java_value = jcall!(DropCellPositionHistory::getInstance()).isCellPositionWithDrop(self.getSelectedGeographicCellPosition().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>)); java_value } || { let java_value = jcall!(WaypointCellPositionHistory::getInstance()).isCellPositionWithDrop(self.getSelectedGeographicCellPosition().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>)); java_value } {
            return true;
        } else {
            return false;
        }
    }
    

    // throws Exception
    pub fn isBuildAttemptValid(&mut self, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>>) -> bool {
        let mut advancedRTSPlayerLayerInterface: Option<Rc<RefCell<dyn AdvancedRTSPlayerLayerInterface>>> = (|| -> Option<Rc<RefCell<dyn AdvancedRTSPlayerLayerInterface>>> { panic!("ClassCastException: a Rust dyn RTSPlayerLayerInterface cannot be cast to dyn AdvancedRTSPlayerLayerInterface") })();
        let mut advancedPlayerOwnedRTSLayers: Option<Rc<RefCell<AdvancedPlayerOwnedRTSLayers>>> = jcall!(advancedRTSPlayerLayerInterface).getAdvancedPlayerOwnedRTSLayers();
        let mut totalHouses: i32 = jcall!(jcall!(advancedPlayerOwnedRTSLayers).getHouseList()).size();
        if totalHouses < 1 && jcall!(layerInterface).getName().find(&String::from(" House")).map(|index| index as i32).unwrap_or(-1) < 0 {
            jcall!(rtsPlayerLayerInterface).add(ErrorSound::getInstance());
            if !jcall!(rtsPlayerLayerInterface).implmentsArtificialIntelligenceCompositeInterface() {
                jcall!(GameNotificationEventHandler::getInstance()).fireEvent(self.atLeastOneHouseGameNotificationEvent.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>));
            }
            return false;
        }
        return { let java_arguments = (rtsPlayerLayerInterface.clone(), layerInterface.clone(),); self.base.isBuildAttemptValid(java_arguments.0, java_arguments.1) };
    }
    

    // throws Exception
    pub fn add(&mut self, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>>) {
        jcall!(self.BUILD_BUILDING_RTS_LAYER_EVENT).setRtsLayer(layerInterface.clone().map(|_java_reference| -> Rc<RefCell<dyn crate::org::allbinary::game::layer::PathFindingLayerInterface::PathFindingLayerInterface>> { panic!("ClassCastException: the Rust trait of RTSLayer does not have PathFindingLayerInterface as a base trait") }));
        jcall!(BuildingEventHandler::getInstance()).fireEvent(self.BUILD_BUILDING_RTS_LAYER_EVENT.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>));
        if jcall!(((|| -> Option<Rc<RefCell<dyn AdvancedRTSPlayerLayerInterface>>> { panic!("ClassCastException: a Rust dyn RTSPlayerLayerInterface cannot be cast to dyn AdvancedRTSPlayerLayerInterface") })())).isLocalPlayer() {
            jcall!(LocalPlayerBuildingEventHandler::getInstance()).fireEvent(self.BUILD_BUILDING_RTS_LAYER_EVENT.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>));
        }
        { let java_arguments = (rtsPlayerLayerInterface.clone(), layerManager.clone(), layerInterface.clone(),); self.base.add(java_arguments.0, java_arguments.1, java_arguments.2) };
    }
}
impl std::ops::Deref for BuildingAdvancedRTSFormInput {
    type Target = BuildingRTSFormInput;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for BuildingAdvancedRTSFormInput {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::input::form::BuildingRTSFormInput::BuildingRTSFormInputDyn for BuildingAdvancedRTSFormInput {
    fn as_BuildingRTSFormInput(&self) -> &crate::org::allbinary::game::input::form::BuildingRTSFormInput::BuildingRTSFormInput { &self.base }
    fn as_BuildingRTSFormInput_mut(&mut self) -> &mut crate::org::allbinary::game::input::form::BuildingRTSFormInput::BuildingRTSFormInput { &mut self.base }
    fn isPositionBlocked(&mut self) -> bool { Self::isPositionBlocked(self) }
    fn isBuildAttemptValid(&mut self, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>>) -> bool { Self::isBuildAttemptValid(self, rtsPlayerLayerInterface, layerInterface) }
    fn add(&mut self, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>>) { Self::add(self, rtsPlayerLayerInterface, layerManager, layerInterface) }
}
impl crate::org::allbinary::game::input::form::RTSFormInput::RTSFormInputDyn for BuildingAdvancedRTSFormInput {
    fn as_RTSFormInput(&self) -> &crate::org::allbinary::game::input::form::RTSFormInput::RTSFormInput { &self.base.base }
    fn as_RTSFormInput_mut(&mut self) -> &mut crate::org::allbinary::game::input::form::RTSFormInput::RTSFormInput { &mut self.base.base }
    fn setAllBinaryGameLayerManager(&mut self, mut allBinaryGameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>) { Self::setAllBinaryGameLayerManager(self, allBinaryGameLayerManager) }
    fn processGameSpecific(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>, p2: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSPlayerLayerInterface::RTSPlayerLayerInterface>>>, p3: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, p4: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, p5: i32) { crate::org::allbinary::game::input::form::BuildingRTSFormInput::BuildingRTSFormInput::processGameSpecific(crate::org::allbinary::game::input::form::BuildingRTSFormInput::BuildingRTSFormInputDyn::as_BuildingRTSFormInput_mut(self), p1, p2, p3, p4, p5) }
}


