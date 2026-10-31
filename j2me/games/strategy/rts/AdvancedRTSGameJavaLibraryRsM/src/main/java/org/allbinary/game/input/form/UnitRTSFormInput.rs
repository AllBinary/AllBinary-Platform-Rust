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
use crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItem;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::game::layer::AdvancedPlayerOwnedRTSLayers::AdvancedPlayerOwnedRTSLayers;
use crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer;
use crate::org::allbinary::game::layer::AdvancedRTSPlayerLayerInterface::AdvancedRTSPlayerLayerInterface;
use crate::org::allbinary::game::layer::RTSGameStrings::RTSGameStrings;
use crate::org::allbinary::game::layer::RTSLayer::RTSLayer;
use crate::org::allbinary::game::layer::RTSPlayerLayerInterface::RTSPlayerLayerInterface;
use crate::org::allbinary::game::layer::capital::Capital::Capital;
use crate::org::allbinary::game::layer::unit::UnitLayer::UnitLayer;
use crate::org::allbinary::game::layer::waypoint::WorkWaypoint::WorkWaypoint;
use crate::org::allbinary::media::audio::BuildingSound::BuildingSound;
use crate::org::allbinary::logic::java::bool::BooleanFactory::BooleanFactory;
use crate::org::allbinary::game::identification::Group::Group;
use crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManager;
use crate::org::allbinary::game::layer::GeographicMapCellPositionAreaBase::GeographicMapCellPositionAreaBase;
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
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCompositeInterface::GeographicMapCompositeInterface;
use super::RTSFormInput::RTSFormInput;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct UnitRTSFormInput {
    pub base: RTSFormInput,
    pub noMoneyGameNotificationEvent: Option<Rc<RefCell<GameNotificationEvent>>>,
    pub newUnitGameNotificationEvent: Option<Rc<RefCell<GameNotificationEvent>>>,
}

thread_local! { pub static UnitRTSFormInput_DECAL_ID: Option<i32> = jcall!(SmallIntegerSingletonFactory::getInstance()).getAt(23); }

impl UnitRTSFormInput {
    

    pub fn new_struct(mut groupInterface: Vec<Option<Rc<RefCell<Group>>>>) -> Self {
        let mut this = Self {
            base: RTSFormInput::new_struct(groupInterface.clone()),
            noMoneyGameNotificationEvent: None,
            newUnitGameNotificationEvent: None,
        };
        // super(...) is in the struct literal above
        let mut smallIntegerSingletonFactory: Option<Rc<RefCell<SmallIntegerSingletonFactory>>> = SmallIntegerSingletonFactory::getInstance();
        let mut basicColorFactory: Option<Rc<RefCell<BasicColorFactory>>> = BasicColorFactory::getInstance();
        jcall!(this.getHashtable()).put(crate::org::allbinary::game::layer::waypoint::WorkWaypoint::WorkWaypoint_ID.with(Clone::clone).clone(), jcall!(smallIntegerSingletonFactory).getAt(50));
        this
    }

