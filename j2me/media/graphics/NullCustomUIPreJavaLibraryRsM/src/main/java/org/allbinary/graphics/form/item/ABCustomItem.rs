// Generated Code Do Not Modify
// package org.allbinary.graphics.form.item
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
// use crate::javax::microedition::lcdui::Screen::Screen;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ABCustomItem {
}

pub static mut ABCustomItem_NULL_CUSTOM_ITEM: std::cell::LazyCell<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = std::cell::LazyCell::new(|| jcall!(NullUtil::getInstance()).NULL_OBJECT.clone());

impl ABCustomItem {
    

    pub fn getNullInstance() -> Option<Rc<RefCell<dyn ABCustomItemDyn>>> {
        if match (&unsafe { ABCustomItem_NULL_CUSTOM_ITEM.clone() }, &jcall!(NullUtil::getInstance()).NULL_OBJECT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            unsafe { *ABCustomItem_NULL_CUSTOM_ITEM = ABCustomItem::new(jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), jcall!(BasicColorFactory::getInstance()).BLACK.clone(), jcall!(BasicColorFactory::getInstance()).WHITE.clone()).map(|reference| reference as Rc<RefCell<dyn ABCustomItemDyn>>) };
        }
        return (unsafe { ABCustomItem_NULL_CUSTOM_ITEM.clone() }).clone() /* as ABCustomItem */;
    }
    

    pub fn new_struct(mut label: String, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new(mut label: String, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(label, backgroundBasicColor, foregroundBasicColor))))
    }
    

    pub fn setOwner(&mut self, mut owner: Option<Rc<RefCell<Screen>>>) {
    }
    

    pub fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) {
    }
    

    pub fn paintUnselected(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) {
    }
    

    pub fn isFocusable(&mut self) -> bool {
        return false;
    }
    

    pub fn getLabel(&mut self) -> String {
        return jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
    }
    

    pub fn setLabel(&mut self, mut label: String) {
    }
    

    pub fn getHeight(&mut self) -> i32 {
        return 0;
    }
    

    pub fn setHeight(&mut self, mut height: i32) {
    }
    

    pub fn getValue(&mut self) -> f32 {
        return 0.0_f32;
    }
    

    pub fn setValue(&mut self, mut value: f32) {
    }
    

    pub fn getMinimumWidth(&mut self) -> i32 {
        return 0;
    }
    

    pub fn getMinimumHeight(&mut self) -> i32 {
        return 0;
    }
    

    pub fn preMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
    }
}
pub trait ABCustomItemDyn {
    fn as_ABCustomItem(&self) -> &ABCustomItem;
    fn as_ABCustomItem_mut(&mut self) -> &mut ABCustomItem;
    fn setOwner(&mut self, mut owner: Option<Rc<RefCell<Screen>>>) { ABCustomItem::setOwner(self.as_ABCustomItem_mut(), owner) }
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { ABCustomItem::paintXY(self.as_ABCustomItem_mut(), graphics, x, y) }
    fn paintUnselected(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { ABCustomItem::paintUnselected(self.as_ABCustomItem_mut(), graphics, x, y) }
    fn isFocusable(&mut self) -> bool { ABCustomItem::isFocusable(self.as_ABCustomItem_mut()) }
    fn getLabel(&mut self) -> String { ABCustomItem::getLabel(self.as_ABCustomItem_mut()) }
    fn setLabel(&mut self, mut label: String) { ABCustomItem::setLabel(self.as_ABCustomItem_mut(), label) }
    fn getHeight(&mut self) -> i32 { ABCustomItem::getHeight(self.as_ABCustomItem_mut()) }
    fn setHeight(&mut self, mut height: i32) { ABCustomItem::setHeight(self.as_ABCustomItem_mut(), height) }
    fn getValue(&mut self) -> f32 { ABCustomItem::getValue(self.as_ABCustomItem_mut()) }
    fn setValue(&mut self, mut value: f32) { ABCustomItem::setValue(self.as_ABCustomItem_mut(), value) }
    fn getMinimumWidth(&mut self) -> i32 { ABCustomItem::getMinimumWidth(self.as_ABCustomItem_mut()) }
    fn getMinimumHeight(&mut self) -> i32 { ABCustomItem::getMinimumHeight(self.as_ABCustomItem_mut()) }
    fn preMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { ABCustomItem::preMeasurement(self.as_ABCustomItem_mut(), graphics) }
}
impl ABCustomItemDyn for ABCustomItem {
    fn as_ABCustomItem(&self) -> &ABCustomItem { self }
    fn as_ABCustomItem_mut(&mut self) -> &mut ABCustomItem { self }
}


