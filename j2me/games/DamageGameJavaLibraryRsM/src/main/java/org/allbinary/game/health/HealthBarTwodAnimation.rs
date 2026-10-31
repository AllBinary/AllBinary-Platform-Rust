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
// package org.allbinary.game.health
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use super::HealthBarAnimation::HealthBarAnimation;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct HealthBarTwodAnimation {
    pub base: HealthBarAnimation,
}

impl crate::org::allbinary::graphics::paint::LocationPaintableInterface::LocationPaintableInterface for HealthBarTwodAnimation {
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}

impl HealthBarTwodAnimation {
    

    pub fn new_struct(mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut location: i32) -> Self {
        let mut this = Self {
            base: HealthBarAnimation::new_struct(layerInterface.clone(), location.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut location: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(layerInterface, location))))
    }
    

    pub fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) {
        let mut ax: i32 = jcall!(jcall!(self.allbinaryLayer).getViewPosition()).getX();
        jcall!(self.basicSetColorUtil).setBasicColorP3(graphics.clone(), self.basicColor.clone(), self.colorP.clone());
        jcall!(graphics).fillRect(ax.clone(), self.getY() - 1, self.x2.clone(), self.thickness.clone());
    }
}
impl std::ops::Deref for HealthBarTwodAnimation {
    type Target = HealthBarAnimation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for HealthBarTwodAnimation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::health::HealthBarAnimation::HealthBarAnimationDyn for HealthBarTwodAnimation {
    fn as_HealthBarAnimation(&self) -> &crate::org::allbinary::game::health::HealthBarAnimation::HealthBarAnimation { &self.base }
    fn as_HealthBarAnimation_mut(&mut self) -> &mut crate::org::allbinary::game::health::HealthBarAnimation::HealthBarAnimation { &mut self.base }
}
impl crate::org::allbinary::animation::Animation::AnimationDyn for HealthBarTwodAnimation {
    fn as_Animation(&self) -> &crate::org::allbinary::animation::Animation::Animation { &self.base.base }
    fn as_Animation_mut(&mut self) -> &mut crate::org::allbinary::animation::Animation::Animation { &mut self.base.base }
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for HealthBarTwodAnimation {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::animation::Animation::Animation::set(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self), p1) }
}


