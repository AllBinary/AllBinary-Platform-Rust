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
use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItem;
use crate::java::lang::Exception::*;
use super::ItemIndexPaintable::ItemIndexPaintable;
use super::MultipleScrollSelectionHorizontalForm::MultipleScrollSelectionHorizontalForm;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MultipleScrollSelectionFormHorizontalPaintable {
    pub base: ItemIndexPaintable,
    pub multipleScrollSelectionForm: Option<Rc<RefCell<MultipleScrollSelectionHorizontalForm>>>,
}


impl MultipleScrollSelectionFormHorizontalPaintable {
    

    pub fn new_struct(mut multipleScrollSelectionForm: Option<Rc<RefCell<MultipleScrollSelectionHorizontalForm>>>) -> Self {
        let mut this = Self {
            base: ItemIndexPaintable::new_struct(),
            multipleScrollSelectionForm: None,
        };
        this.multipleScrollSelectionForm = multipleScrollSelectionForm.clone();
        this
    }

    pub fn new(mut multipleScrollSelectionForm: Option<Rc<RefCell<MultipleScrollSelectionHorizontalForm>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(multipleScrollSelectionForm))))
    }
    

    // throws Exception
    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, mut index: i32, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut dx: i32, mut dy: i32) -> i32 {
        return jcall!(self.multipleScrollSelectionForm).paintHorizontal(graphics.clone(), index.clone(), item.clone(), dx.clone(), dy.clone());
    }
}
impl std::ops::Deref for MultipleScrollSelectionFormHorizontalPaintable {
    type Target = ItemIndexPaintable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for MultipleScrollSelectionFormHorizontalPaintable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::form::ItemIndexPaintable::ItemIndexPaintableDyn for MultipleScrollSelectionFormHorizontalPaintable {
    fn as_ItemIndexPaintable(&self) -> &crate::org::allbinary::graphics::form::ItemIndexPaintable::ItemIndexPaintable { &self.base }
    fn as_ItemIndexPaintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::ItemIndexPaintable::ItemIndexPaintable { &mut self.base }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, mut index: i32, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut dx: i32, mut dy: i32) -> i32 { Self::paint(self, graphics, index, item, dx, dy) }
}


