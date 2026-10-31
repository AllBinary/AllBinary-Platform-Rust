// 
// AllBinary Open License Version 1 
// Copyright (c) 2022 AllBinary 
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
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::graphics::GPoint::GPoint;
use crate::org::allbinary::layer::Layer::Layer;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LinePathAnimation {
}

thread_local! { pub static LinePathAnimation_instance: Option<Rc<RefCell<dyn LinePathAnimationDyn>>> = LinePathAnimation::new().map(|reference| reference as Rc<RefCell<dyn LinePathAnimationDyn>>); }

impl LinePathAnimation {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn LinePathAnimationDyn>>> {
        return LinePathAnimation_instance.with(Clone::clone).clone();
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>, mut nextPoint: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>, mut tiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::Layer::LayerDyn>>>) {
        jcall!(graphics).drawLine(jcall!(nextPoint).getX(), jcall!(nextPoint).getY(), jcall!(point).getX(), jcall!(point).getY());
    }
}
pub trait LinePathAnimationDyn {
    fn as_LinePathAnimation(&self) -> &LinePathAnimation;
    fn as_LinePathAnimation_mut(&mut self) -> &mut LinePathAnimation;
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>, mut nextPoint: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>, mut tiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::Layer::LayerDyn>>>) { LinePathAnimation::paint(self.as_LinePathAnimation_mut(), graphics, point, nextPoint, tiledLayer) }
}
impl LinePathAnimationDyn for LinePathAnimation {
    fn as_LinePathAnimation(&self) -> &LinePathAnimation { self }
    fn as_LinePathAnimation_mut(&mut self) -> &mut LinePathAnimation { self }
}


