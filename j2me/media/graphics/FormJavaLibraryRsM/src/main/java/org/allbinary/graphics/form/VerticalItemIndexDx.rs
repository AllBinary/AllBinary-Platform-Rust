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
use crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItem;
use crate::java::lang::Exception::*;
use super::ItemIndexDx::ItemIndexDx;
use super::ScrollCurrentSelectionForm::ScrollCurrentSelectionForm;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct VerticalItemIndexDx {
    pub base: ItemIndexDx,
    pub scrollCurrentSelectionForm: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ScrollCurrentSelectionForm::ScrollCurrentSelectionFormDyn>>>,
}


impl VerticalItemIndexDx {
    

    pub fn new_struct(mut scrollCurrentSelectionForm: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ScrollCurrentSelectionForm::ScrollCurrentSelectionFormDyn>>>) -> Self {
        let mut this = Self {
            base: ItemIndexDx::new_struct(),
            scrollCurrentSelectionForm: None,
        };
        this.scrollCurrentSelectionForm = scrollCurrentSelectionForm.clone();
        this
    }

    pub fn new(mut scrollCurrentSelectionForm: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ScrollCurrentSelectionForm::ScrollCurrentSelectionFormDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(scrollCurrentSelectionForm))))
    }
    

    // throws Exception
    pub fn getDx(&mut self, mut index: i32, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut dx: i32, mut dy: i32) -> i32 {
        return jcall!(self.scrollCurrentSelectionForm).getXVerticalForm(index.clone(), item.clone(), dx.clone(), dy.clone());
    }
}
impl std::ops::Deref for VerticalItemIndexDx {
    type Target = ItemIndexDx;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for VerticalItemIndexDx {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::form::ItemIndexDx::ItemIndexDxDyn for VerticalItemIndexDx {
    fn as_ItemIndexDx(&self) -> &crate::org::allbinary::graphics::form::ItemIndexDx::ItemIndexDx { &self.base }
    fn as_ItemIndexDx_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::ItemIndexDx::ItemIndexDx { &mut self.base }
    fn getDx(&mut self, mut index: i32, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut dx: i32, mut dy: i32) -> i32 { Self::getDx(self, index, item, dx, dy) }
}


