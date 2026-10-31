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
// package org.allbinary.game.configuration
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::configuration::feature::Features::Features;
use crate::org::allbinary::game::configuration::feature::GameFeatureChoiceGroups::GameFeatureChoiceGroups;
use crate::org::allbinary::game::configuration::feature::InGameFeatureChoiceGroups::InGameFeatureChoiceGroups;
use crate::org::allbinary::game::configuration::feature::SensorFeatureFactory::SensorFeatureFactory;
use crate::org::allbinary::game::configuration::feature::TouchFeatureFactory::TouchFeatureFactory;
use crate::org::allbinary::init::Init::Init;
use crate::org::allbinary::input::gyro::OrientationData::OrientationData;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InGameFeatures {
    pub base: Init,
}


impl InGameFeatures {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: Init::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn init(&mut self) {
        let mut LABEL: String = String::from("Screen Buttons");
        let mut orientationData: Option<Rc<RefCell<OrientationData>>> = OrientationData::getInstance();
        let mut exclusiveOrientationSensorVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = (jcall!(jcall!(GameFeatureChoiceGroups::getExclusiveInstance()).get()).get((jcall!(orientationData).ORIENTATION_SENSOR_INPUT).clone() /* as Any */)).clone() /* as BasicArrayList */;
        let mut inGameFeatureChoiceGroups: Option<Rc<RefCell<InGameFeatureChoiceGroups>>> = InGameFeatureChoiceGroups::getExclusiveInstance();
        if exclusiveOrientationSensorVector.is_some() {
            let mut inGameExclusiveOrientationSensorVectorCanBeNull: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(jcall!(inGameFeatureChoiceGroups).get()).get((jcall!(orientationData).ORIENTATION_SENSOR_INPUT).clone() /* as Any */);
            if inGameExclusiveOrientationSensorVectorCanBeNull.is_none() {
                self.addToInGameMenu();
            } else {
                let mut inGameExclusiveOrientationSensorVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = (inGameExclusiveOrientationSensorVectorCanBeNull).clone() /* as BasicArrayList */;
                if jcall!(inGameExclusiveOrientationSensorVector).size() == 0 {
                    self.addToInGameMenu();
                }
            }
        }
        let mut features: Option<Rc<RefCell<Features>>> = Features::getInstance();
        if jcall!(features).isFeature(jcall!(TouchFeatureFactory::getInstance()).SHOW_SCREEN_BUTTONS.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) || jcall!(features).isFeature(jcall!(TouchFeatureFactory::getInstance()).AUTO_HIDE_SHOW_SCREEN_BUTTONS.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) || jcall!(features).isFeature(jcall!(TouchFeatureFactory::getInstance()).HIDE_SCREEN_BUTTONS.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
            let mut exclusiveScreenButtonsVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            let mut touchFeatureFactory: Option<Rc<RefCell<TouchFeatureFactory>>> = TouchFeatureFactory::getInstance();
            jcall!(exclusiveScreenButtonsVector).add(crate::java_into_object(jcall!(touchFeatureFactory).AUTO_HIDE_SHOW_SCREEN_BUTTONS.clone()));
            jcall!(exclusiveScreenButtonsVector).add(crate::java_into_object(jcall!(touchFeatureFactory).SHOW_SCREEN_BUTTONS.clone()));
            jcall!(exclusiveScreenButtonsVector).add(crate::java_into_object(jcall!(touchFeatureFactory).HIDE_SCREEN_BUTTONS.clone()));
            jcall!(inGameFeatureChoiceGroups).add(LABEL.clone(), exclusiveScreenButtonsVector.clone());
        }
    }
    

    pub fn addToInGameMenu(&mut self) {
        let mut orientationData: Option<Rc<RefCell<OrientationData>>> = OrientationData::getInstance();
        let mut inGameExclusiveOrientationSensorVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        jcall!(inGameExclusiveOrientationSensorVector).add(crate::java_into_object(jcall!(SensorFeatureFactory::getInstance()).ORIENTATION_SENSORS.clone()));
        jcall!(inGameExclusiveOrientationSensorVector).add(crate::java_into_object(jcall!(SensorFeatureFactory::getInstance()).NO_ORIENTATION.clone()));
        jcall!(InGameFeatureChoiceGroups::getExclusiveInstance()).add(jcall!(orientationData).ORIENTATION_SENSOR_INPUT.clone(), inGameExclusiveOrientationSensorVector.clone());
    }
    

    pub fn isAny(&mut self) -> bool {
        let mut features: Option<Rc<RefCell<Features>>> = Features::getInstance();
        let mut touchFeatureFactory: Option<Rc<RefCell<TouchFeatureFactory>>> = TouchFeatureFactory::getInstance();
        if jcall!(features).isFeature(jcall!(touchFeatureFactory).SHOW_SCREEN_BUTTONS.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) || jcall!(features).isFeature(jcall!(touchFeatureFactory).AUTO_HIDE_SHOW_SCREEN_BUTTONS.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) || jcall!(features).isFeature(jcall!(touchFeatureFactory).HIDE_SCREEN_BUTTONS.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
            return true;
        }
        let mut exclusiveOrientationSensorVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = (jcall!(jcall!(GameFeatureChoiceGroups::getExclusiveInstance()).get()).get((jcall!(OrientationData::getInstance()).ORIENTATION_SENSOR_INPUT).clone() /* as Any */)).clone() /* as BasicArrayList */;
        if exclusiveOrientationSensorVector.is_some() {
            return true;
        }
        return false;
    }
}
impl std::ops::Deref for InGameFeatures {
    type Target = Init;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for InGameFeatures {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::init::Init::InitDyn for InGameFeatures {
    fn as_Init(&self) -> &crate::org::allbinary::init::Init::Init { &self.base }
    fn as_Init_mut(&mut self) -> &mut crate::org::allbinary::init::Init::Init { &mut self.base }
}
impl crate::org::allbinary::init::InitInterface::InitInterface for InGameFeatures {
    fn init(&mut self) { Self::init(self) }
}


