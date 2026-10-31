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
// package org.allbinary.media.graphics.geography.map.racetrack.threed
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayer;
use crate::org::allbinary::media::graphics::geography::map::racetrack::RaceTrackData::RaceTrackData;
use crate::org::allbinary::media::graphics::geography::map::racetrack::RaceTrackInfo::RaceTrackInfo;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AllBinaryThreedTiledLayerFactory {
}

thread_local! { pub static AllBinaryThreedTiledLayerFactory_instance: Option<Rc<RefCell<AllBinaryThreedTiledLayerFactory>>> = AllBinaryThreedTiledLayerFactory::new(); }

impl AllBinaryThreedTiledLayerFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<AllBinaryThreedTiledLayerFactory>>> {
        return AllBinaryThreedTiledLayerFactory_instance.with(Clone::clone).clone();
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn getInstance2(&mut self, mut raceTrackInfo: Option<Rc<RefCell<RaceTrackInfo>>>, mut raceTrackData: Option<Rc<RefCell<RaceTrackData>>>) -> Option<Rc<RefCell<AllBinaryTiledLayer>>> {
        panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("No Threed"))));
    }
}

