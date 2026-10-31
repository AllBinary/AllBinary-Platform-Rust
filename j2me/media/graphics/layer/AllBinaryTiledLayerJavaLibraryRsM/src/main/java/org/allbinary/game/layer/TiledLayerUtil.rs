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
// package org.allbinary.game.layer
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::AllBinaryTiledLayer::AllBinaryTiledLayer;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TiledLayerUtil {
}

thread_local! { pub static TiledLayerUtil_instance: Option<Rc<RefCell<TiledLayerUtil>>> = TiledLayerUtil::new(); }

impl TiledLayerUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<TiledLayerUtil>>> {
        return TiledLayerUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn keepOnBorderdMapX(&mut self, mut terrainTiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>>, mut x: i32, mut width: i32, mut mapBorderBufferWidth: i32) -> i32 {
        let mut newX: i32 = x.clone();
        let mut maxRight: i32 = jcall!(terrainTiledLayer).getWidth() - width - mapBorderBufferWidth;
        if x + width > maxRight {
            newX = maxRight.clone();
        }
        if x < mapBorderBufferWidth {
            newX = mapBorderBufferWidth.clone();
        }
        return newX.clone();
    }
    

    pub fn keepOnBorderdMapY(&mut self, mut terrainTiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>>, mut y: i32, mut height: i32, mut mapBorderBufferHeight: i32) -> i32 {
        let mut newY: i32 = y.clone();
        let mut maxBottom: i32 = jcall!(terrainTiledLayer).getHeight() - height - mapBorderBufferHeight;
        if y + height > maxBottom {
            newY = maxBottom.clone();
        }
        if y < mapBorderBufferHeight {
            newY = mapBorderBufferHeight.clone();
        }
        return newY.clone();
    }
    

    pub fn keepOnMapX(&mut self, mut terrainTiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>>, mut x: i32, mut width: i32) -> i32 {
        let mut newX: i32 = x.clone();
        let mut maxRight: i32 = jcall!(terrainTiledLayer).getWidth() - width;
        if x >= maxRight {
            newX = maxRight.clone();
        }
        if x < 0 {
            newX = 0;
        }
        return newX.clone();
    }
    

    pub fn keepOnMapY(&mut self, mut terrainTiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>>, mut y: i32, mut height: i32) -> i32 {
        let mut newY: i32 = y.clone();
        let mut maxBottom: i32 = jcall!(terrainTiledLayer).getHeight() - height;
        if y >= maxBottom {
            newY = maxBottom.clone();
        }
        if y < 0 {
            newY = 0;
        }
        return newY.clone();
    }
}

