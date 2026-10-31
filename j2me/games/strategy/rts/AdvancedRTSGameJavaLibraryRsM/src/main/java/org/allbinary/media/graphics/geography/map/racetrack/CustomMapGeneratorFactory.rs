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
use super::CustomMapGeneratorBaseFactory::CustomMapGeneratorBaseFactory;
use super::CustomMapGenerator::CustomMapGenerator;
use super::CustomMapGeneratorBase::CustomMapGeneratorBase;
use super::RaceTrackGeographicMap::RaceTrackGeographicMap;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CustomMapGeneratorFactory {
    pub base: CustomMapGeneratorBaseFactory,
}


impl CustomMapGeneratorFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: CustomMapGeneratorBaseFactory::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn create(&mut self, mut raceTrackGeographicMap: Option<Rc<RefCell<RaceTrackGeographicMap>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::racetrack::CustomMapGeneratorBase::CustomMapGeneratorBaseDyn>>> {
        return CustomMapGenerator::new(raceTrackGeographicMap.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::racetrack::CustomMapGeneratorBase::CustomMapGeneratorBaseDyn>>);
    }
}
impl std::ops::Deref for CustomMapGeneratorFactory {
    type Target = CustomMapGeneratorBaseFactory;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for CustomMapGeneratorFactory {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::media::graphics::geography::map::racetrack::CustomMapGeneratorBaseFactory::CustomMapGeneratorBaseFactoryDyn for CustomMapGeneratorFactory {
    fn as_CustomMapGeneratorBaseFactory(&self) -> &crate::org::allbinary::media::graphics::geography::map::racetrack::CustomMapGeneratorBaseFactory::CustomMapGeneratorBaseFactory { &self.base }
    fn as_CustomMapGeneratorBaseFactory_mut(&mut self) -> &mut crate::org::allbinary::media::graphics::geography::map::racetrack::CustomMapGeneratorBaseFactory::CustomMapGeneratorBaseFactory { &mut self.base }
    fn create(&mut self, mut raceTrackGeographicMap: Option<Rc<RefCell<RaceTrackGeographicMap>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::racetrack::CustomMapGeneratorBase::CustomMapGeneratorBaseDyn>>> { Self::create(self, raceTrackGeographicMap) }
}


