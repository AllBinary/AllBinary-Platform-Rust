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
// package org.allbinary.game.input
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::javax::microedition::lcdui::Canvas::Canvas;
use crate::org::allbinary::game::layer::RTSGameStrings::RTSGameStrings;
use crate::org::allbinary::game::layer::RTSLayer::RTSLayer;
use crate::org::allbinary::game::layer::RTSLayerInfoPaintable::RTSLayerInfoPaintable;
use crate::org::allbinary::game::layer::RTSPlayerLayerInterface::RTSPlayerLayerInterface;
use crate::org::allbinary::game::layer::capital::Capital::Capital;
use crate::org::allbinary::media::audio::DowngradeSound::DowngradeSound;
use crate::org::allbinary::media::audio::UpgradeSound::UpgradeSound;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::util::BasicArrayListUtil::BasicArrayListUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::java::bool::BooleanFactory::BooleanFactory;
use crate::org::allbinary::game::configuration::feature::Features::Features;
use crate::org::allbinary::game::configuration::feature::InputFeatureFactory::InputFeatureFactory;
use crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent;
use crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManager;
use crate::org::allbinary::game::layer::hud::event::GameNotificationEvent::GameNotificationEvent;
use crate::org::allbinary::game::layer::hud::event::GameNotificationEventHandler::GameNotificationEventHandler;
use crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager;
use crate::org::allbinary::logic::util::visitor::Visitor::Visitor;
use crate::org::allbinary::logic::math::SmallIntegerSingletonFactory::SmallIntegerSingletonFactory;
use crate::org::allbinary::media::audio::ErrorSound::ErrorSound;
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMap;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCompositeInterface::GeographicMapCompositeInterface;
use crate::java::lang::Exception::*;
use super::PlayerGameInput::PlayerGameInput;
use super::GameInputProcessor::GameInputProcessor;
use super::SelectRTSLayerVisitorFactoryInterface::SelectRTSLayerVisitorFactoryInterface;
use super::RTSPlayerGameInput::RTSPlayerGameInput;
use super::SelectedRTSLayersPlayerUpgradeGameInputProcessor::SelectedRTSLayersPlayerUpgradeGameInputProcessor;
use super::SelectedRTSLayersPlayerDowngradeGameInputProcessor::SelectedRTSLayersPlayerDowngradeGameInputProcessor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SelectedRTSLayersPlayerGameInput {
    pub base: PlayerGameInput,
    pub inputProcessorArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInputProcessor::GameInputProcessorDyn>>>>,
    pub list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub isSingleKeyProcessing: bool,
    pub selectedRTSLayersList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub preSelectedRTSLayersList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub paintSelectedRTSLayersList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>,
    pub selectRTSLayerVisitorInterface: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::visitor::Visitor::VisitorDyn>>>,
    pub upgradeGameNotificationEvent: Option<Rc<RefCell<GameNotificationEvent>>>,
    pub noMoneyGameNotificationEvent: Option<Rc<RefCell<GameNotificationEvent>>>,
    pub downgradeGameNotificationEvent: Option<Rc<RefCell<GameNotificationEvent>>>,
}


impl SelectedRTSLayersPlayerGameInput {
    

