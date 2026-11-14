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
use crate::javax::microedition::lcdui::ChoiceGroup::ChoiceGroup;
use crate::javax::microedition::lcdui::Item::Item;
use crate::javax::microedition::lcdui::ItemStateListener::ItemStateListener;
use crate::org::allbinary::game::configuration::GameConfigurationGauge::GameConfigurationGauge;
use crate::org::allbinary::game::configuration::GameConfigurationUtil::GameConfigurationUtil;
use crate::org::allbinary::game::configuration::GameOptionsForm::GameOptionsForm;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::java::lang::Exception::*;
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

pub static mut GameFeatureItemStateListener_toggleList: std::cell::LazyCell<Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>> = std::cell::LazyCell::new(|| BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>));
impl ItemStateListener for GameFeatureItemStateListener {
    fn itemStateChanged(&mut self, mut item: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Item::ItemDyn>>>) { Self::itemStateChanged(self, item) }
}

impl GameFeatureItemStateListener {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn new_struct(mut gameOptionsForm: Option<Rc<RefCell<GameOptionsForm>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            gameOptionsForm: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.gameOptionsForm = gameOptionsForm.clone();
        let mut gameFeatureFactory: Option<Rc<RefCell<GameFeatureFactory>>> = GameFeatureFactory::getInstance();
        GameFeatureItemStateListener::add({ let java_field = jcall!(gameFeatureFactory).ARTIFICIAL_INTELLEGENCE_PROCESSOR.clone(); java_field }.clone());
        GameFeatureItemStateListener::add({ let java_field = jcall!(gameFeatureFactory).COLLIDABLE_INTERFACE_LAYER_PROCESSOR.clone(); java_field }.clone());
        GameFeatureItemStateListener::add({ let java_field = jcall!(gameFeatureFactory).DAMAGE_FLOATERS.clone(); java_field }.clone());
        GameFeatureItemStateListener::add({ let java_field = jcall!(gameFeatureFactory).DROPPED_ITEMS.clone(); java_field }.clone());
        GameFeatureItemStateListener::add({ let java_field = jcall!(gameFeatureFactory).GAME_INPUT_LAYER_PROCESSOR.clone(); java_field }.clone());
        GameFeatureItemStateListener::add({ let java_field = jcall!(gameFeatureFactory).HEALTH_BARS.clone(); java_field }.clone());
        GameFeatureItemStateListener::add({ let java_field = jcall!(gameFeatureFactory).SOUND.clone(); java_field }.clone());
        GameFeatureItemStateListener::add({ let java_field = jcall!(gameFeatureFactory).TICKABLE_LAYER_PROCESSOR.clone(); java_field }.clone());
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
    

    pub fn itemStateChanged(&mut self, mut item: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Item::ItemDyn>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut itemLabel: String = jcall!(item).getLabel();
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append({ let java_field = jcall!(CommonLabels::getInstance()).ITEM_LABEL.clone(); java_field }.clone()).append(itemLabel.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("itemStateChanged"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            if (item).is_some() /* instanceof GameConfigurationGauge */ {
                jcall!(GameConfigurationUtil::getInstance()).change(self.gameOptionsForm.clone(), (item).clone().map(|reference| { assert!({ let java_borrowed = reference.borrow(); let java_any: &dyn std::any::Any = &*java_borrowed; java_any.is::<GameConfigurationGauge>() }, "ClassCastException: not a GameConfigurationGauge"); unsafe { Rc::from_raw(Rc::into_raw(reference) as *const RefCell<GameConfigurationGauge>) } }));
            } else if (item).is_some() /* instanceof ChoiceGroup */ {
                let mut gameFeatureUtil: Option<Rc<RefCell<GameFeatureUtil>>> = GameFeatureUtil::getInstance();
                if jcall!(gameFeatureUtil).isExclusive(itemLabel.clone()) {
                    jcall!(gameFeatureUtil).updateExclusiveForChoiceGroup((|| -> Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::ChoiceGroup::ChoiceGroupDyn>>> { panic!("ClassCastException: a Rust dyn Item cannot be cast to ChoiceGroup") })());
                } else {
                    jcall!(gameFeatureUtil).updateMultiple((|| -> Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::ChoiceGroup::ChoiceGroupDyn>>> { panic!("ClassCastException: a Rust dyn Item cannot be cast to ChoiceGroup") })());
                }
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(self.logUtil).put({ let java_field = jcall!(commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("itemStateChanged"), crate::java_into_object(e.clone()));
            }
        }
    }
}

