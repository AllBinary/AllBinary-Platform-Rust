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
use crate::org::allbinary::game::configuration::feature::Feature::Feature;
use crate::org::allbinary::game::configuration::feature::Features::Features;
use crate::org::allbinary::game::configuration::feature::GameFeatureUtil::GameFeatureUtil;
use crate::org::allbinary::logic::communication::log::ForcedLogUtil::ForcedLogUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use super::GameFeatureListenerInterface::GameFeatureListenerInterface;
use super::GameFeatureEvent::GameFeatureEvent;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BaseChangedGameFeatureListener {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub gameFeatureUtil: Option<Rc<RefCell<GameFeatureUtil>>>,
    pub list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub changed: bool,
}

impl GameFeatureListenerInterface for BaseChangedGameFeatureListener {
    fn onGameFeatureChange(&mut self, mut gameFeatureEvent: Option<Rc<RefCell<GameFeatureEvent>>>) { Self::onGameFeatureChange(self, gameFeatureEvent) }
}
impl crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface for BaseChangedGameFeatureListener {
    fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { Self::onEvent(self, eventObject) }
}

impl BaseChangedGameFeatureListener {

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            gameFeatureUtil: None,
            list: None,
            changed: false,
        };
        this.logUtil = LogUtil::getInstance();
        this.gameFeatureUtil = GameFeatureUtil::getInstance();
        this.list = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.changed = true;
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) {
        ForcedLogUtil::log(jcall!(CommonStrings::getInstance()).NOT_IMPLEMENTED.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())));
    }
    

    pub fn onGameFeatureChange(&mut self, mut gameFeatureEvent: Option<Rc<RefCell<GameFeatureEvent>>>) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(self.gameFeatureUtil).GAME_FEATURE_CHANGED.clone()).append(jcall!(gameFeatureEvent).getWhatChanged()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.gameFeatureUtil).ON_GAME_FEATURE_CHANGE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        { let java_receiver = self.list.clone(); let java_arguments = (crate::java_into_object(jcall!(gameFeatureEvent).getGameOption()),); jcall!(java_receiver).add(java_arguments.0); };
        self.setChanged(true);
    }
    

    pub fn add(&mut self, mut gameFeature: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>>) {
        jcall!(self.list).add(crate::java_into_object(gameFeature.clone()));
    }
    

    pub fn remove(&mut self, mut gameFeature: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>>) {
        jcall!(self.list).remove(crate::java_into_object(gameFeature.clone()));
    }
    

    pub fn setChanged(&mut self, mut initialized: bool) {
        self.changed = initialized.clone();
        if !self.isChanged() {
            jcall!(self.list).clear();
        }
    }
    

    pub fn isChangedFeature(&mut self, mut gameFeature: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>>) -> bool {
        let mut isChanged: bool = jcall!(self.list).contains(crate::java_into_object(gameFeature.clone()));
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(String::from("GameFeature: "));
        jcall!(stringBuffer).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(gameFeature.clone())));
        jcall!(stringBuffer).append(String::from(" isFeature: "));
        jcall!(stringBuffer).appendboolean(jcall!(Features::getInstance()).isFeature(gameFeature.clone()));
        jcall!(stringBuffer).append(String::from(" isChanged: "));
        jcall!(stringBuffer).appendboolean(isChanged.clone());
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("isChanged"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        return isChanged.clone();
    }
    

    pub fn isChanged(&mut self) -> bool {
        return self.changed.clone();
    }
}
pub trait BaseChangedGameFeatureListenerDyn {
    fn as_BaseChangedGameFeatureListener(&self) -> &BaseChangedGameFeatureListener;
    fn as_BaseChangedGameFeatureListener_mut(&mut self) -> &mut BaseChangedGameFeatureListener;
    fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { BaseChangedGameFeatureListener::onEvent(self.as_BaseChangedGameFeatureListener_mut(), eventObject) }
    fn onGameFeatureChange(&mut self, mut gameFeatureEvent: Option<Rc<RefCell<GameFeatureEvent>>>) { BaseChangedGameFeatureListener::onGameFeatureChange(self.as_BaseChangedGameFeatureListener_mut(), gameFeatureEvent) }
    fn add(&mut self, mut gameFeature: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>>) { BaseChangedGameFeatureListener::add(self.as_BaseChangedGameFeatureListener_mut(), gameFeature) }
    fn remove(&mut self, mut gameFeature: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>>) { BaseChangedGameFeatureListener::remove(self.as_BaseChangedGameFeatureListener_mut(), gameFeature) }
    fn setChanged(&mut self, mut initialized: bool) { BaseChangedGameFeatureListener::setChanged(self.as_BaseChangedGameFeatureListener_mut(), initialized) }
    fn isChangedFeature(&mut self, mut gameFeature: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>>) -> bool { BaseChangedGameFeatureListener::isChangedFeature(self.as_BaseChangedGameFeatureListener_mut(), gameFeature) }
    fn isChanged(&mut self) -> bool { BaseChangedGameFeatureListener::isChanged(self.as_BaseChangedGameFeatureListener_mut()) }
}
impl BaseChangedGameFeatureListenerDyn for BaseChangedGameFeatureListener {
    fn as_BaseChangedGameFeatureListener(&self) -> &BaseChangedGameFeatureListener { self }
    fn as_BaseChangedGameFeatureListener_mut(&mut self) -> &mut BaseChangedGameFeatureListener { self }
}


