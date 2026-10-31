// 
// AllBinary Open License Version 1 
// Copyright (c) 2003 AllBinary 
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
use crate::org::allbinary::game::input::RTSPlayerGameInput::RTSPlayerGameInput;
use crate::org::allbinary::game::layer::DemoLockedWithCostLayerInterfaceFactoryInterface::DemoLockedWithCostLayerInterfaceFactoryInterface;
use crate::org::allbinary::game::layer::GeographicMapCellPositionAreaBase::GeographicMapCellPositionAreaBase;
use crate::org::allbinary::game::layer::RTSGameStrings::RTSGameStrings;
use crate::org::allbinary::game::layer::RTSLayer::RTSLayer;
use crate::org::allbinary::game::layer::RTSPlayerLayerInterface::RTSPlayerLayerInterface;
use crate::org::allbinary::game::layer::capital::Capital::Capital;
use crate::org::allbinary::game::layer::item::LayerInterfaceFactoryImageItem::LayerInterfaceFactoryImageItem;
use crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItem;
use crate::org::allbinary::media::audio::BuildingSound::BuildingSound;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::java::bool::BooleanFactory::BooleanFactory;
use crate::org::allbinary::logic::system::security::licensing::LockedFeatureNotificationUtil::LockedFeatureNotificationUtil;
use crate::org::allbinary::logic::system::security::licensing::LockedUtil::LockedUtil;
use crate::org::allbinary::game::identification::Group::Group;
use crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManager;
use crate::org::allbinary::game::layer::hud::event::GameNotificationEvent::GameNotificationEvent;
use crate::org::allbinary::game::layer::hud::event::GameNotificationEventHandler::GameNotificationEventHandler;
use crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer;
use crate::org::allbinary::graphics::GPoint::GPoint;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager;
use crate::org::allbinary::logic::math::SmallIntegerSingletonFactory::SmallIntegerSingletonFactory;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::media::audio::ErrorSound::ErrorSound;
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMap;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPosition;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellType;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCompositeInterface::GeographicMapCompositeInterface;
use crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory;
use crate::org::allbinary::media::graphics::geography::map::drop::DropCellPositionHistory::DropCellPositionHistory;
use crate::org::allbinary::media::graphics::geography::map::racetrack::RaceTrackGeographicMapCellTypeFactory::RaceTrackGeographicMapCellTypeFactory;
use super::RTSFormInput::RTSFormInput;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BuildingRTSFormInput {
    pub base: RTSFormInput,
    pub isUnitProducer: bool,
    pub buildOnPathGameNotificationEvent: Option<Rc<RefCell<GameNotificationEvent>>>,
    pub spotTakenGameNotificationEvent: Option<Rc<RefCell<GameNotificationEvent>>>,
    pub selectBuildSpotGameNotificationEvent: Option<Rc<RefCell<GameNotificationEvent>>>,
    pub roadCollisionGameNotificationEvent: Option<Rc<RefCell<GameNotificationEvent>>>,
    pub noMoneyGameNotificationEvent: Option<Rc<RefCell<GameNotificationEvent>>>,
    pub buildingCollisionGameNotificationEvent: Option<Rc<RefCell<GameNotificationEvent>>>,
    pub structureToCloseCollisionGameNotificationEvent: Option<Rc<RefCell<GameNotificationEvent>>>,
    pub buildingGameNotificationEvent: Option<Rc<RefCell<GameNotificationEvent>>>,
    pub mapEdgeGameNotificationEvent: Option<Rc<RefCell<GameNotificationEvent>>>,
    pub dropCellPositionHistory: Option<Rc<RefCell<DropCellPositionHistory>>>,
}


impl BuildingRTSFormInput {
    

    pub fn new_struct(mut groupInterface: Vec<Option<Rc<RefCell<Group>>>>, mut isUnitProducer: bool) -> Self {
        let mut this = Self {
            base: RTSFormInput::new_struct(groupInterface.clone()),
            isUnitProducer: false,
            buildOnPathGameNotificationEvent: None,
            spotTakenGameNotificationEvent: None,
            selectBuildSpotGameNotificationEvent: None,
            roadCollisionGameNotificationEvent: None,
            noMoneyGameNotificationEvent: None,
            buildingCollisionGameNotificationEvent: None,
            structureToCloseCollisionGameNotificationEvent: None,
            buildingGameNotificationEvent: None,
            mapEdgeGameNotificationEvent: None,
            dropCellPositionHistory: None,
        };
        this.dropCellPositionHistory = DropCellPositionHistory::getInstance();
        // super(...) is in the struct literal above
        this.isUnitProducer = isUnitProducer.clone();
        let mut smallIntegerSingletonFactory: Option<Rc<RefCell<SmallIntegerSingletonFactory>>> = SmallIntegerSingletonFactory::getInstance();
        let mut basicColorFactory: Option<Rc<RefCell<BasicColorFactory>>> = BasicColorFactory::getInstance();
        this
    }

