//
// AllBinary Open License Version 1
// Copyright (c) 2011 AllBinary
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
// package org.allbinary.game.configuration.feature
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::configuration::event::GameFeatureEvent::GameFeatureEvent;
use crate::org::allbinary::game::configuration::event::GameFeatureEventHandler::GameFeatureEventHandler;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystem;
use crate::org::allbinary::logic::system::os::OperatingSystemFactory::OperatingSystemFactory;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use super::Feature::Feature;
use super::GameFeatureFactory::GameFeatureFactory;
use super::InputFeatureFactory::InputFeatureFactory;
use super::SensorFeatureFactory::SensorFeatureFactory;
use super::TouchFeatureFactory::TouchFeatureFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct Features {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub defaultList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}

pub static mut Features_SINGLETON: std::cell::LazyCell<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = std::cell::LazyCell::new(|| jcall!(NullUtil::getInstance()).NULL_OBJECT.clone());

impl Features {
    

    pub fn getInstance() -> Option<Rc<RefCell<Features>>> {
        if match (&unsafe { Features_SINGLETON.clone() }, &jcall!(NullUtil::getInstance()).NULL_OBJECT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            unsafe { *Features_SINGLETON = Features::new() };
        }
        return (unsafe { Features_SINGLETON.clone() }).clone() /* as Features */;
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            list: None,
            defaultList: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.list = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.defaultList = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.init();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn addDefault(&mut self, mut gameFeature: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>>) {
        self.add(gameFeature.clone());
        if !jcall!(self.defaultList).contains(crate::java_into_object(gameFeature.clone())) {
            jcall!(self.defaultList).add(crate::java_into_object(gameFeature.clone()));
        }
    }
    

    // throws Exception
    pub fn add(&mut self, mut gameFeature: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>>) {
        if !jcall!(self.list).contains(crate::java_into_object(gameFeature.clone())) {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(CommonLabels::getInstance()).START_LABEL.clone()).append(jcall!(gameFeature).toString()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).ADD.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            jcall!(self.list).add(crate::java_into_object(gameFeature.clone()));
            jcall!(GameFeatureEventHandler::getInstance()).fireEvent(GameFeatureEvent::new(gameFeature.clone(), jcall!(gameFeature).toString()));
        }
    }
    

    // throws Exception
    pub fn removeDefault(&mut self, mut gameFeature: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>>) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(CommonLabels::getInstance()).START_LABEL.clone()).append(jcall!(gameFeature).toString()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("removeDefault"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        self.remove(gameFeature.clone());
        jcall!(self.defaultList).remove(crate::java_into_object(gameFeature.clone()));
    }
    

    // throws Exception
    pub fn remove(&mut self, mut gameFeature: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>>) {
        if jcall!(self.list).contains(crate::java_into_object(gameFeature.clone())) {
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).REMOVE.clone());
            jcall!(self.list).remove(crate::java_into_object(gameFeature.clone()));
            jcall!(GameFeatureEventHandler::getInstance()).fireEvent(GameFeatureEvent::new(gameFeature.clone(), jcall!(gameFeature).toString()));
        }
    }
    

    pub fn init(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut gameFeatureFactory: Option<Rc<RefCell<GameFeatureFactory>>> = GameFeatureFactory::getInstance();
            let mut inputFeatureFactory: Option<Rc<RefCell<InputFeatureFactory>>> = InputFeatureFactory::getInstance();
            let mut sensorFeatureFactory: Option<Rc<RefCell<SensorFeatureFactory>>> = SensorFeatureFactory::getInstance();
            let mut touchFeatureFactory: Option<Rc<RefCell<TouchFeatureFactory>>> = TouchFeatureFactory::getInstance();
            self.addDefault(jcall!(gameFeatureFactory).ARTIFICIAL_INTELLEGENCE_PROCESSOR.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>));
            self.addDefault(jcall!(gameFeatureFactory).COLLIDABLE_INTERFACE_LAYER_PROCESSOR.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>));
            self.addDefault(jcall!(gameFeatureFactory).GAME_INPUT_LAYER_PROCESSOR.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>));
            self.addDefault(jcall!(gameFeatureFactory).TICKABLE_LAYER_PROCESSOR.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>));
            self.addDefault(jcall!(inputFeatureFactory).MULTI_KEY_PRESS.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>));
            self.addDefault(jcall!(inputFeatureFactory).REMOVE_DUPLICATE_KEY_PRESSES.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>));
            self.addDefault(jcall!(gameFeatureFactory).SCREEN_SHAKE.clone());
            self.addDefault(jcall!(gameFeatureFactory).POST_IMAGE_LOADING_MODIFICATION.clone());
            let mut operatingSystemInterface: Option<Rc<RefCell<dyn crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystemDyn>>> = jcall!(OperatingSystemFactory::getInstance()).getOperatingSystemInstance();
            if jcall!(operatingSystemInterface).isOverScan() {
                self.addDefault(jcall!(sensorFeatureFactory).NO_ORIENTATION.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>));
                self.addDefault(jcall!(touchFeatureFactory).HIDE_SCREEN_BUTTONS.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>));
            } else {
                self.addDefault(jcall!(sensorFeatureFactory).ORIENTATION_SENSORS.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>));
                self.addDefault(jcall!(touchFeatureFactory).AUTO_HIDE_SHOW_SCREEN_BUTTONS.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>));
            }
            self.addDefault(jcall!(touchFeatureFactory).TOUCH_ENABLED.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>));
            { let java_arguments = (jcall!(MainFeatureFactory::MainFeatureFactory::getInstance()).FULL_SCREEN.clone(),); self.addDefault(java_arguments.0) };
            { let java_arguments = (jcall!(MainFeatureFactory::MainFeatureFactory::getInstance()).LOAD_ONDEMAND.clone(),); self.addDefault(java_arguments.0) };
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            PreLogUtil::putOE(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).INIT.clone(), e.clone());
            }
        }
    }
    

    pub fn isDefault(&mut self, mut gameFeature: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>>) -> bool {
        return jcall!(self.defaultList).contains(crate::java_into_object(gameFeature.clone()));
    }
    

    pub fn isFeature(&mut self, mut gameFeature: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>>) -> bool {
        return jcall!(self.list).contains(crate::java_into_object(gameFeature.clone()));
    }
    

    // throws Exception
    pub fn toggle(&mut self, mut gameFeature: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>>) {
        if self.isFeature(gameFeature.clone()) {
            self.remove(gameFeature.clone());
        } else {
            self.add(gameFeature.clone());
        }
    }
}

