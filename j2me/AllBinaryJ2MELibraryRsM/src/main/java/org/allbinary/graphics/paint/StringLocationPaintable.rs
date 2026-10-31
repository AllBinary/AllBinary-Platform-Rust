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
use super::LocationPaintable::LocationPaintable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct StringLocationPaintable {
    pub base: LocationPaintable,
    pub value: String,
}

impl crate::org::allbinary::graphics::paint::LocationPaintableInterface::LocationPaintableInterface for StringLocationPaintable {
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}

impl StringLocationPaintable {
    

    pub fn new_struct(mut value: String) -> Self {
        let mut this = Self {
            base: LocationPaintable::new_struct(),
            value: String::new(),
        };
        this.value = value.clone();
        this
    }

    pub fn new(mut value: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(value))))
    }
    

    pub fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) {
        jcall!(graphics).drawString(self.value.clone(), x.clone(), y.clone(), 0);
    }
}
impl std::ops::Deref for StringLocationPaintable {
    type Target = LocationPaintable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for StringLocationPaintable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::paint::LocationPaintable::LocationPaintableDyn for StringLocationPaintable {
    fn as_LocationPaintable(&self) -> &crate::org::allbinary::graphics::paint::LocationPaintable::LocationPaintable { &self.base }
    fn as_LocationPaintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::LocationPaintable::LocationPaintable { &mut self.base }
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}


