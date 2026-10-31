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
use super::LinePathAnimation::LinePathAnimation;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LinePathRelativeAnimation {
    pub base: LinePathAnimation,
}

thread_local! { pub static LinePathRelativeAnimation_instanceC: Option<Rc<RefCell<LinePathRelativeAnimation>>> = LinePathRelativeAnimation::new(); }

impl LinePathRelativeAnimation {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: LinePathAnimation::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::LinePathAnimation::LinePathAnimationDyn>>> {
        return LinePathRelativeAnimation_instanceC.with(Clone::clone).clone();
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>, mut nextPoint: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>, mut tiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::Layer::LayerDyn>>>) {
        jcall!(graphics).drawLine(jcall!(nextPoint).getX() - jcall!(tiledLayer).getXP(), jcall!(nextPoint).getY() - jcall!(tiledLayer).getYP(), jcall!(point).getX() - jcall!(tiledLayer).getXP(), jcall!(point).getY() - jcall!(tiledLayer).getYP());
    }
}
impl std::ops::Deref for LinePathRelativeAnimation {
    type Target = LinePathAnimation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for LinePathRelativeAnimation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::layer::LinePathAnimation::LinePathAnimationDyn for LinePathRelativeAnimation {
    fn as_LinePathAnimation(&self) -> &crate::org::allbinary::game::layer::LinePathAnimation::LinePathAnimation { &self.base }
    fn as_LinePathAnimation_mut(&mut self) -> &mut crate::org::allbinary::game::layer::LinePathAnimation::LinePathAnimation { &mut self.base }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>, mut nextPoint: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>, mut tiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::Layer::LayerDyn>>>) { Self::paint(self, graphics, point, nextPoint, tiledLayer) }
}


