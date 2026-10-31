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
// package org.allbinary.animation
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::khronos::opengles::GL::GL;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::graphics::color::BasicColorSetUtil::BasicColorSetUtil;
use crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface;
use super::AnimationInterface::AnimationInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct Animation {
    pub basicSetColorUtil: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::color::BasicColorSetUtil::BasicColorSetUtilDyn>>>,
    pub basicColor: Option<Rc<RefCell<BasicColor>>>,
    pub colorP: i32,
    pub changeBasicColorP: Option<Rc<RefCell<BasicColor>>>,
    pub changeColorP: i32,
    pub alphaP: i32,
}

impl AnimationInterface for Animation {
    fn nextFrame(&mut self) { Self::nextFrame(self) }
    fn paintThreedXYZ(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32, mut z: i32) { Self::paintThreedXYZ(self, graphics, x, y, z) }
}
impl crate::org::allbinary::graphics::paint::LocationPaintableInterface::LocationPaintableInterface for Animation {
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}
impl OpenGLSurfaceChangedInterface for Animation {
    fn set(&mut self, mut gl: Option<Rc<RefCell<dyn GL>>>) { Self::set(self, gl) }
}

impl Animation {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            basicSetColorUtil: None,
            basicColor: None,
            colorP: 0,
            changeBasicColorP: None,
            changeColorP: 0,
            alphaP: 0,
        };
        this.basicSetColorUtil = BasicColorSetUtil::getInstance();
        this.basicColor = jcall!(BasicColorFactory::getInstance()).WHITE.clone();
        this.changeBasicColorP = jcall!(BasicColorFactory::getInstance()).NULL_COLOR.clone();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn setAlpha(&mut self, mut alpha: i32) {
        self.alphaP = alpha.clone();
    }
    

    // throws Exception
    pub fn nextFrame(&mut self) {
    }
    

    pub fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) {
        { let java_receiver = self.basicSetColorUtil.clone(); let java_arguments = (graphics.clone(), self.getBasicColorP(), self.getColor(),); jcall!(java_receiver).setBasicColorP3(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn paintThreedXYZ(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32, mut z: i32) {
    }
    

    pub fn isThreed(&mut self) -> bool {
        return false;
    }
    

    pub fn getBasicColorP(&mut self) -> Option<Rc<RefCell<BasicColor>>> {
        return self.basicColor.clone();
    }
    

    pub fn setBasicColorP(&mut self, mut basicColor: Option<Rc<RefCell<BasicColor>>>) {
        self.basicColor = basicColor.clone();
        self.colorP = jcall!(self.basicColor).intValue();
    }
    

    pub fn setBackgroundBasicColorP(&mut self, mut basicColor: Option<Rc<RefCell<BasicColor>>>) {
    }
    

    pub fn getChangeBasicColor(&mut self) -> Option<Rc<RefCell<BasicColor>>> {
        return self.changeBasicColorP.clone();
    }
    

    pub fn changeBasicColor(&mut self, mut basicColor: Option<Rc<RefCell<BasicColor>>>) {
        self.changeBasicColorP = basicColor.clone();
        self.changeColorP = jcall!(self.changeBasicColorP).intValue();
    }
    

    pub fn getChangeColor(&mut self) -> i32 {
        return self.changeColorP.clone();
    }
    

    pub fn getColor(&mut self) -> i32 {
        return self.colorP.clone();
    }
    

    pub fn setScale(&mut self, mut scaleX: f32, mut scaleY: f32) {
    }
    

    pub fn setMaxScale(&mut self, mut maxScaleX: f32, mut maxScaleY: f32) {
    }
    

    // throws Exception
    pub fn set(&mut self, mut gl: Option<Rc<RefCell<dyn GL>>>) {
    }
    

    pub fn setDx(&mut self, mut dx: i32) {
    }
    

    pub fn getDx(&mut self) -> i32 {
        return 0;
    }
    

    pub fn setDy(&mut self, mut dy: i32) {
    }
    

    pub fn getDy(&mut self) -> i32 {
        return 0;
    }
}
pub trait AnimationDyn: crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface {
    fn as_Animation(&self) -> &Animation;
    fn as_Animation_mut(&mut self) -> &mut Animation;
    fn setAlpha(&mut self, mut alpha: i32) { Animation::setAlpha(self.as_Animation_mut(), alpha) }
    fn nextFrame(&mut self) { Animation::nextFrame(self.as_Animation_mut()) }
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Animation::paintXY(self.as_Animation_mut(), graphics, x, y) }
    fn paintThreedXYZ(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32, mut z: i32) { Animation::paintThreedXYZ(self.as_Animation_mut(), graphics, x, y, z) }
    fn isThreed(&mut self) -> bool { Animation::isThreed(self.as_Animation_mut()) }
    fn getBasicColorP(&mut self) -> Option<Rc<RefCell<BasicColor>>> { Animation::getBasicColorP(self.as_Animation_mut()) }
    fn setBasicColorP(&mut self, mut basicColor: Option<Rc<RefCell<BasicColor>>>) { Animation::setBasicColorP(self.as_Animation_mut(), basicColor) }
    fn setBackgroundBasicColorP(&mut self, mut basicColor: Option<Rc<RefCell<BasicColor>>>) { Animation::setBackgroundBasicColorP(self.as_Animation_mut(), basicColor) }
    fn getChangeBasicColor(&mut self) -> Option<Rc<RefCell<BasicColor>>> { Animation::getChangeBasicColor(self.as_Animation_mut()) }
    fn changeBasicColor(&mut self, mut basicColor: Option<Rc<RefCell<BasicColor>>>) { Animation::changeBasicColor(self.as_Animation_mut(), basicColor) }
    fn getChangeColor(&mut self) -> i32 { Animation::getChangeColor(self.as_Animation_mut()) }
    fn getColor(&mut self) -> i32 { Animation::getColor(self.as_Animation_mut()) }
    fn setScale(&mut self, mut scaleX: f32, mut scaleY: f32) { Animation::setScale(self.as_Animation_mut(), scaleX, scaleY) }
    fn setMaxScale(&mut self, mut maxScaleX: f32, mut maxScaleY: f32) { Animation::setMaxScale(self.as_Animation_mut(), maxScaleX, maxScaleY) }
    fn setDx(&mut self, mut dx: i32) { Animation::setDx(self.as_Animation_mut(), dx) }
    fn getDx(&mut self) -> i32 { Animation::getDx(self.as_Animation_mut()) }
    fn setDy(&mut self, mut dy: i32) { Animation::setDy(self.as_Animation_mut(), dy) }
    fn getDy(&mut self) -> i32 { Animation::getDy(self.as_Animation_mut()) }
}
impl AnimationDyn for Animation {
    fn as_Animation(&self) -> &Animation { self }
    fn as_Animation_mut(&mut self) -> &mut Animation { self }
}


