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
// package org.allbinary.game.paint
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::graphics::color::ColorCompositeInterface::ColorCompositeInterface;
use crate::org::allbinary::graphics::paint::Paintable::Paintable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ColorFillBasePaintable {
    pub base: Paintable,
    pub basicColor: Option<Rc<RefCell<BasicColor>>>,
    pub colorP: i32,
}

impl ColorCompositeInterface for ColorFillBasePaintable {
    fn getBasicColorP(&mut self) -> Option<Rc<RefCell<BasicColor>>> { Self::getBasicColorP(self) }
}
impl crate::org::allbinary::graphics::color::BasicColorCompositeSetterInterface::BasicColorCompositeSetterInterface for ColorFillBasePaintable {
    fn setBasicColorP(&mut self, mut basicColor: Option<Rc<RefCell<BasicColor>>>) { Self::setBasicColorP(self, basicColor) }
}

impl ColorFillBasePaintable {
    

    pub fn new_struct(mut basicColor: Option<Rc<RefCell<BasicColor>>>) -> Self {
        let mut this = Self {
            base: Paintable::new_struct(),
            basicColor: None,
            colorP: 0,
        };
        this.basicColor = jcall!(BasicColorFactory::getInstance()).WHITE.clone();
        this.setBasicColorP(basicColor.clone());
        this
    }

    pub fn new(mut basicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(basicColor))))
    }
    

    pub fn setBasicColorP(&mut self, mut basicColor: Option<Rc<RefCell<BasicColor>>>) {
        self.basicColor = basicColor.clone();
        { let java_arguments = (((self.getBasicColorP()).unwrap() as i32),); self.setColor(java_arguments.0) };
    }
    

    pub fn getBasicColorP(&mut self) -> Option<Rc<RefCell<BasicColor>>> {
        return self.basicColor.clone();
    }
    

    pub fn getColor(&mut self) -> i32 {
        return self.colorP.clone();
    }
    

    pub fn setColor(&mut self, mut color: i32) {
        self.colorP = color.clone();
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
    }
}
impl std::ops::Deref for ColorFillBasePaintable {
    type Target = Paintable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ColorFillBasePaintable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait ColorFillBasePaintableDyn: crate::org::allbinary::graphics::paint::Paintable::PaintableDyn {
    fn as_ColorFillBasePaintable(&self) -> &ColorFillBasePaintable;
    fn as_ColorFillBasePaintable_mut(&mut self) -> &mut ColorFillBasePaintable;
    fn getBasicColorP(&mut self) -> Option<Rc<RefCell<BasicColor>>> { ColorFillBasePaintable::getBasicColorP(self.as_ColorFillBasePaintable_mut()) }
    fn getColor(&mut self) -> i32 { ColorFillBasePaintable::getColor(self.as_ColorFillBasePaintable_mut()) }
    fn setColor(&mut self, mut color: i32) { ColorFillBasePaintable::setColor(self.as_ColorFillBasePaintable_mut(), color) }
}
impl ColorFillBasePaintableDyn for ColorFillBasePaintable {
    fn as_ColorFillBasePaintable(&self) -> &ColorFillBasePaintable { self }
    fn as_ColorFillBasePaintable_mut(&mut self) -> &mut ColorFillBasePaintable { self }
}

impl crate::org::allbinary::graphics::paint::Paintable::PaintableDyn for ColorFillBasePaintable {
    fn as_Paintable(&self) -> &crate::org::allbinary::graphics::paint::Paintable::Paintable { &self.base }
    fn as_Paintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::Paintable::Paintable { &mut self.base }
    fn setBasicColorP(&mut self, mut basicColor: Option<Rc<RefCell<BasicColor>>>) { Self::setBasicColorP(self, basicColor) }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
}


