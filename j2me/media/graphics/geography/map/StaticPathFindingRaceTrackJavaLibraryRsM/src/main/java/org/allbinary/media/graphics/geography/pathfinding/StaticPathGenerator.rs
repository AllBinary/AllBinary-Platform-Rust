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
// package org.allbinary.media.graphics.geography.pathfinding
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::graphics::CellPosition::CellPosition;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::math::SmallIntegerSingletonFactory::SmallIntegerSingletonFactory;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMap;
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMapCellPositionFactory::BasicGeographicMapCellPositionFactory;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellHistory::GeographicMapCellHistory;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPosition;
use crate::org::allbinary::media::graphics::geography::map::PathData::PathData;
use crate::org::allbinary::util::BasicArrayListUtil::BasicArrayListUtil;
use super::PathGeneratorInterface::PathGeneratorInterface;
use super::PathCacheFactory::PathCacheFactory;
use super::PathFindingInfo::PathFindingInfo;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct StaticPathGenerator {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub basicArrayListUtil: Option<Rc<RefCell<BasicArrayListUtil>>>,
}

impl PathGeneratorInterface for StaticPathGenerator {
    fn createList(&mut self, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>, mut geographicMapCellHistory: Option<Rc<RefCell<GeographicMapCellHistory>>>, mut pathFindingInfo: Option<Rc<RefCell<PathFindingInfo>>>, mut totalPaths: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Self::createList(self, geographicMapInterface, geographicMapCellHistory, pathFindingInfo, totalPaths) }
}

impl StaticPathGenerator {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            basicArrayListUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.basicArrayListUtil = BasicArrayListUtil::getInstance();
        PreLogUtil::put(String::from("Using Static Path Finding"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(CommonStrings::getInstance()).CONSTRUCTOR.clone());
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn init(&mut self, mut geographicMapInterface: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut totalPaths: i32) {
    }
    

    // throws Exception
    pub fn getGeographicMapCellPositionListFromBasicGeographicMapCellPositionList(&mut self, mut geographicMapCellPositionFactory: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMapCellPositionFactory::BasicGeographicMapCellPositionFactoryDyn>>>, mut pathList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut size: i32 = jcall!(pathList).size();
        let mut basicGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>> = None;
        let mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = None;
        for index in (0..size) {
            basicGeographicMapCellPosition = { let java_object = (jcall!(pathList).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>>>().expect("ClassCastException").clone()) };
            geographicMapCellPosition = jcall!(geographicMapCellPositionFactory).getAt(jcall!(basicGeographicMapCellPosition).getColumn(), jcall!(basicGeographicMapCellPosition).getRow());
            jcall!(list).add(crate::java_into_object(geographicMapCellPosition.clone()));
        }
        return list.clone();
    }
    

    // throws Exception
    pub fn createList(&mut self, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>, mut geographicMapCellHistory: Option<Rc<RefCell<GeographicMapCellHistory>>>, mut pathFindingInfo: Option<Rc<RefCell<PathFindingInfo>>>, mut totalPaths: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut pathCacheFactory: Option<Rc<RefCell<PathCacheFactory>>> = PathCacheFactory::getInstance();
            let mut mapIdInteger: Option<i32> = jcall!(jcall!(geographicMapInterface).getAllBinaryTiledLayer()).getDataId();
            let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(pathCacheFactory).getOrCreate(mapIdInteger.clone());
            if match (&list, &jcall!(self.basicArrayListUtil).getImmutableInstance()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                list = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
                let mut smallIntegerSingletonFactory: Option<Rc<RefCell<SmallIntegerSingletonFactory>>> = SmallIntegerSingletonFactory::getInstance();
                let mut geographicMapCellPositionFactory: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMapCellPositionFactory::BasicGeographicMapCellPositionFactoryDyn>>> = jcall!(geographicMapInterface).getGeographicMapCellPositionFactory();
                let mut id: i32 = jcall!(PathData::getInstance()).OFFSET + ((mapIdInteger).unwrap() as i32);
                let mut basicList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(pathCacheFactory).getOrCreate(jcall!(smallIntegerSingletonFactory).getAt(id.clone()));
                let mut size: i32 = jcall!(basicList).size();
                let mut pathList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = None;
                for index in (0..size) {
                    pathList = { let java_arguments = (geographicMapCellPositionFactory.clone(), { let java_object = (jcall!(basicList).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>>().expect("ClassCastException").clone()) },); self.getGeographicMapCellPositionListFromBasicGeographicMapCellPositionList(java_arguments.0, java_arguments.1) };
                    jcall!(list).add(crate::java_into_object(pathList.clone()));
                }
                jcall!(pathCacheFactory).add(mapIdInteger.clone(), list.clone());
                jcall!(pathCacheFactory).remove(jcall!(smallIntegerSingletonFactory).getAt(id.clone()));
            }
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("Using Cached Path(s): ")).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(list.clone()))).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).GET_INSTANCE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            return list.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).GET_INSTANCE.clone(), crate::java_into_object(e.clone()));
            return BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            }
        }
    }
}

