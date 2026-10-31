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
// package org.allbinary.media.graphics.geography.map.topview
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellType;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellTypeFactory::GeographicMapCellTypeFactory;
use crate::org::allbinary::media::graphics::geography::map::racetrack::RaceTrackGeographicMapCellType::RaceTrackGeographicMapCellType;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListS::BasicArrayListS;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicTopViewGeographicMapCellType {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub name: String,
    pub cost: i32,
    pub types: Vec<i32>,
}


impl BasicTopViewGeographicMapCellType {
    

    pub fn createType(mut name: String, mut r#type: i32, mut cost: i32) -> Option<Rc<RefCell<BasicTopViewGeographicMapCellType>>> {
        let mut types: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListS::new(1).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        jcall!(types).add(r#type.clone());
        return BasicTopViewGeographicMapCellType::new(name.clone(), types.clone(), cost.clone());
    }
    

    pub fn new_struct(mut name: String, mut types: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut cost: i32) -> Self {
        let mut this = Self {
            logUtil: None,
            name: String::new(),
            cost: 0,
            types: Vec::new(),
        };
        this.logUtil = LogUtil::getInstance();
        let mut size: i32 = jcall!(types).size();
        let mut typeArray: Vec<i32> = vec![0; (size) as usize];
        let mut typeAsInteger: Option<i32> = None;
        let mut r#type: i32 = 0;
        for index in (0..size) {
            typeAsInteger = ((jcall!(types).get(index.clone())).clone() /* as Integer */);
            r#type = ((typeAsInteger).unwrap() as i32);
            if jcall!(GeographicMapCellTypeFactory::getInstance()).getGeographicMapCellTypeArray()[(r#type) as usize].is_none() {
                RaceTrackGeographicMapCellType::new(name.clone(), r#type.clone(), cost.clone(), 0);
            } else {
            }
            typeArray[(index) as usize] = r#type.clone();
        }
        this.name = name.clone();
        this.cost = cost.clone();
        this.types = typeArray.clone();
        this
    }

    pub fn new(mut name: String, mut types: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut cost: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(name, types, cost))))
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
    

    pub fn getTypes(&mut self) -> Vec<i32> {
        return self.types.clone();
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringMaker: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut size: i32 = (self.types.len() as i32).clone();
        for index in (0..size) {
            jcall!(stringMaker).appendint(self.types[(index) as usize].clone()).append(jcall!(CommonSeps::getInstance()).COMMA.clone());
        }
        return jcall!(stringMaker).toString();
    }
}

