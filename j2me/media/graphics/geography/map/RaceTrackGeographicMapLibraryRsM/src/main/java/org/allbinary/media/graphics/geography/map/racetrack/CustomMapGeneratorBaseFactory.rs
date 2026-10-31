// 
// AllBinary Open License Version 1 
// Copyright (c) 2022 AllBinary 
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
use super::CustomMapGeneratorBase::CustomMapGeneratorBase;
use super::RaceTrackGeographicMap::RaceTrackGeographicMap;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CustomMapGeneratorBaseFactory {
}

thread_local! { pub static CustomMapGeneratorBaseFactory_instance: Option<Rc<RefCell<dyn CustomMapGeneratorBaseFactoryDyn>>> = CustomMapGeneratorBaseFactory::new().map(|reference| reference as Rc<RefCell<dyn CustomMapGeneratorBaseFactoryDyn>>); }

impl CustomMapGeneratorBaseFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn CustomMapGeneratorBaseFactoryDyn>>> {
        return CustomMapGeneratorBaseFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn create(&mut self, mut raceTrackGeographicMap: Option<Rc<RefCell<RaceTrackGeographicMap>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::racetrack::CustomMapGeneratorBase::CustomMapGeneratorBaseDyn>>> {
        return CustomMapGeneratorBase::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::racetrack::CustomMapGeneratorBase::CustomMapGeneratorBaseDyn>>);
    }
}
pub trait CustomMapGeneratorBaseFactoryDyn {
    fn as_CustomMapGeneratorBaseFactory(&self) -> &CustomMapGeneratorBaseFactory;
    fn as_CustomMapGeneratorBaseFactory_mut(&mut self) -> &mut CustomMapGeneratorBaseFactory;
    fn create(&mut self, mut raceTrackGeographicMap: Option<Rc<RefCell<RaceTrackGeographicMap>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::racetrack::CustomMapGeneratorBase::CustomMapGeneratorBaseDyn>>> { CustomMapGeneratorBaseFactory::create(self.as_CustomMapGeneratorBaseFactory_mut(), raceTrackGeographicMap) }
}
impl CustomMapGeneratorBaseFactoryDyn for CustomMapGeneratorBaseFactory {
    fn as_CustomMapGeneratorBaseFactory(&self) -> &CustomMapGeneratorBaseFactory { self }
    fn as_CustomMapGeneratorBaseFactory_mut(&mut self) -> &mut CustomMapGeneratorBaseFactory { self }
}


