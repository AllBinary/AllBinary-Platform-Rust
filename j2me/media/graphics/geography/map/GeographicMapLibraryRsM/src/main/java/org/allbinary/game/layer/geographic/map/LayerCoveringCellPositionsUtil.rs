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
use crate::org::allbinary::media::graphics::geography::map::CellPositionsUtil::CellPositionsUtil;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPosition;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LayerCoveringCellPositionsUtil {
    pub cellPositionsUtil: Option<Rc<RefCell<CellPositionsUtil>>>,
}

thread_local! { pub static LayerCoveringCellPositionsUtil_instance: Option<Rc<RefCell<LayerCoveringCellPositionsUtil>>> = LayerCoveringCellPositionsUtil::new(); }

impl LayerCoveringCellPositionsUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            cellPositionsUtil: None,
        };
        this.cellPositionsUtil = CellPositionsUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<LayerCoveringCellPositionsUtil>>> {
        return LayerCoveringCellPositionsUtil_instance.with(Clone::clone).clone();
    }
    

    // throws Exception
    pub fn getAllXY(&mut self, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut x: i32, mut y: i32, mut reusableList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut topLeftGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = jcall!(geographicMapInterface).getCellPositionAtXY(x.clone(), y.clone());
        return self.getAll(geographicMapInterface.clone(), topLeftGeographicMapCellPosition.clone(), layerInterface.clone(), reusableList.clone());
    }
    

    // throws Exception
    pub fn getAll(&mut self, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>, mut topLeftGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut reusableList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        jcall!(reusableList).clear();
        let mut columns: i32 = jcall!(layerInterface).getWidth() / jcall!(jcall!(geographicMapInterface).getAllBinaryTiledLayer()).getCellWidth();
        let mut rows: i32 = jcall!(layerInterface).getHeight() / jcall!(jcall!(geographicMapInterface).getAllBinaryTiledLayer()).getCellHeight();
        if columns == 0 {
            columns = 1;
        }
        if rows == 0 {
            rows = 1;
        }
        return jcall!(self.cellPositionsUtil).getAll(geographicMapInterface.clone(), topLeftGeographicMapCellPosition.clone(), columns.clone(), rows.clone(), reusableList.clone());
    }
}

