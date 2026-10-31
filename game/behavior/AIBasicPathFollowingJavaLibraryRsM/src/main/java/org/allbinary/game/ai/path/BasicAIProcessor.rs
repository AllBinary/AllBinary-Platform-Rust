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
// package org.allbinary.game.ai.path
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::direction::Direction::Direction;
use crate::org::allbinary::direction::DirectionFactory::DirectionFactory;
use crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager;
use crate::org::allbinary::math::AngleInfo::AngleInfo;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellHistory::GeographicMapCellHistory;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPosition;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapDirectionUtil::GeographicMapDirectionUtil;
use crate::org::allbinary::media::graphics::geography::pathfinding::GeographicMapCellPathHistoryInfo::GeographicMapCellPathHistoryInfo;
use super::BasicDirectionAIHelper::BasicDirectionAIHelper;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicAIProcessor {
    pub name: String,
    pub geographicMapCellHistory: Option<Rc<RefCell<GeographicMapCellHistory>>>,
    pub geographicMapCellPositionBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub geographicMapCellPathHistoryInfo: Option<Rc<RefCell<GeographicMapCellPathHistoryInfo>>>,
    pub basicAI: Option<Rc<RefCell<BasicDirectionAIHelper>>>,
    pub geographicMapDirectionUtil: Option<Rc<RefCell<GeographicMapDirectionUtil>>>,
}


impl BasicAIProcessor {
    

    pub fn new_struct(mut name: String, mut geographicMapCellHistory: Option<Rc<RefCell<GeographicMapCellHistory>>>, mut geographicMapCellPathHistoryInfo: Option<Rc<RefCell<GeographicMapCellPathHistoryInfo>>>, mut chosenPathList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut angleInfo: Option<Rc<RefCell<AngleInfo>>>, mut seed: i32) -> Self {
        let mut this = Self {
            name: String::new(),
            geographicMapCellHistory: None,
            geographicMapCellPositionBasicArrayList: None,
            geographicMapCellPathHistoryInfo: None,
            basicAI: None,
            geographicMapDirectionUtil: None,
        };
        this.geographicMapDirectionUtil = GeographicMapDirectionUtil::getInstance();
        this.name = name.clone();
        this.geographicMapCellHistory = geographicMapCellHistory.clone();
        this.geographicMapCellPositionBasicArrayList = chosenPathList.clone();
        this.geographicMapCellPathHistoryInfo = geographicMapCellPathHistoryInfo.clone();
        this.setName(name.clone());
        this.setNewPath(geographicMapCellHistory.clone(), chosenPathList.clone());
        this.basicAI = BasicDirectionAIHelper::new(this.getName(), angleInfo.clone());
        this
    }

