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
// package org.allbinary.media.graphics.geography.map
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::BasicGeographicMapCellPositionFactory::BasicGeographicMapCellPositionFactory;
use super::BasicGeographicMap::BasicGeographicMap;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GeographicMapCellPositionBaseFactory {
}


impl GeographicMapCellPositionBaseFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn getInstance(&mut self, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMapCellPositionFactory::BasicGeographicMapCellPositionFactoryDyn>>> {
        return BasicGeographicMapCellPositionFactory::new(geographicMapInterface.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMapCellPositionFactory::BasicGeographicMapCellPositionFactoryDyn>>);
    }
}
pub trait GeographicMapCellPositionBaseFactoryDyn {
    fn as_GeographicMapCellPositionBaseFactory(&self) -> &GeographicMapCellPositionBaseFactory;
    fn as_GeographicMapCellPositionBaseFactory_mut(&mut self) -> &mut GeographicMapCellPositionBaseFactory;
    fn getInstance(&mut self, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMapCellPositionFactory::BasicGeographicMapCellPositionFactoryDyn>>> { GeographicMapCellPositionBaseFactory::getInstance(self.as_GeographicMapCellPositionBaseFactory_mut(), geographicMapInterface) }
}
impl GeographicMapCellPositionBaseFactoryDyn for GeographicMapCellPositionBaseFactory {
    fn as_GeographicMapCellPositionBaseFactory(&self) -> &GeographicMapCellPositionBaseFactory { self }
    fn as_GeographicMapCellPositionBaseFactory_mut(&mut self) -> &mut GeographicMapCellPositionBaseFactory { self }
}


