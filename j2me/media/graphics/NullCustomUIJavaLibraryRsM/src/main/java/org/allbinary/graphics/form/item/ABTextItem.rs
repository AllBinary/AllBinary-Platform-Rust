// Generated Code Do Not Modify
// package org.allbinary.graphics.form.item
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use super::CustomCustomItem::CustomCustomItem;
use super::ABCustomItemInterface::ABCustomItemInterface;
use super::ABStringComponent::ABStringComponent;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ABTextItem {
    pub base: CustomCustomItem,
}

// impl ABCustomItemInterface for ABTextItem with the methods of the trait

impl ABTextItem {
    

    pub fn new_struct(mut label: String, mut layout: i32, mut altText: String, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Self {
        let mut this = Self {
            base: CustomCustomItem::new_struct(label.clone(), backgroundBasicColor.clone(), foregroundBasicColor.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut label: String, mut layout: i32, mut altText: String, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(label, layout, altText, backgroundBasicColor, foregroundBasicColor))))
    }
    

    pub fn keyPressed(&mut self, mut keyCode: i32) {
    }
    

    pub fn paintUnselected(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) {
    }
    

    pub fn getLabelStringComponent(&mut self) -> Option<Rc<RefCell<ABStringComponent>>> {
        return ABStringComponent_NULL_STRING_COMPONENT.with(Clone::clone).clone();
    }
}
impl std::ops::Deref for ABTextItem {
    type Target = CustomCustomItem;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ABTextItem {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait ABTextItemDyn: crate::org::allbinary::graphics::form::item::CustomCustomItem::CustomCustomItemDyn {
    fn as_ABTextItem(&self) -> &ABTextItem;
    fn as_ABTextItem_mut(&mut self) -> &mut ABTextItem;
    fn keyPressed(&mut self, mut keyCode: i32) { ABTextItem::keyPressed(self.as_ABTextItem_mut(), keyCode) }
    fn getLabelStringComponent(&mut self) -> Option<Rc<RefCell<ABStringComponent>>> { ABTextItem::getLabelStringComponent(self.as_ABTextItem_mut()) }
}
impl ABTextItemDyn for ABTextItem {
    fn as_ABTextItem(&self) -> &ABTextItem { self }
    fn as_ABTextItem_mut(&mut self) -> &mut ABTextItem { self }
}

impl crate::org::allbinary::graphics::form::item::CustomCustomItem::CustomCustomItemDyn for ABTextItem {
    fn as_CustomCustomItem(&self) -> &crate::org::allbinary::graphics::form::item::CustomCustomItem::CustomCustomItem { &self.base }
    fn as_CustomCustomItem_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::item::CustomCustomItem::CustomCustomItem { &mut self.base }
}
impl crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn for ABTextItem {
    fn as_ABCustomItem(&self) -> &crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItem { &self.base.base }
    fn as_ABCustomItem_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItem { &mut self.base.base }
    fn paintUnselected(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintUnselected(self, graphics, x, y) }
}


