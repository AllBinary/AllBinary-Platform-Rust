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
use super::InitUpdatePaintableInterface::InitUpdatePaintableInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InitUpdatePaintable {
    pub base: Paintable,
}

// impl InitUpdatePaintableInterface for InitUpdatePaintable with the methods of the trait

impl InitUpdatePaintable {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: Paintable::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn init(&mut self) {
    }
    

    pub fn update(&mut self) {
    }
}
impl std::ops::Deref for InitUpdatePaintable {
    type Target = Paintable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for InitUpdatePaintable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait InitUpdatePaintableDyn: crate::org::allbinary::graphics::paint::Paintable::PaintableDyn {
    fn as_InitUpdatePaintable(&self) -> &InitUpdatePaintable;
    fn as_InitUpdatePaintable_mut(&mut self) -> &mut InitUpdatePaintable;
    fn init(&mut self) { InitUpdatePaintable::init(self.as_InitUpdatePaintable_mut()) }
    fn update(&mut self) { InitUpdatePaintable::update(self.as_InitUpdatePaintable_mut()) }
}
impl InitUpdatePaintableDyn for InitUpdatePaintable {
    fn as_InitUpdatePaintable(&self) -> &InitUpdatePaintable { self }
    fn as_InitUpdatePaintable_mut(&mut self) -> &mut InitUpdatePaintable { self }
}

impl crate::org::allbinary::graphics::paint::Paintable::PaintableDyn for InitUpdatePaintable {
    fn as_Paintable(&self) -> &crate::org::allbinary::graphics::paint::Paintable::Paintable { &self.base }
    fn as_Paintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::Paintable::Paintable { &mut self.base }
}


