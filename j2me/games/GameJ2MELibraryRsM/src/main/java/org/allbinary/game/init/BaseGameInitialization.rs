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
// package org.allbinary.game.init
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::CommandListener::CommandListener;
use crate::org::allbinary::animation::FeaturedAnimationInterfaceFactoryInterfaceFactory::FeaturedAnimationInterfaceFactoryInterfaceFactory;
use crate::org::allbinary::game::configuration::GameConfigurationCentral::GameConfigurationCentral;
use crate::org::allbinary::game::configuration::event::ChangedGameFeatureListener::ChangedGameFeatureListener;
use crate::org::allbinary::game::configuration::feature::InputFeatureFactory::InputFeatureFactory;
use crate::org::allbinary::game::configuration::feature::MainFeatureFactory::MainFeatureFactory;
use crate::org::allbinary::game::input::PlatformInputMappingFactory::PlatformInputMappingFactory;
use crate::org::allbinary::game::input::event::GameKeyEventFactory::GameKeyEventFactory;
use crate::org::allbinary::game::resource::FeaturedResourceRelativeRelationshipFactory::FeaturedResourceRelativeRelationshipFactory;
use crate::org::allbinary::game::resource::ResourceInitialization::ResourceInitialization;
use crate::org::allbinary::game::resource::ResourceLoadingLevelFactory::ResourceLoadingLevelFactory;
use crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvas;
use crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvasFactory::ProgressCanvasFactory;
use crate::org::allbinary::image::GameFeatureImageCacheFactory::GameFeatureImageCacheFactory;
use crate::org::allbinary::input::motion::CompleteMotionGestureInputToGameMotionGestureInput::CompleteMotionGestureInputToGameMotionGestureInput;
use crate::org::allbinary::input::motion::button::BasicTouchInputFactory::BasicTouchInputFactory;
use crate::org::allbinary::input::motion::button::CancelTouchButtonInputFactory::CancelTouchButtonInputFactory;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use super::GameInitializationInterface::GameInitializationInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BaseGameInitialization {
    pub resourceInitializationArray: Vec<Option<Rc<RefCell<ResourceInitialization>>>>,
    pub portion: i32,
    pub initialized: bool,
    pub EARLY_RESOURCES: i32,
    pub GAME_RESOURCES: i32,
    pub EARLY_CHANGABLE_RESOURCES: i32,
    pub GAME_CHANGABLE_RESOURCES: i32,
    pub gameInitialized: bool,
    pub allLoaded: bool,
}

thread_local! { pub static BaseGameInitialization_NULL_BASE_GAME_INITIALIZATION: Option<Rc<RefCell<dyn BaseGameInitializationDyn>>> = BaseGameInitialization::new(vec![None; (0) as usize], 0).map(|reference| reference as Rc<RefCell<dyn BaseGameInitializationDyn>>); }
impl GameInitializationInterface for BaseGameInitialization {
    fn init(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut commandListener: Option<Rc<RefCell<CommandListener>>>, mut level: i32) { Self::init(self, abeClientInformation, commandListener, level) }
}

impl BaseGameInitialization {
    

    pub fn new_struct(mut resourceInitializationArray: Vec<Option<Rc<RefCell<ResourceInitialization>>>>, mut portion: i32) -> Self {
        let mut this = Self {
            resourceInitializationArray: Vec::new(),
            portion: 0,
            initialized: false,
            EARLY_RESOURCES: 0,
            GAME_RESOURCES: 0,
            EARLY_CHANGABLE_RESOURCES: 0,
            GAME_CHANGABLE_RESOURCES: 0,
            gameInitialized: false,
            allLoaded: false,
        };
        this.EARLY_RESOURCES = 0;
        this.GAME_RESOURCES = 1;
        this.EARLY_CHANGABLE_RESOURCES = 2;
        this.GAME_CHANGABLE_RESOURCES = 3;
        this.resourceInitializationArray = resourceInitializationArray.clone();
        this.portion = portion.clone();
        this
    }

