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
// package org.allbinary.graphics.canvas.transition.progress
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::graphics::paint::Paintable::Paintable;
use super::ProgressCanvas::ProgressCanvas;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ProgressPaintable {
    pub base: Paintable,
    pub progressCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvasDyn>>>,
}


impl ProgressPaintable {
    

    pub fn new_struct(mut progressCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvasDyn>>>) -> Self {
        let mut this = Self {
            base: Paintable::new_struct(),
            progressCanvas: None,
        };
        this.progressCanvas = progressCanvas.clone();
        this
    }

    pub fn new(mut progressCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvasDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(progressCanvas))))
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
        jcall!(self.progressCanvas).paint2(graphics.clone());
    }
}
impl std::ops::Deref for ProgressPaintable {
    type Target = Paintable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ProgressPaintable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::paint::Paintable::PaintableDyn for ProgressPaintable {
    fn as_Paintable(&self) -> &crate::org::allbinary::graphics::paint::Paintable::Paintable { &self.base }
    fn as_Paintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::Paintable::Paintable { &mut self.base }
}
impl crate::org::allbinary::graphics::paint::PaintableInterface::PaintableInterface for ProgressPaintable {
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::paint(self, graphics) }
    fn paintThreed(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { crate::org::allbinary::graphics::paint::Paintable::Paintable::paintThreed(crate::org::allbinary::graphics::paint::Paintable::PaintableDyn::as_Paintable_mut(self), p1) }
}


