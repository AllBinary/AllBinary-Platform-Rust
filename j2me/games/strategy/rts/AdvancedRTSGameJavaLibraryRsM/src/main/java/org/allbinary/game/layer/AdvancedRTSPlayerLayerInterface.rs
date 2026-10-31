// 
// AllBinary Open License Version 1 
// Copyright (c) 2006 AllBinary 
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
use crate::org::allbinary::game::GameType::GameType;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use super::RTSPlayerLayerInterface::RTSPlayerLayerInterface;
use super::AdvancedPlayerOwnedRTSLayers::AdvancedPlayerOwnedRTSLayers;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait AdvancedRTSPlayerLayerInterface: RTSPlayerLayerInterface {
    

    fn getAdvancedPlayerOwnedRTSLayers(&mut self) -> Option<Rc<RefCell<AdvancedPlayerOwnedRTSLayers>>>;
    

    fn isLocalPlayer(&mut self) -> bool;
    

    fn getGameType(&mut self) -> Option<Rc<RefCell<GameType>>>;
    

    fn getDecalBasicColor(&mut self) -> Option<Rc<RefCell<BasicColor>>>;
}


