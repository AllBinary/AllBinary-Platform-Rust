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
// package org.allbinary.game.input.form
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::game::input::RTSPlayerGameInput::RTSPlayerGameInput;
use crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer;
use crate::org::allbinary::game::layer::AdvancedRTSPlayerLayerInterface::AdvancedRTSPlayerLayerInterface;
use crate::org::allbinary::game::layer::RTSGameStrings::RTSGameStrings;
use crate::org::allbinary::game::layer::RTSLayer::RTSLayer;
use crate::org::allbinary::game::layer::RTSLayerEvent::RTSLayerEvent;
use crate::org::allbinary::game::layer::RTSLayerUtil::RTSLayerUtil;
use crate::org::allbinary::game::layer::RTSPlayerLayerInterface::RTSPlayerLayerInterface;
use crate::org::allbinary::game::layer::capital::Capital::Capital;
use crate::org::allbinary::game::layer::waypoint::WaypointCellPositionHistory::WaypointCellPositionHistory;
use crate::org::allbinary::game::layer::waypoint::WaypointLayer::WaypointLayer;
use crate::org::allbinary::game::layer::waypoint::event::WaypointEventHandlerFactory::WaypointEventHandlerFactory;
use crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItem;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::logic::java::bool::BooleanFactory::BooleanFactory;
use crate::org::allbinary::game::identification::Group::Group;
use crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManager;
use crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayer;
use crate::org::allbinary::game::layer::GeographicMapCellPositionAreaBase::GeographicMapCellPositionAreaBase;
use crate::org::allbinary::game::layer::NullPathFindingLayer::NullPathFindingLayer;
use crate::org::allbinary::game::layer::NullRTSLayer::NullRTSLayer;
use crate::org::allbinary::game::layer::PathFindingLayerInterface::PathFindingLayerInterface;
use crate::org::allbinary::game::layer::hud::event::GameNotificationEvent::GameNotificationEvent;
use crate::org::allbinary::game::layer::hud::event::GameNotificationEventHandler::GameNotificationEventHandler;
use crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer;
use crate::org::allbinary::graphics::GPoint::GPoint;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager;
use crate::org::allbinary::layer::Layer::Layer;
use crate::org::allbinary::logic::math::SmallIntegerSingletonFactory::SmallIntegerSingletonFactory;
use crate::org::allbinary::media::audio::ErrorSound::ErrorSound;
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMap;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPosition;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellType;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCompositeInterface::GeographicMapCompositeInterface;
use crate::org::allbinary::media::graphics::geography::map::drop::DropCellPositionHistory::DropCellPositionHistory;
use crate::org::allbinary::media::graphics::geography::map::racetrack::RaceTrackGeographicMapCellTypeFactory::RaceTrackGeographicMapCellTypeFactory;
use super::RTSFormInput::RTSFormInput;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct WaypointRTSFormInput {
    pub base: RTSFormInput,
    pub WAYPOINT_EVENT: Option<Rc<RefCell<RTSLayerEvent>>>,
    pub dragToSpotGameNotificationEvent: Option<Rc<RefCell<GameNotificationEvent>>>,
    pub spotTakenGameNotificationEvent: Option<Rc<RefCell<GameNotificationEvent>>>,
    pub buildingCollisionGameNotificationEvent: Option<Rc<RefCell<GameNotificationEvent>>>,
    pub roadCollisionGameNotificationEvent: Option<Rc<RefCell<GameNotificationEvent>>>,
    pub newWaypointGameNotificationEvent: Option<Rc<RefCell<GameNotificationEvent>>>,
    pub noMoneyGameNotificationEvent: Option<Rc<RefCell<GameNotificationEvent>>>,
    pub isPrimaryWaypointCreator: bool,
    pub stickyAssociatedRtsLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>,
}


impl WaypointRTSFormInput {
    

