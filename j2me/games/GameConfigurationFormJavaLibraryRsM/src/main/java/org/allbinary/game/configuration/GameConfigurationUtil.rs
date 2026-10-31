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
// use crate::javax::microedition::lcdui::Item::Item;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::math::SmallIntegerSingletonFactory::SmallIntegerSingletonFactory;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::GameConfigurationSingleton::GameConfigurationSingleton;
use super::GameConfiguration::GameConfiguration;
use super::GameOptionsForm::GameOptionsForm;
use super::GameConfigurationGauge::GameConfigurationGauge;
use super::GameConfigurationCentral::GameConfigurationCentral;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GameConfigurationUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub GAUGE_UPDATE: String,
    pub FROM: String,
    pub TO: String,
    pub COMPETITION_VALUE: i32,
}

thread_local! { pub static GameConfigurationUtil_instance: Option<Rc<RefCell<GameConfigurationUtil>>> = GameConfigurationUtil::new(); }

impl GameConfigurationUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            GAUGE_UPDATE: String::new(),
            FROM: String::new(),
            TO: String::new(),
            COMPETITION_VALUE: 0,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.GAUGE_UPDATE = String::from("Gauge Update: ");
        this.FROM = String::from(" from: ");
        this.TO = String::from(" to: ");
        this.COMPETITION_VALUE = 1;
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<GameConfigurationUtil>>> {
        return GameConfigurationUtil_instance.with(Clone::clone).clone();
    }
    

    // throws Exception
    pub fn change(&mut self, mut gameOptionsForm: Option<Rc<RefCell<GameOptionsForm>>>, mut gauge: Option<Rc<RefCell<GameConfigurationGauge>>>) {
        self.update(gauge.clone());
        let mut gameConfigurationSingleton: Option<Rc<RefCell<GameConfigurationSingleton>>> = GameConfigurationSingleton::getInstance();
        let mut gameConfiguration: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::GameConfiguration::GameConfigurationDyn>>> = jcall!(gameConfigurationSingleton).getInstanceByName(jcall!(gauge).getLabel());
        self.updateChallange(gameOptionsForm.clone(), gameConfiguration.clone());
    }
    

    // throws Exception
    pub fn update(&mut self, mut gauge: Option<Rc<RefCell<GameConfigurationGauge>>>) {
        let mut gameConfigurationSingleton: Option<Rc<RefCell<GameConfigurationSingleton>>> = GameConfigurationSingleton::getInstance();
        let mut gameConfiguration: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::GameConfiguration::GameConfigurationDyn>>> = jcall!(gameConfigurationSingleton).getInstanceByName(jcall!(gauge).getLabel());
        let mut smallIntegerSingletonFactory: Option<Rc<RefCell<SmallIntegerSingletonFactory>>> = SmallIntegerSingletonFactory::getInstance();
        let mut value: Option<i32> = jcall!(smallIntegerSingletonFactory).getAt(jcall!(gauge).getValue() + ((jcall!(gameConfiguration).getMinValue()).unwrap() as i32));
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(self.GAUGE_UPDATE.clone());
        jcall!(stringBuffer).append(jcall!(gameConfiguration).getName());
        jcall!(stringBuffer).append(self.FROM.clone());
        jcall!(stringBuffer).appendint(((jcall!(gameConfiguration).getValue()).unwrap() as i32));
        jcall!(stringBuffer).append(self.TO.clone());
        jcall!(stringBuffer).appendint(((value).unwrap() as i32));
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).UPDATE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        jcall!(gameConfiguration).setValue(value.clone());
    }
    

    // throws Exception
    pub fn setDefault(&mut self, mut gauge: Option<Rc<RefCell<GameConfigurationGauge>>>) {
        let mut gameConfigurationSingleton: Option<Rc<RefCell<GameConfigurationSingleton>>> = GameConfigurationSingleton::getInstance();
        let mut gameConfiguration: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::GameConfiguration::GameConfigurationDyn>>> = jcall!(gameConfigurationSingleton).getInstanceByName(jcall!(gauge).getLabel());
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(String::from("Gauge Default: "));
        jcall!(stringBuffer).append(jcall!(gameConfiguration).getName());
        jcall!(stringBuffer).append(self.TO.clone());
        jcall!(stringBuffer).appendint(((jcall!(gameConfiguration).getDefaultValue()).unwrap() as i32));
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setDefault"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        jcall!(gauge).setValue(((jcall!(gameConfiguration).getDefaultValue()).unwrap() as i32) - ((jcall!(gameConfiguration).getMinValue()).unwrap() as i32));
        jcall!(gameConfiguration).setValue(jcall!(gameConfiguration).getDefaultValue());
    }
    

    // throws Exception
    pub fn updateChallange(&mut self, mut gameOptionsForm: Option<Rc<RefCell<GameOptionsForm>>>, mut gameConfiguration: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::GameConfiguration::GameConfigurationDyn>>>) {
        let mut gameConfigurationSingleton: Option<Rc<RefCell<GameConfigurationSingleton>>> = GameConfigurationSingleton::getInstance();
        let mut gameConfigurationCentral: Option<Rc<RefCell<GameConfigurationCentral>>> = GameConfigurationCentral::getInstance();
        if match (&gameConfiguration, &jcall!(gameConfigurationCentral).CHALLENGE_LEVEL) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("updateChallange"));
            jcall!(jcall!(gameConfigurationCentral).COLLIDE_DAMAGE).setValue(jcall!(gameConfiguration).getValue());
            jcall!(jcall!(gameConfigurationCentral).ATTACK_CHALLENGE_LEVEL).setValue(jcall!(gameConfiguration).getValue());
            jcall!(jcall!(gameConfigurationCentral).DURABILITY_CHALLENGE_LEVEL).setValue(jcall!(gameConfiguration).getValue());
            jcall!(jcall!(gameConfigurationCentral).SPEED_CHALLENGE_LEVEL).setValue(jcall!(gameConfiguration).getValue());
            let mut size: i32 = jcall!(gameOptionsForm).size();
            for index in (0..size) {
                let mut item: Option<Rc<RefCell<Item>>> = jcall!(gameOptionsForm).get(index.clone());
                if (item).is_some() /* instanceof GameConfigurationGauge */ {
                    let mut gauge: Option<Rc<RefCell<GameConfigurationGauge>>> = (item).clone() /* as GameConfigurationGauge */;
                    let mut name: String = jcall!(item).getLabel();
                    let mut nextGameConfiguration: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::GameConfiguration::GameConfigurationDyn>>> = jcall!(gameConfigurationSingleton).getInstanceByName(name.clone());
                    if match (&nextGameConfiguration, &jcall!(gameConfigurationCentral).COLLIDE_DAMAGE) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                        jcall!(gauge).setValue(((jcall!(jcall!(gameConfigurationCentral).COLLIDE_DAMAGE).getValue()).unwrap() as i32) - ((jcall!(gameConfiguration).getMinValue()).unwrap() as i32));
                    } else if match (&nextGameConfiguration, &jcall!(gameConfigurationCentral).DURABILITY_CHALLENGE_LEVEL) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                        jcall!(gauge).setValue(((jcall!(jcall!(gameConfigurationCentral).DURABILITY_CHALLENGE_LEVEL).getValue()).unwrap() as i32) - ((jcall!(gameConfiguration).getMinValue()).unwrap() as i32));
                    } else if match (&nextGameConfiguration, &jcall!(gameConfigurationCentral).ATTACK_CHALLENGE_LEVEL) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                        jcall!(gauge).setValue(((jcall!(jcall!(gameConfigurationCentral).ATTACK_CHALLENGE_LEVEL).getValue()).unwrap() as i32) - ((jcall!(gameConfiguration).getMinValue()).unwrap() as i32));
                    } else if match (&nextGameConfiguration, &jcall!(gameConfigurationCentral).SPEED_CHALLENGE_LEVEL) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                        jcall!(gauge).setValue(((jcall!(jcall!(gameConfigurationCentral).SPEED_CHALLENGE_LEVEL).getValue()).unwrap() as i32) - ((jcall!(gameConfiguration).getMinValue()).unwrap() as i32));
                    }
                }
            }
        }
    }
    

    pub fn updateCompetitionValue(&mut self) {
        jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("updateCompetitionValue"));
        let mut gameConfigurationCentral: Option<Rc<RefCell<GameConfigurationCentral>>> = GameConfigurationCentral::getInstance();
        self.COMPETITION_VALUE = ((jcall!(jcall!(gameConfigurationCentral).COLLIDE_DAMAGE).getValue()).unwrap() as i32) + ((jcall!(jcall!(gameConfigurationCentral).ATTACK_CHALLENGE_LEVEL).getValue()).unwrap() as i32) + ((jcall!(jcall!(gameConfigurationCentral).DURABILITY_CHALLENGE_LEVEL).getValue()).unwrap() as i32) + ((jcall!(jcall!(gameConfigurationCentral).SPEED_CHALLENGE_LEVEL).getValue()).unwrap() as i32);
        self.COMPETITION_VALUE = (self.COMPETITION_VALUE >> 2);
    }
    

    pub fn getCompetitionValue(&mut self) -> i32 {
        return self.COMPETITION_VALUE.clone();
    }
}

