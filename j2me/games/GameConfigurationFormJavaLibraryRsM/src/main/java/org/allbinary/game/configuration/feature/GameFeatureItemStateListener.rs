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
// use crate::javax::microedition::lcdui::ChoiceGroup::ChoiceGroup;
// use crate::javax::microedition::lcdui::Item::Item;
// use crate::javax::microedition::lcdui::ItemStateListener::ItemStateListener;
use crate::org::allbinary::game::configuration::GameConfigurationGauge::GameConfigurationGauge;
use crate::org::allbinary::game::configuration::GameConfigurationUtil::GameConfigurationUtil;
use crate::org::allbinary::game::configuration::GameOptionsForm::GameOptionsForm;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use super::GameFeatureFactory::GameFeatureFactory;
use super::GameFeature::GameFeature;
use super::GameFeatureUtil::GameFeatureUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GameFeatureItemStateListener {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub gameOptionsForm: Option<Rc<RefCell<GameOptionsForm>>>,
}

pub static mut GameFeatureItemStateListener_toggleList: std::cell::LazyCell<Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>> = std::cell::LazyCell::new(|| BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>));
// impl ItemStateListener for GameFeatureItemStateListener with the methods of the trait

impl GameFeatureItemStateListener {
    

    pub fn new_struct(mut gameOptionsForm: Option<Rc<RefCell<GameOptionsForm>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            gameOptionsForm: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.gameOptionsForm = gameOptionsForm.clone();
        let mut gameFeatureFactory: Option<Rc<RefCell<GameFeatureFactory>>> = GameFeatureFactory::getInstance();
        GameFeatureItemStateListener::add(jcall!(gameFeatureFactory).ARTIFICIAL_INTELLEGENCE_PROCESSOR.clone());
        GameFeatureItemStateListener::add(jcall!(gameFeatureFactory).COLLIDABLE_INTERFACE_LAYER_PROCESSOR.clone());
        GameFeatureItemStateListener::add(jcall!(gameFeatureFactory).DAMAGE_FLOATERS.clone());
        GameFeatureItemStateListener::add(jcall!(gameFeatureFactory).DROPPED_ITEMS.clone());
        GameFeatureItemStateListener::add(jcall!(gameFeatureFactory).GAME_INPUT_LAYER_PROCESSOR.clone());
        GameFeatureItemStateListener::add(jcall!(gameFeatureFactory).HEALTH_BARS.clone());
        GameFeatureItemStateListener::add(jcall!(gameFeatureFactory).SOUND.clone());
        GameFeatureItemStateListener::add(jcall!(gameFeatureFactory).TICKABLE_LAYER_PROCESSOR.clone());
        this
    }

    pub fn new(mut gameOptionsForm: Option<Rc<RefCell<GameOptionsForm>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(gameOptionsForm))))
    }
    

    pub fn add(mut gameFeature: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::GameFeature::GameFeatureDyn>>>) {
        if !jcall!(unsafe { GameFeatureItemStateListener_toggleList.clone() }).contains(crate::java_into_object(gameFeature.clone())) {
            jcall!(unsafe { GameFeatureItemStateListener_toggleList.clone() }).add(crate::java_into_object(gameFeature.clone()));
        }
    }
    

    pub fn itemStateChanged(&mut self, mut item: Option<Rc<RefCell<Item>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut itemLabel: String = jcall!(item).getLabel();
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(CommonLabels::getInstance()).ITEM_LABEL.clone()).append(itemLabel.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("itemStateChanged"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            if (item).is_some() /* instanceof GameConfigurationGauge */ {
                jcall!(GameConfigurationUtil::getInstance()).change(self.gameOptionsForm.clone(), (item).clone() /* as GameConfigurationGauge */);
            } else if (item).is_some() /* instanceof ChoiceGroup */ {
                let mut gameFeatureUtil: Option<Rc<RefCell<GameFeatureUtil>>> = GameFeatureUtil::getInstance();
                if jcall!(gameFeatureUtil).isExclusive(itemLabel.clone()) {
                    jcall!(gameFeatureUtil).updateExclusiveForChoiceGroup((item).clone() /* as ChoiceGroup */);
                } else {
                    jcall!(gameFeatureUtil).updateMultiple((item).clone() /* as ChoiceGroup */);
                }
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(self.logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("itemStateChanged"), crate::java_into_object(e.clone()));
            }
        }
    }
}

