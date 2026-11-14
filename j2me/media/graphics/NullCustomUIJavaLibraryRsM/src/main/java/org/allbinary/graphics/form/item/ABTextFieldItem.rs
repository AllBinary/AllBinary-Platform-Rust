// Generated Code Do Not Modify
// package org.allbinary.graphics.form.item
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::javax::microedition::lcdui::Canvas::Canvas;
use crate::javax::microedition::lcdui::Font::Font;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::util::visitor::Visitor::Visitor;
use super::ABTextItem::ABTextItem;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ABTextFieldItem {
    pub base: ABTextItem,
    pub font: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Font::FontDyn>>>,
}


impl ABTextFieldItem {
    

    pub fn new_struct(mut canvas: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Canvas::CanvasDyn>>>, mut visitor: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::visitor::Visitor::VisitorDyn>>>, mut label: String, mut value: String, mut maxSize: i32, mut layout: i32, mut altText: String, mut font: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Font::FontDyn>>>, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Self {
        let mut this = Self {
            base: ABTextItem::new_struct(label.clone(), layout.clone(), altText.clone(), backgroundBasicColor.clone(), foregroundBasicColor.clone()),
            font: None,
        };
        // super(...) is in the struct literal above
        this.font = font.clone();
        this
    }

    pub fn new(mut canvas: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Canvas::CanvasDyn>>>, mut visitor: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::visitor::Visitor::VisitorDyn>>>, mut label: String, mut value: String, mut maxSize: i32, mut layout: i32, mut altText: String, mut font: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Font::FontDyn>>>, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(canvas, visitor, label, value, maxSize, layout, altText, font, backgroundBasicColor, foregroundBasicColor))))
    }
    

    pub fn getLabel(&mut self) -> String {
        return { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone();
    }
    

    pub fn setLabel(&mut self, mut label: String) {
    }
    

    pub fn getString(&mut self) -> String {
        return { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone();
    }
    

    pub fn setString(&mut self, mut string: String) {
    }
    

    pub fn setFocus(&mut self, mut state: bool) {
    }
    

    pub fn getFontHeight(&mut self) -> i32 {
        return jcall!(self.font).getHeight();
    }
    

    pub fn setBackgroundBasicColorP(&mut self, mut basicColor: Option<Rc<RefCell<BasicColor>>>) {
    }
    

    pub fn setForegroundBasicColorP(&mut self, mut basicColor: Option<Rc<RefCell<BasicColor>>>) {
    }
}
impl std::ops::Deref for ABTextFieldItem {
    type Target = ABTextItem;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ABTextFieldItem {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::form::item::ABTextItem::ABTextItemDyn for ABTextFieldItem {
    fn as_ABTextItem(&self) -> &crate::org::allbinary::graphics::form::item::ABTextItem::ABTextItem { &self.base }
    fn as_ABTextItem_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::item::ABTextItem::ABTextItem { &mut self.base }
}
impl crate::org::allbinary::graphics::form::item::CustomCustomItem::CustomCustomItemDyn for ABTextFieldItem {
    fn as_CustomCustomItem(&self) -> &crate::org::allbinary::graphics::form::item::CustomCustomItem::CustomCustomItem { &self.base.base }
    fn as_CustomCustomItem_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::item::CustomCustomItem::CustomCustomItem { &mut self.base.base }
}
impl crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn for ABTextFieldItem {
    fn as_ABCustomItem(&self) -> &crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItem { &self.base.base.base }
    fn as_ABCustomItem_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItem { &mut self.base.base.base }
    fn paintXY(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, p2: i32, p3: i32) { crate::org::allbinary::graphics::form::item::ABTextItem::ABTextItem::paintXY(crate::org::allbinary::graphics::form::item::ABTextItem::ABTextItemDyn::as_ABTextItem_mut(self), p1, p2, p3) }
    fn paintUnselected(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, p2: i32, p3: i32) { crate::org::allbinary::graphics::form::item::ABTextItem::ABTextItem::paintUnselected(crate::org::allbinary::graphics::form::item::ABTextItem::ABTextItemDyn::as_ABTextItem_mut(self), p1, p2, p3) }
    fn getLabel(&mut self) -> String { Self::getLabel(self) }
    fn setLabel(&mut self, mut label: String) { Self::setLabel(self, label) }
    fn getMinimumWidth(&mut self) -> i32 { crate::org::allbinary::graphics::form::item::ABTextItem::ABTextItem::getMinimumWidth(crate::org::allbinary::graphics::form::item::ABTextItem::ABTextItemDyn::as_ABTextItem_mut(self)) }
    fn getMinimumHeight(&mut self) -> i32 { crate::org::allbinary::graphics::form::item::ABTextItem::ABTextItem::getMinimumHeight(crate::org::allbinary::graphics::form::item::ABTextItem::ABTextItemDyn::as_ABTextItem_mut(self)) }
}
impl crate::org::allbinary::graphics::form::item::ABCustomItemInterface::ABCustomItemInterface for ABTextFieldItem {
    fn getMinimumWidth(&mut self) -> i32 { crate::org::allbinary::graphics::form::item::ABTextItem::ABTextItem::getMinimumWidth(crate::org::allbinary::graphics::form::item::ABTextItem::ABTextItemDyn::as_ABTextItem_mut(self)) }
    fn getMinimumHeight(&mut self) -> i32 { crate::org::allbinary::graphics::form::item::ABTextItem::ABTextItem::getMinimumHeight(crate::org::allbinary::graphics::form::item::ABTextItem::ABTextItemDyn::as_ABTextItem_mut(self)) }
    fn getLabel(&mut self) -> String { Self::getLabel(self) }
    fn paintXY(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, p2: i32, p3: i32) { crate::org::allbinary::graphics::form::item::ABTextItem::ABTextItem::paintXY(crate::org::allbinary::graphics::form::item::ABTextItem::ABTextItemDyn::as_ABTextItem_mut(self), p1, p2, p3) }
    fn paintUnselected(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, p2: i32, p3: i32) { crate::org::allbinary::graphics::form::item::ABTextItem::ABTextItem::paintUnselected(crate::org::allbinary::graphics::form::item::ABTextItem::ABTextItemDyn::as_ABTextItem_mut(self), p1, p2, p3) }
}
impl crate::org::allbinary::graphics::font::UpdateMyFontInterface::UpdateMyFontInterface for ABTextFieldItem {
    fn updateMeasurement(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { crate::org::allbinary::graphics::form::item::CustomCustomItem::CustomCustomItem::updateMeasurement(crate::org::allbinary::graphics::form::item::CustomCustomItem::CustomCustomItemDyn::as_CustomCustomItem_mut(self), p1) }
}