    pub fn new_struct(mut groupInterface: Vec<Option<Rc<RefCell<Group>>>>, mut isPrimaryWaypointCreator: bool) -> Self {
        let mut this = Self {
            base: RTSFormInput::new_struct(groupInterface.clone()),
            WAYPOINT_EVENT: None,
            dragToSpotGameNotificationEvent: None,
            spotTakenGameNotificationEvent: None,
            buildingCollisionGameNotificationEvent: None,
            roadCollisionGameNotificationEvent: None,
            newWaypointGameNotificationEvent: None,
            noMoneyGameNotificationEvent: None,
            isPrimaryWaypointCreator: false,
            stickyAssociatedRtsLayer: None,
        };
        this.WAYPOINT_EVENT = RTSLayerEvent::new(crate::org::allbinary::game::layer::NullRTSLayer::NullRTSLayer_NULL_RTS_LAYER.with(Clone::clone).clone());
        this.stickyAssociatedRtsLayer = crate::org::allbinary::game::layer::NullPathFindingLayer::NullPathFindingLayer_NULL_PATH_FINDING_LAYER.with(Clone::clone).clone();
        // super(...) is in the struct literal above
        this.isPrimaryWaypointCreator = isPrimaryWaypointCreator.clone();
        let mut smallIntegerSingletonFactory: Option<Rc<RefCell<SmallIntegerSingletonFactory>>> = SmallIntegerSingletonFactory::getInstance();
        let mut basicColorFactory: Option<Rc<RefCell<BasicColorFactory>>> = BasicColorFactory::getInstance();
        this
    }

