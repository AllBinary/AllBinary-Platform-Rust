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
// package org.allbinary.graphics.form
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::graphics::paint::Paintable::Paintable;
use super::MultipleScrollSelectionHorizontalForm::MultipleScrollSelectionHorizontalForm;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MultipleScrollSelectionFormRectPaintable {
    pub base: Paintable,
    pub multipleScrollSelectionForm: Option<Rc<RefCell<MultipleScrollSelectionHorizontalForm>>>,
}


impl MultipleScrollSelectionFormRectPaintable {
    

    pub fn new_struct(mut multipleScrollSelectionForm: Option<Rc<RefCell<MultipleScrollSelectionHorizontalForm>>>) -> Self {
        let mut this = Self {
            base: Paintable::new_struct(),
            multipleScrollSelectionForm: None,
        };
        this.multipleScrollSelectionForm = multipleScrollSelectionForm.clone();
        this
    }

    pub fn new(mut multipleScrollSelectionForm: Option<Rc<RefCell<MultipleScrollSelectionHorizontalForm>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(multipleScrollSelectionForm))))
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
        jcall!(self.multipleScrollSelectionForm).fillRect(graphics.clone());
    }
}
impl std::ops::Deref for MultipleScrollSelectionFormRectPaintable {
    type Target = Paintable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for MultipleScrollSelectionFormRectPaintable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::paint::Paintable::PaintableDyn for MultipleScrollSelectionFormRectPaintable {
    fn as_Paintable(&self) -> &crate::org::allbinary::graphics::paint::Paintable::Paintable { &self.base }
    fn as_Paintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::Paintable::Paintable { &mut self.base }
}
impl crate::org::allbinary::graphics::paint::PaintableInterface::PaintableInterface for MultipleScrollSelectionFormRectPaintable {
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::paint(self, graphics) }
    fn paintThreed(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { crate::org::allbinary::graphics::paint::Paintable::Paintable::paintThreed(crate::org::allbinary::graphics::paint::Paintable::PaintableDyn::as_Paintable_mut(self), p1) }
}


