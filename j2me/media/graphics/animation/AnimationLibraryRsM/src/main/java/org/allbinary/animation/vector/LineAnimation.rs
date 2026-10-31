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
// package org.allbinary.animation.vector
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::animation::Animation::Animation;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::color::ColorCompositeInterface::ColorCompositeInterface;
use crate::org::allbinary::layer::PositionInterface::PositionInterface;
use crate::org::allbinary::logic::NullUtil::NullUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LineAnimation {
    pub base: Animation,
    pub newLine: Vec<Vec<i32>>,
    pub tempLine: Vec<Vec<i32>>,
    pub line: Vec<Vec<i32>>,
}

// impl ColorCompositeInterface for LineAnimation with the methods of the trait
impl PositionInterface for LineAnimation {
    fn setPosition(&mut self, mut x: i32, mut y: i32, mut z: i32) { Self::setPosition(self, x, y, z) }
}
impl crate::org::allbinary::graphics::paint::LocationPaintableInterface::LocationPaintableInterface for LineAnimation {
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}

impl LineAnimation {
    

    pub fn new_struct(mut basicColor: Option<Rc<RefCell<BasicColor>>>) -> Self {
        let mut this = Self {
            base: Animation::new_struct(),
            newLine: Vec::new(),
            tempLine: Vec::new(),
            line: Vec::new(),
        };
        this.newLine = vec![vec![0; (2) as usize]; (2) as usize];
        this.tempLine = jcall!(NullUtil::getInstance()).NULL_INT_ARRAY_ARRAY.clone();
        this.line = vec![vec![0; (2) as usize]; (2) as usize];
        this.setBasicColorP(basicColor.clone());
        this
    }

    pub fn new(mut basicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(basicColor))))
    }
    

    pub fn init(&mut self, mut x: i32, mut y: i32) {
        self.newLine[(0) as usize][(0) as usize] = x.clone();
        self.newLine[(0) as usize][(1) as usize] = y.clone();
        self.newLine[(1) as usize][(0) as usize] = x.clone();
        self.newLine[(1) as usize][(1) as usize] = y.clone();
        self.tempLine = self.line.clone();
        self.line = self.newLine.clone();
        self.newLine = self.tempLine.clone();
    }
    

    pub fn setPosition(&mut self, mut x: i32, mut y: i32, mut z: i32) {
        self.newLine[(0) as usize][(0) as usize] = self.line[(1) as usize][(0) as usize].clone();
        self.newLine[(0) as usize][(1) as usize] = self.line[(1) as usize][(1) as usize].clone();
        self.newLine[(1) as usize][(0) as usize] = x.clone();
        self.newLine[(1) as usize][(1) as usize] = y.clone();
        self.tempLine = self.line.clone();
        self.line = self.newLine.clone();
        self.newLine = self.tempLine.clone();
    }
    

    pub fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) {
        let mut currentLine: Vec<Vec<i32>> = self.line.clone();
        { let java_receiver = self.basicSetColorUtil.clone(); let java_arguments = (graphics.clone(), self.getBasicColorP(), self.getColor(),); jcall!(java_receiver).setBasicColorP3(java_arguments.0, java_arguments.1, java_arguments.2); };
        jcall!(graphics).drawLine(currentLine[(0) as usize][(0) as usize].clone(), currentLine[(0) as usize][(1) as usize].clone(), currentLine[(1) as usize][(0) as usize].clone(), currentLine[(1) as usize][(1) as usize].clone());
    }
    

    pub fn nextFrame(&mut self) {
    }
}
impl std::ops::Deref for LineAnimation {
    type Target = Animation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for LineAnimation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::animation::Animation::AnimationDyn for LineAnimation {
    fn as_Animation(&self) -> &crate::org::allbinary::animation::Animation::Animation { &self.base }
    fn as_Animation_mut(&mut self) -> &mut crate::org::allbinary::animation::Animation::Animation { &mut self.base }
    fn nextFrame(&mut self) { Self::nextFrame(self) }
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for LineAnimation {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::animation::Animation::Animation::set(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self), p1) }
}


