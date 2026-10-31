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
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::graphics::paint::Paintable::Paintable;
use super::PaintableForm::PaintableForm;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct FormPaintable {
    pub base: Paintable,
    pub form: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::PaintableForm::PaintableFormDyn>>>,
}


impl FormPaintable {
    

    pub fn new_struct(mut form: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::PaintableForm::PaintableFormDyn>>>) -> Self {
        let mut this = Self {
            base: Paintable::new_struct(),
            form: None,
        };
        this.form = form.clone();
        this
    }

    pub fn new(mut form: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::PaintableForm::PaintableFormDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(form))))
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        jcall!(self.form).paint(graphics.clone());
    }
    

    pub fn getForm(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::PaintableForm::PaintableFormDyn>>> {
        return self.form.clone();
    }
}
impl std::ops::Deref for FormPaintable {
    type Target = Paintable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for FormPaintable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::paint::Paintable::PaintableDyn for FormPaintable {
    fn as_Paintable(&self) -> &crate::org::allbinary::graphics::paint::Paintable::Paintable { &self.base }
    fn as_Paintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::Paintable::Paintable { &mut self.base }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
}


