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
// package org.allbinary.game.layer
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::GameInfo::GameInfo;
use crate::org::allbinary::game::input::NullPlayerGameInputComposite::NullPlayerGameInputComposite;
use crate::org::allbinary::game::input::PlayerGameInputCompositeInterface::PlayerGameInputCompositeInterface;
use crate::org::allbinary::graphics::ItemColorFactory::ItemColorFactory;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager;
use crate::org::allbinary::layer::LayerProcessor::LayerProcessor;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::math::SmallIntegerSingletonFactory::SmallIntegerSingletonFactory;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AllBinaryGameLayerManager {
    pub base: AllBinaryLayerManager,
    pub backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>,
    pub foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>,
    pub gameInfo: Option<Rc<RefCell<GameInfo>>>,
    pub playerGameInputCompositeInterface: Option<Rc<RefCell<dyn PlayerGameInputCompositeInterface>>>,
}

pub static mut AllBinaryGameLayerManager_NULL_ALLBINARY_LAYER_MANAGER: std::cell::LazyCell<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = std::cell::LazyCell::new(|| jcall!(NullUtil::getInstance()).NULL_OBJECT.clone());
thread_local! { pub static AllBinaryGameLayerManager_ID: Option<i32> = jcall!(SmallIntegerSingletonFactory::getInstance()).getAt(22); }

impl AllBinaryGameLayerManager {
    

    pub fn getNullInstance() -> Option<Rc<RefCell<dyn AllBinaryGameLayerManagerDyn>>> {
        if match (&unsafe { AllBinaryGameLayerManager_NULL_ALLBINARY_LAYER_MANAGER.clone() }, &jcall!(NullUtil::getInstance()).NULL_OBJECT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            unsafe { *AllBinaryGameLayerManager_NULL_ALLBINARY_LAYER_MANAGER = AllBinaryGameLayerManager::new(jcall!(BasicColorFactory::getInstance()).BLACK.clone(), jcall!(BasicColorFactory::getInstance()).WHITE.clone(), crate::org::allbinary::game::GameInfo::GameInfo_NONE.with(Clone::clone).clone()).map(|reference| reference as Rc<RefCell<dyn AllBinaryGameLayerManagerDyn>>) };
        }
        return (unsafe { AllBinaryGameLayerManager_NULL_ALLBINARY_LAYER_MANAGER.clone() }).clone() /* as AllBinaryGameLayerManager */;
    }
    

    pub fn new_struct(mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut gameInfo: Option<Rc<RefCell<GameInfo>>>) -> Self {
        let mut this = Self {
            base: AllBinaryLayerManager::new_struct(),
            backgroundBasicColor: None,
            foregroundBasicColor: None,
            gameInfo: None,
            playerGameInputCompositeInterface: None,
        };
        this.playerGameInputCompositeInterface = crate::org::allbinary::game::input::NullPlayerGameInputComposite::NullPlayerGameInputComposite_NULL_PLAYER_GAME_INPUT_COMPOSITE.with(Clone::clone).clone();
        this.backgroundBasicColor = backgroundBasicColor.clone();
        this.foregroundBasicColor = foregroundBasicColor.clone();
        let mut itemColorFactory: Option<Rc<RefCell<ItemColorFactory>>> = ItemColorFactory::getInstance();
        jcall!(itemColorFactory).INVERT_PAINT = jcall!(this.backgroundBasicColor).intValue();
        jcall!(itemColorFactory).PAINT = jcall!(this.foregroundBasicColor).intValue();
        this.gameInfo = gameInfo.clone();
        this
    }

