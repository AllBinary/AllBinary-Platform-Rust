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
use super::Paintable::Paintable;
use super::PaintableInterface::PaintableInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PaintableComposite {
    pub base: Paintable,
    pub paintableArray: Vec<Option<Rc<RefCell<dyn PaintableInterface>>>>,
}

impl PaintableInterface for PaintableComposite {
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
    fn paintThreed(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paintThreed(self, graphics) }
}

impl PaintableComposite {
    

    pub fn new_struct(mut paintableArray: Vec<Option<Rc<RefCell<dyn PaintableInterface>>>>) -> Self {
        let mut this = Self {
            base: Paintable::new_struct(),
            paintableArray: Vec::new(),
        };
        this.paintableArray = paintableArray.clone();
        this
    }

    pub fn new(mut paintableArray: Vec<Option<Rc<RefCell<dyn PaintableInterface>>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(paintableArray))))
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        for index in (0..=(self.paintableArray.len() as i32) - 1).rev() {
            jcall!(self.paintableArray[(index) as usize]).paint(graphics.clone());
        }
    }
    

    pub fn paintThreed(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        for index in (0..=(self.paintableArray.len() as i32) - 1).rev() {
            jcall!(self.paintableArray[(index) as usize]).paintThreed(graphics.clone());
        }
    }
}
impl std::ops::Deref for PaintableComposite {
    type Target = Paintable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for PaintableComposite {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::paint::Paintable::PaintableDyn for PaintableComposite {
    fn as_Paintable(&self) -> &crate::org::allbinary::graphics::paint::Paintable::Paintable { &self.base }
    fn as_Paintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::Paintable::Paintable { &mut self.base }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
    fn paintThreed(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paintThreed(self, graphics) }
}


