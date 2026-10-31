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
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use super::GeographicMapCellType::GeographicMapCellType;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GeographicMapCellTypeFactory {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub geographicMapCellTypeArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellTypeDyn>>>>,
}

thread_local! { pub static GeographicMapCellTypeFactory_instance: Option<Rc<RefCell<dyn GeographicMapCellTypeFactoryDyn>>> = GeographicMapCellTypeFactory::new().map(|reference| reference as Rc<RefCell<dyn GeographicMapCellTypeFactoryDyn>>); }

impl GeographicMapCellTypeFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            geographicMapCellTypeArray: Vec::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.geographicMapCellTypeArray = vec![None; (512) as usize];
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn GeographicMapCellTypeFactoryDyn>>> {
        return GeographicMapCellTypeFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn get(&mut self, mut r#type: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellTypeDyn>>> {
        return self.geographicMapCellTypeArray[(r#type) as usize].clone();
    }
    

    pub fn getGeographicMapCellTypeArray(&mut self) -> Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellTypeDyn>>>> {
        return self.geographicMapCellTypeArray.clone();
    }
    

    pub fn getStartType(&mut self) -> i32 {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getEndType(&mut self) -> i32 {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getEmptyType(&mut self) -> i32 {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn isPath(&mut self, mut cellType: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellTypeDyn>>>) -> bool {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
}
pub trait GeographicMapCellTypeFactoryDyn {
    fn as_GeographicMapCellTypeFactory(&self) -> &GeographicMapCellTypeFactory;
    fn as_GeographicMapCellTypeFactory_mut(&mut self) -> &mut GeographicMapCellTypeFactory;
    fn get(&mut self, mut r#type: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellTypeDyn>>> { GeographicMapCellTypeFactory::get(self.as_GeographicMapCellTypeFactory_mut(), r#type) }
    fn getGeographicMapCellTypeArray(&mut self) -> Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellTypeDyn>>>> { GeographicMapCellTypeFactory::getGeographicMapCellTypeArray(self.as_GeographicMapCellTypeFactory_mut()) }
    fn getStartType(&mut self) -> i32 { GeographicMapCellTypeFactory::getStartType(self.as_GeographicMapCellTypeFactory_mut()) }
    fn getEndType(&mut self) -> i32 { GeographicMapCellTypeFactory::getEndType(self.as_GeographicMapCellTypeFactory_mut()) }
    fn getEmptyType(&mut self) -> i32 { GeographicMapCellTypeFactory::getEmptyType(self.as_GeographicMapCellTypeFactory_mut()) }
    fn isPath(&mut self, mut cellType: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellTypeDyn>>>) -> bool { GeographicMapCellTypeFactory::isPath(self.as_GeographicMapCellTypeFactory_mut(), cellType) }
}
impl GeographicMapCellTypeFactoryDyn for GeographicMapCellTypeFactory {
    fn as_GeographicMapCellTypeFactory(&self) -> &GeographicMapCellTypeFactory { self }
    fn as_GeographicMapCellTypeFactory_mut(&mut self) -> &mut GeographicMapCellTypeFactory { self }
}


