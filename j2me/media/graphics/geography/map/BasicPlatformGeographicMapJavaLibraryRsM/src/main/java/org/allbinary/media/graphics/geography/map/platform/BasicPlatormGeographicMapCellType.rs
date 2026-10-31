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
// package org.allbinary.media.graphics.geography.map.platform
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::math::SmallIntegerSingletonFactory::SmallIntegerSingletonFactory;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellType;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListS::BasicArrayListS;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicPlatormGeographicMapCellType {
    pub types: Vec<i32>,
}


impl BasicPlatormGeographicMapCellType {
    

    pub fn createType(mut r#type: i32) -> Option<Rc<RefCell<BasicPlatormGeographicMapCellType>>> {
        let mut types: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListS::new(1).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        jcall!(types).add(jcall!(SmallIntegerSingletonFactory::getInstance()).getAt(r#type.clone()));
        let mut basicPlatormGeographicMapCellType: Option<Rc<RefCell<BasicPlatormGeographicMapCellType>>> = BasicPlatormGeographicMapCellType::new(types.clone());
        return basicPlatormGeographicMapCellType.clone();
    }
    

    pub fn new_struct(mut types: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Self {
        let mut this = Self {
            types: Vec::new(),
        };
        let mut size: i32 = jcall!(types).size();
        let mut typeArray: Vec<i32> = vec![0; (size) as usize];
        let mut typeAsInteger: Option<i32> = None;
        let mut r#type: i32 = 0;
        for index in (0..size) {
            typeAsInteger = ((jcall!(types).get(index.clone())).clone() /* as Integer */);
            r#type = ((typeAsInteger).unwrap() as i32);
            GeographicMapCellType::new(r#type.clone(), 0).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellTypeDyn>>);
            typeArray[(index) as usize] = r#type.clone();
        }
        this.types = typeArray.clone();
        this
    }

    pub fn new(mut types: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(types))))
    }
    

    pub fn isType(&mut self, mut r#type: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellTypeDyn>>>) -> bool {
        return { let java_arguments = (jcall!(r#type).getType(),); self.hasType(java_arguments.0) };
    }
    

    pub fn hasType(&mut self, mut r#type: i32) -> bool {
        let mut size: i32 = (self.types.len() as i32).clone();
        for index in (0..size) {
            if self.types[(index) as usize] == r#type {
                return true;
            }
        }
        return false;
    }
}

