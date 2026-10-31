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
// use crate::javax::microedition::lcdui::Command::Command;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use super::ABTextItem::ABTextItem;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CommandTextItem {
    pub base: ABTextItem,
    pub command: Option<Rc<RefCell<Command>>>,
}


impl CommandTextItem {
    

    pub fn new_struct(mut command: Option<Rc<RefCell<Command>>>, mut layout: i32, mut altText: String, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Self {
        let mut this = Self {
            base: ABTextItem::new_struct(jcall!(command).getLabel(), layout.clone(), altText.clone(), backgroundBasicColor.clone(), foregroundBasicColor.clone()),
            command: None,
        };
        // super(...) is in the struct literal above
        this.command = command.clone();
        this
    }

    pub fn new(mut command: Option<Rc<RefCell<Command>>>, mut layout: i32, mut altText: String, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(command, layout, altText, backgroundBasicColor, foregroundBasicColor))))
    }
    

    pub fn isFocusable(&mut self) -> bool {
        return true;
    }
    

    pub fn getCommand(&mut self) -> Option<Rc<RefCell<Command>>> {
        return self.command.clone();
    }
}
impl std::ops::Deref for CommandTextItem {
    type Target = ABTextItem;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for CommandTextItem {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::form::item::ABTextItem::ABTextItemDyn for CommandTextItem {
    fn as_ABTextItem(&self) -> &crate::org::allbinary::graphics::form::item::ABTextItem::ABTextItem { &self.base }
    fn as_ABTextItem_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::item::ABTextItem::ABTextItem { &mut self.base }
}
impl crate::org::allbinary::graphics::form::item::CustomCustomItem::CustomCustomItemDyn for CommandTextItem {
    fn as_CustomCustomItem(&self) -> &crate::org::allbinary::graphics::form::item::CustomCustomItem::CustomCustomItem { &self.base.base }
    fn as_CustomCustomItem_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::item::CustomCustomItem::CustomCustomItem { &mut self.base.base }
}
impl crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn for CommandTextItem {
    fn as_ABCustomItem(&self) -> &crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItem { &self.base.base.base }
    fn as_ABCustomItem_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItem { &mut self.base.base.base }
    fn isFocusable(&mut self) -> bool { Self::isFocusable(self) }
}


