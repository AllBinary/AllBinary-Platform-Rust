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
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use super::RaceTrackFrictionProperties::RaceTrackFrictionProperties;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RaceTrackInfo {
    pub id: Option<i32>,
    pub name: String,
    pub raceTrackFrictionProperties: Option<Rc<RefCell<RaceTrackFrictionProperties>>>,
    pub foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>,
    pub backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>,
    pub qualifyTime: i32,
    pub worstContinuePosition: i32,
    pub totalLaps: i32,
}


impl RaceTrackInfo {
    

    pub fn new_struct(mut id: Option<i32>, mut name: String, mut raceTrackFrictionProperties: Option<Rc<RefCell<RaceTrackFrictionProperties>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut qualifyTime: i32, mut worstContinuePosition: i32, mut totalLaps: i32) -> Self {
        let mut this = Self {
            id: None,
            name: String::new(),
            raceTrackFrictionProperties: None,
            foregroundBasicColor: None,
            backgroundBasicColor: None,
            qualifyTime: 0,
            worstContinuePosition: 0,
            totalLaps: 0,
        };
        this.id = id.clone();
        this.name = name.clone();
        this.raceTrackFrictionProperties = raceTrackFrictionProperties.clone();
        this.backgroundBasicColor = backgroundBasicColor.clone();
        this.foregroundBasicColor = foregroundBasicColor.clone();
        this.qualifyTime = qualifyTime.clone();
        this.worstContinuePosition = worstContinuePosition.clone();
        this.totalLaps = totalLaps.clone();
        this
    }

    pub fn new(mut id: Option<i32>, mut name: String, mut raceTrackFrictionProperties: Option<Rc<RefCell<RaceTrackFrictionProperties>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut qualifyTime: i32, mut worstContinuePosition: i32, mut totalLaps: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(id, name, raceTrackFrictionProperties, foregroundBasicColor, backgroundBasicColor, qualifyTime, worstContinuePosition, totalLaps))))
    }
    

    pub fn getId(&mut self) -> Option<i32> {
        return self.id.clone();
    }
    

    pub fn setId(&mut self, mut id: Option<i32>) {
        self.id = id.clone();
    }
    

    pub fn getName(&mut self) -> String {
        return self.name.clone();
    }
    

    pub fn setName(&mut self, mut name: String) {
        self.name = name.clone();
    }
    

    pub fn getQualifyTime(&mut self) -> i32 {
        return self.qualifyTime.clone();
    }
    

    pub fn setQualifyTime(&mut self, mut qualifyTime: i32) {
        self.qualifyTime = qualifyTime.clone();
    }
    

    pub fn getWorstContinuePosition(&mut self) -> i32 {
        return self.worstContinuePosition.clone();
    }
    

    pub fn setWorstContinuePosition(&mut self, mut worstContinuePosition: i32) {
        self.worstContinuePosition = worstContinuePosition.clone();
    }
    

    pub fn getTotalLaps(&mut self) -> i32 {
        return self.totalLaps.clone();
    }
    

    pub fn setTotalLaps(&mut self, mut totalLaps: i32) {
        self.totalLaps = totalLaps.clone();
    }
    

    pub fn getForegroundBasicColor(&mut self) -> Option<Rc<RefCell<BasicColor>>> {
        return self.foregroundBasicColor.clone();
    }
    

    pub fn setForegroundBasicColor(&mut self, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) {
        self.foregroundBasicColor = foregroundBasicColor.clone();
    }
    

    pub fn getBackgroundBasicColor(&mut self) -> Option<Rc<RefCell<BasicColor>>> {
        return self.backgroundBasicColor.clone();
    }
    

    pub fn setBackgroundBasicColor(&mut self, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>) {
        self.backgroundBasicColor = backgroundBasicColor.clone();
    }
    

    pub fn getRaceTrackFrictionProperties(&mut self) -> Option<Rc<RefCell<RaceTrackFrictionProperties>>> {
        return self.raceTrackFrictionProperties.clone();
    }
    

    pub fn setRaceTrackFrictionProperties(&mut self, mut raceTrackFrictionProperties: Option<Rc<RefCell<RaceTrackFrictionProperties>>>) {
        self.raceTrackFrictionProperties = raceTrackFrictionProperties.clone();
    }
}

