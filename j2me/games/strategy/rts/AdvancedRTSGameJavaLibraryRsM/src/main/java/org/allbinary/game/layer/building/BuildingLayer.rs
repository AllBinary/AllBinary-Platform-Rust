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
// package org.allbinary.game.layer.building
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::ag::weapon::media::audio::ExplosionBasicSound::ExplosionBasicSound;
use crate::org::allbinary::animation::NullAnimationFactory::NullAnimationFactory;
use crate::org::allbinary::animation::NullIndexedAnimationFactory::NullIndexedAnimationFactory;
use crate::org::allbinary::game::identification::GroupFactory::GroupFactory;
use crate::org::allbinary::game::input::form::NullRTSFormInputFactory::NullRTSFormInputFactory;
use crate::org::allbinary::game::input::form::RTSFormInput::RTSFormInput;
use crate::org::allbinary::game::input::form::VisibleCellPositionsSingleton::VisibleCellPositionsSingleton;
use crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer;
use crate::org::allbinary::game::layer::AdvancedRTSPlayerLayerInterface::AdvancedRTSPlayerLayerInterface;
use crate::org::allbinary::game::layer::AdvancedRTSProperties::AdvancedRTSProperties;
use crate::org::allbinary::game::layer::CollidableRTSBehavior::CollidableRTSBehavior;
use crate::org::allbinary::game::layer::RTSLayerUtil::RTSLayerUtil;
use crate::org::allbinary::game::layer::RTSPlayerLayerInterface::RTSPlayerLayerInterface;
use crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintable;
use crate::org::allbinary::game::layer::waypoint::Waypoint::Waypoint;
use crate::org::allbinary::game::view::TileLayerPositionIntoViewPosition::TileLayerPositionIntoViewPosition;
use crate::org::allbinary::graphics::RectangleFactory::RectangleFactory;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::animation::AnimationInterfaceFactoryInterface::AnimationInterfaceFactoryInterface;
use crate::org::allbinary::animation::ProceduralAnimationInterfaceFactoryInterface::ProceduralAnimationInterfaceFactoryInterface;
use crate::org::allbinary::animation::RotationAnimationInterfaceCompositeInterface::RotationAnimationInterfaceCompositeInterface;
use crate::org::allbinary::direction::Direction::Direction;
use crate::org::allbinary::direction::DirectionFactory::DirectionFactory;
use crate::org::allbinary::game::combat::damage::DamageFloaters::DamageFloaters;
use crate::org::allbinary::game::health::Health::Health;
use crate::org::allbinary::game::identification::Group::Group;
use crate::org::allbinary::game::layer::GeographicMapCellPositionAreaBase::GeographicMapCellPositionAreaBase;
use crate::org::allbinary::game::layer::NullPathFindingLayer::NullPathFindingLayer;
use crate::org::allbinary::game::tracking::TrackingEvent::TrackingEvent;
use crate::org::allbinary::game::tracking::TrackingEventHandler::TrackingEventHandler;
use crate::org::allbinary::game::tracking::TrackingEventListenerInterface::TrackingEventListenerInterface;
use crate::org::allbinary::graphics::Rectangle::Rectangle;
use crate::org::allbinary::graphics::paint::Paintable::Paintable;
use crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager;
use crate::org::allbinary::media::audio::SecondaryPlayerQueueFactory::SecondaryPlayerQueueFactory;
use crate::org::allbinary::media::audio::SelectSound::SelectSound;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPosition;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapDirectionUtil::GeographicMapDirectionUtil;
use crate::org::allbinary::media::graphics::geography::map::drop::DropCellPositionHistory::DropCellPositionHistory;
use crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelper;
use crate::org::allbinary::util::BasicArrayListS::BasicArrayListS;
use crate::org::allbinary::game::multiplayer::layer::RemoteInfo::RemoteInfo;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use super::BuildingPropertiesFactory::BuildingPropertiesFactory;
use super::BuildingInfoHudPaintable::BuildingInfoHudPaintable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BuildingLayer {
    pub base: AdvancedRTSGameLayer,
    pub buildingLevelCost: i32,
    pub productivity: i32,
    pub efficiency: i32,
    pub efficiencyPerLevel: i32,
    pub trackingEvent: Option<Rc<RefCell<TrackingEvent>>>,
    pub damageFloaters: Option<Rc<RefCell<dyn crate::org::allbinary::game::combat::damage::DamageFloaters::DamageFloatersDyn>>>,
    pub damageFloatersPaintableInterface: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>,
    pub healthBar: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>,
    pub pathsHashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>,
    pub local: bool,
    pub timeDelayHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>>,
}