    pub fn new(mut resourceInitializationArray: Vec<Option<Rc<RefCell<ResourceInitialization>>>>, mut portion: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(resourceInitializationArray, portion))))
    }
    

    // throws Exception
    pub fn initKey(&mut self, mut portion: i32) {
    }
    

    // throws Exception
    pub fn initKeyMapping(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut portion: i32) {
        if jcall!(ChangedGameFeatureListener::getInstance()).isChangedFeature(jcall!(InputFeatureFactory::getInstance()).INPUT_MAPPING.clone()) {
            jcall!(jcall!(PlatformInputMappingFactory::getInstance()).getPersistentInputMappingInstance()).init(abeClientInformation.clone());
            jcall!(ProgressCanvasFactory::getInstance()).addNormalPortion(50, String::from("Game Keys"));
            jcall!(ChangedGameFeatureListener::getInstance()).remove(jcall!(InputFeatureFactory::getInstance()).INPUT_MAPPING.clone());
        }
    }
    

    // throws Exception
    pub fn init(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut commandListener: Option<Rc<RefCell<CommandListener>>>, mut level: i32) {
        let mut resourceLoadingLevelFactory: Option<Rc<RefCell<ResourceLoadingLevelFactory>>> = ResourceLoadingLevelFactory::getInstance();
        if !self.isInitialized() && (level == jcall!(jcall!(resourceLoadingLevelFactory).LOAD_ALL).getLevel() || level == jcall!(jcall!(resourceLoadingLevelFactory).LOAD_EARLY).getLevel()) {
            let mut localPortion: i32 = 40;
            if level == jcall!(jcall!(resourceLoadingLevelFactory).LOAD_EARLY).getLevel() {
                localPortion = 8;
            }
            self.setInitialized(true);
            { let java_arguments = (self.getPortion(),); self.initKey(java_arguments.0) };
            { let java_arguments = (abeClientInformation.clone(), self.getPortion(),); self.initKeyMapping(java_arguments.0, java_arguments.1) };
            jcall!(GameKeyEventFactory::getInstance()).init();
            let mut progressCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvasDyn>>> = ProgressCanvasFactory::getInstance();
            jcall!(progressCanvas).addNormalPortion(localPortion.clone(), String::from("Game Key Events"));
            jcall!(BasicTouchInputFactory::getInstance()).init(jcall!(jcall!(PlatformInputMappingFactory::getInstance()).getPersistentInputMappingInstance()).getInputMapping());
            CancelTouchButtonInputFactory::getInstance();
            jcall!(progressCanvas).addNormalPortion(localPortion.clone(), String::from("Touch Input"));
            jcall!(CompleteMotionGestureInputToGameMotionGestureInput::getInstance()).init();
            jcall!(progressCanvas).addNormalPortion(localPortion.clone(), String::from("Motion Input"));
            GameFeatureImageCacheFactory::init();
            jcall!(progressCanvas).addNormalPortion(localPortion.clone(), String::from("Image Cache"));
            jcall!(self.resourceInitializationArray[(self.EARLY_RESOURCES) as usize]).init();
        }
    }
    

    // throws Exception
    pub fn resourceInitialization(&mut self, mut level: i32) {
        let mut resourceLoadingLevelFactory: Option<Rc<RefCell<ResourceLoadingLevelFactory>>> = ResourceLoadingLevelFactory::getInstance();
        if self.resourceAnimationChange() && level == jcall!(jcall!(resourceLoadingLevelFactory).LOAD_EARLY).getLevel() {
            self.clearResources();
            jcall!(self.resourceInitializationArray[(self.EARLY_CHANGABLE_RESOURCES) as usize]).init();
        }
        if self.resourceAnimationChange() && level == jcall!(jcall!(resourceLoadingLevelFactory).LOAD_ALL).getLevel() {
            if self.allLoaded {
                self.clearResources();
            }
            if jcall!(jcall!(FeaturedAnimationInterfaceFactoryInterfaceFactory::getInstance()).getList()).size() == 0 {
                jcall!(self.resourceInitializationArray[(self.EARLY_CHANGABLE_RESOURCES) as usize]).init();
            }
            jcall!(self.resourceInitializationArray[(self.GAME_CHANGABLE_RESOURCES) as usize]).init();
            self.allLoaded = true;
        }
    }
    

    pub fn resourceAnimationChange(&mut self) -> bool {
        let mut changedGameFeatureListener: Option<Rc<RefCell<ChangedGameFeatureListener>>> = ChangedGameFeatureListener::getInstance();
        let mut gameConfigurationCentral: Option<Rc<RefCell<GameConfigurationCentral>>> = GameConfigurationCentral::getInstance();
        if jcall!(changedGameFeatureListener).isChangedGameConfiguration(jcall!(gameConfigurationCentral).SCALE.clone()) {
            return true;
        }
        if jcall!(ChangedGameFeatureListener::getInstance()).isChangedFeature(jcall!(MainFeatureFactory::getInstance()).STATIC.clone()) {
            return true;
        }
        return false;
    }
    

    pub fn clearResources(&mut self) {
        self.allLoaded = false;
        GameFeatureImageCacheFactory::releaseAll();
        let mut featuredAnimationInterfaceFactoryInterfaceFactory: Option<Rc<RefCell<FeaturedAnimationInterfaceFactoryInterfaceFactory>>> = FeaturedAnimationInterfaceFactoryInterfaceFactory::getInstance();
        jcall!(featuredAnimationInterfaceFactoryInterfaceFactory).clear();
        jcall!(FeaturedResourceRelativeRelationshipFactory::getInstance()).clear();
    }
    

    pub fn setGameInitialized(&mut self, mut gameInitialized: bool) {
        self.gameInitialized = gameInitialized.clone();
    }
    

    pub fn isGameInitialized(&mut self) -> bool {
        return self.gameInitialized.clone();
    }
    

    pub fn getPortion(&mut self) -> i32 {
        return self.portion.clone();
    }
    

    pub fn setInitialized(&mut self, mut initialized: bool) {
        self.initialized = initialized.clone();
    }
    

    pub fn isInitialized(&mut self) -> bool {
        return self.initialized.clone();
    }
}
pub trait BaseGameInitializationDyn {
    fn as_BaseGameInitialization(&self) -> &BaseGameInitialization;
    fn as_BaseGameInitialization_mut(&mut self) -> &mut BaseGameInitialization;
    fn initKey(&mut self, mut portion: i32) { BaseGameInitialization::initKey(self.as_BaseGameInitialization_mut(), portion) }
    fn initKeyMapping(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut portion: i32) { BaseGameInitialization::initKeyMapping(self.as_BaseGameInitialization_mut(), abeClientInformation, portion) }
    fn init(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut commandListener: Option<Rc<RefCell<CommandListener>>>, mut level: i32) { BaseGameInitialization::init(self.as_BaseGameInitialization_mut(), abeClientInformation, commandListener, level) }
    fn resourceInitialization(&mut self, mut level: i32) { BaseGameInitialization::resourceInitialization(self.as_BaseGameInitialization_mut(), level) }
    fn resourceAnimationChange(&mut self) -> bool { BaseGameInitialization::resourceAnimationChange(self.as_BaseGameInitialization_mut()) }
    fn clearResources(&mut self) { BaseGameInitialization::clearResources(self.as_BaseGameInitialization_mut()) }
    fn setGameInitialized(&mut self, mut gameInitialized: bool) { BaseGameInitialization::setGameInitialized(self.as_BaseGameInitialization_mut(), gameInitialized) }
    fn isGameInitialized(&mut self) -> bool { BaseGameInitialization::isGameInitialized(self.as_BaseGameInitialization_mut()) }
    fn getPortion(&mut self) -> i32 { BaseGameInitialization::getPortion(self.as_BaseGameInitialization_mut()) }
    fn setInitialized(&mut self, mut initialized: bool) { BaseGameInitialization::setInitialized(self.as_BaseGameInitialization_mut(), initialized) }
    fn isInitialized(&mut self) -> bool { BaseGameInitialization::isInitialized(self.as_BaseGameInitialization_mut()) }
}
impl BaseGameInitializationDyn for BaseGameInitialization {
    fn as_BaseGameInitialization(&self) -> &BaseGameInitialization { self }
    fn as_BaseGameInitialization_mut(&mut self) -> &mut BaseGameInitialization { self }
}


