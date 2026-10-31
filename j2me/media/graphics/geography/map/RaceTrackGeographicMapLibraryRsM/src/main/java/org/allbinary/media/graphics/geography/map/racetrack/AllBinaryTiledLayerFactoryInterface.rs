// Generated Code Do Not Modify
// package org.allbinary.media.graphics.geography.map.racetrack
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayer;
use super::RaceTrackInfo::RaceTrackInfo;
use super::RaceTrackData::RaceTrackData;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait AllBinaryTiledLayerFactoryInterface {
    

    // throws Exception
    fn getInstance(&mut self, raceTrackInfo: Option<Rc<RefCell<RaceTrackInfo>>>, raceTrackData: Option<Rc<RefCell<RaceTrackData>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>>;
    

    // throws Exception
    fn getMiniInstance(&mut self, raceTrackData: Option<Rc<RefCell<RaceTrackData>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>>;

    fn toString(&mut self) -> String { String::from(std::any::type_name::<Self>()) }
}


