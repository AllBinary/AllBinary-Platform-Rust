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
// package org.allbinary.game.displayable.canvas
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::graphics::form::ItemPaintable::ItemPaintable;
use crate::org::allbinary::graphics::form::ItemPaintableFactory::ItemPaintableFactory;
use crate::org::allbinary::graphics::form::PaintableForm::PaintableForm;
use crate::org::allbinary::graphics::form::ScrollCurrentSelectionForm::ScrollCurrentSelectionForm;
use super::PreGameLockablePaintable::PreGameLockablePaintable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PreGameLockableFormPaintableFactory {
    pub base: ItemPaintableFactory,
    pub lockedIndex: i32,
}


impl PreGameLockableFormPaintableFactory {
    

    pub fn new_struct(mut lockedIndex: i32) -> Self {
        let mut this = Self {
            base: ItemPaintableFactory::new_struct(),
            lockedIndex: 0,
        };
        this.lockedIndex = lockedIndex.clone();
        this
    }

    pub fn new(mut lockedIndex: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(lockedIndex))))
    }
    

    // throws Exception
    pub fn getInstanceItemPaintable(&mut self, mut paintableForm: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::PaintableForm::PaintableFormDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ItemPaintable::ItemPaintableDyn>>> {
        return PreGameLockablePaintable::new((paintableForm).clone() /* as ScrollCurrentSelectionForm */, self.lockedIndex.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::form::ItemPaintable::ItemPaintableDyn>>);
    }
}
impl std::ops::Deref for PreGameLockableFormPaintableFactory {
    type Target = ItemPaintableFactory;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for PreGameLockableFormPaintableFactory {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::form::ItemPaintable::ItemPaintableDyn for PreGameLockableFormPaintableFactory {
    fn as_ItemPaintable(&self) -> &crate::org::allbinary::graphics::form::ItemPaintable::ItemPaintable { &self.base.base }
    fn as_ItemPaintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::ItemPaintable::ItemPaintable { &mut self.base.base }
}
impl crate::org::allbinary::graphics::form::ItemPaintableFactory::ItemPaintableFactoryDyn for PreGameLockableFormPaintableFactory {
    fn as_ItemPaintableFactory(&self) -> &crate::org::allbinary::graphics::form::ItemPaintableFactory::ItemPaintableFactory { &self.base }
    fn as_ItemPaintableFactory_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::ItemPaintableFactory::ItemPaintableFactory { &mut self.base }
    fn getInstanceItemPaintable(&mut self, mut paintableForm: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::PaintableForm::PaintableFormDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ItemPaintable::ItemPaintableDyn>>> { Self::getInstanceItemPaintable(self, paintableForm) }
}


