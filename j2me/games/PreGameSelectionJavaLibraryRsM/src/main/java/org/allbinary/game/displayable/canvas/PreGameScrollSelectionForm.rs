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
use crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItem;
use crate::org::allbinary::graphics::Rectangle::Rectangle;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::form::FormTypeFactory::FormTypeFactory;
use crate::org::allbinary::graphics::form::ItemPaintableFactory::ItemPaintableFactory;
use crate::org::allbinary::graphics::form::ScrollCurrentSelectionForm::ScrollCurrentSelectionForm;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PreGameScrollSelectionForm {
    pub base: ScrollCurrentSelectionForm,
}


impl PreGameScrollSelectionForm {
    

    pub fn new_struct(mut title: String, mut items: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>>, mut formPaintableFactory: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ItemPaintableFactory::ItemPaintableFactoryDyn>>>, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Self {
        let mut this = Self {
            base: ScrollCurrentSelectionForm::new_struct(title.clone(), items.clone(), formPaintableFactory.clone(), 0, 3, false, backgroundBasicColor.clone(), foregroundBasicColor.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut title: String, mut items: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>>, mut formPaintableFactory: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ItemPaintableFactory::ItemPaintableFactoryDyn>>>, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(title, items, formPaintableFactory, backgroundBasicColor, foregroundBasicColor))))
    }
    

    // throws Exception
    pub fn init(&mut self, mut rectangle: Option<Rc<RefCell<Rectangle>>>) {
        self.base.init(rectangle.clone(), jcall!(FormTypeFactory::getInstance()).TEMP_HORIZONTAL_FORM.clone());
    }
}
impl std::ops::Deref for PreGameScrollSelectionForm {
    type Target = ScrollCurrentSelectionForm;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for PreGameScrollSelectionForm {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::form::PaintableForm::PaintableFormDyn for PreGameScrollSelectionForm {
    fn as_PaintableForm(&self) -> &crate::org::allbinary::graphics::form::PaintableForm::PaintableForm { &self.base.base.base }
    fn as_PaintableForm_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::PaintableForm::PaintableForm { &mut self.base.base.base }
    fn init(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::graphics::Rectangle::Rectangle>>>, p2: Option<Rc<RefCell<crate::org::allbinary::graphics::form::FormType::FormType>>>) { crate::org::allbinary::graphics::form::ScrollCurrentSelectionForm::ScrollCurrentSelectionForm::init(crate::org::allbinary::graphics::form::ScrollCurrentSelectionForm::ScrollCurrentSelectionFormDyn::as_ScrollCurrentSelectionForm_mut(self), p1, p2) }
}
impl crate::org::allbinary::graphics::form::ScrollCurrentSelectionForm::ScrollCurrentSelectionFormDyn for PreGameScrollSelectionForm {
    fn as_ScrollCurrentSelectionForm(&self) -> &crate::org::allbinary::graphics::form::ScrollCurrentSelectionForm::ScrollCurrentSelectionForm { &self.base }
    fn as_ScrollCurrentSelectionForm_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::ScrollCurrentSelectionForm::ScrollCurrentSelectionForm { &mut self.base }
}
impl crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionFormDyn for PreGameScrollSelectionForm {
    fn as_ScrollSelectionForm(&self) -> &crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionForm { &self.base.base }
    fn as_ScrollSelectionForm_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionForm { &mut self.base.base }
    fn getDiffX(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>) -> i32 { crate::org::allbinary::graphics::form::ScrollCurrentSelectionForm::ScrollCurrentSelectionForm::getDiffX(crate::org::allbinary::graphics::form::ScrollCurrentSelectionForm::ScrollCurrentSelectionFormDyn::as_ScrollCurrentSelectionForm_mut(self), p1) }
    fn getDx(&mut self) -> i32 { crate::org::allbinary::graphics::form::ScrollCurrentSelectionForm::ScrollCurrentSelectionForm::getDx(crate::org::allbinary::graphics::form::ScrollCurrentSelectionForm::ScrollCurrentSelectionFormDyn::as_ScrollCurrentSelectionForm_mut(self)) }
    fn getDy(&mut self) -> i32 { crate::org::allbinary::graphics::form::ScrollCurrentSelectionForm::ScrollCurrentSelectionForm::getDy(crate::org::allbinary::graphics::form::ScrollCurrentSelectionForm::ScrollCurrentSelectionFormDyn::as_ScrollCurrentSelectionForm_mut(self)) }
}


