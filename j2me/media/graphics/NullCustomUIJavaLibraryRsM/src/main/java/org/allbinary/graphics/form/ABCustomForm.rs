// Generated Code Do Not Modify
// package org.allbinary.graphics.form
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItem;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::CustomScreen::CustomScreen;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ABCustomForm {
    pub base: CustomScreen,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
}


impl ABCustomForm {
    

    pub fn new_struct(mut title: String, mut items: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>>, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Self {
        let mut this = Self {
            base: CustomScreen::new_struct(),
            logUtil: None,
            commonStrings: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this
    }

    pub fn new(mut title: String, mut items: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>>, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(title, items, backgroundBasicColor, foregroundBasicColor))))
    }
    

    pub fn getAllitems(&mut self) -> Vec<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>> {
        return vec![None; (0) as usize];
    }
    

    pub fn size(&mut self) -> i32 {
        return 0;
    }
    

    pub fn setSelectedIndex(&mut self, mut index: i32) {
    }
    

    pub fn getSelectedIndex(&mut self) -> i32 {
        return 0;
    }
    

    pub fn deleteAll(&mut self) {
    }
    

    pub fn delete(&mut self, mut itemNum: i32) {
    }
    

    pub fn append(&mut self, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>) -> i32 {
        return -1;
    }
    

    pub fn insert(&mut self, mut itemNum: i32, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>) {
    }
    

    pub fn set(&mut self, mut itemNum: i32, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>) {
    }
    

    pub fn get(&mut self, mut itemNum: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>> {
        return ABCustomItem::getNullInstance();
    }
    

    pub fn getWidth(&mut self) -> i32 {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
    }
    

    pub fn getTitle(&mut self) -> String {
        return jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
    }
}
impl std::ops::Deref for ABCustomForm {
    type Target = CustomScreen;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ABCustomForm {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait ABCustomFormDyn: crate::org::allbinary::graphics::form::CustomScreen::CustomScreenDyn {
    fn as_ABCustomForm(&self) -> &ABCustomForm;
    fn as_ABCustomForm_mut(&mut self) -> &mut ABCustomForm;
    fn getAllitems(&mut self) -> Vec<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>> { ABCustomForm::getAllitems(self.as_ABCustomForm_mut()) }
    fn size(&mut self) -> i32 { ABCustomForm::size(self.as_ABCustomForm_mut()) }
    fn setSelectedIndex(&mut self, mut index: i32) { ABCustomForm::setSelectedIndex(self.as_ABCustomForm_mut(), index) }
    fn getSelectedIndex(&mut self) -> i32 { ABCustomForm::getSelectedIndex(self.as_ABCustomForm_mut()) }
    fn deleteAll(&mut self) { ABCustomForm::deleteAll(self.as_ABCustomForm_mut()) }
    fn delete(&mut self, mut itemNum: i32) { ABCustomForm::delete(self.as_ABCustomForm_mut(), itemNum) }
    fn append(&mut self, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>) -> i32 { ABCustomForm::append(self.as_ABCustomForm_mut(), item) }
    fn insert(&mut self, mut itemNum: i32, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>) { ABCustomForm::insert(self.as_ABCustomForm_mut(), itemNum, item) }
    fn set(&mut self, mut itemNum: i32, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>) { ABCustomForm::set(self.as_ABCustomForm_mut(), itemNum, item) }
    fn get(&mut self, mut itemNum: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>> { ABCustomForm::get(self.as_ABCustomForm_mut(), itemNum) }
    fn getWidth(&mut self) -> i32 { ABCustomForm::getWidth(self.as_ABCustomForm_mut()) }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { ABCustomForm::paint(self.as_ABCustomForm_mut(), graphics) }
    fn getTitle(&mut self) -> String { ABCustomForm::getTitle(self.as_ABCustomForm_mut()) }
}
impl ABCustomFormDyn for ABCustomForm {
    fn as_ABCustomForm(&self) -> &ABCustomForm { self }
    fn as_ABCustomForm_mut(&mut self) -> &mut ABCustomForm { self }
}

impl crate::org::allbinary::graphics::form::CustomDisplayable::CustomDisplayableDyn for ABCustomForm {
    fn as_CustomDisplayable(&self) -> &crate::org::allbinary::graphics::form::CustomDisplayable::CustomDisplayable { &self.base.base }
    fn as_CustomDisplayable_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::CustomDisplayable::CustomDisplayable { &mut self.base.base }
}
impl crate::org::allbinary::graphics::form::CustomScreen::CustomScreenDyn for ABCustomForm {
    fn as_CustomScreen(&self) -> &crate::org::allbinary::graphics::form::CustomScreen::CustomScreen { &self.base }
    fn as_CustomScreen_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::CustomScreen::CustomScreen { &mut self.base }
}


