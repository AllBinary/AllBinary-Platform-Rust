// 
// AllBinary Open License Version 1 
// Copyright (c) 2007 AllBinary 
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
// package org.allbinary.game.input.resource
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::input::motion::button::TouchButtonResource::TouchButtonResource;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TouchButtonUpgradeResource {
    pub base: TouchButtonResource,
}

thread_local! { pub static TouchButtonUpgradeResource_instance: Option<Rc<RefCell<dyn crate::org::allbinary::input::motion::button::TouchButtonResource::TouchButtonResourceDyn>>> = TouchButtonUpgradeResource::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::input::motion::button::TouchButtonResource::TouchButtonResourceDyn>>); }

impl TouchButtonUpgradeResource {
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn crate::org::allbinary::input::motion::button::TouchButtonResource::TouchButtonResourceDyn>>> {
        return TouchButtonUpgradeResource_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: TouchButtonResource::new_struct(String::from("/touch_button_upgrade_64_by_64.png"), String::from("/touch_button_upgrade_hint_64_by_64.png")),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}
impl std::ops::Deref for TouchButtonUpgradeResource {
    type Target = TouchButtonResource;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for TouchButtonUpgradeResource {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::input::motion::button::TouchButtonResource::TouchButtonResourceDyn for TouchButtonUpgradeResource {
    fn as_TouchButtonResource(&self) -> &crate::org::allbinary::input::motion::button::TouchButtonResource::TouchButtonResource { &self.base }
    fn as_TouchButtonResource_mut(&mut self) -> &mut crate::org::allbinary::input::motion::button::TouchButtonResource::TouchButtonResource { &mut self.base }
}