    pub fn new(mut groupInterface: Vec<Option<Rc<RefCell<Group>>>>, mut isPrimaryWaypointCreator: bool) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(groupInterface, isPrimaryWaypointCreator))));
            this.dragToSpotGameNotificationEvent = GameNotificationEvent::new((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()), jcall!(RTSGameStrings::getInstance()).DRAG_TO_SPOT.clone(), jcall!(smallIntegerSingletonFactory).getAt(2), jcall!(basicColorFactory).WHITE.clone(), jcall!(BooleanFactory::getInstance()).FALSE.clone());
            this.spotTakenGameNotificationEvent = GameNotificationEvent::new((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()), jcall!(RTSGameStrings::getInstance()).SPOT_TAKEN.clone(), jcall!(smallIntegerSingletonFactory).getAt(2), jcall!(basicColorFactory).WHITE.clone(), jcall!(BooleanFactory::getInstance()).FALSE.clone());
            this.buildingCollisionGameNotificationEvent = GameNotificationEvent::new((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()), jcall!(RTSGameStrings::getInstance()).BUILDING_COLLISION.clone(), jcall!(smallIntegerSingletonFactory).getAt(2), jcall!(basicColorFactory).WHITE.clone(), jcall!(BooleanFactory::getInstance()).FALSE.clone());
            this.roadCollisionGameNotificationEvent = GameNotificationEvent::new((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()), jcall!(RTSGameStrings::getInstance()).ROAD_COLLISION.clone(), jcall!(smallIntegerSingletonFactory).getAt(2), jcall!(basicColorFactory).WHITE.clone(), jcall!(BooleanFactory::getInstance()).FALSE.clone());
            this.newWaypointGameNotificationEvent = GameNotificationEvent::new((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()), jcall!(RTSGameStrings::getInstance()).NEW_WAYPOINT.clone(), jcall!(smallIntegerSingletonFactory).getAt(2), jcall!(basicColorFactory).WHITE.clone(), jcall!(BooleanFactory::getInstance()).FALSE.clone());
            this.noMoneyGameNotificationEvent = GameNotificationEvent::new((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()), jcall!(RTSGameStrings::getInstance()).NO_MONEY.clone(), jcall!(smallIntegerSingletonFactory).getAt(2), jcall!(basicColorFactory).WHITE.clone(), jcall!(BooleanFactory::getInstance()).FALSE.clone());
        this
    }
    

    // throws Exception
    pub fn setAllBinaryGameLayerManager(&mut self, mut allBinaryGameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>) {
        self.base.setAllBinaryGameLayerManager(allBinaryGameLayerManager.clone());
        let mut geographicMapCompositeInterface: Option<Rc<RefCell<dyn GeographicMapCompositeInterface>>> = (allBinaryGameLayerManager).clone() /* as GeographicMapCompositeInterface */;
        let mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>> = jcall!(geographicMapCompositeInterface).getGeographicMapInterface()[(0) as usize].clone();
        { let java_receiver = self.dragToSpotGameNotificationEvent.clone(); let java_arguments = (jcall!(geographicMapInterface).getForegroundBasicColor(),); jcall!(java_receiver).setBasicColorP(java_arguments.0); };
        { let java_receiver = self.spotTakenGameNotificationEvent.clone(); let java_arguments = (jcall!(geographicMapInterface).getForegroundBasicColor(),); jcall!(java_receiver).setBasicColorP(java_arguments.0); };
        { let java_receiver = self.buildingCollisionGameNotificationEvent.clone(); let java_arguments = (jcall!(geographicMapInterface).getForegroundBasicColor(),); jcall!(java_receiver).setBasicColorP(java_arguments.0); };
        { let java_receiver = self.roadCollisionGameNotificationEvent.clone(); let java_arguments = (jcall!(geographicMapInterface).getForegroundBasicColor(),); jcall!(java_receiver).setBasicColorP(java_arguments.0); };
        { let java_receiver = self.newWaypointGameNotificationEvent.clone(); let java_arguments = (jcall!(geographicMapInterface).getForegroundBasicColor(),); jcall!(java_receiver).setBasicColorP(java_arguments.0); };
        { let java_receiver = self.noMoneyGameNotificationEvent.clone(); let java_arguments = (jcall!(geographicMapInterface).getForegroundBasicColor(),); jcall!(java_receiver).setBasicColorP(java_arguments.0); };
    }
    

    // throws Exception
    pub fn processAtPoint(&mut self, mut associatedRtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>) {
        self.base.process(layerManager.clone());
        if self.isStickyItemSelected() || match (&associatedRtsLayer, &CollidableDestroyableDamageableLayer::getNullInstance()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            if self.isStickyItemSelected() && match (&associatedRtsLayer, &CollidableDestroyableDamageableLayer::getNullInstance()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                let mut geographicMapCompositeInterface: Option<Rc<RefCell<dyn GeographicMapCompositeInterface>>> = (layerManager).clone() /* as GeographicMapCompositeInterface */;
                let mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>> = jcall!(geographicMapCompositeInterface).getGeographicMapInterface()[(0) as usize].clone();
                let mut allBinaryTiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>> = jcall!(geographicMapInterface).getAllBinaryTiledLayer();
                let mut x: i32 = jcall!(point).getX() + jcall!(allBinaryTiledLayer).getXP();
                let mut y: i32 = jcall!(point).getY() + jcall!(allBinaryTiledLayer).getYP();
                let mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = jcall!(geographicMapInterface).getCellPositionAtXY(x.clone(), y.clone());
                self.processCellPosition(rtsPlayerLayerInterface.clone(), layerManager.clone(), geographicMapCellPosition.clone());
            } else {
                if !jcall!(rtsPlayerLayerInterface).implmentsArtificialIntelligenceCompositeInterface() {
                    jcall!(GameNotificationEventHandler::getInstance()).fireEvent(self.dragToSpotGameNotificationEvent.clone());
                }
            }
        } else {
            if !jcall!(rtsPlayerLayerInterface).implmentsArtificialIntelligenceCompositeInterface() {
                jcall!(GameNotificationEventHandler::getInstance()).fireEvent(self.spotTakenGameNotificationEvent.clone());
            }
        }
    }
    

    // throws Exception
    pub fn processCellPosition(&mut self, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) {
        let mut itemIndex: i32 = self.getSelectedStickyItemIndex();
        if match (&self.newUnconstructedRTSLayerInterfaceArray[(itemIndex) as usize], &CollidableDestroyableDamageableLayer::getNullInstance()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            self.newUnconstructedRTSLayerInterfaceArray[(itemIndex) as usize] = { let java_arguments = (layerManager.clone(), self.getSelectedStickyItem(), geographicMapCellPosition.clone(),); self.getInstance(java_arguments.0, java_arguments.1, java_arguments.2) };
        } else {
            let mut cellPoint: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = jcall!(geographicMapCellPosition).getPoint();
            let mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>> = (self.newUnconstructedRTSLayerInterfaceArray[(itemIndex) as usize]).clone() /* as RTSLayer */;
            jcall!(rtsLayer).setPosition(jcall!(cellPoint).getX(), jcall!(cellPoint).getY(), jcall!(rtsLayer).getZP());
            let mut geographicMapCompositeInterface: Option<Rc<RefCell<dyn GeographicMapCompositeInterface>>> = (jcall!(rtsLayer).as_RTSLayer_mut().allBinaryGameLayerManagerP).clone() /* as GeographicMapCompositeInterface */;
            let mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>> = jcall!(geographicMapCompositeInterface).getGeographicMapInterface()[(0) as usize].clone();
            jcall!(jcall!(rtsLayer).as_RTSLayer_mut().geographicMapCellPositionAreaBase).update(geographicMapInterface.clone());
        }
        self.attemptBuild(rtsPlayerLayerInterface.clone(), layerManager.clone(), (self.newUnconstructedRTSLayerInterfaceArray[(itemIndex) as usize]).clone() /* as RTSLayer */, itemIndex.clone());
    }
    

    // throws Exception
    pub fn processStickyGameSpecific(&mut self, mut associatedRtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut index: i32) {
        jcall!(self.logUtil).putF(format!("{}{}", String::from("Set Sticking Item: "), item), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("processSticky"));
        self.setSelectedStickyItem(item.clone());
        self.setSelectedStickyItemIndex(index.clone());
        self.setStickyItemSelected(true);
        self.stickyAssociatedRtsLayer = (associatedRtsLayer).clone() /* as RTSLayer */;
        jcall!(self.getHashtable()).put(crate::org::allbinary::layer::Layer::Layer_ID.with(Clone::clone).clone(), associatedRtsLayer.clone());
    }
    

    // throws Exception
    pub fn attemptBuild(&mut self, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>>, mut itemIndex: i32) -> bool {
        jcall!(self.logUtil).putF(format!("{}{}", String::from("Layer: "), layerInterface), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("attemptBuild"));
        if layerInterface.is_none() {
            jcall!(self.logUtil).put(String::from("Layer was null"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("attemptBuild"), crate::java_into_object(format!("{}: {:?}", "Exception", ())));
            return false;
        }
        let mut geographicMapCellPositionArea: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::GeographicMapCellPositionAreaBase::GeographicMapCellPositionAreaBaseDyn>>> = jcall!(layerInterface).as_RTSLayer_mut().geographicMapCellPositionAreaBase.clone();
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(geographicMapCellPositionArea).getOccupyingGeographicMapCellPositionList();
        if jcall!(DropCellPositionHistory::getInstance()).anyCellPositionWithDrop(list.clone()) || jcall!(WaypointCellPositionHistory::getInstance()).anyCellPositionWithDrop(list.clone()) {
            jcall!(rtsPlayerLayerInterface).add(ErrorSound::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>));
            if !jcall!(rtsPlayerLayerInterface).implmentsArtificialIntelligenceCompositeInterface() {
                jcall!(GameNotificationEventHandler::getInstance()).fireEvent(self.buildingCollisionGameNotificationEvent.clone());
            }
            return false;
        }
        let mut geographicMapCompositeInterface: Option<Rc<RefCell<dyn GeographicMapCompositeInterface>>> = (layerManager).clone() /* as GeographicMapCompositeInterface */;
        let mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>> = jcall!(geographicMapCompositeInterface).getGeographicMapInterface()[(0) as usize].clone();
        let mut raceTrackGeographicMapCellTypeFactory: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::racetrack::RaceTrackGeographicMapCellTypeFactory::RaceTrackGeographicMapCellTypeFactoryDyn>>> = (jcall!(geographicMapInterface).getGeographicMapCellTypeFactory()).clone() /* as RaceTrackGeographicMapCellTypeFactory */;
        let mut geographicMapCellType: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellTypeDyn>>> = None;
        {
            let mut index: i32 = jcall!(list).size();
            while { index -= 1; index } >= 0 {
            geographicMapCellType = jcall!(geographicMapInterface).getCellTypeAt({ let java_object = (jcall!(list).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>>().expect("ClassCastException").clone()) });
            if jcall!(raceTrackGeographicMapCellTypeFactory).isPath(geographicMapCellType.clone()) {
                jcall!(rtsPlayerLayerInterface).add(ErrorSound::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>));
                if !jcall!(rtsPlayerLayerInterface).implmentsArtificialIntelligenceCompositeInterface() {
                    jcall!(GameNotificationEventHandler::getInstance()).fireEvent(self.roadCollisionGameNotificationEvent.clone());
                }
                return false;
            }
            }
        }
        let mut cost: i32 = jcall!(layerInterface).getCost();
        let mut capital: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::capital::Capital::CapitalDyn>>> = jcall!(rtsPlayerLayerInterface).getCapital();
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(String::from("Trying to Build: "));
        jcall!(stringBuffer).append(jcall!(layerInterface).getName());
        jcall!(stringBuffer).append(String::from(" for: $"));
        jcall!(stringBuffer).appendint(cost.clone());
        jcall!(stringBuffer).append(String::from(" with "));
        jcall!(stringBuffer).appendint(jcall!(capital).getTotalMoney());
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("attemptBuild"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        if cost <= jcall!(capital).getTotalMoney() {
            jcall!(layerInterface).construct(rtsPlayerLayerInterface.clone());
            self.newUnconstructedRTSLayerInterfaceArray[(itemIndex) as usize] = CollidableDestroyableDamageableLayer::getNullInstance();
            jcall!(capital).removeMoney(cost.clone());
            jcall!(WaypointCellPositionHistory::getInstance()).addList(list.clone(), layerInterface.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>));
            let mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = jcall!(geographicMapInterface).getCellPositionAtXY(jcall!(layerInterface).getXP(), jcall!(layerInterface).getYP());
            let mut waypointLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::waypoint::WaypointLayer::WaypointLayerDyn>>> = (layerInterface).clone() /* as WaypointLayer */;
            self.addWayPoint(waypointLayer.clone());
            let mut rtsPlayerGameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::RTSPlayerGameInput::RTSPlayerGameInputDyn>>> = (jcall!(rtsPlayerLayerInterface).getPlayerGameInput()).clone() /* as RTSPlayerGameInput */;
            jcall!(rtsPlayerGameInput).setSelectedRTSLayer(layerInterface.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>), geographicMapCellPosition.clone());
            let mut advancedRTSPlayerLayerInterface: Option<Rc<RefCell<dyn AdvancedRTSPlayerLayerInterface>>> = panic!("ClassCastException: a Rust dyn RTSPlayerLayerInterface cannot be cast to dyn AdvancedRTSPlayerLayerInterface");
            jcall!(jcall!(advancedRTSPlayerLayerInterface).getAdvancedPlayerOwnedRTSLayers()).addWaypoint(layerInterface.clone());
            jcall!(layerManager).append(layerInterface.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>));
            let mut waypointAvancedRTSGameLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn>>> = (waypointLayer).clone() /* as AdvancedRTSGameLayer */;
            jcall!(rtsPlayerLayerInterface).add(jcall!(jcall!(jcall!(waypointAvancedRTSGameLayer).getWaypointBehavior()).getWaypoint()).getSound());
            if !jcall!(rtsPlayerLayerInterface).implmentsArtificialIntelligenceCompositeInterface() {
                jcall!(GameNotificationEventHandler::getInstance()).fireEvent(self.newWaypointGameNotificationEvent.clone());
            }
            return true;
        } else {
            jcall!(rtsPlayerLayerInterface).add(ErrorSound::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>));
            if !jcall!(rtsPlayerLayerInterface).implmentsArtificialIntelligenceCompositeInterface() {
                jcall!(GameNotificationEventHandler::getInstance()).fireEvent(self.noMoneyGameNotificationEvent.clone());
            }
            return false;
        }
    }
    

    // throws Exception
    pub fn addWayPoint(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::waypoint::WaypointLayer::WaypointLayerDyn>>>) {
        let mut stickyAssociatedAdvandedRtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn>>> = (self.stickyAssociatedRtsLayer).clone() /* as AdvancedRTSGameLayer */;
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(jcall!(stickyAssociatedAdvandedRtsLayer).getWaypointBehavior()).getOwnedWaypointList();
        jcall!(RTSLayerUtil::getInstance()).destroyAndClear(list.clone());
        jcall!(list).add(crate::java_into_object(layerInterface.clone()));
        if self.isPrimaryWaypointCreator {
            jcall!(PrimaryWaypointHelper::PrimaryWaypointHelper::getInstance()).setWaypointLayer(layerInterface.clone());
        }
        jcall!(self.WAYPOINT_EVENT).setRtsLayer(layerInterface.clone().map(|reference| reference as Rc<RefCell<dyn PathFindingLayerInterface>>));
        jcall!(WaypointEventHandlerFactory::getInstance(jcall!(layerInterface).getGroupInterface()[(0) as usize].clone())).fireEvent(self.WAYPOINT_EVENT.clone());
    }
}
impl std::ops::Deref for WaypointRTSFormInput {
    type Target = RTSFormInput;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for WaypointRTSFormInput {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::input::form::RTSFormInput::RTSFormInputDyn for WaypointRTSFormInput {
    fn as_RTSFormInput(&self) -> &crate::org::allbinary::game::input::form::RTSFormInput::RTSFormInput { &self.base }
    fn as_RTSFormInput_mut(&mut self) -> &mut crate::org::allbinary::game::input::form::RTSFormInput::RTSFormInput { &mut self.base }
    fn setAllBinaryGameLayerManager(&mut self, mut allBinaryGameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>) { Self::setAllBinaryGameLayerManager(self, allBinaryGameLayerManager) }
    fn processAtPoint(&mut self, mut associatedRtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>) { Self::processAtPoint(self, associatedRtsLayer, rtsPlayerLayerInterface, layerManager, point) }
    fn processStickyGameSpecific(&mut self, mut associatedRtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut index: i32) { Self::processStickyGameSpecific(self, associatedRtsLayer, rtsPlayerLayerInterface, layerManager, item, index) }
}


