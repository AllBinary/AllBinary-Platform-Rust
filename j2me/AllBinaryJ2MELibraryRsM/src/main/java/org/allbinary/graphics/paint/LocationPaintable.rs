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
use super::LocationPaintableInterface::LocationPaintableInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LocationPaintable {
}

thread_local! { pub static LocationPaintable_instance: Option<Rc<RefCell<dyn LocationPaintableDyn>>> = LocationPaintable::new().map(|reference| reference as Rc<RefCell<dyn LocationPaintableDyn>>); }
impl LocationPaintableInterface for LocationPaintable {
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}

impl LocationPaintable {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn LocationPaintableDyn>>> {
        return LocationPaintable_instance.with(Clone::clone).clone();
    }
    

    pub fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) {
    }
}
pub trait LocationPaintableDyn {
    fn as_LocationPaintable(&self) -> &LocationPaintable;
    fn as_LocationPaintable_mut(&mut self) -> &mut LocationPaintable;
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { LocationPaintable::paintXY(self.as_LocationPaintable_mut(), graphics, x, y) }
}
impl LocationPaintableDyn for LocationPaintable {
    fn as_LocationPaintable(&self) -> &LocationPaintable { self }
    fn as_LocationPaintable_mut(&mut self) -> &mut LocationPaintable { self }
}


