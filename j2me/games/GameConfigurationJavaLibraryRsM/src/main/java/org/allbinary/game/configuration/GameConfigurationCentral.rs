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
use crate::org::allbinary::game::configuration::persistance::GameConfigurationPersistanceSingleton::GameConfigurationPersistanceSingleton;
use crate::org::allbinary::game::configuration::persistance::KeyValuePersistance::KeyValuePersistance;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::math::SmallIntegerSingletonFactory::SmallIntegerSingletonFactory;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use super::GameConfiguration::GameConfiguration;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GameConfigurationCentral {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub SCALE: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::GameConfiguration::GameConfigurationDyn>>>,
    pub ORIENTATION: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::GameConfiguration::GameConfigurationDyn>>>,
    pub SENSOR_UPDATE_RATE: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::GameConfiguration::GameConfigurationDyn>>>,
    pub VIBRATION: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::GameConfiguration::GameConfigurationDyn>>>,
    pub CHALLENGE_LEVEL: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::GameConfiguration::GameConfigurationDyn>>>,
    pub COLLIDE_DAMAGE: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::GameConfiguration::GameConfigurationDyn>>>,
    pub DURABILITY_CHALLENGE_LEVEL: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::GameConfiguration::GameConfigurationDyn>>>,
    pub SPEED_CHALLENGE_LEVEL: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::GameConfiguration::GameConfigurationDyn>>>,
    pub ATTACK_CHALLENGE_LEVEL: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::GameConfiguration::GameConfigurationDyn>>>,
    pub CONTROL_LEVEL: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::GameConfiguration::GameConfigurationDyn>>>,
    pub PLAYER_INPUT_WAIT: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::GameConfiguration::GameConfigurationDyn>>>,
    pub SPEED: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::GameConfiguration::GameConfigurationDyn>>>,
    pub SOUND_VOLUME: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::GameConfiguration::GameConfigurationDyn>>>,
    pub MAX_GAME_OBJECTS: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::GameConfiguration::GameConfigurationDyn>>>,
    pub MAX_LAYERS: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::GameConfiguration::GameConfigurationDyn>>>,
    pub gameControlFidelity: i32,
}

pub static mut GameConfigurationCentral_SINGLETON: std::cell::LazyCell<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = std::cell::LazyCell::new(|| jcall!(NullUtil::getInstance()).NULL_OBJECT.clone());

impl GameConfigurationCentral {
    

