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
// package org.allbinary.game.layer
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::ag::layer::hud::basic::notification::GameNotificationHud::GameNotificationHud;
use crate::org::allbinary::game::combat::destroy::event::DestroyedEvent::DestroyedEvent;
use crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInput;
use crate::org::allbinary::game::input::form::NullRTSFormInputFactory::NullRTSFormInputFactory;
use crate::org::allbinary::game::input::form::RTSFormInput::RTSFormInput;
use crate::org::allbinary::game::layer::capital::Capital::Capital;
use crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionForm;
use crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager;
use crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject;
use crate::org::allbinary::media::audio::Sound::Sound;
use super::NullPathFindingLayer::NullPathFindingLayer;
use super::RTSPlayerLayerInterface::RTSPlayerLayerInterface;
use super::PlayerOwnedRTSLayers::PlayerOwnedRTSLayers;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NullRTSLayer {
    pub base: NullPathFindingLayer,
}

thread_local! { pub static NullRTSLayer_NULL_RTS_LAYER: Option<Rc<RefCell<NullRTSLayer>>> = NullRTSLayer::new(); }
// impl RTSPlayerLayerInterface for NullRTSLayer with the methods of the trait
impl crate::org::allbinary::game::input::GameKeyEventSourceInterface::GameKeyEventSourceInterface for NullRTSLayer {
    fn getSourceId(&mut self) -> i32 { self.base.getSourceId() }
}
impl crate::org::allbinary::game::combat::destroy::DestroyableInterface::DestroyableInterface for NullRTSLayer {
    fn isDestroyed(&mut self) -> bool { self.base.isDestroyed() }
}

impl NullRTSLayer {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: NullPathFindingLayer::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getName(&mut self) -> String {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // throws Exception
    pub fn processInput(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn initInputProcessors(&mut self) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getGameNotificationHud(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::ag::layer::hud::basic::notification::GameNotificationHud::GameNotificationHudDyn>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getCurrentScrollSelectionForm(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionFormDyn>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn setCurrentScrollSelectionForm(&mut self, mut currentScrollSelectionForm: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionFormDyn>>>) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getBuildingScrollSelectionForm(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionFormDyn>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getPlayerGameInput(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInputDyn>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // throws Exception
    pub fn onDestroyed(&mut self, mut destroyedEvent: Option<Rc<RefCell<DestroyedEvent>>>) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // throws Exception
    pub fn processTick(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getCapital(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::capital::Capital::CapitalDyn>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getRTSFormInput(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::input::form::RTSFormInput::RTSFormInputDyn>>> {
        return NullRTSFormInputFactory::getInstance();
    }
    

    pub fn add(&mut self, mut sound: Option<Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>>) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getPlayerOwnedRTSLayers(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::PlayerOwnedRTSLayers::PlayerOwnedRTSLayersDyn>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn implmentsArtificialIntelligenceCompositeInterface(&mut self) -> bool {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
}
impl std::ops::Deref for NullRTSLayer {
    type Target = NullPathFindingLayer;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for NullRTSLayer {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::layer::NullPathFindingLayer::NullPathFindingLayerDyn for NullRTSLayer {
    fn as_NullPathFindingLayer(&self) -> &crate::org::allbinary::game::layer::NullPathFindingLayer::NullPathFindingLayer { &self.base }
    fn as_NullPathFindingLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::NullPathFindingLayer::NullPathFindingLayer { &mut self.base }
    fn getName(&mut self) -> String { Self::getName(self) }
}


