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
// package org.allbinary.game.view
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayer;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::layer::Layer::Layer;
use crate::org::allbinary::layer::NullLayer::NullLayer;
use crate::org::allbinary::view::ViewPosition::ViewPosition;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct StaticTileLayerIntoPositionViewPosition {
    pub base: ViewPosition,
}

pub static mut StaticTileLayerIntoPositionViewPosition_tiledLayer: std::cell::LazyCell<Option<Rc<RefCell<dyn crate::org::allbinary::layer::Layer::LayerDyn>>>> = std::cell::LazyCell::new(|| NullLayer::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::layer::Layer::LayerDyn>>));
pub static mut StaticTileLayerIntoPositionViewPosition_layer: std::cell::LazyCell<Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>> = std::cell::LazyCell::new(|| crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer_NULL_ALLBINARY_LAYER.with(Clone::clone).clone());

impl StaticTileLayerIntoPositionViewPosition {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: ViewPosition::new_struct(0, 0, 0),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getX(&mut self) -> i32 {
        return self.base.getX() - jcall!(unsafe { StaticTileLayerIntoPositionViewPosition_tiledLayer.clone() }).getXP();
    }
    

    pub fn getY(&mut self) -> i32 {
        return self.base.getY() - jcall!(unsafe { StaticTileLayerIntoPositionViewPosition_tiledLayer.clone() }).getYP();
    }
    

    pub fn getZ(&mut self) -> i32 {
        return self.base.getZ() - jcall!(unsafe { StaticTileLayerIntoPositionViewPosition_tiledLayer.clone() }).getZP();
    }
    

    pub fn setTiledLayer(mut tiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>>) {
        unsafe { *StaticTileLayerIntoPositionViewPosition_tiledLayer = tiledLayer.clone() };
    }
}
impl std::ops::Deref for StaticTileLayerIntoPositionViewPosition {
    type Target = ViewPosition;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for StaticTileLayerIntoPositionViewPosition {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::view::ViewPosition::ViewPositionDyn for StaticTileLayerIntoPositionViewPosition {
    fn as_ViewPosition(&self) -> &crate::org::allbinary::view::ViewPosition::ViewPosition { &self.base }
    fn as_ViewPosition_mut(&mut self) -> &mut crate::org::allbinary::view::ViewPosition::ViewPosition { &mut self.base }
}
impl crate::org::allbinary::view::ViewPositionBase::ViewPositionBaseDyn for StaticTileLayerIntoPositionViewPosition {
    fn as_ViewPositionBase(&self) -> &crate::org::allbinary::view::ViewPositionBase::ViewPositionBase { &self.base.base }
    fn as_ViewPositionBase_mut(&mut self) -> &mut crate::org::allbinary::view::ViewPositionBase::ViewPositionBase { &mut self.base.base }
    fn getX2(&mut self) -> i32 { crate::org::allbinary::view::ViewPosition::ViewPosition::getX2(crate::org::allbinary::view::ViewPosition::ViewPositionDyn::as_ViewPosition_mut(self)) }
    fn getY2(&mut self) -> i32 { crate::org::allbinary::view::ViewPosition::ViewPosition::getY2(crate::org::allbinary::view::ViewPosition::ViewPositionDyn::as_ViewPosition_mut(self)) }
    fn getZ2(&mut self) -> i32 { crate::org::allbinary::view::ViewPosition::ViewPosition::getZ2(crate::org::allbinary::view::ViewPosition::ViewPositionDyn::as_ViewPosition_mut(self)) }
    fn setAllbinaryLayer(&mut self, p1: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) { crate::org::allbinary::view::ViewPosition::ViewPosition::setAllbinaryLayer(crate::org::allbinary::view::ViewPosition::ViewPositionDyn::as_ViewPosition_mut(self), p1) }
}
impl crate::org::allbinary::graphics::GPoint::GPointDyn for StaticTileLayerIntoPositionViewPosition {
    fn as_GPoint(&self) -> &crate::org::allbinary::graphics::GPoint::GPoint { &self.base.base.base }
    fn as_GPoint_mut(&mut self) -> &mut crate::org::allbinary::graphics::GPoint::GPoint { &mut self.base.base.base }
    fn getX(&mut self) -> i32 { Self::getX(self) }
    fn getY(&mut self) -> i32 { Self::getY(self) }
    fn getZ(&mut self) -> i32 { Self::getZ(self) }
}