    pub fn getInstance() -> Option<Rc<RefCell<GameConfigurationCentral>>> {
        if match (&unsafe { GameConfigurationCentral_SINGLETON.clone() }, &jcall!(NullUtil::getInstance()).NULL_OBJECT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            unsafe { *GameConfigurationCentral_SINGLETON = GameConfigurationCentral::new() };
        }
        return (unsafe { GameConfigurationCentral_SINGLETON.clone() }).clone() /* as GameConfigurationCentral */;
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            SCALE: None,
            ORIENTATION: None,
            SENSOR_UPDATE_RATE: None,
            VIBRATION: None,
            CHALLENGE_LEVEL: None,
            COLLIDE_DAMAGE: None,
            DURABILITY_CHALLENGE_LEVEL: None,
            SPEED_CHALLENGE_LEVEL: None,
            ATTACK_CHALLENGE_LEVEL: None,
            CONTROL_LEVEL: None,
            PLAYER_INPUT_WAIT: None,
            SPEED: None,
            SOUND_VOLUME: None,
            MAX_GAME_OBJECTS: None,
            MAX_LAYERS: None,
            gameControlFidelity: 0,
        };
        this.logUtil = LogUtil::getInstance();
        this.gameControlFidelity = 36;
        let mut smallIntegerSingletonFactory: Option<Rc<RefCell<SmallIntegerSingletonFactory>>> = SmallIntegerSingletonFactory::getInstance();
        this.SCALE = GameConfiguration::new(String::from("Scale"), jcall!(smallIntegerSingletonFactory).getAt(2), jcall!(smallIntegerSingletonFactory).getAt(2), jcall!(smallIntegerSingletonFactory).getAt(3)).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::GameConfiguration::GameConfigurationDyn>>);
        this.ORIENTATION = GameConfiguration::new(String::from("Orientation"), jcall!(smallIntegerSingletonFactory).getAt(0), jcall!(smallIntegerSingletonFactory).getAt(0), jcall!(smallIntegerSingletonFactory).getAt(8)).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::GameConfiguration::GameConfigurationDyn>>);
        this.SENSOR_UPDATE_RATE = GameConfiguration::new(String::from("Sensor Update Rate"), jcall!(smallIntegerSingletonFactory).getAt(0), jcall!(smallIntegerSingletonFactory).getAt(0), jcall!(smallIntegerSingletonFactory).getAt(3)).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::GameConfiguration::GameConfigurationDyn>>);
        this.VIBRATION = GameConfiguration::new(String::from("Vibration"), jcall!(smallIntegerSingletonFactory).getAt(0), jcall!(smallIntegerSingletonFactory).getAt(0), jcall!(smallIntegerSingletonFactory).getAt(3)).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::GameConfiguration::GameConfigurationDyn>>);
        this.CHALLENGE_LEVEL = GameConfiguration::new(String::from("Global Challenge Level"), jcall!(smallIntegerSingletonFactory).getAt(3), jcall!(smallIntegerSingletonFactory).getAt(1), jcall!(smallIntegerSingletonFactory).getAt(10)).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::GameConfiguration::GameConfigurationDyn>>);
        this.COLLIDE_DAMAGE = GameConfiguration::new(String::from("Collide Damage Challenge Level"), jcall!(smallIntegerSingletonFactory).getAt(3), jcall!(smallIntegerSingletonFactory).getAt(1), jcall!(smallIntegerSingletonFactory).getAt(10)).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::GameConfiguration::GameConfigurationDyn>>);
        this.DURABILITY_CHALLENGE_LEVEL = GameConfiguration::new(String::from("Enemy Durability Challenge Level"), jcall!(smallIntegerSingletonFactory).getAt(3), jcall!(smallIntegerSingletonFactory).getAt(1), jcall!(smallIntegerSingletonFactory).getAt(10)).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::GameConfiguration::GameConfigurationDyn>>);
        this.SPEED_CHALLENGE_LEVEL = GameConfiguration::new(String::from("Enemy Speed Challenge Level"), jcall!(smallIntegerSingletonFactory).getAt(3), jcall!(smallIntegerSingletonFactory).getAt(1), jcall!(smallIntegerSingletonFactory).getAt(10)).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::GameConfiguration::GameConfigurationDyn>>);
        this.ATTACK_CHALLENGE_LEVEL = GameConfiguration::new(String::from("Enemy Attack Challenge Level"), jcall!(smallIntegerSingletonFactory).getAt(3), jcall!(smallIntegerSingletonFactory).getAt(1), jcall!(smallIntegerSingletonFactory).getAt(10)).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::GameConfiguration::GameConfigurationDyn>>);
        this.CONTROL_LEVEL = GameConfiguration::new(String::from("Control Fidelity"), jcall!(smallIntegerSingletonFactory).getAt(5), jcall!(smallIntegerSingletonFactory).getAt(1), jcall!(smallIntegerSingletonFactory).getAt(10)).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::GameConfiguration::GameConfigurationDyn>>);
        this.PLAYER_INPUT_WAIT = GameConfiguration::new(String::from("Input Wait"), jcall!(smallIntegerSingletonFactory).getAt(5), jcall!(smallIntegerSingletonFactory).getAt(1), jcall!(smallIntegerSingletonFactory).getAt(10)).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::GameConfiguration::GameConfigurationDyn>>);
        this.SPEED = GameConfiguration::new(String::from("Game Speed (Device Specific)"), jcall!(smallIntegerSingletonFactory).getAt(5), jcall!(smallIntegerSingletonFactory).getAt(1), jcall!(smallIntegerSingletonFactory).getAt(10)).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::GameConfiguration::GameConfigurationDyn>>);
        this.SOUND_VOLUME = GameConfiguration::new(String::from("Sound Volume"), jcall!(smallIntegerSingletonFactory).getAt(6), jcall!(smallIntegerSingletonFactory).getAt(0), jcall!(smallIntegerSingletonFactory).getAt(10)).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::GameConfiguration::GameConfigurationDyn>>);
        this.MAX_GAME_OBJECTS = GameConfiguration::new(String::from("Max Game Objects"), jcall!(smallIntegerSingletonFactory).getAt(20), jcall!(smallIntegerSingletonFactory).getAt(1), jcall!(smallIntegerSingletonFactory).getAt(50)).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::GameConfiguration::GameConfigurationDyn>>);
        this.MAX_LAYERS = GameConfiguration::new(String::from("Max Layers"), jcall!(smallIntegerSingletonFactory).getAt(20), jcall!(smallIntegerSingletonFactory).getAt(1), jcall!(smallIntegerSingletonFactory).getAt(50)).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::GameConfiguration::GameConfigurationDyn>>);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn load(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>) {
        let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut smallIntegerSingletonFactory: Option<Rc<RefCell<SmallIntegerSingletonFactory>>> = SmallIntegerSingletonFactory::getInstance();
            let mut keyValuePersistance: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::persistance::KeyValuePersistance::KeyValuePersistanceDyn>>> = GameConfigurationPersistanceSingleton::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::persistance::KeyValuePersistance::KeyValuePersistanceDyn>>);
            jcall!(keyValuePersistance).clear();
            jcall!(keyValuePersistance).loadAll(abeClientInformation.clone());
            let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(keyValuePersistance).getIds();
            if jcall!(list).size() > 0 {
                let mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> = jcall!(keyValuePersistance).get(0);
                let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = (jcall!(self.SCALE).getName()).clone() /* as Any */;
                let mut value: String = (jcall!(hashtable).get(anyType.clone())).clone() /* as String */;
                { let java_receiver = self.SCALE.clone(); let java_arguments = (jcall!(smallIntegerSingletonFactory).getAt((((value.clone()).to_string()).unwrap() as i32)),); jcall!(java_receiver).setValue(java_arguments.0); };
            } else {
                jcall!(self.logUtil).putF(String::from("No Game Configuration To Load"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).LOAD.clone());
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).LOAD.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn setGameControlFidelity(&mut self, mut gameControlFidelity: i32) {
        self.gameControlFidelity = gameControlFidelity.clone();
    }
    

    pub fn getGameControlFidelity(&mut self) -> i32 {
        return self.gameControlFidelity.clone();
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut commonStrings: Option<Rc<RefCell<CommonSeps>>> = CommonSeps::getInstance();
        jcall!(stringBuffer).append(jcall!(commonStrings).COMMA_SEP.clone());
        jcall!(stringBuffer).append(jcall!(self.SCALE).getName());
        jcall!(stringBuffer).append(jcall!(commonStrings).EQUALS.clone());
        jcall!(stringBuffer).appendint(((jcall!(self.SCALE).getValue()).unwrap() as i32));
        jcall!(stringBuffer).append(jcall!(commonStrings).COMMA_SEP.clone());
        jcall!(stringBuffer).append(jcall!(self.ORIENTATION).getName());
        jcall!(stringBuffer).append(jcall!(commonStrings).EQUALS.clone());
        jcall!(stringBuffer).appendint(((jcall!(self.ORIENTATION).getValue()).unwrap() as i32));
        jcall!(stringBuffer).append(jcall!(commonStrings).COMMA_SEP.clone());
        jcall!(stringBuffer).append(jcall!(self.SENSOR_UPDATE_RATE).getName());
        jcall!(stringBuffer).append(jcall!(commonStrings).EQUALS.clone());
        jcall!(stringBuffer).appendint(((jcall!(self.SENSOR_UPDATE_RATE).getValue()).unwrap() as i32));
        jcall!(stringBuffer).append(jcall!(commonStrings).COMMA_SEP.clone());
        jcall!(stringBuffer).append(jcall!(self.VIBRATION).getName());
        jcall!(stringBuffer).append(jcall!(commonStrings).EQUALS.clone());
        jcall!(stringBuffer).appendint(((jcall!(self.VIBRATION).getValue()).unwrap() as i32));
        jcall!(stringBuffer).append(jcall!(commonStrings).COMMA_SEP.clone());
        jcall!(stringBuffer).append(jcall!(self.CHALLENGE_LEVEL).getName());
        jcall!(stringBuffer).append(jcall!(commonStrings).EQUALS.clone());
        jcall!(stringBuffer).appendint(((jcall!(self.CHALLENGE_LEVEL).getValue()).unwrap() as i32));
        jcall!(stringBuffer).append(jcall!(commonStrings).COMMA_SEP.clone());
        jcall!(stringBuffer).append(jcall!(self.COLLIDE_DAMAGE).getName());
        jcall!(stringBuffer).append(jcall!(commonStrings).EQUALS.clone());
        jcall!(stringBuffer).appendint(((jcall!(self.COLLIDE_DAMAGE).getValue()).unwrap() as i32));
        jcall!(stringBuffer).append(jcall!(commonStrings).COMMA_SEP.clone());
        jcall!(stringBuffer).append(jcall!(self.DURABILITY_CHALLENGE_LEVEL).getName());
        jcall!(stringBuffer).append(jcall!(commonStrings).EQUALS.clone());
        jcall!(stringBuffer).appendint(((jcall!(self.DURABILITY_CHALLENGE_LEVEL).getValue()).unwrap() as i32));
        jcall!(stringBuffer).append(jcall!(commonStrings).COMMA_SEP.clone());
        jcall!(stringBuffer).append(jcall!(self.SPEED_CHALLENGE_LEVEL).getName());
        jcall!(stringBuffer).append(jcall!(commonStrings).EQUALS.clone());
        jcall!(stringBuffer).appendint(((jcall!(self.SPEED_CHALLENGE_LEVEL).getValue()).unwrap() as i32));
        jcall!(stringBuffer).append(jcall!(commonStrings).COMMA_SEP.clone());
        jcall!(stringBuffer).append(jcall!(self.ATTACK_CHALLENGE_LEVEL).getName());
        jcall!(stringBuffer).append(jcall!(commonStrings).EQUALS.clone());
        jcall!(stringBuffer).appendint(((jcall!(self.ATTACK_CHALLENGE_LEVEL).getValue()).unwrap() as i32));
        jcall!(stringBuffer).append(jcall!(commonStrings).COMMA_SEP.clone());
        jcall!(stringBuffer).append(jcall!(self.CONTROL_LEVEL).getName());
        jcall!(stringBuffer).append(jcall!(commonStrings).EQUALS.clone());
        jcall!(stringBuffer).appendint(((jcall!(self.CONTROL_LEVEL).getValue()).unwrap() as i32));
        jcall!(stringBuffer).append(jcall!(commonStrings).COMMA_SEP.clone());
        jcall!(stringBuffer).append(jcall!(self.SPEED).getName());
        jcall!(stringBuffer).append(jcall!(commonStrings).EQUALS.clone());
        jcall!(stringBuffer).appendint(((jcall!(self.SPEED).getValue()).unwrap() as i32));
        jcall!(stringBuffer).append(jcall!(commonStrings).COMMA_SEP.clone());
        jcall!(stringBuffer).append(jcall!(self.MAX_GAME_OBJECTS).getName());
        jcall!(stringBuffer).append(jcall!(commonStrings).EQUALS.clone());
        jcall!(stringBuffer).appendint(((jcall!(self.MAX_GAME_OBJECTS).getValue()).unwrap() as i32));
        jcall!(stringBuffer).append(jcall!(commonStrings).COMMA_SEP.clone());
        jcall!(stringBuffer).append(jcall!(self.MAX_LAYERS).getName());
        jcall!(stringBuffer).append(jcall!(commonStrings).EQUALS.clone());
        jcall!(stringBuffer).appendint(((jcall!(self.MAX_LAYERS).getValue()).unwrap() as i32));
        return jcall!(stringBuffer).toString();
    }
}

