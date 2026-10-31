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
use crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItem;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ItemIndexDx {
}

thread_local! { pub static ItemIndexDx_instance: Option<Rc<RefCell<dyn ItemIndexDxDyn>>> = ItemIndexDx::new().map(|reference| reference as Rc<RefCell<dyn ItemIndexDxDyn>>); }

impl ItemIndexDx {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn ItemIndexDxDyn>>> {
        return ItemIndexDx_instance.with(Clone::clone).clone();
    }
    

    // throws Exception
    pub fn getDx(&mut self, mut index: i32, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut dx: i32, mut dy: i32) -> i32 {
        return 0;
    }
}
pub trait ItemIndexDxDyn {
    fn as_ItemIndexDx(&self) -> &ItemIndexDx;
    fn as_ItemIndexDx_mut(&mut self) -> &mut ItemIndexDx;
    fn getDx(&mut self, mut index: i32, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut dx: i32, mut dy: i32) -> i32 { ItemIndexDx::getDx(self.as_ItemIndexDx_mut(), index, item, dx, dy) }
}
impl ItemIndexDxDyn for ItemIndexDx {
    fn as_ItemIndexDx(&self) -> &ItemIndexDx { self }
    fn as_ItemIndexDx_mut(&mut self) -> &mut ItemIndexDx { self }
}


