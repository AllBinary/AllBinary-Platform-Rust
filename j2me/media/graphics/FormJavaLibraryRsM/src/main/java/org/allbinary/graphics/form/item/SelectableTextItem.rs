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
// package org.allbinary.graphics.form.item
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use super::ABTextItem::ABTextItem;
use super::ABStringComponent::ABStringComponent;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SelectableTextItem {
    pub base: ABTextItem,
}


impl SelectableTextItem {
    

    pub fn new_struct(mut label: String, mut layout: i32, mut altText: String, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Self {
        let mut this = Self {
            base: ABTextItem::new_struct(label.clone(), layout.clone(), altText.clone(), backgroundBasicColor.clone(), foregroundBasicColor.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut label: String, mut layout: i32, mut altText: String, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(label, layout, altText, backgroundBasicColor, foregroundBasicColor))))
    }
    

    pub fn isFocusable(&mut self) -> bool {
        return true;
    }
    

    pub fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, mut x: i32, mut y: i32) {
        jcall!(self.myFontProcessor).process(graphics.clone());
        let mut stringComponent: Option<Rc<RefCell<ABStringComponent>>> = self.getLabelStringComponent();
        if stringComponent.is_some() {
            jcall!(graphics).setColor(jcall!(jcall!(stringComponent).getForegroundBasicColor()).intValue());
        }
        jcall!(graphics).drawString(self.getLabel(), x + 2, y.clone(), 0);
    }
}
impl std::ops::Deref for SelectableTextItem {
    type Target = ABTextItem;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for SelectableTextItem {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::form::item::ABTextItem::ABTextItemDyn for SelectableTextItem {
    fn as_ABTextItem(&self) -> &crate::org::allbinary::graphics::form::item::ABTextItem::ABTextItem { &self.base }
    fn as_ABTextItem_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::item::ABTextItem::ABTextItem { &mut self.base }
}
impl crate::org::allbinary::graphics::form::item::CustomCustomItem::CustomCustomItemDyn for SelectableTextItem {
    fn as_CustomCustomItem(&self) -> &crate::org::allbinary::graphics::form::item::CustomCustomItem::CustomCustomItem { &self.base.base }
    fn as_CustomCustomItem_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::item::CustomCustomItem::CustomCustomItem { &mut self.base.base }
}
impl crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn for SelectableTextItem {
    fn as_ABCustomItem(&self) -> &crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItem { &self.base.base.base }
    fn as_ABCustomItem_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItem { &mut self.base.base.base }
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
    fn paintUnselected(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, p2: i32, p3: i32) { crate::org::allbinary::graphics::form::item::ABTextItem::ABTextItem::paintUnselected(crate::org::allbinary::graphics::form::item::ABTextItem::ABTextItemDyn::as_ABTextItem_mut(self), p1, p2, p3) }
    fn isFocusable(&mut self) -> bool { Self::isFocusable(self) }
    fn getLabel(&mut self) -> String { crate::org::allbinary::graphics::form::item::ABTextItem::ABTextItem::getLabel(crate::org::allbinary::graphics::form::item::ABTextItem::ABTextItemDyn::as_ABTextItem_mut(self)) }
    fn getMinimumWidth(&mut self) -> i32 { crate::org::allbinary::graphics::form::item::ABTextItem::ABTextItem::getMinimumWidth(crate::org::allbinary::graphics::form::item::ABTextItem::ABTextItemDyn::as_ABTextItem_mut(self)) }
    fn getMinimumHeight(&mut self) -> i32 { crate::org::allbinary::graphics::form::item::ABTextItem::ABTextItem::getMinimumHeight(crate::org::allbinary::graphics::form::item::ABTextItem::ABTextItemDyn::as_ABTextItem_mut(self)) }
}
impl crate::org::allbinary::graphics::form::item::ABCustomItemInterface::ABCustomItemInterface for SelectableTextItem {
    fn getMinimumWidth(&mut self) -> i32 { crate::org::allbinary::graphics::form::item::ABTextItem::ABTextItem::getMinimumWidth(crate::org::allbinary::graphics::form::item::ABTextItem::ABTextItemDyn::as_ABTextItem_mut(self)) }
    fn getMinimumHeight(&mut self) -> i32 { crate::org::allbinary::graphics::form::item::ABTextItem::ABTextItem::getMinimumHeight(crate::org::allbinary::graphics::form::item::ABTextItem::ABTextItemDyn::as_ABTextItem_mut(self)) }
    fn getLabel(&mut self) -> String { crate::org::allbinary::graphics::form::item::ABTextItem::ABTextItem::getLabel(crate::org::allbinary::graphics::form::item::ABTextItem::ABTextItemDyn::as_ABTextItem_mut(self)) }
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
    fn paintUnselected(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, p2: i32, p3: i32) { crate::org::allbinary::graphics::form::item::ABTextItem::ABTextItem::paintUnselected(crate::org::allbinary::graphics::form::item::ABTextItem::ABTextItemDyn::as_ABTextItem_mut(self), p1, p2, p3) }
}
impl crate::org::allbinary::graphics::font::UpdateMyFontInterface::UpdateMyFontInterface for SelectableTextItem {
    fn updateMeasurement(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { crate::org::allbinary::graphics::form::item::CustomCustomItem::CustomCustomItem::updateMeasurement(crate::org::allbinary::graphics::form::item::CustomCustomItem::CustomCustomItemDyn::as_CustomCustomItem_mut(self), p1) }
}


