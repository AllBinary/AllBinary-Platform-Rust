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
// package org.allbinary.game.configuration.event
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::configuration::GameConfiguration::GameConfiguration;
use super::BaseChangedGameFeatureListener::BaseChangedGameFeatureListener;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ChangedGameFeatureListener {
    pub base: BaseChangedGameFeatureListener,
}

thread_local! { pub static ChangedGameFeatureListener_gameFeatureListener: Option<Rc<RefCell<ChangedGameFeatureListener>>> = ChangedGameFeatureListener::new(); }

impl ChangedGameFeatureListener {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: BaseChangedGameFeatureListener::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<ChangedGameFeatureListener>>> {
        return ChangedGameFeatureListener_gameFeatureListener.with(Clone::clone).clone();
    }
    

    pub fn isChangedGameConfiguration(&mut self, mut gameConfiguration: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::GameConfiguration::GameConfigurationDyn>>>) -> bool {
        return jcall!(self.list).contains(crate::java_into_object(gameConfiguration.clone()));
    }
}
impl std::ops::Deref for ChangedGameFeatureListener {
    type Target = BaseChangedGameFeatureListener;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ChangedGameFeatureListener {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::configuration::event::BaseChangedGameFeatureListener::BaseChangedGameFeatureListenerDyn for ChangedGameFeatureListener {
    fn as_BaseChangedGameFeatureListener(&self) -> &crate::org::allbinary::game::configuration::event::BaseChangedGameFeatureListener::BaseChangedGameFeatureListener { &self.base }
    fn as_BaseChangedGameFeatureListener_mut(&mut self) -> &mut crate::org::allbinary::game::configuration::event::BaseChangedGameFeatureListener::BaseChangedGameFeatureListener { &mut self.base }
}


