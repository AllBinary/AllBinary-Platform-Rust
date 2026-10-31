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
use super::RaceTrackGeographicMapCellTypeFactory::RaceTrackGeographicMapCellTypeFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NormalRaceTrackGeographicMapCellTypeFactory {
    pub base: RaceTrackGeographicMapCellTypeFactory,
}

thread_local! { pub static NormalRaceTrackGeographicMapCellTypeFactory_instanceC: Option<Rc<RefCell<NormalRaceTrackGeographicMapCellTypeFactory>>> = NormalRaceTrackGeographicMapCellTypeFactory::new(); }

impl NormalRaceTrackGeographicMapCellTypeFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<NormalRaceTrackGeographicMapCellTypeFactory>>> {
        return NormalRaceTrackGeographicMapCellTypeFactory_instanceC.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: RaceTrackGeographicMapCellTypeFactory::new_struct(),
        };
        this.EMPTY_CELL_TYPE = jcall!(EmptyRaceRaceTrackGeographicMapCellTypeFactory::EmptyRaceRaceTrackGeographicMapCellTypeFactory::getInstance()).EMPTY_CELL_TYPE.clone();
        this.EASY_CELL_TYPE = this.EMPTY_CELL_TYPE.clone();
        this.FINISH_LINE_ROAD_CELL_TYPE = this.DEFAULT_FINISH_LINE_ROAD_CELL_TYPE.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellTypeDyn>>);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}
impl std::ops::Deref for NormalRaceTrackGeographicMapCellTypeFactory {
    type Target = RaceTrackGeographicMapCellTypeFactory;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for NormalRaceTrackGeographicMapCellTypeFactory {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::media::graphics::geography::map::racetrack::RaceTrackGeographicMapCellTypeFactory::RaceTrackGeographicMapCellTypeFactoryDyn for NormalRaceTrackGeographicMapCellTypeFactory {
    fn as_RaceTrackGeographicMapCellTypeFactory(&self) -> &crate::org::allbinary::media::graphics::geography::map::racetrack::RaceTrackGeographicMapCellTypeFactory::RaceTrackGeographicMapCellTypeFactory { &self.base }
    fn as_RaceTrackGeographicMapCellTypeFactory_mut(&mut self) -> &mut crate::org::allbinary::media::graphics::geography::map::racetrack::RaceTrackGeographicMapCellTypeFactory::RaceTrackGeographicMapCellTypeFactory { &mut self.base }
}
impl crate::org::allbinary::media::graphics::geography::map::GeographicMapCellTypeFactory::GeographicMapCellTypeFactoryDyn for NormalRaceTrackGeographicMapCellTypeFactory {
    fn as_GeographicMapCellTypeFactory(&self) -> &crate::org::allbinary::media::graphics::geography::map::GeographicMapCellTypeFactory::GeographicMapCellTypeFactory { &self.base.base }
    fn as_GeographicMapCellTypeFactory_mut(&mut self) -> &mut crate::org::allbinary::media::graphics::geography::map::GeographicMapCellTypeFactory::GeographicMapCellTypeFactory { &mut self.base.base }
    fn getStartType(&mut self) -> i32 { crate::org::allbinary::media::graphics::geography::map::racetrack::RaceTrackGeographicMapCellTypeFactory::RaceTrackGeographicMapCellTypeFactory::getStartType(crate::org::allbinary::media::graphics::geography::map::racetrack::RaceTrackGeographicMapCellTypeFactory::RaceTrackGeographicMapCellTypeFactoryDyn::as_RaceTrackGeographicMapCellTypeFactory_mut(self)) }
    fn getEndType(&mut self) -> i32 { crate::org::allbinary::media::graphics::geography::map::racetrack::RaceTrackGeographicMapCellTypeFactory::RaceTrackGeographicMapCellTypeFactory::getEndType(crate::org::allbinary::media::graphics::geography::map::racetrack::RaceTrackGeographicMapCellTypeFactory::RaceTrackGeographicMapCellTypeFactoryDyn::as_RaceTrackGeographicMapCellTypeFactory_mut(self)) }
    fn getEmptyType(&mut self) -> i32 { crate::org::allbinary::media::graphics::geography::map::racetrack::RaceTrackGeographicMapCellTypeFactory::RaceTrackGeographicMapCellTypeFactory::getEmptyType(crate::org::allbinary::media::graphics::geography::map::racetrack::RaceTrackGeographicMapCellTypeFactory::RaceTrackGeographicMapCellTypeFactoryDyn::as_RaceTrackGeographicMapCellTypeFactory_mut(self)) }
    fn isPath(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellTypeDyn>>>) -> bool { crate::org::allbinary::media::graphics::geography::map::racetrack::RaceTrackGeographicMapCellTypeFactory::RaceTrackGeographicMapCellTypeFactory::isPath(crate::org::allbinary::media::graphics::geography::map::racetrack::RaceTrackGeographicMapCellTypeFactory::RaceTrackGeographicMapCellTypeFactoryDyn::as_RaceTrackGeographicMapCellTypeFactory_mut(self), p1) }
}


