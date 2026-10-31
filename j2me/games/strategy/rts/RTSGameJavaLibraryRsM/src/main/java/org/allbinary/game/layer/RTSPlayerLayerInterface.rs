// 
// AllBinary Open License Version 1 
// Copyright (c) 2003 AllBinary 
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
use crate::org::allbinary::game::ag::layer::hud::basic::notification::GameNotificationHudCompositeInterface::GameNotificationHudCompositeInterface;
use crate::org::allbinary::game::input::BuildMenuCompositeInterface::BuildMenuCompositeInterface;
use crate::org::allbinary::game::input::form::RTSFormInput::RTSFormInput;
use crate::org::allbinary::game::layer::capital::Capital::Capital;
use crate::org::allbinary::game::combat::destroy::event::DestroyedEventListenerInterface::DestroyedEventListenerInterface;
use crate::org::allbinary::game::input::PlayerGameInputCompositeInterface::PlayerGameInputCompositeInterface;
use crate::org::allbinary::game::tick::TickableInterface::TickableInterface;
use crate::org::allbinary::media::audio::Sound::Sound;
use super::PathFindingLayerInterface::PathFindingLayerInterface;
use super::PlayerOwnedRTSLayers::PlayerOwnedRTSLayers;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait RTSPlayerLayerInterface: GameNotificationHudCompositeInterface + BuildMenuCompositeInterface + PlayerGameInputCompositeInterface + DestroyedEventListenerInterface + TickableInterface + PathFindingLayerInterface {
    

    fn getCapital(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::capital::Capital::CapitalDyn>>>;
    

    fn getRTSFormInput(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::input::form::RTSFormInput::RTSFormInputDyn>>>;
    

    fn add(&mut self, sound: Option<Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>>);
    

    fn getPlayerOwnedRTSLayers(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::PlayerOwnedRTSLayers::PlayerOwnedRTSLayersDyn>>>;
    

    fn implmentsArtificialIntelligenceCompositeInterface(&mut self) -> bool;
}


