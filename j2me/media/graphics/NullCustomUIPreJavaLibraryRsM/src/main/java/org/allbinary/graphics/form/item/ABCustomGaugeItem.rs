// Generated Code Do Not Modify
// package org.allbinary.graphics.form.item
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use super::ABCustomItem::ABCustomItem;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ABCustomGaugeItem {
    pub base: ABCustomItem,
}

thread_local! { pub static ABCustomGaugeItem_NULL_GAUGE_ITEM: Option<Rc<RefCell<ABCustomGaugeItem>>> = ABCustomGaugeItem::new(jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), 1, 0, jcall!(BasicColorFactory::getInstance()).BLACK.clone(), jcall!(BasicColorFactory::getInstance()).WHITE.clone()); }

impl ABCustomGaugeItem {
    

    pub fn new_struct(mut label: String, mut maxValue: i32, mut initialValue: i32, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Self {
        let mut this = Self {
            base: ABCustomItem::new_struct(label.clone(), backgroundBasicColor.clone(), foregroundBasicColor.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut label: String, mut maxValue: i32, mut initialValue: i32, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(label, maxValue, initialValue, backgroundBasicColor, foregroundBasicColor))))
    }
}
impl std::ops::Deref for ABCustomGaugeItem {
    type Target = ABCustomItem;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ABCustomGaugeItem {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn for ABCustomGaugeItem {
    fn as_ABCustomItem(&self) -> &crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItem { &self.base }
    fn as_ABCustomItem_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItem { &mut self.base }
}