    pub fn new(mut name: String, mut geographicMapCellHistory: Option<Rc<RefCell<GeographicMapCellHistory>>>, mut geographicMapCellPathHistoryInfo: Option<Rc<RefCell<GeographicMapCellPathHistoryInfo>>>, mut chosenPathList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut angleInfo: Option<Rc<RefCell<AngleInfo>>>, mut seed: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(name, geographicMapCellHistory, geographicMapCellPathHistoryInfo, chosenPathList, angleInfo, seed))))
    }
    

    pub fn setNewPath(&mut self, mut geographicMapCellHistory: Option<Rc<RefCell<GeographicMapCellHistory>>>, mut chosenPathList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        self.setGeographicMapCellHistory(geographicMapCellHistory.clone());
        self.geographicMapCellPositionBasicArrayList = chosenPathList.clone();
        self.init();
    }
    

    pub fn init(&mut self) {
        self.geographicMapCellPathHistoryInfo = GeographicMapCellPathHistoryInfo::new();
        { let java_receiver = self.geographicMapCellPathHistoryInfo.clone(); let java_arguments = ({ let java_object = ({ let java_receiver = self.geographicMapCellPositionBasicArrayList.clone(); let java_arguments = (jcall!(self.geographicMapCellPositionBasicArrayList).size() - 1,); let java_result = jcall!(java_receiver).get(java_arguments.0); java_result }).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>>().expect("ClassCastException").clone()) },); jcall!(java_receiver).setPreviousOnPathGeographicMapCellPosition(java_arguments.0); };
        { let java_receiver = self.geographicMapCellPathHistoryInfo.clone(); let java_arguments = ({ let java_object = ({ let java_receiver = self.geographicMapCellPositionBasicArrayList.clone(); let java_arguments = (jcall!(self.geographicMapCellPositionBasicArrayList).size() - 1,); let java_result = jcall!(java_receiver).get(java_arguments.0); java_result }).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>>().expect("ClassCastException").clone()) },); jcall!(java_receiver).setNextOnPathGeographicMapCellPosition(java_arguments.0); };
        self.update();
    }
    

    // throws Exception
    pub fn goTowardNextChosenOnPathPosition(&mut self, mut currentGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> i32 {
        let mut keyDirection: i32 = -1;
        let mut goToGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = jcall!(self.geographicMapCellPathHistoryInfo).getNextChosenOnPathGeographicMapCellPosition();
        let mut geographicMapDirectionData: Option<Rc<RefCell<Direction>>> = jcall!(self.geographicMapDirectionUtil).getDirectionFromCellPositionToAdjacentCellPosition(currentGeographicMapCellPosition.clone(), goToGeographicMapCellPosition.clone());
        if match (&geographicMapDirectionData, &jcall!(DirectionFactory::getInstance()).NOT_BORDERED_WITH) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            geographicMapDirectionData = jcall!(self.geographicMapDirectionUtil).getDirectionFromCellPositionToCellPosition(currentGeographicMapCellPosition.clone(), goToGeographicMapCellPosition.clone());
        }
        keyDirection = jcall!(self.basicAI).getAIKeyPressedFromDirection(geographicMapDirectionData.clone());
        return keyDirection.clone();
    }
    

    pub fn update(&mut self) {
        { let java_receiver = self.geographicMapCellPathHistoryInfo.clone(); let java_arguments = (jcall!(self.getGeographicMapCellHistory()).getFirstUnvisited(),); jcall!(java_receiver).setNextUnvisitedOnPathGeographicMapCellPosition(java_arguments.0); };
        { let java_receiver = self.geographicMapCellPathHistoryInfo.clone(); let java_arguments = (jcall!(self.geographicMapCellPathHistoryInfo).getNextUnvisitedOnPathGeographicMapCellPosition(),); jcall!(java_receiver).setNextChosenOnPathGeographicMapCellPosition(java_arguments.0); };
    }
    

    // throws Exception
    pub fn processAI(&mut self, mut myManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut currentGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> i32 {
        let mut keyDirection: i32 = -1;
        if jcall!(self.geographicMapCellPositionBasicArrayList).contains(crate::java_into_object(currentGeographicMapCellPosition.clone())) {
            self.update();
            keyDirection = self.goTowardNextChosenOnPathPosition(currentGeographicMapCellPosition.clone());
            jcall!(self.geographicMapCellPathHistoryInfo).setPreviousOnPathGeographicMapCellPosition(currentGeographicMapCellPosition.clone());
        } else {
            keyDirection = self.goTowardNextChosenOnPathPosition(currentGeographicMapCellPosition.clone());
        }
        return keyDirection.clone();
    }
    

    pub fn getName(&mut self) -> String {
        return self.name.clone();
    }
    

    pub fn setName(&mut self, mut name: String) {
        self.name = name.clone();
    }
    

    pub fn getGeographicMapCellPathHistoryInfo(&mut self) -> Option<Rc<RefCell<GeographicMapCellPathHistoryInfo>>> {
        return self.geographicMapCellPathHistoryInfo.clone();
    }
    

    pub fn setGeographicMapCellPathHistoryInfo(&mut self, mut geographicMapCellPathHistoryInfo: Option<Rc<RefCell<GeographicMapCellPathHistoryInfo>>>) {
        self.geographicMapCellPathHistoryInfo = geographicMapCellPathHistoryInfo.clone();
    }
    

    pub fn getGeographicMapCellHistory(&mut self) -> Option<Rc<RefCell<GeographicMapCellHistory>>> {
        return self.geographicMapCellHistory.clone();
    }
    

    pub fn setGeographicMapCellHistory(&mut self, mut geographicMapCellHistory: Option<Rc<RefCell<GeographicMapCellHistory>>>) {
        self.geographicMapCellHistory = geographicMapCellHistory.clone();
    }
}

