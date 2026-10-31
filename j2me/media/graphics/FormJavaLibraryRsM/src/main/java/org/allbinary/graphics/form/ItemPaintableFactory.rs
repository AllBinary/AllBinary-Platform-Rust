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
use super::ItemPaintable::ItemPaintable;
use super::PaintableForm::PaintableForm;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ItemPaintableFactory {
    pub base: ItemPaintable,
}

thread_local! { pub static ItemPaintableFactory_instance: Option<Rc<RefCell<dyn ItemPaintableFactoryDyn>>> = ItemPaintableFactory::new().map(|reference| reference as Rc<RefCell<dyn ItemPaintableFactoryDyn>>); }

impl ItemPaintableFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: ItemPaintable::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn ItemPaintableFactoryDyn>>> {
        return ItemPaintableFactory_instance.with(Clone::clone).clone();
    }
    

    // throws Exception
    pub fn getInstanceItemPaintable(&mut self, mut paintableForm: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::PaintableForm::PaintableFormDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ItemPaintable::ItemPaintableDyn>>> {
        return ItemPaintableFactory_instance.with(Clone::clone).clone();
    }
}
impl std::ops::Deref for ItemPaintableFactory {
    type Target = ItemPaintable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ItemPaintableFactory {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait ItemPaintableFactoryDyn: crate::org::allbinary::graphics::form::ItemPaintable::ItemPaintableDyn {
    fn as_ItemPaintableFactory(&self) -> &ItemPaintableFactory;
    fn as_ItemPaintableFactory_mut(&mut self) -> &mut ItemPaintableFactory;
    fn getInstanceItemPaintable(&mut self, mut paintableForm: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::PaintableForm::PaintableFormDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ItemPaintable::ItemPaintableDyn>>> { ItemPaintableFactory::getInstanceItemPaintable(self.as_ItemPaintableFactory_mut(), paintableForm) }
}
impl ItemPaintableFactoryDyn for ItemPaintableFactory {
    fn as_ItemPaintableFactory(&self) -> &ItemPaintableFactory { self }
    fn as_ItemPaintableFactory_mut(&mut self) -> &mut ItemPaintableFactory { self }
}

impl crate::org::allbinary::graphics::form::ItemPaintable::ItemPaintableDyn for ItemPaintableFactory {
    fn as_ItemPaintable(&self) -> &crate::org::allbinary::graphics::form::ItemPaintable::ItemPaintable { &self.base }
    fn as_ItemPaintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::ItemPaintable::ItemPaintable { &mut self.base }
}


