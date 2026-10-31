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
use crate::org::allbinary::game::layer::RTSGameStrings::RTSGameStrings;
use crate::org::allbinary::game::layer::RTSInterface::RTSInterface;
use crate::org::allbinary::game::layer::RTSPlayerLayerInterface::RTSPlayerLayerInterface;
use crate::org::allbinary::game::layer::TechnologyRTSInterfaceImageItem::TechnologyRTSInterfaceImageItem;
use crate::org::allbinary::game::layer::capital::Capital::Capital;
use crate::org::allbinary::game::rts::technology::event::TechEventHandler::TechEventHandler;
use crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItem;
use crate::org::allbinary::media::audio::BuildingSound::BuildingSound;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::logic::java::bool::BooleanFactory::BooleanFactory;
use crate::org::allbinary::game::identification::Group::Group;
use crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManager;
use crate::org::allbinary::game::layer::hud::event::GameNotificationEvent::GameNotificationEvent;
use crate::org::allbinary::game::layer::hud::event::GameNotificationEventHandler::GameNotificationEventHandler;
use crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager;
use crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject;
use crate::org::allbinary::logic::math::SmallIntegerSingletonFactory::SmallIntegerSingletonFactory;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::media::audio::ErrorSound::ErrorSound;
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMap;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCompositeInterface::GeographicMapCompositeInterface;
use super::RTSFormInput::RTSFormInput;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TechRTSFormInput {
    pub base: RTSFormInput,
    pub EVENT: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>,
    pub noMoneyGameNotificationEvent: Option<Rc<RefCell<GameNotificationEvent>>>,
}


impl TechRTSFormInput {
    

    pub fn new_struct(mut groupInterface: Vec<Option<Rc<RefCell<Group>>>>) -> Self {
        let mut this = Self {
            base: RTSFormInput::new_struct(groupInterface.clone()),
            EVENT: None,
            noMoneyGameNotificationEvent: None,
        };
        this.EVENT = AllBinaryEventObject::new(this).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>);
        // super(...) is in the struct literal above
        let mut basicColorFactory: Option<Rc<RefCell<BasicColorFactory>>> = BasicColorFactory::getInstance();
        this
    }

    pub fn new(mut groupInterface: Vec<Option<Rc<RefCell<Group>>>>) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(groupInterface))));
            this.noMoneyGameNotificationEvent = GameNotificationEvent::new((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()), jcall!(RTSGameStrings::getInstance()).NO_MONEY.clone(), jcall!(SmallIntegerSingletonFactory::getInstance()).getAt(2), jcall!(basicColorFactory).WHITE.clone(), jcall!(BooleanFactory::getInstance()).FALSE.clone());
        this
    }
    

    // throws Exception
    pub fn setAllBinaryGameLayerManager(&mut self, mut allBinaryGameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>) {
        self.base.setAllBinaryGameLayerManager(allBinaryGameLayerManager.clone());
        let mut geographicMapCompositeInterface: Option<Rc<RefCell<dyn GeographicMapCompositeInterface>>> = (allBinaryGameLayerManager).clone() /* as GeographicMapCompositeInterface */;
        let mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>> = jcall!(geographicMapCompositeInterface).getGeographicMapInterface()[(0) as usize].clone();
        { let java_receiver = self.noMoneyGameNotificationEvent.clone(); let java_arguments = (jcall!(geographicMapInterface).getForegroundBasicColor(),); jcall!(java_receiver).setBasicColorP(java_arguments.0); };
    }
    

    // throws Exception
    pub fn processGameSpecific(&mut self, mut associatedRtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut itemIndex: i32) {
        self.base.process(layerManager.clone());
        let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
        let mut stringMaker: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut technologyRTSInterfaceImageItem: Option<Rc<RefCell<TechnologyRTSInterfaceImageItem>>> = (item).clone() /* as TechnologyRTSInterfaceImageItem */;
        let mut rtsInterface: Option<Rc<RefCell<dyn RTSInterface>>> = jcall!(technologyRTSInterfaceImageItem).getRtsInterface();
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringMaker).append(String::from("isUpgradeable: ")).appendboolean(jcall!(rtsInterface).isUpgradeable()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).PROCESS.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        if jcall!(rtsInterface).isUpgradeable() {
            let mut cost: i32 = jcall!(rtsInterface).getUpgradeCost();
            let mut capital: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::capital::Capital::CapitalDyn>>> = jcall!(rtsPlayerLayerInterface).getCapital();
            jcall!(stringMaker).delete(0, jcall!(stringMaker).length());
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringMaker).appendint(cost.clone()).append(String::from("<=")).appendint(jcall!(capital).getTotalMoney()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).PROCESS.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            if cost <= jcall!(capital).getTotalMoney() {
                jcall!(rtsPlayerLayerInterface).add(BuildingSound::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>));
                jcall!(capital).removeMoney(cost.clone());
                jcall!(rtsInterface).upgrade();
                jcall!(technologyRTSInterfaceImageItem).update();
                jcall!(TechEventHandler::getInstance()).fireEvent(self.EVENT.clone());
            } else {
                jcall!(rtsPlayerLayerInterface).add(ErrorSound::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>));
                if !jcall!(rtsPlayerLayerInterface).implmentsArtificialIntelligenceCompositeInterface() {
                    jcall!(GameNotificationEventHandler::getInstance()).fireEvent(self.noMoneyGameNotificationEvent.clone());
                }
            }
        }
    }
}
impl std::ops::Deref for TechRTSFormInput {
    type Target = RTSFormInput;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for TechRTSFormInput {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::input::form::RTSFormInput::RTSFormInputDyn for TechRTSFormInput {
    fn as_RTSFormInput(&self) -> &crate::org::allbinary::game::input::form::RTSFormInput::RTSFormInput { &self.base }
    fn as_RTSFormInput_mut(&mut self) -> &mut crate::org::allbinary::game::input::form::RTSFormInput::RTSFormInput { &mut self.base }
    fn setAllBinaryGameLayerManager(&mut self, mut allBinaryGameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>) { Self::setAllBinaryGameLayerManager(self, allBinaryGameLayerManager) }
    fn processGameSpecific(&mut self, mut associatedRtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut itemIndex: i32) { Self::processGameSpecific(self, associatedRtsLayer, rtsPlayerLayerInterface, layerManager, item, itemIndex) }
}


