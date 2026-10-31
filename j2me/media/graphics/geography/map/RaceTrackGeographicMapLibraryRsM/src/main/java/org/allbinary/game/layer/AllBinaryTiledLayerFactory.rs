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
// use crate::javax::microedition::lcdui::game::TiledLayer::TiledLayer;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::logic::math::SmallIntegerSingletonFactory::SmallIntegerSingletonFactory;
use crate::org::allbinary::media::graphics::geography::map::racetrack::AllBinaryTiledLayerFactoryInterface::AllBinaryTiledLayerFactoryInterface;
use crate::org::allbinary::media::graphics::geography::map::racetrack::RaceTrackData::RaceTrackData;
use crate::org::allbinary::media::graphics::geography::map::racetrack::RaceTrackInfo::RaceTrackInfo;
use crate::org::allbinary::media::graphics::geography::map::racetrack::TiledLayerFactory::TiledLayerFactory;
use super::AllBinaryTiledLayer::AllBinaryTiledLayer;
use super::AllBinaryJ2METiledLayer::AllBinaryJ2METiledLayer;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AllBinaryTiledLayerFactory {
}

impl AllBinaryTiledLayerFactoryInterface for AllBinaryTiledLayerFactory {
    fn getInstance(&mut self, mut raceTrackInfo: Option<Rc<RefCell<RaceTrackInfo>>>, mut raceTrackData: Option<Rc<RefCell<RaceTrackData>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>> { Self::getInstance(self, raceTrackInfo, raceTrackData) }
    fn getMiniInstance(&mut self, mut raceTrackData: Option<Rc<RefCell<RaceTrackData>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>> { Self::getMiniInstance(self, raceTrackData) }
}

impl AllBinaryTiledLayerFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn getInstance(&mut self, mut raceTrackInfo: Option<Rc<RefCell<RaceTrackInfo>>>, mut raceTrackData: Option<Rc<RefCell<RaceTrackData>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>> {
        let mut allbinaryTiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>> = None;
        let mut tiledLayer: Option<Rc<RefCell<TiledLayer>>> = jcall!(TiledLayerFactory::new()).getInstance(raceTrackData.clone());
        allbinaryTiledLayer = AllBinaryJ2METiledLayer::new(jcall!(raceTrackData).getId(), tiledLayer.clone(), jcall!(raceTrackData).getMapArray(), jcall!(jcall!(BasicColorFactory::getInstance()).RED).intValue()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>);
        jcall!(allbinaryTiledLayer).setCells(jcall!(raceTrackData).getMapArray());
        return allbinaryTiledLayer.clone();
    }
    

    // throws Exception
    pub fn getMiniInstance(&mut self, mut raceTrackData: Option<Rc<RefCell<RaceTrackData>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>> {
        let mut smallIntegerSingletonFactory: Option<Rc<RefCell<SmallIntegerSingletonFactory>>> = SmallIntegerSingletonFactory::getInstance();
        let mut tiledLayer: Option<Rc<RefCell<TiledLayer>>> = jcall!(TiledLayerFactory::new()).getMiniInstance(raceTrackData.clone());
        let mut allbinaryTiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>> = AllBinaryJ2METiledLayer::new(jcall!(smallIntegerSingletonFactory).getAt(((jcall!(raceTrackData).getId()).unwrap() as i32) + 100 + 1), tiledLayer.clone(), jcall!(raceTrackData).getMapArray(), jcall!(jcall!(BasicColorFactory::getInstance()).RED).intValue()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>);
        jcall!(allbinaryTiledLayer).setCells(jcall!(raceTrackData).getMapArray());
        return allbinaryTiledLayer.clone();
    }
}

