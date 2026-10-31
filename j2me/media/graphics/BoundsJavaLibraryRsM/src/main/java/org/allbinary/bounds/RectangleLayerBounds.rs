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
// package org.allbinary.bounds
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::graphics::Rectangle::Rectangle;
use super::LayerBounds::LayerBounds;
use super::BoundsVisitorInterface::BoundsVisitorInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RectangleLayerBounds {
    pub base: LayerBounds,
}


impl RectangleLayerBounds {
    

    pub fn new_struct(mut rectangle: Option<Rc<RefCell<Rectangle>>>) -> Self {
        let mut this = Self {
            base: LayerBounds::new_struct(rectangle.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut rectangle: Option<Rc<RefCell<Rectangle>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(rectangle))))
    }
    

    pub fn visit(&mut self, mut boundsVisitorInterface: Option<Rc<RefCell<dyn BoundsVisitorInterface>>>) {
        if jcall!(boundsVisitorInterface).getX() > jcall!(self.rectangle).getMaxX() {
            jcall!(boundsVisitorInterface).maxX();
        }
        if jcall!(boundsVisitorInterface).getY() > jcall!(self.rectangle).getMaxY() {
            jcall!(boundsVisitorInterface).maxY();
        }
        if jcall!(boundsVisitorInterface).getX() < jcall!(jcall!(self.rectangle).getPoint()).getX() {
            jcall!(boundsVisitorInterface).minX();
        }
        if jcall!(boundsVisitorInterface).getY() < jcall!(jcall!(self.rectangle).getPoint()).getY() {
            jcall!(boundsVisitorInterface).minY();
        }
    }
}
impl std::ops::Deref for RectangleLayerBounds {
    type Target = LayerBounds;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for RectangleLayerBounds {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::bounds::LayerBounds::LayerBoundsDyn for RectangleLayerBounds {
    fn as_LayerBounds(&self) -> &crate::org::allbinary::bounds::LayerBounds::LayerBounds { &self.base }
    fn as_LayerBounds_mut(&mut self) -> &mut crate::org::allbinary::bounds::LayerBounds::LayerBounds { &mut self.base }
    fn visit(&mut self, mut boundsVisitorInterface: Option<Rc<RefCell<dyn BoundsVisitorInterface>>>) { Self::visit(self, boundsVisitorInterface) }
}


