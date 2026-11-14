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
use super::ScrollSelectionForm::ScrollSelectionForm;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ScrollSelectionFormTempHorizontalDx {
    pub base: ItemIndexDx,
    pub scrollSelectionForm: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionFormDyn>>>,
}


impl ScrollSelectionFormTempHorizontalDx {
    

    pub fn new_struct(mut scrollSelectionForm: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionFormDyn>>>) -> Self {
        let mut this = Self {
            base: ItemIndexDx::new_struct(),
            scrollSelectionForm: None,
        };
        this.scrollSelectionForm = scrollSelectionForm.clone();
        this
    }

    pub fn new(mut scrollSelectionForm: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionFormDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(scrollSelectionForm))))
    }
    

    // throws Exception
    pub fn getDx(&mut self, mut index: i32, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut dx: i32, mut dy: i32) -> i32 {
        return jcall!(self.scrollSelectionForm).getSelectedIndexForPointTempHorizontalDx(index.clone(), item.clone(), dx.clone(), dy.clone());
    }
}
impl std::ops::Deref for ScrollSelectionFormTempHorizontalDx {
    type Target = ItemIndexDx;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ScrollSelectionFormTempHorizontalDx {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::form::ItemIndexDx::ItemIndexDxDyn for ScrollSelectionFormTempHorizontalDx {
    fn as_ItemIndexDx(&self) -> &crate::org::allbinary::graphics::form::ItemIndexDx::ItemIndexDx { &self.base }
    fn as_ItemIndexDx_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::ItemIndexDx::ItemIndexDx { &mut self.base }
    fn getDx(&mut self, mut index: i32, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut dx: i32, mut dy: i32) -> i32 { Self::getDx(self, index, item, dx, dy) }
}


