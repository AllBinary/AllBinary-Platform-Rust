// 
// AllBinary Open License Version 1 
// Copyright (c) 2006 AllBinary 
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
// package org.allbinary.game.layer.waypoint
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::graphics::CellPosition::CellPosition;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct WaypointCellPositionHistory {
    pub positionList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub layerList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}

thread_local! { pub static WaypointCellPositionHistory_SINGLETON: Option<Rc<RefCell<WaypointCellPositionHistory>>> = WaypointCellPositionHistory::new(); }

impl WaypointCellPositionHistory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            positionList: None,
            layerList: None,
        };
        this.positionList = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.layerList = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<WaypointCellPositionHistory>>> {
        return WaypointCellPositionHistory_SINGLETON.with(Clone::clone).clone();
    }
    

    pub fn add(&mut self, mut cellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>>, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) {
        jcall!(self.positionList).add(crate::java_into_object(cellPosition.clone()));
        jcall!(self.layerList).add(crate::java_into_object(layerInterface.clone()));
    }
    

    pub fn addList(&mut self, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) {
        for index in (0..=jcall!(list).size() - 1).rev() {
            { let java_arguments = ({ let java_object = (jcall!(list).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>>>().expect("ClassCastException").clone()) }, layerInterface.clone(),); self.add(java_arguments.0, java_arguments.1) };
        }
    }
    

    pub fn remove(&mut self, mut cellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>>) {
        let mut index: i32 = jcall!(self.positionList).indexOf(crate::java_into_object(cellPosition.clone()));
        if index >= 0 {
            jcall!(self.positionList).removeAt(index.clone());
            jcall!(self.layerList).removeAt(index.clone());
        }
    }
    

    pub fn removeLayer(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) {
        let mut index: i32 = 0;
        while index != -1 {
            index = jcall!(self.layerList).indexOf(crate::java_into_object(layerInterface.clone()));
            if index >= 0 {
                jcall!(self.positionList).removeAt(index.clone());
                jcall!(self.layerList).removeAt(index.clone());
            }
        }
    }
    

    pub fn clear(&mut self) {
        jcall!(self.positionList).clear();
        jcall!(self.layerList).clear();
    }
    

    pub fn isCellPositionWithDrop(&mut self, mut cellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>>) -> bool {
        return jcall!(self.positionList).contains(crate::java_into_object(cellPosition.clone()));
    }
    

    pub fn anyCellPositionWithDrop(&mut self, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> bool {
        for index in (0..=jcall!(list).size() - 1).rev() {
            if { let java_arguments = ({ let java_object = (jcall!(list).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>>>().expect("ClassCastException").clone()) },); self.isCellPositionWithDrop(java_arguments.0) } {
                return true;
            }
        }
        return false;
    }
    

    pub fn getLayerInterface(&mut self, mut cellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> {
        let mut index: i32 = jcall!(self.positionList).indexOf(crate::java_into_object(cellPosition.clone()));
        if index >= 0 {
            return { let java_object = (jcall!(self.layerList).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>>().expect("ClassCastException").clone()) };
        } else {
            return crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer_NULL_ALLBINARY_LAYER.with(Clone::clone).clone();
        }
    }
}

