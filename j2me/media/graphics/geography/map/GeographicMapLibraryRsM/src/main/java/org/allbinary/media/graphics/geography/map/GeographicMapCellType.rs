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
use crate::org::allbinary::logic::math::SmallIntegerSingletonFactory::SmallIntegerSingletonFactory;
use super::GeographicMapCellTypeFactory::GeographicMapCellTypeFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GeographicMapCellType {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub r#type: i32,
}

thread_local! { pub static GeographicMapCellType_NULL_GEOGRAPHIC_MAP_CELL_TYPE_ARRAY: Vec<Option<Rc<RefCell<dyn GeographicMapCellTypeDyn>>>> = vec![None; (0) as usize]; }
thread_local! { pub static GeographicMapCellType_NULL_GEOGRAPHIC_MAP_CELL_TYPE: Option<Rc<RefCell<dyn GeographicMapCellTypeDyn>>> = GeographicMapCellType::new(i32::MIN.clone(), 0).map(|reference| reference as Rc<RefCell<dyn GeographicMapCellTypeDyn>>); }

impl GeographicMapCellType {
    

    pub fn new_struct(mut r#type: i32, mut reset: i32) -> Self {
        let mut this = Self {
            logUtil: None,
            r#type: 0,
        };
        this.logUtil = LogUtil::getInstance();
        this.r#type = r#type.clone();
        if r#type != i32::MIN {
            let mut geographicMapCellTypeFactory: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellTypeFactory::GeographicMapCellTypeFactoryDyn>>> = GeographicMapCellTypeFactory::getInstance();
            let mut geographicMapCellTypeArray: Vec<Option<Rc<RefCell<dyn GeographicMapCellTypeDyn>>>> = jcall!(geographicMapCellTypeFactory).getGeographicMapCellTypeArray();
            if geographicMapCellTypeArray[(r#type) as usize].is_none() || reset == 1 {
                geographicMapCellTypeArray[(r#type) as usize] = this;
            } else {
                if reset != -1 {
                    panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
                }
            }
        }
        this
    }

    pub fn new(mut r#type: i32, mut reset: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(r#type, reset))))
    }
    

    pub fn getType(&mut self) -> i32 {
        return self.r#type.clone();
    }
    

    pub fn toString(&mut self) -> String {
        return jcall!(jcall!(SmallIntegerSingletonFactory::getInstance()).getAt(self.r#type.clone())).toString();
    }
}
pub trait GeographicMapCellTypeDyn {
    fn as_GeographicMapCellType(&self) -> &GeographicMapCellType;
    fn as_GeographicMapCellType_mut(&mut self) -> &mut GeographicMapCellType;
    fn getType(&mut self) -> i32 { GeographicMapCellType::getType(self.as_GeographicMapCellType_mut()) }
    fn toString(&mut self) -> String { GeographicMapCellType::toString(self.as_GeographicMapCellType_mut()) }
}
impl GeographicMapCellTypeDyn for GeographicMapCellType {
    fn as_GeographicMapCellType(&self) -> &GeographicMapCellType { self }
    fn as_GeographicMapCellType_mut(&mut self) -> &mut GeographicMapCellType { self }
}


