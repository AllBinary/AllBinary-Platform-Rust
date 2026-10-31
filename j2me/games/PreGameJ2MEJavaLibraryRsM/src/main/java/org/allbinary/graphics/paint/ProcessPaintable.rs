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
use super::Paintable::Paintable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ProcessPaintable {
    pub base: Paintable,
}


impl ProcessPaintable {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: Paintable::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn process(&mut self) {
    }
}
impl std::ops::Deref for ProcessPaintable {
    type Target = Paintable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ProcessPaintable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait ProcessPaintableDyn: crate::org::allbinary::graphics::paint::Paintable::PaintableDyn {
    fn as_ProcessPaintable(&self) -> &ProcessPaintable;
    fn as_ProcessPaintable_mut(&mut self) -> &mut ProcessPaintable;
    fn process(&mut self) { ProcessPaintable::process(self.as_ProcessPaintable_mut()) }
}
impl ProcessPaintableDyn for ProcessPaintable {
    fn as_ProcessPaintable(&self) -> &ProcessPaintable { self }
    fn as_ProcessPaintable_mut(&mut self) -> &mut ProcessPaintable { self }
}

impl crate::org::allbinary::graphics::paint::Paintable::PaintableDyn for ProcessPaintable {
    fn as_Paintable(&self) -> &crate::org::allbinary::graphics::paint::Paintable::Paintable { &self.base }
    fn as_Paintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::Paintable::Paintable { &mut self.base }
}