    pub fn new_struct(mut towerInfoPaintable: Option<Rc<RefCell<RTSLayerInfoPaintable>>>, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut playerInputId: i32, mut selectRTSLayerVisitorFactoryInterface: Option<Rc<RefCell<dyn SelectRTSLayerVisitorFactoryInterface>>>) -> Self {
        let mut this = Self {
            base: PlayerGameInput::new_struct(list.clone(), BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayListD::BasicArrayListDDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>), playerInputId.clone()),
            inputProcessorArray: Vec::new(),
            list: None,
            isSingleKeyProcessing: false,
            selectedRTSLayersList: None,
            preSelectedRTSLayersList: None,
            paintSelectedRTSLayersList: None,
            rtsPlayerLayerInterface: None,
            selectRTSLayerVisitorInterface: None,
            upgradeGameNotificationEvent: None,
            noMoneyGameNotificationEvent: None,
            downgradeGameNotificationEvent: None,
        };
        this.inputProcessorArray = vec![None; ({ let java_field = jcall!(InputFactory::InputFactory::getInstance()).MAX.clone(); java_field }) as usize];
        this.isSingleKeyProcessing = jcall!(Features::getInstance()).isFeature({ let java_field = jcall!(InputFeatureFactory::getInstance()).SINGLE_KEY_REPEAT_PRESS.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) || jcall!(Features::getInstance()).isFeature({ let java_field = jcall!(InputFeatureFactory::getInstance()).SINGLE_KEY_PRESS.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>));
        this.selectedRTSLayersList = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.preSelectedRTSLayersList = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.paintSelectedRTSLayersList = jcall!(BasicArrayListUtil::getInstance()).getImmutableInstance();
        // super(...) is in the struct literal above
        this.initInputProcessors();
        this.rtsPlayerLayerInterface = rtsPlayerLayerInterface.clone();
        this.list = list.clone();
        this
    }

