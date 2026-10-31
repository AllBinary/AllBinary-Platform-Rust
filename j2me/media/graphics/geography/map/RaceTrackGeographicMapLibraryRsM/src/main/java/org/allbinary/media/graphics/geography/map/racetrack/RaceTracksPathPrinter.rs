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
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::string::CommonLabels::CommonLabels;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RaceTracksPathPrinter {
}


impl RaceTracksPathPrinter {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn printPath(&mut self, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        let mut size: i32 = jcall!(list).size();
        for index in (0..size) {
            let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(list).get(index.clone());
            PreLogUtil::put(jcall!(StringMaker::new()).append(String::from("pathList.add(cellPositionFactory.getInstance")).append({ let value = jcall!(anyType); value.as_ref().downcast_ref::<String>().cloned().unwrap_or_else(|| format!("{:?}", value.as_ref().type_id())) }).append(String::from(");")).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("printPath"));
        }
    }
    

    pub fn printPaths(&mut self, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        let mut size: i32 = jcall!(list).size();
        for index in (0..size) {
            println!("{}", jcall!(StringMaker::new()).append(String::from(" Path ")).appendint(index.clone()).append(jcall!(CommonLabels::getInstance()).COLON_SEP.clone()).toString());
            { let java_arguments = ({ let java_object = (jcall!(list).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>>().expect("ClassCastException").clone()) },); self.printPath(java_arguments.0) };
        }
    }
}
pub trait RaceTracksPathPrinterDyn {
    fn as_RaceTracksPathPrinter(&self) -> &RaceTracksPathPrinter;
    fn as_RaceTracksPathPrinter_mut(&mut self) -> &mut RaceTracksPathPrinter;
    fn printPath(&mut self, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { RaceTracksPathPrinter::printPath(self.as_RaceTracksPathPrinter_mut(), list) }
    fn printPaths(&mut self, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { RaceTracksPathPrinter::printPaths(self.as_RaceTracksPathPrinter_mut(), list) }
}
impl RaceTracksPathPrinterDyn for RaceTracksPathPrinter {
    fn as_RaceTracksPathPrinter(&self) -> &RaceTracksPathPrinter { self }
    fn as_RaceTracksPathPrinter_mut(&mut self) -> &mut RaceTracksPathPrinter { self }
}


