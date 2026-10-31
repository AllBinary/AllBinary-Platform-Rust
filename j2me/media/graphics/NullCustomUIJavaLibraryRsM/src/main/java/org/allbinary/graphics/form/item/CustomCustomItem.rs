// Generated Code Do Not Modify
// package org.allbinary.graphics.form.item
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessor;
use crate::org::allbinary::graphics::font::UpdateMyFontInterface::UpdateMyFontInterface;
use crate::org::allbinary::graphics::font::UpdateMyFontProcessor::UpdateMyFontProcessor;
use super::ABCustomItem::ABCustomItem;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CustomCustomItem {
    pub base: ABCustomItem,
    pub updateMyFontProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>>,
    pub myFontProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>>,
}

impl UpdateMyFontInterface for CustomCustomItem {
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::updateMeasurement(self, graphics) }
}

impl CustomCustomItem {
    

    pub fn new_struct(mut label: String, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Self {
        let mut this = Self {
            base: ABCustomItem::new_struct(label.clone(), backgroundBasicColor.clone(), foregroundBasicColor.clone()),
            updateMyFontProcessor: None,
            myFontProcessor: None,
        };
        this.updateMyFontProcessor = UpdateMyFontProcessor::new(this).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>);
        this.myFontProcessor = this.updateMyFontProcessor.clone();
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut label: String, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(label, backgroundBasicColor, foregroundBasicColor))))
    }
    

    pub fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
    }
}
impl std::ops::Deref for CustomCustomItem {
    type Target = ABCustomItem;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for CustomCustomItem {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait CustomCustomItemDyn: crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn {
    fn as_CustomCustomItem(&self) -> &CustomCustomItem;
    fn as_CustomCustomItem_mut(&mut self) -> &mut CustomCustomItem;
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { CustomCustomItem::updateMeasurement(self.as_CustomCustomItem_mut(), graphics) }
}
impl CustomCustomItemDyn for CustomCustomItem {
    fn as_CustomCustomItem(&self) -> &CustomCustomItem { self }
    fn as_CustomCustomItem_mut(&mut self) -> &mut CustomCustomItem { self }
}

impl crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn for CustomCustomItem {
    fn as_ABCustomItem(&self) -> &crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItem { &self.base }
    fn as_ABCustomItem_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItem { &mut self.base }
}


