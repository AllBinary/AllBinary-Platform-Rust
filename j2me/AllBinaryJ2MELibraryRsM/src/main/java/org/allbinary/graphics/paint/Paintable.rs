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
// package org.allbinary.graphics.paint
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use super::PaintableInterface::PaintableInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct Paintable {
}

impl PaintableInterface for Paintable {
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
    fn paintThreed(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paintThreed(self, graphics) }
}

impl Paintable {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn setBasicColorP(&mut self, mut basicColor: Option<Rc<RefCell<BasicColor>>>) {
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
    }
    

    pub fn paintThreed(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
    }
}
pub trait PaintableDyn {
    fn as_Paintable(&self) -> &Paintable;
    fn as_Paintable_mut(&mut self) -> &mut Paintable;
    fn setBasicColorP(&mut self, mut basicColor: Option<Rc<RefCell<BasicColor>>>) { Paintable::setBasicColorP(self.as_Paintable_mut(), basicColor) }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Paintable::paint(self.as_Paintable_mut(), graphics) }
    fn paintThreed(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Paintable::paintThreed(self.as_Paintable_mut(), graphics) }
}
impl PaintableDyn for Paintable {
    fn as_Paintable(&self) -> &Paintable { self }
    fn as_Paintable_mut(&mut self) -> &mut Paintable { self }
}


