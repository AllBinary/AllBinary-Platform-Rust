// Generated Code Do Not Modify
// package org.allbinary.graphics.form.item
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Image::Image;
// use crate::javax::microedition::lcdui::NullImage::NullImage;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use super::ABCustomItem::ABCustomItem;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ABCustomImageItem {
    pub base: ABCustomItem,
    pub yOffset: i32,
}


impl ABCustomImageItem {
    

    pub fn new_struct(mut label: String, mut image: Option<Rc<RefCell<Image>>>, mut layout: i32, mut altText: String, mut basicColor: Option<Rc<RefCell<BasicColor>>>, mut yOffset: i32) -> Self {
        let mut this = Self {
            base: ABCustomItem::new_struct(label.clone(), basicColor.clone(), basicColor.clone()),
            yOffset: 0,
        };
        this.yOffset = 0;
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut label: String, mut image: Option<Rc<RefCell<Image>>>, mut layout: i32, mut altText: String, mut basicColor: Option<Rc<RefCell<BasicColor>>>, mut yOffset: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(label, image, layout, altText, basicColor, yOffset))))
    }
    

    pub fn getImage(&mut self) -> Option<Rc<RefCell<Image>>> {
        return crate::javax::microedition::lcdui::NullImage::NullImage_NULL_IMAGE.with(Clone::clone).clone();
    }
}
impl std::ops::Deref for ABCustomImageItem {
    type Target = ABCustomItem;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ABCustomImageItem {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait ABCustomImageItemDyn: crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn {
    fn as_ABCustomImageItem(&self) -> &ABCustomImageItem;
    fn as_ABCustomImageItem_mut(&mut self) -> &mut ABCustomImageItem;
    fn getImage(&mut self) -> Option<Rc<RefCell<Image>>> { ABCustomImageItem::getImage(self.as_ABCustomImageItem_mut()) }
}
impl ABCustomImageItemDyn for ABCustomImageItem {
    fn as_ABCustomImageItem(&self) -> &ABCustomImageItem { self }
    fn as_ABCustomImageItem_mut(&mut self) -> &mut ABCustomImageItem { self }
}

impl crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn for ABCustomImageItem {
    fn as_ABCustomItem(&self) -> &crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItem { &self.base }
    fn as_ABCustomItem_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItem { &mut self.base }
}


