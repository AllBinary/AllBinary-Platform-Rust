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
// package org.allbinary.media.graphics.geography.map.racetrack
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::game::TiledLayer::TiledLayer;
use super::RaceTrackData::RaceTrackData;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TiledLayerFactory {
}


impl TiledLayerFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance(&mut self, mut raceTrackData: Option<Rc<RefCell<RaceTrackData>>>) -> Option<Rc<RefCell<TiledLayer>>> {
        let mut mapArray: Vec<Vec<i32>> = jcall!(raceTrackData).getMapArray();
        return TiledLayer::new((mapArray[(0) as usize].len() as i32).clone(), (mapArray.len() as i32).clone(), jcall!(raceTrackData).getTileSetImage(), jcall!(raceTrackData).getCellWidth(), jcall!(raceTrackData).getCellHeight());
    }
    

    pub fn getMiniInstance(&mut self, mut raceTrackData: Option<Rc<RefCell<RaceTrackData>>>) -> Option<Rc<RefCell<TiledLayer>>> {
        let mut mapArray: Vec<Vec<i32>> = jcall!(raceTrackData).getMapArray();
        return TiledLayer::new((mapArray[(0) as usize].len() as i32).clone(), (mapArray.len() as i32).clone(), jcall!(raceTrackData).getMiniTileSetImage(), jcall!(raceTrackData).getMiniCellWidth(), jcall!(raceTrackData).getMiniCellHeight());
    }
}

