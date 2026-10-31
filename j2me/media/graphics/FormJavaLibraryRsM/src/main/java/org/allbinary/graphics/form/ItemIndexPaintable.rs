// 
// AllBinary Open License Version 1 
// Copyright (c) 2025 AllBinary 
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
use crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItem;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ItemIndexPaintable {
}

thread_local! { pub static ItemIndexPaintable_instance: Option<Rc<RefCell<dyn ItemIndexPaintableDyn>>> = ItemIndexPaintable::new().map(|reference| reference as Rc<RefCell<dyn ItemIndexPaintableDyn>>); }

impl ItemIndexPaintable {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn ItemIndexPaintableDyn>>> {
        return ItemIndexPaintable_instance.with(Clone::clone).clone();
    }
    

    // throws Exception
    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut index: i32, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut dx: i32, mut dy: i32) -> i32 {
        return 0;
    }
}
pub trait ItemIndexPaintableDyn {
    fn as_ItemIndexPaintable(&self) -> &ItemIndexPaintable;
    fn as_ItemIndexPaintable_mut(&mut self) -> &mut ItemIndexPaintable;
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut index: i32, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut dx: i32, mut dy: i32) -> i32 { ItemIndexPaintable::paint(self.as_ItemIndexPaintable_mut(), graphics, index, item, dx, dy) }
}
impl ItemIndexPaintableDyn for ItemIndexPaintable {
    fn as_ItemIndexPaintable(&self) -> &ItemIndexPaintable { self }
    fn as_ItemIndexPaintable_mut(&mut self) -> &mut ItemIndexPaintable { self }
}


