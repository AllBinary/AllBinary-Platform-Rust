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
// package org.allbinary.game.layer.geographic.map
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMap;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPosition;
use crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LayerPartialCellPositionsUtil {
}

thread_local! { pub static LayerPartialCellPositionsUtil_instance: Option<Rc<RefCell<LayerPartialCellPositionsUtil>>> = LayerPartialCellPositionsUtil::new(); }

impl LayerPartialCellPositionsUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn getAll(&mut self, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut reusableList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        jcall!(reusableList).clear();
        let mut topLeftGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = jcall!(geographicMapInterface).getCellPositionAtXYNoThrow(jcall!(layerInterface).getXP(), jcall!(layerInterface).getYP());
        let mut topRightGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = jcall!(geographicMapInterface).getCellPositionAtXYNoThrow(jcall!(layerInterface).getXP() + jcall!(layerInterface).getWidth(), jcall!(layerInterface).getYP());
        let mut bottomLeftGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = jcall!(geographicMapInterface).getCellPositionAtXYNoThrow(jcall!(layerInterface).getXP(), jcall!(layerInterface).getYP() + jcall!(layerInterface).getHeight());
        let mut bottomRightGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = jcall!(geographicMapInterface).getCellPositionAtXYNoThrow(jcall!(layerInterface).getXP() + jcall!(layerInterface).getWidth(), jcall!(layerInterface).getYP() + jcall!(layerInterface).getHeight());
        if !match (&topLeftGeographicMapCellPosition, &crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            jcall!(reusableList).add(crate::java_into_object(topLeftGeographicMapCellPosition.clone()));
        }
        if !match (&topRightGeographicMapCellPosition, &crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } && !match (&topLeftGeographicMapCellPosition, &topRightGeographicMapCellPosition) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            jcall!(reusableList).add(crate::java_into_object(topRightGeographicMapCellPosition.clone()));
        }
        if !match (&bottomLeftGeographicMapCellPosition, &crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } && !jcall!(reusableList).contains(crate::java_into_object(bottomLeftGeographicMapCellPosition.clone())) {
            jcall!(reusableList).add(crate::java_into_object(bottomLeftGeographicMapCellPosition.clone()));
        }
        if !match (&bottomRightGeographicMapCellPosition, &crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } && !jcall!(reusableList).contains(crate::java_into_object(bottomRightGeographicMapCellPosition.clone())) {
            jcall!(reusableList).add(crate::java_into_object(bottomRightGeographicMapCellPosition.clone()));
        }
        return reusableList.clone();
    }
    

    // throws Exception
    pub fn getAllDXY(&mut self, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut dx: i32, mut dy: i32, mut reusableList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        jcall!(reusableList).clear();
        let mut topLeftGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = jcall!(geographicMapInterface).getCellPositionAtXYNoThrow(jcall!(layerInterface).getXP() + dx, jcall!(layerInterface).getYP() + dy);
        let mut topRightGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = jcall!(geographicMapInterface).getCellPositionAtXYNoThrow(jcall!(layerInterface).getXP() + jcall!(layerInterface).getWidth() + dx, jcall!(layerInterface).getYP() + dy);
        let mut bottomLeftGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = jcall!(geographicMapInterface).getCellPositionAtXYNoThrow(jcall!(layerInterface).getXP() + dx, jcall!(layerInterface).getYP() + jcall!(layerInterface).getHeight() + dy);
        let mut bottomRightGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = jcall!(geographicMapInterface).getCellPositionAtXYNoThrow(jcall!(layerInterface).getXP() + jcall!(layerInterface).getWidth() + dx, jcall!(layerInterface).getYP() + jcall!(layerInterface).getHeight() + dy);
        if !match (&topLeftGeographicMapCellPosition, &crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            jcall!(reusableList).add(crate::java_into_object(topLeftGeographicMapCellPosition.clone()));
        }
        if !match (&topRightGeographicMapCellPosition, &crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } && !match (&topLeftGeographicMapCellPosition, &topRightGeographicMapCellPosition) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            jcall!(reusableList).add(crate::java_into_object(topRightGeographicMapCellPosition.clone()));
        }
        if !match (&bottomLeftGeographicMapCellPosition, &crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } && !jcall!(reusableList).contains(crate::java_into_object(bottomLeftGeographicMapCellPosition.clone())) {
            jcall!(reusableList).add(crate::java_into_object(bottomLeftGeographicMapCellPosition.clone()));
        }
        if !match (&bottomRightGeographicMapCellPosition, &crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } && !jcall!(reusableList).contains(crate::java_into_object(bottomRightGeographicMapCellPosition.clone())) {
            jcall!(reusableList).add(crate::java_into_object(bottomRightGeographicMapCellPosition.clone()));
        }
        return reusableList.clone();
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<LayerPartialCellPositionsUtil>>> {
        return LayerPartialCellPositionsUtil_instance.with(Clone::clone).clone();
    }
}

