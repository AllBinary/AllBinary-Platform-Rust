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
// package org.allbinary.game.configuration.persistance
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::KeyValuePersistance::KeyValuePersistance;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GameConfigurationPersistanceSingleton {
    pub base: KeyValuePersistance,
}

pub static mut GameConfigurationPersistanceSingleton_GAME_PERSISTANCE: std::cell::LazyCell<Option<Rc<RefCell<GameConfigurationPersistanceSingleton>>>> = std::cell::LazyCell::new(|| GameConfigurationPersistanceSingleton::new());

impl GameConfigurationPersistanceSingleton {
    

    pub fn getInstance() -> Option<Rc<RefCell<GameConfigurationPersistanceSingleton>>> {
        return unsafe { GameConfigurationPersistanceSingleton_GAME_PERSISTANCE.clone() }.clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: KeyValuePersistance::new_struct(jcall!(GamePersistanceStrings::GamePersistanceStrings::getInstance()).SAVED_GAME_CONFIGURATION_RECORD_ID.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}
impl std::ops::Deref for GameConfigurationPersistanceSingleton {
    type Target = KeyValuePersistance;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for GameConfigurationPersistanceSingleton {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::configuration::persistance::BasicPersitance::BasicPersitanceDyn for GameConfigurationPersistanceSingleton {
    fn as_BasicPersitance(&self) -> &crate::org::allbinary::game::configuration::persistance::BasicPersitance::BasicPersitance { &self.base.base }
    fn as_BasicPersitance_mut(&mut self) -> &mut crate::org::allbinary::game::configuration::persistance::BasicPersitance::BasicPersitance { &mut self.base.base }
}
impl crate::org::allbinary::game::configuration::persistance::KeyValuePersistance::KeyValuePersistanceDyn for GameConfigurationPersistanceSingleton {
    fn as_KeyValuePersistance(&self) -> &crate::org::allbinary::game::configuration::persistance::KeyValuePersistance::KeyValuePersistance { &self.base }
    fn as_KeyValuePersistance_mut(&mut self) -> &mut crate::org::allbinary::game::configuration::persistance::KeyValuePersistance::KeyValuePersistance { &mut self.base }
}


