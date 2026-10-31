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
use super::ColorFillPaintable::ColorFillPaintable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct OverScanColorFillPaintable {
    pub base: ColorFillPaintable,
}


impl OverScanColorFillPaintable {
    

    pub fn new_struct(mut basicColor: Option<Rc<RefCell<BasicColor>>>) -> Self {
        let mut this = Self {
            base: ColorFillPaintable::new_struct(basicColor.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut basicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(basicColor))))
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        jcall!(graphics).setColor(self.colorP.clone());
        jcall!(graphics).fillRect(-jcall!(graphics).getClipX(), -jcall!(graphics).getClipY(), jcall!(self.displayInfoSingleton).getLastWidth() + (jcall!(graphics).getClipX() * 2), jcall!(self.displayInfoSingleton).getLastHeight() + (jcall!(graphics).getClipY() * 2));
    }
}
impl std::ops::Deref for OverScanColorFillPaintable {
    type Target = ColorFillPaintable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for OverScanColorFillPaintable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::paint::ColorFillBasePaintable::ColorFillBasePaintableDyn for OverScanColorFillPaintable {
    fn as_ColorFillBasePaintable(&self) -> &crate::org::allbinary::game::paint::ColorFillBasePaintable::ColorFillBasePaintable { &self.base.base }
    fn as_ColorFillBasePaintable_mut(&mut self) -> &mut crate::org::allbinary::game::paint::ColorFillBasePaintable::ColorFillBasePaintable { &mut self.base.base }
}
impl crate::org::allbinary::game::paint::ColorFillPaintable::ColorFillPaintableDyn for OverScanColorFillPaintable {
    fn as_ColorFillPaintable(&self) -> &crate::org::allbinary::game::paint::ColorFillPaintable::ColorFillPaintable { &self.base }
    fn as_ColorFillPaintable_mut(&mut self) -> &mut crate::org::allbinary::game::paint::ColorFillPaintable::ColorFillPaintable { &mut self.base }
}
impl crate::org::allbinary::graphics::paint::Paintable::PaintableDyn for OverScanColorFillPaintable {
    fn as_Paintable(&self) -> &crate::org::allbinary::graphics::paint::Paintable::Paintable { &self.base.base.base }
    fn as_Paintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::Paintable::Paintable { &mut self.base.base.base }
    fn setBasicColorP(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::graphics::color::BasicColor::BasicColor>>>) { crate::org::allbinary::game::paint::ColorFillBasePaintable::ColorFillBasePaintable::setBasicColorP(crate::org::allbinary::game::paint::ColorFillBasePaintable::ColorFillBasePaintableDyn::as_ColorFillBasePaintable_mut(self), p1) }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
}


