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
use super::RaceTrackGeographicMapCellType::RaceTrackGeographicMapCellType;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RaceTrackGeographicMapCellTypes {
    pub BOTTOM_LEFT_TURN_ROAD_CELL_TYPE: Option<Rc<RefCell<RaceTrackGeographicMapCellType>>>,
    pub BOTTOM_RIGHT_TURN_ROAD_CELL_TYPE: Option<Rc<RefCell<RaceTrackGeographicMapCellType>>>,
    pub TOP_LEFT_TURN_ROAD_CELL_TYPE: Option<Rc<RefCell<RaceTrackGeographicMapCellType>>>,
    pub TOP_RIGHT_TURN_ROAD_CELL_TYPE: Option<Rc<RefCell<RaceTrackGeographicMapCellType>>>,
    pub HORIZONTAL_STRAIGHT_ROAD_CELL_TYPE: Option<Rc<RefCell<RaceTrackGeographicMapCellType>>>,
    pub VERTICAL_STRAIGHT_ROAD_CELL_TYPE: Option<Rc<RefCell<RaceTrackGeographicMapCellType>>>,
    pub START_LINE_ROAD_CELL_TYPE: Option<Rc<RefCell<RaceTrackGeographicMapCellType>>>,
    pub DEFAULT_FINISH_LINE_ROAD_CELL_TYPE: Option<Rc<RefCell<RaceTrackGeographicMapCellType>>>,
}

thread_local! { pub static RaceTrackGeographicMapCellTypes_instance: Option<Rc<RefCell<RaceTrackGeographicMapCellTypes>>> = RaceTrackGeographicMapCellTypes::new(); }

impl RaceTrackGeographicMapCellTypes {
    

    pub fn getInstance() -> Option<Rc<RefCell<RaceTrackGeographicMapCellTypes>>> {
        return RaceTrackGeographicMapCellTypes_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            BOTTOM_LEFT_TURN_ROAD_CELL_TYPE: None,
            BOTTOM_RIGHT_TURN_ROAD_CELL_TYPE: None,
            TOP_LEFT_TURN_ROAD_CELL_TYPE: None,
            TOP_RIGHT_TURN_ROAD_CELL_TYPE: None,
            HORIZONTAL_STRAIGHT_ROAD_CELL_TYPE: None,
            VERTICAL_STRAIGHT_ROAD_CELL_TYPE: None,
            START_LINE_ROAD_CELL_TYPE: None,
            DEFAULT_FINISH_LINE_ROAD_CELL_TYPE: None,
        };
        this.BOTTOM_LEFT_TURN_ROAD_CELL_TYPE = RaceTrackGeographicMapCellType::new(String::from("Bottom Left Turn"), 1, 1, 0);
        this.BOTTOM_RIGHT_TURN_ROAD_CELL_TYPE = RaceTrackGeographicMapCellType::new(String::from("Bottom Right Turn"), 2, 1, 0);
        this.TOP_LEFT_TURN_ROAD_CELL_TYPE = RaceTrackGeographicMapCellType::new(String::from("Top Left Turn"), 3, 1, 0);
        this.TOP_RIGHT_TURN_ROAD_CELL_TYPE = RaceTrackGeographicMapCellType::new(String::from("Top Right Turne"), 4, 1, 0);
        this.HORIZONTAL_STRAIGHT_ROAD_CELL_TYPE = RaceTrackGeographicMapCellType::new(String::from("Horizontal Straight"), 5, 1, 0);
        this.VERTICAL_STRAIGHT_ROAD_CELL_TYPE = RaceTrackGeographicMapCellType::new(String::from("Vertical Straight"), 6, 1, 0);
        this.START_LINE_ROAD_CELL_TYPE = RaceTrackGeographicMapCellType::new(String::from("Start Line"), 7, 1, 0);
        this.DEFAULT_FINISH_LINE_ROAD_CELL_TYPE = RaceTrackGeographicMapCellType::new(String::from("Finish Line"), 8, 1, 0);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

