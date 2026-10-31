// 
// AllBinary Open License Version 1 
// Copyright (c) 2025 AllBinary 
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
// package org.allbinary.business.advertisement
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::GameType::GameType;
use crate::org::allbinary::game::state::GameState::GameState;
use crate::org::allbinary::logic::communication::log::ForcedLogUtil::ForcedLogUtil;
use crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject;
use crate::org::allbinary::logic::util::event::EventStrings::EventStrings;
use super::AdConfiguration::AdConfiguration;
use super::Advertisements::Advertisements;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GameAdStateBase {
    pub adConfigurationP: Option<Rc<RefCell<dyn crate::org::allbinary::business::advertisement::AdConfiguration::AdConfigurationDyn>>>,
    pub advertisementsP: Option<Rc<RefCell<Advertisements>>>,
    pub okayToShowAdsP: bool,
}


impl GameAdStateBase {
    

    pub fn new_struct(mut adConfiguration: Option<Rc<RefCell<dyn crate::org::allbinary::business::advertisement::AdConfiguration::AdConfigurationDyn>>>) -> Self {
        let mut this = Self {
            adConfigurationP: None,
            advertisementsP: None,
            okayToShowAdsP: false,
        };
        this.advertisementsP = Advertisements::new();
        this.adConfigurationP = adConfiguration.clone();
        this
    }

    pub fn new(mut adConfiguration: Option<Rc<RefCell<dyn crate::org::allbinary::business::advertisement::AdConfiguration::AdConfigurationDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(adConfiguration))))
    }
    

    pub fn initViewArray(&mut self, mut viewArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>>) {
    }
    

    pub fn onStart(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) {
        self.init();
    }
    

    pub fn init(&mut self) {
    }
    

    pub fn isShowingAtLocation(&mut self, mut location: i32) -> bool {
        return false;
    }
    

    pub fn isShowingAt(&mut self) -> bool {
        return false;
    }
    

    pub fn show(&mut self) {
    }
    

    pub fn hide(&mut self) {
    }
    

    pub fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) {
        ForcedLogUtil::log(jcall!(EventStrings::getInstance()).PERFORMANCE_MESSAGE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())));
    }
    

    pub fn processAdState(&mut self, mut gameState: Option<Rc<RefCell<GameState>>>, mut gameType: Option<Rc<RefCell<GameType>>>) {
    }
    

    pub fn processPageAdState(&mut self) {
    }
    

    // throws Exception
    pub fn playingAdState(&mut self) {
    }
    

    // throws Exception
    pub fn playingAdStateWH(&mut self, mut width: i32, mut height: i32) {
    }
    

    // throws Exception
    pub fn notPlayingAdState(&mut self) {
    }
    

    pub fn setGameIsReady(&mut self, mut gameIsReady: bool) {
    }
    

    pub fn isGameIsReady(&mut self) -> bool {
        return false;
    }
    

    pub fn getAdvertisements(&mut self) -> Option<Rc<RefCell<Advertisements>>> {
        return self.advertisementsP.clone();
    }
    

    pub fn getAdConfiguration(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::business::advertisement::AdConfiguration::AdConfigurationDyn>>> {
        return self.adConfigurationP.clone();
    }
    

    pub fn isOkayToShowPageAd(&mut self) -> bool {
        return false;
    }
    

    pub fn setOkayToShowAds(&mut self, mut okayToShowAds: bool) {
        self.okayToShowAdsP = okayToShowAds.clone();
    }
    

    pub fn isOkayToShowAds(&mut self) -> bool {
        return self.okayToShowAdsP.clone();
    }
    

    pub fn isEnabled(&mut self) -> bool {
        return false;
    }
}
pub trait GameAdStateBaseDyn {
    fn as_GameAdStateBase(&self) -> &GameAdStateBase;
    fn as_GameAdStateBase_mut(&mut self) -> &mut GameAdStateBase;
    fn initViewArray(&mut self, mut viewArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>>) { GameAdStateBase::initViewArray(self.as_GameAdStateBase_mut(), viewArray) }
    fn onStart(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) { GameAdStateBase::onStart(self.as_GameAdStateBase_mut(), anyType) }
    fn init(&mut self) { GameAdStateBase::init(self.as_GameAdStateBase_mut()) }
    fn isShowingAtLocation(&mut self, mut location: i32) -> bool { GameAdStateBase::isShowingAtLocation(self.as_GameAdStateBase_mut(), location) }
    fn isShowingAt(&mut self) -> bool { GameAdStateBase::isShowingAt(self.as_GameAdStateBase_mut()) }
    fn show(&mut self) { GameAdStateBase::show(self.as_GameAdStateBase_mut()) }
    fn hide(&mut self) { GameAdStateBase::hide(self.as_GameAdStateBase_mut()) }
    fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { GameAdStateBase::onEvent(self.as_GameAdStateBase_mut(), eventObject) }
    fn processAdState(&mut self, mut gameState: Option<Rc<RefCell<GameState>>>, mut gameType: Option<Rc<RefCell<GameType>>>) { GameAdStateBase::processAdState(self.as_GameAdStateBase_mut(), gameState, gameType) }
    fn processPageAdState(&mut self) { GameAdStateBase::processPageAdState(self.as_GameAdStateBase_mut()) }
    fn playingAdState(&mut self) { GameAdStateBase::playingAdState(self.as_GameAdStateBase_mut()) }
    fn playingAdStateWH(&mut self, mut width: i32, mut height: i32) { GameAdStateBase::playingAdStateWH(self.as_GameAdStateBase_mut(), width, height) }
    fn notPlayingAdState(&mut self) { GameAdStateBase::notPlayingAdState(self.as_GameAdStateBase_mut()) }
    fn setGameIsReady(&mut self, mut gameIsReady: bool) { GameAdStateBase::setGameIsReady(self.as_GameAdStateBase_mut(), gameIsReady) }
    fn isGameIsReady(&mut self) -> bool { GameAdStateBase::isGameIsReady(self.as_GameAdStateBase_mut()) }
    fn getAdvertisements(&mut self) -> Option<Rc<RefCell<Advertisements>>> { GameAdStateBase::getAdvertisements(self.as_GameAdStateBase_mut()) }
    fn getAdConfiguration(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::business::advertisement::AdConfiguration::AdConfigurationDyn>>> { GameAdStateBase::getAdConfiguration(self.as_GameAdStateBase_mut()) }
    fn isOkayToShowPageAd(&mut self) -> bool { GameAdStateBase::isOkayToShowPageAd(self.as_GameAdStateBase_mut()) }
    fn setOkayToShowAds(&mut self, mut okayToShowAds: bool) { GameAdStateBase::setOkayToShowAds(self.as_GameAdStateBase_mut(), okayToShowAds) }
    fn isOkayToShowAds(&mut self) -> bool { GameAdStateBase::isOkayToShowAds(self.as_GameAdStateBase_mut()) }
    fn isEnabled(&mut self) -> bool { GameAdStateBase::isEnabled(self.as_GameAdStateBase_mut()) }
}
impl GameAdStateBaseDyn for GameAdStateBase {
    fn as_GameAdStateBase(&self) -> &GameAdStateBase { self }
    fn as_GameAdStateBase_mut(&mut self) -> &mut GameAdStateBase { self }
}