    pub fn new(mut groupInterface: Vec<Option<Rc<RefCell<Group>>>>) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(groupInterface))));
            this.noMoneyGameNotificationEvent = GameNotificationEvent::new((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()), jcall!(RTSGameStrings::getInstance()).NO_MONEY.clone(), jcall!(smallIntegerSingletonFactory).getAt(2), jcall!(basicColorFactory).WHITE.clone(), jcall!(BooleanFactory::getInstance()).FALSE.clone());
            this.newUnitGameNotificationEvent = GameNotificationEvent::new((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()), jcall!(RTSGameStrings::getInstance()).NEW_UNIT.clone(), jcall!(smallIntegerSingletonFactory).getAt(2), jcall!(basicColorFactory).WHITE.clone(), jcall!(BooleanFactory::getInstance()).FALSE.clone());
        this
    }
    

    // throws Exception
    pub fn setAllBinaryGameLayerManager(&mut self, mut allBinaryGameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>) {
        self.base.setAllBinaryGameLayerManager(allBinaryGameLayerManager.clone());
        let mut geographicMapCompositeInterface: Option<Rc<RefCell<dyn GeographicMapCompositeInterface>>> = (allBinaryGameLayerManager).clone() /* as GeographicMapCompositeInterface */;
        let mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>> = jcall!(geographicMapCompositeInterface).getGeographicMapInterface()[(0) as usize].clone();
        { let java_receiver = self.noMoneyGameNotificationEvent.clone(); let java_arguments = (jcall!(geographicMapInterface).getForegroundBasicColor(),); jcall!(java_receiver).setBasicColorP(java_arguments.0); };
        { let java_receiver = self.newUnitGameNotificationEvent.clone(); let java_arguments = (jcall!(geographicMapInterface).getForegroundBasicColor(),); jcall!(java_receiver).setBasicColorP(java_arguments.0); };
    }
    

    // throws Exception
    pub fn processGameSpecific(&mut self, mut associatedRtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut itemIndex: i32) {
        self.base.process(layerManager.clone());
        let mut associatedRtsLayer2: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>> = (associatedRtsLayer).clone() /* as RTSLayer */;
        let mut geographicMapCellPositionArea: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::GeographicMapCellPositionAreaBase::GeographicMapCellPositionAreaBaseDyn>>> = jcall!(associatedRtsLayer2).as_RTSLayer_mut().geographicMapCellPositionAreaBase.clone();
        let mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = jcall!(geographicMapCellPositionArea).getNextSurroundingGeographicMapCellPosition();
        jcall!(self.getHashtable()).put(crate::org::allbinary::layer::Layer::Layer_ID.with(Clone::clone).clone(), associatedRtsLayer.clone());
        jcall!(self.getHashtable()).put(UnitRTSFormInput_DECAL_ID.with(Clone::clone).clone(), jcall!((panic!("ClassCastException: a Rust dyn RTSPlayerLayerInterface cannot be cast to dyn AdvancedRTSPlayerLayerInterface"))).getDecalBasicColor());
        if match (&self.newUnconstructedRTSLayerInterfaceArray[(itemIndex) as usize], &CollidableDestroyableDamageableLayer::getNullInstance()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            self.newUnconstructedRTSLayerInterfaceArray[(itemIndex) as usize] = self.getInstance(layerManager.clone(), item.clone(), geographicMapCellPosition.clone());
        } else {
            let mut cellPoint: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = jcall!(geographicMapCellPosition).getPoint();
            let mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>> = (self.newUnconstructedRTSLayerInterfaceArray[(itemIndex) as usize]).clone() /* as RTSLayer */;
            jcall!(rtsLayer).setPosition(jcall!(cellPoint).getX(), jcall!(cellPoint).getY(), jcall!(rtsLayer).getZP());
            let mut geographicMapCompositeInterface: Option<Rc<RefCell<dyn GeographicMapCompositeInterface>>> = (jcall!(rtsLayer).as_RTSLayer_mut().allBinaryGameLayerManagerP).clone() /* as GeographicMapCompositeInterface */;
            let mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>> = jcall!(geographicMapCompositeInterface).getGeographicMapInterface()[(0) as usize].clone();
            jcall!(jcall!(rtsLayer).as_RTSLayer_mut().geographicMapCellPositionAreaBase).update(geographicMapInterface.clone());
        }
        let mut cellPoint: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = jcall!(geographicMapCellPosition).getMidPoint();
        let mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>> = (self.newUnconstructedRTSLayerInterfaceArray[(itemIndex) as usize]).clone() /* as RTSLayer */;
        jcall!(rtsLayer).setPosition(jcall!(cellPoint).getX() - jcall!(rtsLayer).getHalfWidth(), jcall!(cellPoint).getY() - jcall!(rtsLayer).getHalfHeight(), jcall!(rtsLayer).getZP());
        self.attemptBuild(associatedRtsLayer2.clone(), rtsPlayerLayerInterface.clone(), layerManager.clone(), (rtsLayer).clone() /* as RTSLayer */, itemIndex.clone());
    }
    

    // throws Exception
    pub fn attemptBuild(&mut self, mut associatedRtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>>, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>>, mut itemIndex: i32) {
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
            jcall!(rtsPlayerLayerInterface).add(BuildingSound::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>));
            jcall!(capital).removeMoney(cost.clone());
            jcall!(AssignWaypointsUtil::AssignWaypointsUtil::getInstance()).set((layerInterface).clone() /* as UnitLayer */, (associatedRtsLayer).clone() /* as AdvancedRTSGameLayer */);
            jcall!(layerManager).appendAt(layerInterface.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>), unsafe { PlayersSingletonFactory_total.clone() }.clone());
            let mut advancedRTSPlayerLayerInterface: Option<Rc<RefCell<dyn AdvancedRTSPlayerLayerInterface>>> = panic!("ClassCastException: a Rust dyn RTSPlayerLayerInterface cannot be cast to dyn AdvancedRTSPlayerLayerInterface");
            let mut advancedPlayerOwnedRTSLayers: Option<Rc<RefCell<AdvancedPlayerOwnedRTSLayers>>> = jcall!(advancedRTSPlayerLayerInterface).getAdvancedPlayerOwnedRTSLayers();
            jcall!(advancedPlayerOwnedRTSLayers).addUnit(layerInterface.clone());
            if !jcall!(rtsPlayerLayerInterface).implmentsArtificialIntelligenceCompositeInterface() {
                jcall!(GameNotificationEventHandler::getInstance()).fireEvent(self.newUnitGameNotificationEvent.clone());
            }
        } else {
            jcall!(rtsPlayerLayerInterface).add(ErrorSound::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>));
            if !jcall!(rtsPlayerLayerInterface).implmentsArtificialIntelligenceCompositeInterface() {
                jcall!(GameNotificationEventHandler::getInstance()).fireEvent(self.noMoneyGameNotificationEvent.clone());
            }
        }
    }
}
impl std::ops::Deref for UnitRTSFormInput {
    type Target = RTSFormInput;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for UnitRTSFormInput {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::input::form::RTSFormInput::RTSFormInputDyn for UnitRTSFormInput {
    fn as_RTSFormInput(&self) -> &crate::org::allbinary::game::input::form::RTSFormInput::RTSFormInput { &self.base }
    fn as_RTSFormInput_mut(&mut self) -> &mut crate::org::allbinary::game::input::form::RTSFormInput::RTSFormInput { &mut self.base }
    fn setAllBinaryGameLayerManager(&mut self, mut allBinaryGameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>) { Self::setAllBinaryGameLayerManager(self, allBinaryGameLayerManager) }
    fn processGameSpecific(&mut self, mut associatedRtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut itemIndex: i32) { Self::processGameSpecific(self, associatedRtsLayer, rtsPlayerLayerInterface, layerManager, item, itemIndex) }
}


