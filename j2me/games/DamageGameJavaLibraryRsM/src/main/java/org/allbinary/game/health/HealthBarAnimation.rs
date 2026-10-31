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
use crate::org::allbinary::animation::Animation::Animation;
use crate::org::allbinary::game::graphics::hud::BasicHudFactory::BasicHudFactory;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct HealthBarAnimation {
    pub base: Animation,
    pub basicColorFactory: Option<Rc<RefCell<BasicColorFactory>>>,
    pub x2: i32,
    pub thickness: i32,
    pub location: i32,
    pub allbinaryLayer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>,
}


impl HealthBarAnimation {
    

    pub fn new_struct(mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut location: i32) -> Self {
        let mut this = Self {
            base: Animation::new_struct(),
            basicColorFactory: None,
            x2: 0,
            thickness: 0,
            location: 0,
            allbinaryLayer: None,
        };
        this.basicColorFactory = BasicColorFactory::getInstance();
        this.allbinaryLayer = layerInterface.clone();
        this.location = location.clone();
        let mut basicHudFactory: Option<Rc<RefCell<BasicHudFactory>>> = BasicHudFactory::getInstance();
        if this.location != jcall!(basicHudFactory).TOPLEFT && this.location != jcall!(basicHudFactory).BOTTOMLEFT {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Location Not Valid"))));
        }
        if jcall!(layerInterface).getWidth() > 40 {
            this.thickness = 3;
        } else if jcall!(layerInterface).getWidth() > 20 {
            this.thickness = 2;
        } else {
            this.thickness = 1;
        }
        this
    }

    pub fn new(mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut location: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(layerInterface, location))))
    }
    

    pub fn onHealthChange(&mut self, mut newX2: i32) {
        self.x2 = newX2.clone();
        let mut quarter: i32 = (jcall!(self.allbinaryLayer).getWidth() >> 2);
        if self.x2 > quarter * 3 {
            self.basicColor = jcall!(self.basicColorFactory).GREEN.clone();
            self.colorP = jcall!(self.basicColor).intValue();
        } else if self.x2 > quarter * 2 {
            self.basicColor = jcall!(self.basicColorFactory).YELLOW.clone();
            self.colorP = jcall!(self.basicColor).intValue();
        } else if self.x2 > quarter {
            self.basicColor = jcall!(self.basicColorFactory).ORANGE.clone();
            self.colorP = jcall!(self.basicColor).intValue();
        } else {
            self.basicColor = jcall!(self.basicColorFactory).RED.clone();
            self.colorP = jcall!(self.basicColor).intValue();
        }
    }
    

    pub fn getY(&mut self) -> i32 {
        let mut basicHudFactory: Option<Rc<RefCell<BasicHudFactory>>> = BasicHudFactory::getInstance();
        if self.location == jcall!(basicHudFactory).TOPLEFT {
            return jcall!(jcall!(self.allbinaryLayer).getViewPosition()).getY() - 4;
        } else if self.location == jcall!(basicHudFactory).BOTTOMLEFT {
            return jcall!(jcall!(self.allbinaryLayer).getViewPosition()).getY2() + 4;
        }
        return -1;
    }
}
impl std::ops::Deref for HealthBarAnimation {
    type Target = Animation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for HealthBarAnimation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait HealthBarAnimationDyn: crate::org::allbinary::animation::Animation::AnimationDyn {
    fn as_HealthBarAnimation(&self) -> &HealthBarAnimation;
    fn as_HealthBarAnimation_mut(&mut self) -> &mut HealthBarAnimation;
    fn onHealthChange(&mut self, mut newX2: i32) { HealthBarAnimation::onHealthChange(self.as_HealthBarAnimation_mut(), newX2) }
    fn getY(&mut self) -> i32 { HealthBarAnimation::getY(self.as_HealthBarAnimation_mut()) }
}
impl HealthBarAnimationDyn for HealthBarAnimation {
    fn as_HealthBarAnimation(&self) -> &HealthBarAnimation { self }
    fn as_HealthBarAnimation_mut(&mut self) -> &mut HealthBarAnimation { self }
}

impl crate::org::allbinary::animation::Animation::AnimationDyn for HealthBarAnimation {
    fn as_Animation(&self) -> &crate::org::allbinary::animation::Animation::Animation { &self.base }
    fn as_Animation_mut(&mut self) -> &mut crate::org::allbinary::animation::Animation::Animation { &mut self.base }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for HealthBarAnimation {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::animation::Animation::Animation::set(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self), p1) }
}


