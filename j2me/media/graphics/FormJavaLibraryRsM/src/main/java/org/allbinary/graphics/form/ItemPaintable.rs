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

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ItemPaintable {
}


impl ItemPaintable {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut currentIndex: i32, mut x: i32, mut y: i32) {
    }
}
pub trait ItemPaintableDyn {
    fn as_ItemPaintable(&self) -> &ItemPaintable;
    fn as_ItemPaintable_mut(&mut self) -> &mut ItemPaintable;
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut currentIndex: i32, mut x: i32, mut y: i32) { ItemPaintable::paint(self.as_ItemPaintable_mut(), graphics, currentIndex, x, y) }
}
impl ItemPaintableDyn for ItemPaintable {
    fn as_ItemPaintable(&self) -> &ItemPaintable { self }
    fn as_ItemPaintable_mut(&mut self) -> &mut ItemPaintable { self }
}