    pub fn new(mut towerInfoPaintable: Option<Rc<RefCell<RTSLayerInfoPaintable>>>, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut playerInputId: i32, mut selectRTSLayerVisitorFactoryInterface: Option<Rc<RefCell<dyn SelectRTSLayerVisitorFactoryInterface>>>) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(towerInfoPaintable.clone(), rtsPlayerLayerInterface.clone(), list.clone(), playerInputId.clone(), selectRTSLayerVisitorFactoryInterface.clone()))));
            jcall!(this).selectRTSLayerVisitorInterface = { let java_value = (jcall!(selectRTSLayerVisitorFactoryInterface).create(this.clone())).clone(); java_value };
            let mut smallIntegerSingletonFactory: Option<Rc<RefCell<SmallIntegerSingletonFactory>>> = SmallIntegerSingletonFactory::getInstance();
            let mut basicColorFactory: Option<Rc<RefCell<BasicColorFactory>>> = BasicColorFactory::getInstance();
            jcall!(this).upgradeGameNotificationEvent = { let java_value = (GameNotificationEvent::new((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()), { let java_field = jcall!(RTSGameStrings::getInstance()).UPGRADE.clone(); java_field }.clone(), jcall!(smallIntegerSingletonFactory).getAt(2), { let java_field = jcall!(basicColorFactory).PINK.clone(); java_field }.clone(), { let java_field = jcall!(BooleanFactory::getInstance()).FALSE.clone(); java_field }.clone())).clone(); java_value };
            jcall!(this).noMoneyGameNotificationEvent = { let java_value = (GameNotificationEvent::new((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()), { let java_field = jcall!(RTSGameStrings::getInstance()).NO_MONEY.clone(); java_field }.clone(), jcall!(smallIntegerSingletonFactory).getAt(2), { let java_field = jcall!(basicColorFactory).PINK.clone(); java_field }.clone(), { let java_field = jcall!(BooleanFactory::getInstance()).FALSE.clone(); java_field }.clone())).clone(); java_value };
            jcall!(this).downgradeGameNotificationEvent = { let java_value = (GameNotificationEvent::new((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()), { let java_field = jcall!(RTSGameStrings::getInstance()).DOWNGRADE.clone(); java_field }.clone(), jcall!(smallIntegerSingletonFactory).getAt(2), { let java_field = jcall!(basicColorFactory).PINK.clone(); java_field }.clone(), { let java_field = jcall!(BooleanFactory::getInstance()).FALSE.clone(); java_field }.clone())).clone(); java_value };
        this
    }
    

    // throws Exception
    pub fn setAllBinaryGameLayerManager(&mut self, mut allBinaryGameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>) {
        let mut geographicMapCompositeInterface: Option<Rc<RefCell<dyn GeographicMapCompositeInterface>>> = (|| -> Option<Rc<RefCell<dyn GeographicMapCompositeInterface>>> { panic!("ClassCastException: a Rust dyn AllBinaryGameLayerManager cannot be cast to GeographicMapCompositeInterface") })();
        let mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>> = jcall!(geographicMapCompositeInterface).getGeographicMapInterface()[(0) as usize].clone();
        { let java_receiver = self.upgradeGameNotificationEvent.clone(); let java_arguments = (jcall!(geographicMapInterface).getForegroundBasicColor(),); jcall!(java_receiver).setBasicColorP(java_arguments.0); };
        { let java_receiver = self.noMoneyGameNotificationEvent.clone(); let java_arguments = (jcall!(geographicMapInterface).getForegroundBasicColor(),); jcall!(java_receiver).setBasicColorP(java_arguments.0); };
        { let java_receiver = self.downgradeGameNotificationEvent.clone(); let java_arguments = (jcall!(geographicMapInterface).getForegroundBasicColor(),); jcall!(java_receiver).setBasicColorP(java_arguments.0); };
    }
    

    pub fn isSelected(&mut self, mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>>) -> bool {
        if jcall!(self.selectedRTSLayersList).contains(crate::java_into_object(rtsLayer.clone())) {
            return true;
        } else {
            return false;
        }
    }
    

    // throws Exception
    pub fn upgrade(&mut self) {
        let mut anyChanged: bool = false;
        for index in (0..={ let java_bound = jcall!(self.selectedRTSLayersList).size() - 1; java_bound }).rev() {
            let mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>> = { let java_object = (jcall!(self.selectedRTSLayersList).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>>>().expect("ClassCastException").clone()) };
            if jcall!(rtsLayer).as_RTSLayer_mut().isUpgradeable() {
                let mut capital: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::capital::Capital::CapitalDyn>>> = jcall!(self.rtsPlayerLayerInterface).getCapital();
                let mut upgradeCost: i32 = jcall!(rtsLayer).as_RTSLayer_mut().getUpgradeCost();
                if upgradeCost <= jcall!(capital).getTotalMoney() {
                    anyChanged = true;
                    { let java_receiver = self.rtsPlayerLayerInterface.clone(); let java_arguments = (UpgradeSound::getInstance(),); jcall!(java_receiver).add(java_arguments.0); };
                    jcall!(rtsLayer).as_RTSLayer_mut().upgrade();
                    jcall!(capital).removeMoney(upgradeCost.clone());
                    if !jcall!(self.rtsPlayerLayerInterface).implmentsArtificialIntelligenceCompositeInterface() {
                        jcall!(GameNotificationEventHandler::getInstance()).fireEvent(self.upgradeGameNotificationEvent.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>));
                    }
                } else {
                    { let java_receiver = self.rtsPlayerLayerInterface.clone(); let java_arguments = (ErrorSound::getInstance(),); jcall!(java_receiver).add(java_arguments.0); };
                    if !jcall!(self.rtsPlayerLayerInterface).implmentsArtificialIntelligenceCompositeInterface() {
                        jcall!(GameNotificationEventHandler::getInstance()).fireEvent(self.noMoneyGameNotificationEvent.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>));
                    }
                }
            }
        }
        if anyChanged {
            let mut rtsPlayerGameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::RTSPlayerGameInput::RTSPlayerGameInputDyn>>> = ((jcall!(self.rtsPlayerLayerInterface).getPlayerGameInput()).clone());
            jcall!(rtsPlayerGameInput).updatePaintable();
        }
    }
    

    // throws Exception
    pub fn downgrade(&mut self) {
        let mut anyChanged: bool = false;
        {
            let mut index: i32 = jcall!(self.selectedRTSLayersList).size();
            while { index -= 1; index } >= 0 {
            let mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>> = { let java_object = (jcall!(self.selectedRTSLayersList).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>>>().expect("ClassCastException").clone()) };
            if jcall!(rtsLayer).as_RTSLayer_mut().isDowngradeable() {
                anyChanged = true;
                { let java_receiver = self.rtsPlayerLayerInterface.clone(); let java_arguments = (DowngradeSound::getInstance(),); jcall!(java_receiver).add(java_arguments.0); };
                let mut downgradeCost: i32 = jcall!(rtsLayer).as_RTSLayer_mut().getDowngradeCost();
                jcall!(rtsLayer).as_RTSLayer_mut().downgrade();
                let mut capital: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::capital::Capital::CapitalDyn>>> = jcall!(self.rtsPlayerLayerInterface).getCapital();
                jcall!(capital).addMoney(downgradeCost.clone());
                if !jcall!(self.rtsPlayerLayerInterface).implmentsArtificialIntelligenceCompositeInterface() {
                    jcall!(GameNotificationEventHandler::getInstance()).fireEvent(self.downgradeGameNotificationEvent.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>));
                }
            }
            }
        }
        if anyChanged {
            let mut rtsPlayerGameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::RTSPlayerGameInput::RTSPlayerGameInputDyn>>> = ((jcall!(self.rtsPlayerLayerInterface).getPlayerGameInput()).clone());
            jcall!(rtsPlayerGameInput).updatePaintable();
        }
    }
    

    pub fn initInputProcessors(&mut self) {
        { let java_index = (crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM1.with(Clone::clone)) as usize; self.inputProcessorArray[java_index] = SelectedRTSLayersPlayerUpgradeGameInputProcessor::new(None /* Java this: a &mut self method has no Rc of itself */).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::GameInputProcessor::GameInputProcessorDyn>>); };
        { let java_index = (crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM3.with(Clone::clone)) as usize; self.inputProcessorArray[java_index] = SelectedRTSLayersPlayerDowngradeGameInputProcessor::new(None /* Java this: a &mut self method has no Rc of itself */).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::GameInputProcessor::GameInputProcessorDyn>>); };
        GameInputProcessorUtil::GameInputProcessorUtil::init(self.inputProcessorArray.clone());
    }
    

    // throws Exception
    pub fn processInputKey(&mut self, mut key: i32) {
        if self.selectedRTSLayersList.is_some() {
            { let java_receiver = self.inputProcessorArray[(key) as usize].clone(); let java_arguments = (AllBinaryGameLayerManager::getNullInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>), crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent_NONE.with(Clone::clone).clone(),); jcall!(java_receiver).processEvent(java_arguments.0, java_arguments.1); };
        }
    }
    

    // throws Exception
    pub fn processInput(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut size: i32 = jcall!(self.list).size();
            let mut key: i32 = 0;
            for index in (0..size) {
                let mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>> = { let java_object = (jcall!(self.list).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<GameKeyEvent>>>>().expect("ClassCastException").clone()) };
                key = { let java_value = (jcall!(gameKeyEvent).getKey()).clone(); java_value };
                self.processInputKey(key.clone());
            }
            if self.isSingleKeyProcessing {
                self.clear();
            } else {
                self.update();
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.gameInputStrings).PROCESS_INPUT.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn getSelectedBasicArrayList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.selectedRTSLayersList.clone();
    }
    

    pub fn addSelectedRTSLayer(&mut self, mut selectedLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>>) {
        self.paintSelectedRTSLayersList = { let java_value = (jcall!(BasicArrayListUtil::getInstance()).getImmutableInstance()).clone(); java_value };
        if match (&selectedLayer, &CollidableDestroyableDamageableLayer::getNullInstance()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            self.deselectAll();
            jcall!(self.selectedRTSLayersList).clear();
        } else {
            if !jcall!(self.selectedRTSLayersList).contains(crate::java_into_object(selectedLayer.clone())) {
                jcall!(self.selectedRTSLayersList).add(crate::java_into_object(selectedLayer.clone()));
            }
        }
        self.paintSelectedRTSLayersList = self.selectedRTSLayersList.clone();
    }
    

    pub fn setSelectedRTSLayer(&mut self, mut selectedLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>) {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(String::from("Selected Layer: "));
        if selectedLayer.is_some() {
            jcall!(stringBuffer).append(jcall!(selectedLayer).getName());
        }
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setSelectedRTSLayer"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        self.paintSelectedRTSLayersList = { let java_value = (jcall!(BasicArrayListUtil::getInstance()).getImmutableInstance()).clone(); java_value };
        jcall!(self.selectRTSLayerVisitorInterface).as_Visitor_mut().visit(crate::java_into_object(selectedLayer.clone()));
        self.deselectAll();
        if jcall!(self.selectedRTSLayersList).size() > 0 {
            jcall!(self.getPreSelectedRTSLayersList()).clear();
            let mut tempList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.getPreSelectedRTSLayersList();
            self.preSelectedRTSLayersList = self.selectedRTSLayersList.clone();
            self.selectedRTSLayersList = tempList.clone();
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("Preselected: ")).append(jcall!(self.preSelectedRTSLayersList).toString()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setSelectedRTSLayer"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        if !match (&selectedLayer, &CollidableDestroyableDamageableLayer::getNullInstance()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            jcall!(self.selectedRTSLayersList).add(crate::java_into_object(selectedLayer.clone()));
        }
        self.paintSelectedRTSLayersList = self.selectedRTSLayersList.clone();
    }
    

    pub fn getLastSelectedRtsLayer(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>> {
        if self.isAnyRTSLayerSelected() {
            return { let java_object = ({ let java_receiver = self.selectedRTSLayersList.clone(); let java_arguments = (jcall!(self.selectedRTSLayersList).size() - 1,); let java_result = jcall!(java_receiver).get(java_arguments.0); java_result }).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>>().expect("ClassCastException").clone()) };
        } else {
            return CollidableDestroyableDamageableLayer::getNullInstance();
        }
    }
    

    pub fn selectAllPreselected(&mut self) {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(String::from("Select all Preselected: "));
        jcall!(stringBuffer).append(jcall!(self.preSelectedRTSLayersList).toString());
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("selectAllPreselected"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        for index in (0..={ let java_bound = jcall!(self.preSelectedRTSLayersList).size() - 1; java_bound }).rev() {
            let mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>> = { let java_object = (jcall!(self.preSelectedRTSLayersList).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>>>().expect("ClassCastException").clone()) };
            jcall!(rtsLayer).select();
        }
    }
    

    pub fn deselectAllPreselected(&mut self) {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(String::from("Deselect all Preselected: "));
        jcall!(stringBuffer).append(jcall!(self.preSelectedRTSLayersList).toString());
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("deselectAllPreselected"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        for index in (0..={ let java_bound = jcall!(self.preSelectedRTSLayersList).size() - 1; java_bound }).rev() {
            let mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>> = { let java_object = (jcall!(self.preSelectedRTSLayersList).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>>>().expect("ClassCastException").clone()) };
            jcall!(rtsLayer).deselect();
        }
        jcall!(self.preSelectedRTSLayersList).clear();
    }
    

    pub fn deselectAll(&mut self) {
        for index in (0..={ let java_bound = jcall!(self.selectedRTSLayersList).size() - 1; java_bound }).rev() {
            let mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>> = { let java_object = (jcall!(self.selectedRTSLayersList).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>>>().expect("ClassCastException").clone()) };
            jcall!(rtsLayer).deselect();
        }
    }
    

    pub fn isAnyRTSLayerSelected(&mut self) -> bool {
        if jcall!(self.selectedRTSLayersList).size() == 0 {
            return false;
        }
        return true;
    }
    

    pub fn getPaintSelectedRTSLayersList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.paintSelectedRTSLayersList.clone();
    }
    

    pub fn getPreSelectedRTSLayersList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.preSelectedRTSLayersList.clone();
    }
}
impl std::ops::Deref for SelectedRTSLayersPlayerGameInput {
    type Target = PlayerGameInput;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for SelectedRTSLayersPlayerGameInput {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::input::GameInput::GameInputDyn for SelectedRTSLayersPlayerGameInput {
    fn as_GameInput(&self) -> &crate::org::allbinary::game::input::GameInput::GameInput { &self.base.base }
    fn as_GameInput_mut(&mut self) -> &mut crate::org::allbinary::game::input::GameInput::GameInput { &mut self.base.base }
}
impl crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInputDyn for SelectedRTSLayersPlayerGameInput {
    fn as_PlayerGameInput(&self) -> &crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInput { &self.base }
    fn as_PlayerGameInput_mut(&mut self) -> &mut crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInput { &mut self.base }
}