// impl RotationAnimationInterfaceCompositeInterface for BuildingLayer with the methods of the trait
// impl TrackingEventListenerInterface for BuildingLayer with the methods of the trait
impl crate::org::allbinary::game::combat::damage::DamageableInterface::DamageableInterface for BuildingLayer {
    fn damage(&mut self, mut damage: i32, mut damageType: i32) { Self::damage(self, damage, damageType) }
    fn getDamage(&mut self, mut damageType: i32) -> i32 { Self::getDamage(self, damageType) }
}
impl crate::org::allbinary::layer::NamedInterface::NamedInterface for BuildingLayer {
    fn getName(&mut self) -> String { self.base.getName() }
}
impl crate::org::allbinary::game::combat::destroy::DestroyableInterface::DestroyableInterface for BuildingLayer {
    fn isDestroyed(&mut self) -> bool { self.base.isDestroyed() }
}

impl BuildingLayer {
    

    // throws Exception
    pub fn createSimulated() -> Option<Rc<RefCell<BuildingLayer>>> {
        let mut nullAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>> = NullAnimationFactory::getFactoryInstance();
        let mut nullIndexedAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>> = NullIndexedAnimationFactory::getFactoryInstance();
        return BuildingLayer::new(crate::org::allbinary::game::multiplayer::layer::RemoteInfo::RemoteInfo_REMOTE_INFO.with(Clone::clone).clone(), SimulatedBuildingPropertiesFactory::SimulatedBuildingPropertiesFactory::getInstance(), AdvancedRTSProperties::createPropertiesSimulated(), jcall!(GroupFactory::getInstance()).NULL_GROUP_ARRAY.clone(), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), crate::org::allbinary::game::health::Health::Health_NULL_HEALTH.with(Clone::clone).clone(), NullRTSFormInputFactory::getInstance(), nullAnimationInterfaceFactoryInterface.clone(), nullIndexedAnimationInterfaceFactoryInterface.clone(), nullAnimationInterfaceFactoryInterface.clone(), nullAnimationInterfaceFactoryInterface.clone(), nullIndexedAnimationInterfaceFactoryInterface.clone(), NullIndexedAnimationFactory::getFactoryInstance(), crate::org::allbinary::graphics::RectangleFactory::RectangleFactory_SINGLETON.with(Clone::clone).clone(), 0, 0);
    }
    

    pub fn new_struct(mut remoteInfo: Option<Rc<RefCell<RemoteInfo>>>, mut buildingPropertiesFactory: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::building::BuildingPropertiesFactory::BuildingPropertiesFactoryDyn>>>, mut advancedRTSProperties: Option<Rc<RefCell<AdvancedRTSProperties>>>, mut groupInterface: Vec<Option<Rc<RefCell<Group>>>>, mut rootName: String, mut name: String, mut healthInterface: Option<Rc<RefCell<Health>>>, mut rtsFormInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::form::RTSFormInput::RTSFormInputDyn>>>, mut animationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>, mut emptyAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>, mut baseAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>, mut buildAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>, mut verticleBuildAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>, mut proceduralAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn ProceduralAnimationInterfaceFactoryInterface>>>, mut rectangle: Option<Rc<RefCell<Rectangle>>>, mut x: i32, mut y: i32) -> Self {
        let mut this = Self {
            base: AdvancedRTSGameLayer::new_struct(remoteInfo.clone(), crate::org::allbinary::game::layer::NullPathFindingLayer::NullPathFindingLayer_NULL_PATH_FINDING_LAYER.with(Clone::clone).clone(), advancedRTSProperties.clone(), groupInterface.clone(), rootName.clone(), name.clone(), healthInterface.clone(), rtsFormInput.clone(), animationInterfaceFactoryInterface.clone(), emptyAnimationInterfaceFactoryInterface.clone(), baseAnimationInterfaceFactoryInterface.clone(), buildAnimationInterfaceFactoryInterface.clone(), verticleBuildAnimationInterfaceFactoryInterface.clone(), proceduralAnimationInterfaceFactoryInterface.clone(), rectangle.clone(), x.clone(), y.clone(), TileLayerPositionIntoViewPosition::new()),
            buildingLevelCost: 0,
            productivity: 0,
            efficiency: 0,
            efficiencyPerLevel: 0,
            trackingEvent: None,
            damageFloaters: None,
            damageFloatersPaintableInterface: None,
            healthBar: None,
            pathsHashtable: None,
            local: false,
            timeDelayHelper: None,
        };
        this.timeDelayHelper = TimeDelayHelper::new(3000).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>);
        // super(...) is in the struct literal above
        this.setCollidableInferface(CollidableRTSBehavior::new(true).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableRTSBehavior::CollidableRTSBehaviorDyn>>));
        this.damageFloatersPaintableInterface = jcall!(buildingPropertiesFactory).as_BuildingPropertiesFactory_mut().damageFloatersPaintableInterface.clone();
        this.pathsHashtable = jcall!(buildingPropertiesFactory).getHashtable();
        this.setMaxLevel(30);
        this.setProductivity(1);
        { let java_arguments = (this.calculateEfficiency(),); this.setEfficiency(java_arguments.0) };
        this.efficiency = this.efficiencyPerLevel.clone();
        this.generateMoveOutOfBuildAreaPaths();
        this
    }

    pub fn new(mut remoteInfo: Option<Rc<RefCell<RemoteInfo>>>, mut buildingPropertiesFactory: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::building::BuildingPropertiesFactory::BuildingPropertiesFactoryDyn>>>, mut advancedRTSProperties: Option<Rc<RefCell<AdvancedRTSProperties>>>, mut groupInterface: Vec<Option<Rc<RefCell<Group>>>>, mut rootName: String, mut name: String, mut healthInterface: Option<Rc<RefCell<Health>>>, mut rtsFormInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::form::RTSFormInput::RTSFormInputDyn>>>, mut animationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>, mut emptyAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>, mut baseAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>, mut buildAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>, mut verticleBuildAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>, mut proceduralAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn ProceduralAnimationInterfaceFactoryInterface>>>, mut rectangle: Option<Rc<RefCell<Rectangle>>>, mut x: i32, mut y: i32) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(remoteInfo, buildingPropertiesFactory, advancedRTSProperties, groupInterface, rootName, name, healthInterface, rtsFormInput, animationInterfaceFactoryInterface, emptyAnimationInterfaceFactoryInterface, baseAnimationInterfaceFactoryInterface, buildAnimationInterfaceFactoryInterface, verticleBuildAnimationInterfaceFactoryInterface, proceduralAnimationInterfaceFactoryInterface, rectangle, x, y))));
            jcall!(this.getWaypointBehavior()).setWaypoint(Waypoint::new((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()), SelectSound::getInstance()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::layer::waypoint::Waypoint::WaypointDyn>>));
            this.damageFloaters = jcall!(buildingPropertiesFactory).getDamageFloaters((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()));
            this.healthBar = jcall!(buildingPropertiesFactory).getHealthBar((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()));
            this.efficiencyPerLevel = jcall!(buildingPropertiesFactory).getEfficiencyPerLevel((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()));
            this.trackingEvent = jcall!(buildingPropertiesFactory).getTrackingEvent((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()));
        this
    }
    

    pub fn initVisibility(&mut self, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>) {
        let mut advancedRTSPlayerLayerInterface: Option<Rc<RefCell<dyn AdvancedRTSPlayerLayerInterface>>> = panic!("ClassCastException: a Rust dyn RTSPlayerLayerInterface cannot be cast to dyn AdvancedRTSPlayerLayerInterface");
        if jcall!(advancedRTSPlayerLayerInterface).isLocalPlayer() {
            self.local = true;
            self.addVisibility();
        } else {
            self.local = false;
        }
        self.base.initVisibility(rtsPlayerLayerInterface.clone());
    }
    

    // throws Exception
    pub fn construct(&mut self, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>) {
        self.base.construct(rtsPlayerLayerInterface.clone());
        jcall!(TrackingEventHandler::getInstance()).addListenerInterface(None /* Java this: a &mut self method has no Rc of itself */);
    }
    

    pub fn onMovement(&mut self, mut trackingEvent: Option<Rc<RefCell<TrackingEvent>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn>>> = (jcall!(trackingEvent).getLayerInterface()).clone() /* as AdvancedRTSGameLayer */;
            if !match (&jcall!(layerInterface).getGroupInterface()[(0) as usize], &self.getGroupInterface()[(0) as usize]) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                jcall!(layerInterface).onMovementFound(self.trackingEvent.clone());
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("onMovement"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    // throws Exception
    pub fn processBuiltTick(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) {
        if jcall!(self.getHealthInterface()).isDamaged() {
            self.setAnimationInterface(self.destroyAnimationInterface.clone());
        }
        if !jcall!(self.getHealthInterface()).isAlive() {
            if self.isReadyForExplosion() {
                let mut currentFrame: i32 = jcall!(self.destroyAnimationInterface).getFrame();
                let mut size: i32 = jcall!(self.destroyAnimationInterface).getSize() - 1;
                if currentFrame == size && !jcall!(self.timeDelayHelper).isTimeTNT() {
                    if !jcall!(self.getHealthInterface()).isAlive() {
                        self.setDestroyed(true);
                    }
                } else {
                    jcall!(self.destroyAnimationInterface).nextFrame();
                }
            } else {
                self.setAnimationInterface(self.destroyAnimationInterface.clone());
                jcall!(SecondaryPlayerQueueFactory::getInstance()).add(ExplosionBasicSound::getInstance());
                jcall!(self.shakeListener).onSmallShakeEvent();
                jcall!(self.vibration).vibrate(self.duration.clone(), 0, 0);
                jcall!(self.timeDelayHelper).setStartTimeTNT();
                self.setReadyForExplosion(true);
            }
        } else {
            self.base.processBuiltTick(allBinaryLayerManager.clone());
        }
        jcall!(self.indexedButShouldBeRotationAnimationInterface).nextFrame();
    }
    

    pub fn calculateEfficiency(&mut self) -> i32 {
        return self.efficiencyPerLevel.clone();
    }
    

    pub fn getCost(&mut self) -> i32 {
        let mut total: i64 = jcall!(RTSLayerUtil::getInstance()).getCostExponential((((self.getLevel() * self.getBuildingLevelCost())) as i64));
        return ((total) as i32);
    }
    

    pub fn getDowngradeCost(&mut self) -> i32 {
        let mut downgradeCost: i64 = jcall!(RTSLayerUtil::getInstance()).getCostExponential(((((self.getLevel() - 1) * self.getBuildingLevelCost())) as i64));
        jcall!(self.logUtil).putF(format!("{}{}", String::from("Cost: "), downgradeCost), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getDowngradeCost"));
        return ((downgradeCost) as i32) * 9 / 10;
    }
    

    pub fn getUpgradeCost(&mut self) -> i32 {
        let mut upgradeCost: i64 = jcall!(RTSLayerUtil::getInstance()).getCostExponential(((((self.getLevel() + 1) * self.getBuildingLevelCost())) as i64));
        return ((upgradeCost) as i32);
    }
    

    pub fn downgrade(&mut self) {
        self.base.downgrade();
        { let java_arguments = (self.getProductivity() - 1,); self.setProductivity(java_arguments.0) };
        { let java_arguments = (self.getEfficiency() - self.calculateEfficiency(),); self.setEfficiency(java_arguments.0) };
        jcall!(self.getHealthInterface()).setMaxHealth(jcall!(self.getHealthInterface()).getMaxHealth() - ((self.getLevel() + 1) * 100));
    }
    

    pub fn upgrade(&mut self) {
        self.base.upgrade();
        { let java_arguments = (self.getProductivity() + 1,); self.setProductivity(java_arguments.0) };
        { let java_arguments = (self.getEfficiency() + self.calculateEfficiency(),); self.setEfficiency(java_arguments.0) };
        jcall!(self.getHealthInterface()).setMaxHealth(jcall!(self.getHealthInterface()).getMaxHealth() + (self.getLevel() * 100));
    }
    

    pub fn getBuildingLevelCost(&mut self) -> i32 {
        return self.buildingLevelCost.clone();
    }
    

    pub fn setBuildingLevelCost(&mut self, mut buildingLevelCost: i32) {
        self.buildingLevelCost = buildingLevelCost.clone();
    }
    

    pub fn getProductivity(&mut self) -> i32 {
        return self.productivity.clone();
    }
    

    pub fn setProductivity(&mut self, mut productivity: i32) {
        self.productivity = productivity.clone();
    }
    

    pub fn getEfficiency(&mut self) -> i32 {
        return self.efficiency.clone();
    }
    

    pub fn setEfficiency(&mut self, mut efficiency: i32) {
        self.efficiency = efficiency.clone();
    }
    

    // throws Exception
    pub fn generateMoveOutOfBuildAreaPaths(&mut self) {
        let mut occupyList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(self.geographicMapCellPositionAreaBase).getOccupyingGeographicMapCellPositionList();
        let mut surroundList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(self.geographicMapCellPositionAreaBase).getSurroundingGeographicMapCellPositionList();
        let mut NO_DIRECTION: Option<Rc<RefCell<Direction>>> = jcall!(DirectionFactory::getInstance()).NO_DIRECTION.clone();
        let mut geographicMapDirectionUtil: Option<Rc<RefCell<GeographicMapDirectionUtil>>> = GeographicMapDirectionUtil::getInstance();
        let mut pathsList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = None;
        let mut occupyGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = None;
        let mut surroundGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = None;
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = None;
        for index2 in (0..=jcall!(occupyList).size() - 1).rev() {
            pathsList = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            occupyGeographicMapCellPosition = { let java_object = (jcall!(occupyList).get(index2.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>>().expect("ClassCastException").clone()) };
            for index in (0..=jcall!(surroundList).size() - 1).rev() {
                surroundGeographicMapCellPosition = { let java_object = (jcall!(surroundList).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>>().expect("ClassCastException").clone()) };
                if !match (&jcall!(geographicMapDirectionUtil).getEightDirectionFromCellPositionToAdjacentCellPosition(surroundGeographicMapCellPosition.clone(), occupyGeographicMapCellPosition.clone()), &NO_DIRECTION) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                    list = BasicArrayListS::new(1).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
                    jcall!(list).add(crate::java_into_object(surroundGeographicMapCellPosition.clone()));
                    jcall!(pathsList).add(crate::java_into_object(list.clone()));
                }
            }
            jcall!(self.pathsHashtable).put(crate::java_into_object(occupyGeographicMapCellPosition.clone()), crate::java_into_object(pathsList.clone()));
        }
    }
    

    pub fn getMoveOutOfBuildAreaPath(&mut self, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut pathsList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = { let java_object = (jcall!(self.pathsHashtable).get(crate::java_into_object(geographicMapCellPosition.clone()))).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>>().expect("ClassCastException").clone()) };
        return pathsList.clone();
    }
    

    pub fn getEndGeographicMapCellPositionList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return jcall!(self.geographicMapCellPositionAreaBase).getSurroundingGeographicMapCellPositionList();
    }
    

    pub fn shouldHandleStartSameAsEnd(&mut self) -> bool {
        return false;
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        if self.isVisible() {
            self.base.paint(graphics.clone());
            jcall!(self.damageFloatersPaintableInterface).paint(graphics.clone());
            jcall!(self.healthBar).paint(graphics.clone());
        }
    }
    

    // throws Exception
    pub fn damage(&mut self, mut damage: i32, mut damageType: i32) {
        self.base.damage(damage.clone(), damageType.clone());
        jcall!(self.damageFloaters).add(damage.clone());
        if damage > 0 {
            jcall!(self.getHealthInterface()).damage(damage.clone());
        }
    }
    

    // throws Exception
    pub fn getDamage(&mut self, mut damageType: i32) -> i32 {
        return 0;
    }
    

    pub fn addVisibility(&mut self) {
        let mut geographicMapCellPositionArea: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::GeographicMapCellPositionAreaBase::GeographicMapCellPositionAreaBaseDyn>>> = self.geographicMapCellPositionAreaBase.clone();
        let mut occupyList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(geographicMapCellPositionArea).getOccupyingGeographicMapCellPositionList();
        jcall!(VisibleCellPositionsSingleton::getInstance()).addStationaryCellPositions(occupyList.clone());
        let mut surroundList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(geographicMapCellPositionArea).getSurroundingGeographicMapCellPositionList();
        jcall!(VisibleCellPositionsSingleton::getInstance()).addStationaryCellPositions(surroundList.clone());
    }
    

    pub fn removeVisibility(&mut self) {
        let mut occupyList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(self.geographicMapCellPositionAreaBase).getOccupyingGeographicMapCellPositionList();
        jcall!(VisibleCellPositionsSingleton::getInstance()).removeStationaryCellPositions(occupyList.clone());
        let mut surroundList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(self.geographicMapCellPositionAreaBase).getSurroundingGeographicMapCellPositionList();
        jcall!(VisibleCellPositionsSingleton::getInstance()).removeStationaryCellPositions(surroundList.clone());
    }
    

    // throws Exception
    pub fn setDestroyed(&mut self, mut destroyed: bool) {
        self.base.setDestroyed(destroyed.clone());
        if self.isDestroyed() {
            jcall!(DropCellPositionHistory::getInstance()).removeAll(None /* Java this: a &mut self method has no Rc of itself */);
            jcall!(TrackingEventHandler::getInstance()).removeListener(None /* Java this: a &mut self method has no Rc of itself */);
            if self.local {
                self.removeVisibility();
            }
            if !jcall!(self.getHealthInterface()).isAlive() {
                let mut damage: i32 = jcall!(self.getHealthInterface()).getMaxHealth();
                if damage > 10 {
                    jcall!(SecondaryPlayerQueueFactory::getInstance()).add(ExplosionBasicSound::getInstance());
                    if damage < 100 {
                        jcall!(self.shakeListener).onSmallShakeEvent();
                        jcall!(self.vibration).vibrate(self.duration.clone(), 0, 0);
                    } else if damage < 1000 {
                        jcall!(self.shakeListener).onMediumShakeEvent();
                        jcall!(self.vibration).vibrate(self.duration * 2, 0, 0);
                    } else if damage < 3000 {
                        jcall!(self.shakeListener).onLargeShakeEvent();
                        jcall!(self.vibration).vibrate(self.duration * 4, 0, 0);
                    }
                }
            }
        }
    }
    

    pub fn createHudPaintable(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintableDyn>>> {
        let mut buildingInfoHudPaintable: Option<Rc<RefCell<BuildingInfoHudPaintable>>> = BuildingInfoHudPaintable::getInstance();
        jcall!(buildingInfoHudPaintable).setBasicColorP(jcall!(self.allBinaryGameLayerManagerP).getForegroundBasicColor());
        jcall!(buildingInfoHudPaintable).setRtsLayer(None /* Java this: a &mut self method has no Rc of itself */);
        return buildingInfoHudPaintable.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintableDyn>>);
    }
    

    pub fn getHudPaintable(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintableDyn>>> {
        let mut buildingInfoHudPaintable: Option<Rc<RefCell<BuildingInfoHudPaintable>>> = BuildingInfoHudPaintable::getInstance();
        return buildingInfoHudPaintable.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintableDyn>>);
    }
    

    pub fn getType(&mut self) -> i32 {
        return BuildingLayer::getStaticType();
    }
    

    pub fn getStaticType() -> i32 {
        return 2;
    }
}
impl std::ops::Deref for BuildingLayer {
    type Target = AdvancedRTSGameLayer;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for BuildingLayer {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn for BuildingLayer {
    fn as_RTSLayer(&self) -> &crate::org::allbinary::game::layer::RTSLayer::RTSLayer { &self.base.base }
    fn as_RTSLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::RTSLayer::RTSLayer { &mut self.base.base }
    fn updateWaypointBehavior(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>) { crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer::updateWaypointBehavior(crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn::as_AdvancedRTSGameLayer_mut(self), p1) }
    fn construct(&mut self, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>) { Self::construct(self, rtsPlayerLayerInterface) }
    fn setSelected(&mut self, p1: bool) { crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer::setSelected(crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn::as_AdvancedRTSGameLayer_mut(self), p1) }
    fn processBuiltTick(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { Self::processBuiltTick(self, allBinaryLayerManager) }
    fn onMovement(&mut self, mut trackingEvent: Option<Rc<RefCell<TrackingEvent>>>) { Self::onMovement(self, trackingEvent) }
    fn createHudPaintable(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintableDyn>>> { Self::createHudPaintable(self) }
    fn getHudPaintable(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintableDyn>>> { Self::getHudPaintable(self) }
    fn setDestroyed(&mut self, mut destroyed: bool) { Self::setDestroyed(self, destroyed) }
    fn getEndGeographicMapCellPositionList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Self::getEndGeographicMapCellPositionList(self) }
    fn shouldHandleStartSameAsEnd(&mut self) -> bool { Self::shouldHandleStartSameAsEnd(self) }
    fn getWaypointBehavior(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>> { crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer::getWaypointBehavior(crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn::as_AdvancedRTSGameLayer_mut(self)) }
    fn getParentLayer(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::PathFindingLayerInterface::PathFindingLayerInterface>>> { crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer::getParentLayer(crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn::as_AdvancedRTSGameLayer_mut(self)) }
    fn getWaypointLogHelper(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::waypoint::WaypointLogHelper::WaypointLogHelperDyn>>> { crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer::getWaypointLogHelper(crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn::as_AdvancedRTSGameLayer_mut(self)) }
    fn getWaypoint2LogHelper(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::waypoint::Waypoint2LogHelper::Waypoint2LogHelperDyn>>> { crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer::getWaypoint2LogHelper(crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn::as_AdvancedRTSGameLayer_mut(self)) }
    fn getWaypointRunnableLogHelper(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::waypoint::WaypointRunnableLogHelper::WaypointRunnableLogHelperDyn>>> { crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer::getWaypointRunnableLogHelper(crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn::as_AdvancedRTSGameLayer_mut(self)) }
    fn shouldAddWaypointFromBuilding(&mut self) -> bool { crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer::shouldAddWaypointFromBuilding(crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn::as_AdvancedRTSGameLayer_mut(self)) }
    fn getMoveOutOfBuildAreaPath(&mut self, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Self::getMoveOutOfBuildAreaPath(self, geographicMapCellPosition) }
    fn getSurroundingGeographicMapCellPositionList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer::getSurroundingGeographicMapCellPositionList(crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn::as_AdvancedRTSGameLayer_mut(self)) }
    fn isWaypointListEmptyOrOnlyTargets(&mut self) -> bool { crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer::isWaypointListEmptyOrOnlyTargets(crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn::as_AdvancedRTSGameLayer_mut(self)) }
    fn buildingChase(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, p2: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> bool { crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer::buildingChase(crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn::as_AdvancedRTSGameLayer_mut(self), p1, p2) }
}
impl crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn for BuildingLayer {
    fn as_AdvancedRTSGameLayer(&self) -> &crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer { &self.base }
    fn as_AdvancedRTSGameLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer { &mut self.base }
    fn initVisibility(&mut self, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>) { Self::initVisibility(self, rtsPlayerLayerInterface) }
}
impl crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn for BuildingLayer {
    fn as_AllBinaryLayer(&self) -> &crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer { &self.base.base.base.base.base.base.base }
    fn as_AllBinaryLayer_mut(&mut self) -> &mut crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer { &mut self.base.base.base.base.base.base.base }
    fn onChangeEvent(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::view::event::ViewPositionEvent::ViewPositionEvent>>>) { crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer::onChangeEvent(crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn::as_AllBinaryGameLayer_mut(self), p1) }
    fn implmentsTickableInterface(&mut self) -> bool { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::implmentsTickableInterface(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self)) }
    fn implmentsGameInputInterface(&mut self) -> bool { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::implmentsGameInputInterface(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self)) }
    fn implmentsCollidableInterface(&mut self) -> bool { crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer::implmentsCollidableInterface(crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn::as_CollidableCompositeLayer_mut(self)) }
    fn getType(&mut self) -> i32 { Self::getType(self) }
    fn toString(&mut self) -> String { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::toString(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self)) }
}
impl crate::org::allbinary::layer::Layer::LayerDyn for BuildingLayer {
    fn as_Layer(&self) -> &crate::org::allbinary::layer::Layer::Layer { &self.base.base.base.base.base.base.base.base }
    fn as_Layer_mut(&mut self) -> &mut crate::org::allbinary::layer::Layer::Layer { &mut self.base.base.base.base.base.base.base.base }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
    fn toStringAppend(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::logic::string::StringMaker::StringMaker>>>) { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::toStringAppend(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self), p1) }
}
impl crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayerDyn for BuildingLayer {
    fn as_MultiPlayerGameLayer(&self) -> &crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayer { &self.base.base.base }
    fn as_MultiPlayerGameLayer_mut(&mut self) -> &mut crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayer { &mut self.base.base.base }
}
impl crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn for BuildingLayer {
    fn as_CollidableDestroyableDamageableLayer(&self) -> &crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer { &self.base.base.base.base }
    fn as_CollidableDestroyableDamageableLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer { &mut self.base.base.base.base }
    fn damage(&mut self, mut damage: i32, mut damageType: i32) { Self::damage(self, damage, damageType) }
    fn getDamage(&mut self, mut damageType: i32) -> i32 { Self::getDamage(self, damageType) }
    fn isDestroyed(&mut self) -> bool { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::isDestroyed(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self)) }
    fn setAllBinaryGameLayerManager(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>) { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::setAllBinaryGameLayerManager(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self), p1) }
    fn onDestroyed(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::game::combat::destroy::event::DestroyedEvent::DestroyedEvent>>>) { crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer::onDestroyed(crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn::as_AdvancedRTSGameLayer_mut(self), p1) }
}
impl crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn for BuildingLayer {
    fn as_AllBinaryGameLayer(&self) -> &crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer { &self.base.base.base.base.base.base }
    fn as_AllBinaryGameLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer { &mut self.base.base.base.base.base.base }
    fn processInput(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::processInput(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self), p1) }
    fn processTick(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::processTick(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self), p1) }
}
impl crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn for BuildingLayer {
    fn as_CollidableCompositeLayer(&self) -> &crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer { &self.base.base.base.base.base }
    fn as_CollidableCompositeLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer { &mut self.base.base.base.base.base }
}
impl crate::org::allbinary::game::layer::RTSInterface::RTSInterface for BuildingLayer {
    fn isSelfUpgradeable(&mut self) -> bool { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::isSelfUpgradeable(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self)) }
    fn isCompleted(&mut self) -> bool { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::isCompleted(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self)) }
    fn getPercentComplete(&mut self) -> i32 { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::getPercentComplete(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self)) }
    fn getLevel(&mut self) -> i32 { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::getLevel(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self)) }
    fn getMaxLevel(&mut self) -> i32 { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::getMaxLevel(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self)) }
    fn getCost(&mut self) -> i32 { Self::getCost(self) }
    fn downgrade(&mut self) { Self::downgrade(self) }
    fn getDowngradeCost(&mut self) -> i32 { Self::getDowngradeCost(self) }
    fn isDowngradeable(&mut self) -> bool { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::isDowngradeable(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self)) }
    fn getUpgradeCost(&mut self) -> i32 { Self::getUpgradeCost(self) }
    fn isUpgradeable(&mut self) -> bool { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::isUpgradeable(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self)) }
    fn upgrade(&mut self) { Self::upgrade(self) }
}
impl crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayerInterface::MultiPlayerGameLayerInterface for BuildingLayer {
    fn getMultiplayerBehavior(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::multiplayer::layer::MultiplayerBehavior::MultiplayerBehaviorDyn>>> { crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayer::getMultiplayerBehavior(crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayerDyn::as_MultiPlayerGameLayer_mut(self)) }
}
impl crate::org::allbinary::game::collision::CollidableInterfaceCompositeInterface::CollidableInterfaceCompositeInterface for BuildingLayer {
    fn getCollidableInferface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehaviorDyn>>> { crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer::getCollidableInferface(crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn::as_CollidableCompositeLayer_mut(self)) }
}
impl crate::org::allbinary::game::input::GameKeyEventSourceInterface::GameKeyEventSourceInterface for BuildingLayer {
    fn getSourceId(&mut self) -> i32 { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::getSourceId(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self)) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for BuildingLayer {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer::set(crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn::as_AllBinaryGameLayer_mut(self), p1) }
}