    pub fn new(mut groupInterface: Vec<Option<Rc<RefCell<Group>>>>, mut isUnitProducer: bool) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(groupInterface, isUnitProducer))));
            this.buildOnPathGameNotificationEvent = GameNotificationEvent::new((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()), jcall!(RTSGameStrings::getInstance()).BUILD_ON_PATH.clone(), jcall!(smallIntegerSingletonFactory).getAt(2), jcall!(basicColorFactory).PINK.clone(), jcall!(BooleanFactory::getInstance()).FALSE.clone());
            this.spotTakenGameNotificationEvent = GameNotificationEvent::new((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()), jcall!(RTSGameStrings::getInstance()).SPOT_TAKEN.clone(), jcall!(smallIntegerSingletonFactory).getAt(2), jcall!(basicColorFactory).PINK.clone(), jcall!(BooleanFactory::getInstance()).FALSE.clone());
            this.selectBuildSpotGameNotificationEvent = GameNotificationEvent::new((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()), jcall!(RTSGameStrings::getInstance()).SELECT_BUILD_SPOT.clone(), jcall!(smallIntegerSingletonFactory).getAt(2), jcall!(basicColorFactory).PINK.clone(), jcall!(BooleanFactory::getInstance()).FALSE.clone());
            this.roadCollisionGameNotificationEvent = GameNotificationEvent::new((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()), jcall!(RTSGameStrings::getInstance()).ROAD_COLLISION.clone(), jcall!(smallIntegerSingletonFactory).getAt(2), jcall!(basicColorFactory).PINK.clone(), jcall!(BooleanFactory::getInstance()).FALSE.clone());
            this.noMoneyGameNotificationEvent = GameNotificationEvent::new((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()), jcall!(RTSGameStrings::getInstance()).NO_MONEY.clone(), jcall!(smallIntegerSingletonFactory).getAt(2), jcall!(basicColorFactory).PINK.clone(), jcall!(BooleanFactory::getInstance()).FALSE.clone());
            this.buildingCollisionGameNotificationEvent = GameNotificationEvent::new((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()), jcall!(RTSGameStrings::getInstance()).BUILDING_COLLISION.clone(), jcall!(smallIntegerSingletonFactory).getAt(2), jcall!(basicColorFactory).PINK.clone(), jcall!(BooleanFactory::getInstance()).FALSE.clone());
            this.structureToCloseCollisionGameNotificationEvent = GameNotificationEvent::new((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()), jcall!(RTSGameStrings::getInstance()).STRUCTURE_TO_CLOSE.clone(), jcall!(smallIntegerSingletonFactory).getAt(2), jcall!(basicColorFactory).PINK.clone(), jcall!(BooleanFactory::getInstance()).FALSE.clone());
            this.buildingGameNotificationEvent = GameNotificationEvent::new((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()), jcall!(RTSGameStrings::getInstance()).BUILDING.clone(), jcall!(smallIntegerSingletonFactory).getAt(2), jcall!(basicColorFactory).PINK.clone(), jcall!(BooleanFactory::getInstance()).FALSE.clone());
            this.mapEdgeGameNotificationEvent = GameNotificationEvent::new((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()), jcall!(RTSGameStrings::getInstance()).MAP_EDGE.clone(), jcall!(smallIntegerSingletonFactory).getAt(2), jcall!(basicColorFactory).PINK.clone(), jcall!(BooleanFactory::getInstance()).FALSE.clone());
        this
    }
    

    // throws Exception
    pub fn setAllBinaryGameLayerManager(&mut self, mut allBinaryGameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>) {
        self.base.setAllBinaryGameLayerManager(allBinaryGameLayerManager.clone());
        let mut geographicMapCompositeInterface: Option<Rc<RefCell<dyn GeographicMapCompositeInterface>>> = (allBinaryGameLayerManager).clone() /* as GeographicMapCompositeInterface */;
        let mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>> = jcall!(geographicMapCompositeInterface).getGeographicMapInterface()[(0) as usize].clone();
        { let java_receiver = self.buildOnPathGameNotificationEvent.clone(); let java_arguments = (jcall!(geographicMapInterface).getForegroundBasicColor(),); jcall!(java_receiver).setBasicColorP(java_arguments.0); };
        { let java_receiver = self.spotTakenGameNotificationEvent.clone(); let java_arguments = (jcall!(geographicMapInterface).getForegroundBasicColor(),); jcall!(java_receiver).setBasicColorP(java_arguments.0); };
        { let java_receiver = self.selectBuildSpotGameNotificationEvent.clone(); let java_arguments = (jcall!(geographicMapInterface).getForegroundBasicColor(),); jcall!(java_receiver).setBasicColorP(java_arguments.0); };
        { let java_receiver = self.roadCollisionGameNotificationEvent.clone(); let java_arguments = (jcall!(geographicMapInterface).getForegroundBasicColor(),); jcall!(java_receiver).setBasicColorP(java_arguments.0); };
        { let java_receiver = self.noMoneyGameNotificationEvent.clone(); let java_arguments = (jcall!(geographicMapInterface).getForegroundBasicColor(),); jcall!(java_receiver).setBasicColorP(java_arguments.0); };
        { let java_receiver = self.buildingCollisionGameNotificationEvent.clone(); let java_arguments = (jcall!(geographicMapInterface).getForegroundBasicColor(),); jcall!(java_receiver).setBasicColorP(java_arguments.0); };
        { let java_receiver = self.structureToCloseCollisionGameNotificationEvent.clone(); let java_arguments = (jcall!(geographicMapInterface).getForegroundBasicColor(),); jcall!(java_receiver).setBasicColorP(java_arguments.0); };
        { let java_receiver = self.buildingGameNotificationEvent.clone(); let java_arguments = (jcall!(geographicMapInterface).getForegroundBasicColor(),); jcall!(java_receiver).setBasicColorP(java_arguments.0); };
        { let java_receiver = self.mapEdgeGameNotificationEvent.clone(); let java_arguments = (jcall!(geographicMapInterface).getForegroundBasicColor(),); jcall!(java_receiver).setBasicColorP(java_arguments.0); };
    }
    

    // throws Exception
    pub fn processGameSpecific(&mut self, mut associatedRtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut itemIndex: i32) {
        self.base.process(layerManager.clone());
        self.buildFromMotionInput(rtsPlayerLayerInterface.clone(), layerManager.clone(), item.clone(), itemIndex.clone());
    }
    

    pub fn isPositionBlocked(&mut self) -> bool {
        return { let java_receiver = self.dropCellPositionHistory.clone(); let java_arguments = (self.getSelectedGeographicCellPosition(),); let java_result = jcall!(java_receiver).isCellPositionWithDrop(java_arguments.0); java_result };
    }
    

    pub fn isDemoLocked(&mut self, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>) -> bool {
        let mut factoryItem: Option<Rc<RefCell<LayerInterfaceFactoryImageItem>>> = (item).clone() /* as LayerInterfaceFactoryImageItem */;
        let mut layerInterfaceFactoryInterface: Option<Rc<RefCell<dyn DemoLockedWithCostLayerInterfaceFactoryInterface>>> = (jcall!(factoryItem).getLayerInterfaceFactoryInterface()).clone() /* as DemoLockedWithCostLayerInterfaceFactoryInterface */;
        if jcall!(LockedUtil::getInstance()).isLocked() && jcall!(layerInterfaceFactoryInterface).isDemoLocked() {
            return true;
        } else {
            return false;
        }
    }
    

    // throws Exception
    pub fn buildFromMotionInput(&mut self, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut itemIndex: i32) {
        let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
        jcall!(self.logUtil).putF(jcall!(commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("buildFromMotionInput"));
        if layerManager.is_none() {
            panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
        }
        let mut geographicMapCompositeInterface: Option<Rc<RefCell<dyn GeographicMapCompositeInterface>>> = (layerManager).clone() /* as GeographicMapCompositeInterface */;
        let mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>> = jcall!(geographicMapCompositeInterface).getGeographicMapInterface()[(0) as usize].clone();
        if self.isDemoLocked(item.clone()) {
            jcall!(rtsPlayerLayerInterface).add(ErrorSound::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>));
            jcall!(LockedFeatureNotificationUtil::getInstance()).fireWithColor(jcall!(geographicMapInterface).getForegroundBasicColor());
            return;
        }
        let mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = self.getSelectedGeographicCellPosition();
        if !match (&geographicMapCellPosition, &crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            if !self.isPositionBlocked() {
                let mut geographicMapCellType: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellTypeDyn>>> = jcall!(geographicMapInterface).getCellTypeAt(geographicMapCellPosition.clone());
                let mut raceTrackGeographicMapCellTypeFactory: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::racetrack::RaceTrackGeographicMapCellTypeFactory::RaceTrackGeographicMapCellTypeFactoryDyn>>> = (jcall!(geographicMapInterface).getGeographicMapCellTypeFactory()).clone() /* as RaceTrackGeographicMapCellTypeFactory */;
                if !jcall!(raceTrackGeographicMapCellTypeFactory).isPath(geographicMapCellType.clone()) {
                    if match (&self.newUnconstructedRTSLayerInterfaceArray[(itemIndex) as usize], &CollidableDestroyableDamageableLayer::getNullInstance()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                        self.newUnconstructedRTSLayerInterfaceArray[(itemIndex) as usize] = self.getInstance(layerManager.clone(), item.clone(), geographicMapCellPosition.clone());
                    } else {
                        let mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>> = (self.newUnconstructedRTSLayerInterfaceArray[(itemIndex) as usize]).clone() /* as RTSLayer */;
                        let mut cellPoint: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = jcall!(geographicMapCellPosition).getPoint();
                        jcall!(rtsLayer).setPosition(jcall!(cellPoint).getX(), jcall!(cellPoint).getY(), jcall!(cellPoint).getZ());
                        jcall!(jcall!(rtsLayer).as_RTSLayer_mut().geographicMapCellPositionAreaBase).update(geographicMapInterface.clone());
                    }
                    let mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>> = (self.newUnconstructedRTSLayerInterfaceArray[(itemIndex) as usize]).clone() /* as RTSLayer */;
                    self.attemptBuild(rtsPlayerLayerInterface.clone(), layerManager.clone(), rtsLayer.clone(), itemIndex.clone());
                } else {
                    jcall!(rtsPlayerLayerInterface).add(ErrorSound::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>));
                    if !jcall!(rtsPlayerLayerInterface).implmentsArtificialIntelligenceCompositeInterface() {
                        jcall!(GameNotificationEventHandler::getInstance()).fireEvent(self.buildOnPathGameNotificationEvent.clone());
                    }
                }
            } else {
                jcall!(rtsPlayerLayerInterface).add(ErrorSound::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>));
                if !jcall!(rtsPlayerLayerInterface).implmentsArtificialIntelligenceCompositeInterface() {
                    jcall!(GameNotificationEventHandler::getInstance()).fireEvent(self.spotTakenGameNotificationEvent.clone());
                }
            }
        } else {
            jcall!(rtsPlayerLayerInterface).add(ErrorSound::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>));
            if !jcall!(rtsPlayerLayerInterface).implmentsArtificialIntelligenceCompositeInterface() {
                jcall!(GameNotificationEventHandler::getInstance()).fireEvent(self.selectBuildSpotGameNotificationEvent.clone());
            }
        }
    }
    

    // throws Exception
    pub fn attemptBuild(&mut self, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>>, mut itemIndex: i32) -> bool {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("Layer: ")).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(layerInterface.clone()))).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("attemptBuild"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        let mut geographicMapCellPositionArea: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::GeographicMapCellPositionAreaBase::GeographicMapCellPositionAreaBaseDyn>>> = jcall!(layerInterface).as_RTSLayer_mut().geographicMapCellPositionAreaBase.clone();
        let mut occupyList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(geographicMapCellPositionArea).getOccupyingGeographicMapCellPositionList();
        if !self.isBuildAttemptValid(rtsPlayerLayerInterface.clone(), layerInterface.clone()) {
            return false;
        }
        let mut geographicMapCellType: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellTypeDyn>>> = None;
        for index in (0..=jcall!(occupyList).size() - 1).rev() {
            let mut geographicMapCompositeInterface: Option<Rc<RefCell<dyn GeographicMapCompositeInterface>>> = (layerManager).clone() /* as GeographicMapCompositeInterface */;
            let mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>> = jcall!(geographicMapCompositeInterface).getGeographicMapInterface()[(0) as usize].clone();
            geographicMapCellType = jcall!(geographicMapInterface).getCellTypeAt({ let java_object = (jcall!(occupyList).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>>().expect("ClassCastException").clone()) });
            let mut raceTrackGeographicMapCellTypeFactory: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::racetrack::RaceTrackGeographicMapCellTypeFactory::RaceTrackGeographicMapCellTypeFactoryDyn>>> = (jcall!(geographicMapInterface).getGeographicMapCellTypeFactory()).clone() /* as RaceTrackGeographicMapCellTypeFactory */;
            if jcall!(raceTrackGeographicMapCellTypeFactory).isPath(geographicMapCellType.clone()) {
                jcall!(rtsPlayerLayerInterface).add(ErrorSound::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>));
                if !jcall!(rtsPlayerLayerInterface).implmentsArtificialIntelligenceCompositeInterface() {
                    jcall!(GameNotificationEventHandler::getInstance()).fireEvent(self.roadCollisionGameNotificationEvent.clone());
                }
                return false;
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
            self.add(rtsPlayerLayerInterface.clone(), layerManager.clone(), layerInterface.clone());
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
    pub fn isBuildAttemptValid(&mut self, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>>) -> bool {
        let mut geographicMapCellPositionArea: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::GeographicMapCellPositionAreaBase::GeographicMapCellPositionAreaBaseDyn>>> = jcall!(layerInterface).as_RTSLayer_mut().geographicMapCellPositionAreaBase.clone();
        let mut occupyList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(geographicMapCellPositionArea).getOccupyingGeographicMapCellPositionList();
        if jcall!(self.dropCellPositionHistory).anyCellPositionWithDrop(occupyList.clone()) {
            jcall!(rtsPlayerLayerInterface).add(ErrorSound::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>));
            if !jcall!(rtsPlayerLayerInterface).implmentsArtificialIntelligenceCompositeInterface() {
                jcall!(GameNotificationEventHandler::getInstance()).fireEvent(self.buildingCollisionGameNotificationEvent.clone());
            }
            return false;
        }
        if self.isUnitProducer && { let java_receiver = self.dropCellPositionHistory.clone(); let java_arguments = (jcall!(geographicMapCellPositionArea).getSurroundingGeographicMapCellPositionList(),); let java_result = jcall!(java_receiver).anyCellPositionWithDrop(java_arguments.0); java_result } {
            jcall!(rtsPlayerLayerInterface).add(ErrorSound::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>));
            if !jcall!(rtsPlayerLayerInterface).implmentsArtificialIntelligenceCompositeInterface() {
                jcall!(GameNotificationEventHandler::getInstance()).fireEvent(self.structureToCloseCollisionGameNotificationEvent.clone());
            }
            return false;
        } else {
            if self.isSurroundingCellsOffMap(layerInterface.clone()) {
                jcall!(rtsPlayerLayerInterface).add(ErrorSound::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>));
                if !jcall!(rtsPlayerLayerInterface).implmentsArtificialIntelligenceCompositeInterface() {
                    jcall!(GameNotificationEventHandler::getInstance()).fireEvent(self.mapEdgeGameNotificationEvent.clone());
                }
                return false;
            }
        }
        return true;
    }
    

    // throws Exception
    pub fn add(&mut self, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>>) {
        let mut geographicMapCellPositionArea: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::GeographicMapCellPositionAreaBase::GeographicMapCellPositionAreaBaseDyn>>> = jcall!(layerInterface).as_RTSLayer_mut().geographicMapCellPositionAreaBase.clone();
        let mut occupyList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(geographicMapCellPositionArea).getOccupyingGeographicMapCellPositionList();
        jcall!(self.dropCellPositionHistory).addAll(occupyList.clone(), layerInterface.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>));
        let mut rtsPlayerGameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::RTSPlayerGameInput::RTSPlayerGameInputDyn>>> = (jcall!(rtsPlayerLayerInterface).getPlayerGameInput()).clone() /* as RTSPlayerGameInput */;
        jcall!(rtsPlayerGameInput).setSelectedRTSLayer(layerInterface.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>), self.getSelectedGeographicCellPosition());
        jcall!(layerManager).append(layerInterface.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>));
        jcall!(rtsPlayerLayerInterface).add(BuildingSound::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>));
        if !jcall!(rtsPlayerLayerInterface).implmentsArtificialIntelligenceCompositeInterface() {
            jcall!(GameNotificationEventHandler::getInstance()).fireEvent(self.buildingGameNotificationEvent.clone());
        }
    }
    

    pub fn isSurroundingCellsOffMap(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>>) -> bool {
        let mut geographicMapCellPositionArea: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::GeographicMapCellPositionAreaBase::GeographicMapCellPositionAreaBaseDyn>>> = jcall!(layerInterface).as_RTSLayer_mut().geographicMapCellPositionAreaBase.clone();
        let mut occupyList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(geographicMapCellPositionArea).getOccupyingGeographicMapCellPositionList();
        let mut surroundList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(geographicMapCellPositionArea).getSurroundingGeographicMapCellPositionList();
        let mut occupySize: i32 = jcall!(occupyList).size();
        let mut surroundSize: i32 = jcall!(surroundList).size();
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(String::from("occupySize: "));
        jcall!(stringBuffer).appendint(occupySize.clone());
        jcall!(stringBuffer).append(String::from(" surroundSize: "));
        jcall!(stringBuffer).appendint(surroundSize.clone());
        jcall!(stringBuffer).append(String::from(" surroundList: "));
        jcall!(stringBuffer).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(surroundList.clone())));
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("isSurroundingCellsOffMap"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        let mut isSurroundOffMap: bool = false;
        if occupySize == 1 && surroundSize != 8 {
            isSurroundOffMap = true;
        } else if occupySize == 4 && surroundSize != 12 {
            isSurroundOffMap = true;
        }
        if occupySize == 0 || isSurroundOffMap {
            return true;
        } else {
            return false;
        }
    }
}
impl std::ops::Deref for BuildingRTSFormInput {
    type Target = RTSFormInput;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for BuildingRTSFormInput {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait BuildingRTSFormInputDyn: crate::org::allbinary::game::input::form::RTSFormInput::RTSFormInputDyn {
    fn as_BuildingRTSFormInput(&self) -> &BuildingRTSFormInput;
    fn as_BuildingRTSFormInput_mut(&mut self) -> &mut BuildingRTSFormInput;
    fn isPositionBlocked(&mut self) -> bool { BuildingRTSFormInput::isPositionBlocked(self.as_BuildingRTSFormInput_mut()) }
    fn isDemoLocked(&mut self, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>) -> bool { BuildingRTSFormInput::isDemoLocked(self.as_BuildingRTSFormInput_mut(), item) }
    fn buildFromMotionInput(&mut self, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut itemIndex: i32) { BuildingRTSFormInput::buildFromMotionInput(self.as_BuildingRTSFormInput_mut(), rtsPlayerLayerInterface, layerManager, item, itemIndex) }
    fn attemptBuild(&mut self, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>>, mut itemIndex: i32) -> bool { BuildingRTSFormInput::attemptBuild(self.as_BuildingRTSFormInput_mut(), rtsPlayerLayerInterface, layerManager, layerInterface, itemIndex) }
    fn isBuildAttemptValid(&mut self, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>>) -> bool { BuildingRTSFormInput::isBuildAttemptValid(self.as_BuildingRTSFormInput_mut(), rtsPlayerLayerInterface, layerInterface) }
    fn add(&mut self, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>>) { BuildingRTSFormInput::add(self.as_BuildingRTSFormInput_mut(), rtsPlayerLayerInterface, layerManager, layerInterface) }
    fn isSurroundingCellsOffMap(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>>) -> bool { BuildingRTSFormInput::isSurroundingCellsOffMap(self.as_BuildingRTSFormInput_mut(), layerInterface) }
}
impl BuildingRTSFormInputDyn for BuildingRTSFormInput {
    fn as_BuildingRTSFormInput(&self) -> &BuildingRTSFormInput { self }
    fn as_BuildingRTSFormInput_mut(&mut self) -> &mut BuildingRTSFormInput { self }
}

impl crate::org::allbinary::game::input::form::RTSFormInput::RTSFormInputDyn for BuildingRTSFormInput {
    fn as_RTSFormInput(&self) -> &crate::org::allbinary::game::input::form::RTSFormInput::RTSFormInput { &self.base }
    fn as_RTSFormInput_mut(&mut self) -> &mut crate::org::allbinary::game::input::form::RTSFormInput::RTSFormInput { &mut self.base }
    fn setAllBinaryGameLayerManager(&mut self, mut allBinaryGameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>) { Self::setAllBinaryGameLayerManager(self, allBinaryGameLayerManager) }
    fn processGameSpecific(&mut self, mut associatedRtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut itemIndex: i32) { Self::processGameSpecific(self, associatedRtsLayer, rtsPlayerLayerInterface, layerManager, item, itemIndex) }
}


