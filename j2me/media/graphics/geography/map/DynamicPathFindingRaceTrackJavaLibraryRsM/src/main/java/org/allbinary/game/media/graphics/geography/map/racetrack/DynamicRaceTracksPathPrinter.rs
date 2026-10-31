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
// package org.allbinary.game.media.graphics.geography.map.racetrack
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::math::SmallIntegerSingletonFactory::SmallIntegerSingletonFactory;
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMap;
use crate::org::allbinary::media::graphics::geography::map::racetrack::RaceTrackGeographicMapInterfaceFactoryInterface::RaceTrackGeographicMapInterfaceFactoryInterface;
use crate::org::allbinary::media::graphics::geography::map::racetrack::RaceTrackRoadsGeographicMapCellHistoryFactory::RaceTrackRoadsGeographicMapCellHistoryFactory;
use crate::org::allbinary::media::graphics::geography::map::racetrack::RaceTracksPathPrinter::RaceTracksPathPrinter;
use crate::org::allbinary::media::graphics::geography::pathfinding::PathFindingInfo::PathFindingInfo;
use crate::org::allbinary::media::graphics::geography::pathfinding::PathGenerator::PathGenerator;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct DynamicRaceTracksPathPrinter {
    pub base: RaceTracksPathPrinter,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}


impl DynamicRaceTracksPathPrinter {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: RaceTracksPathPrinter::new_struct(),
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn printAllPaths(&mut self, mut raceTrackGeographicMapInterfaceFactoryInterface: Option<Rc<RefCell<dyn RaceTrackGeographicMapInterfaceFactoryInterface>>>, mut pathFindingInfo: Option<Rc<RefCell<PathFindingInfo>>>, mut size: i32) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            for index in (0..size) {
                let mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>> = jcall!(raceTrackGeographicMapInterfaceFactoryInterface).getTrackInstance(jcall!(SmallIntegerSingletonFactory::getInstance()).getAt(index.clone()));
                let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(PathGenerator::getInstance()).createList(geographicMapInterface.clone(), RaceTrackRoadsGeographicMapCellHistoryFactory::getInstance(), pathFindingInfo.clone(), 10);
                println!("{}", format!("{}{}", String::from("Track "), index));
                self.printPaths(list.clone());
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(self.logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).PROCESS.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
}
impl std::ops::Deref for DynamicRaceTracksPathPrinter {
    type Target = RaceTracksPathPrinter;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for DynamicRaceTracksPathPrinter {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::media::graphics::geography::map::racetrack::RaceTracksPathPrinter::RaceTracksPathPrinterDyn for DynamicRaceTracksPathPrinter {
    fn as_RaceTracksPathPrinter(&self) -> &crate::org::allbinary::media::graphics::geography::map::racetrack::RaceTracksPathPrinter::RaceTracksPathPrinter { &self.base }
    fn as_RaceTracksPathPrinter_mut(&mut self) -> &mut crate::org::allbinary::media::graphics::geography::map::racetrack::RaceTracksPathPrinter::RaceTracksPathPrinter { &mut self.base }
}