    pub fn new(mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut gameInfo: Option<Rc<RefCell<GameInfo>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(backgroundBasicColor, foregroundBasicColor, gameInfo))))
    }
    

    pub fn getLayerManager(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>> {
        return self;
    }
    

    pub fn init(&mut self) {
    }
    

    pub fn getBackgroundBasicColor(&mut self) -> Option<Rc<RefCell<BasicColor>>> {
        return self.backgroundBasicColor.clone();
    }
    

    pub fn setBackgroundBasicColor(&mut self, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>) {
        self.backgroundBasicColor = backgroundBasicColor.clone();
        jcall!(ItemColorFactory::getInstance()).INVERT_PAINT = jcall!(self.backgroundBasicColor).intValue();
    }
    

    pub fn getForegroundBasicColor(&mut self) -> Option<Rc<RefCell<BasicColor>>> {
        return self.foregroundBasicColor.clone();
    }
    

    pub fn setForegroundBasicColor(&mut self, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) {
        self.foregroundBasicColor = foregroundBasicColor.clone();
        jcall!(ItemColorFactory::getInstance()).PAINT = jcall!(self.foregroundBasicColor).intValue();
    }
    

    pub fn getGameInfo(&mut self) -> Option<Rc<RefCell<GameInfo>>> {
        return self.gameInfo.clone();
    }
    

    pub fn setGameInfo(&mut self, mut gameInfo: Option<Rc<RefCell<GameInfo>>>) {
        self.gameInfo = gameInfo.clone();
    }
    

    pub fn setLayerProcessorList(&mut self, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        let mut layerProcessorInterfaceArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::layer::LayerProcessor::LayerProcessorDyn>>>> = vec![None; (jcall!(list).size()) as usize];
        let mut size: i32 = (layerProcessorInterfaceArray.len() as i32).clone();
        for index in (0..size) {
            layerProcessorInterfaceArray[(index) as usize] = { let java_object = (jcall!(list).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::layer::LayerProcessor::LayerProcessorDyn>>>>().expect("ClassCastException").clone()) };
        }
        self.setLayerProcessorArray(layerProcessorInterfaceArray.clone());
    }
    

    pub fn setPlayerGameInputCompositeInterface(&mut self, mut playerGameInputCompositeInterface: Option<Rc<RefCell<dyn PlayerGameInputCompositeInterface>>>) {
        self.playerGameInputCompositeInterface = playerGameInputCompositeInterface.clone();
    }
    

    pub fn getPlayerGameInputCompositeInterface(&mut self) -> Option<Rc<RefCell<dyn PlayerGameInputCompositeInterface>>> {
        return self.playerGameInputCompositeInterface.clone();
    }
}
impl std::ops::Deref for AllBinaryGameLayerManager {
    type Target = AllBinaryLayerManager;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AllBinaryGameLayerManager {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait AllBinaryGameLayerManagerDyn: crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn {
    fn as_AllBinaryGameLayerManager(&self) -> &AllBinaryGameLayerManager;
    fn as_AllBinaryGameLayerManager_mut(&mut self) -> &mut AllBinaryGameLayerManager;
    fn getLayerManager(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>> { AllBinaryGameLayerManager::getLayerManager(self.as_AllBinaryGameLayerManager_mut()) }
    fn init(&mut self) { AllBinaryGameLayerManager::init(self.as_AllBinaryGameLayerManager_mut()) }
    fn getBackgroundBasicColor(&mut self) -> Option<Rc<RefCell<BasicColor>>> { AllBinaryGameLayerManager::getBackgroundBasicColor(self.as_AllBinaryGameLayerManager_mut()) }
    fn setBackgroundBasicColor(&mut self, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>) { AllBinaryGameLayerManager::setBackgroundBasicColor(self.as_AllBinaryGameLayerManager_mut(), backgroundBasicColor) }
    fn getForegroundBasicColor(&mut self) -> Option<Rc<RefCell<BasicColor>>> { AllBinaryGameLayerManager::getForegroundBasicColor(self.as_AllBinaryGameLayerManager_mut()) }
    fn setForegroundBasicColor(&mut self, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) { AllBinaryGameLayerManager::setForegroundBasicColor(self.as_AllBinaryGameLayerManager_mut(), foregroundBasicColor) }
    fn getGameInfo(&mut self) -> Option<Rc<RefCell<GameInfo>>> { AllBinaryGameLayerManager::getGameInfo(self.as_AllBinaryGameLayerManager_mut()) }
    fn setGameInfo(&mut self, mut gameInfo: Option<Rc<RefCell<GameInfo>>>) { AllBinaryGameLayerManager::setGameInfo(self.as_AllBinaryGameLayerManager_mut(), gameInfo) }
    fn setLayerProcessorList(&mut self, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { AllBinaryGameLayerManager::setLayerProcessorList(self.as_AllBinaryGameLayerManager_mut(), list) }
    fn setPlayerGameInputCompositeInterface(&mut self, mut playerGameInputCompositeInterface: Option<Rc<RefCell<dyn PlayerGameInputCompositeInterface>>>) { AllBinaryGameLayerManager::setPlayerGameInputCompositeInterface(self.as_AllBinaryGameLayerManager_mut(), playerGameInputCompositeInterface) }
    fn getPlayerGameInputCompositeInterface(&mut self) -> Option<Rc<RefCell<dyn PlayerGameInputCompositeInterface>>> { AllBinaryGameLayerManager::getPlayerGameInputCompositeInterface(self.as_AllBinaryGameLayerManager_mut()) }
}
impl AllBinaryGameLayerManagerDyn for AllBinaryGameLayerManager {
    fn as_AllBinaryGameLayerManager(&self) -> &AllBinaryGameLayerManager { self }
    fn as_AllBinaryGameLayerManager_mut(&mut self) -> &mut AllBinaryGameLayerManager { self }
}

impl crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn for AllBinaryGameLayerManager {
    fn as_AllBinaryLayerManager(&self) -> &crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager { &self.base }
    fn as_AllBinaryLayerManager_mut(&mut self) -> &mut crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager { &mut self.base }
}
impl crate::org::allbinary::layer::LayerManager::LayerManagerDyn for AllBinaryGameLayerManager {
    fn as_LayerManager(&self) -> &crate::org::allbinary::layer::LayerManager::LayerManager { &self.base.base }
    fn as_LayerManager_mut(&mut self) -> &mut crate::org::allbinary::layer::LayerManager::LayerManager { &mut self.base.base }
    fn append(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) { crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager::append(crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn::as_AllBinaryLayerManager_mut(self), p1) }
    fn appendAt(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, p2: i32) { crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager::appendAt(crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn::as_AllBinaryLayerManager_mut(self), p1, p2) }
    fn remove(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) { crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager::remove(crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn::as_AllBinaryLayerManager_mut(self), p1) }
    fn cleanup(&mut self) { crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager::cleanup(crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn::as_AllBinaryLayerManager_mut(self)) }
}


