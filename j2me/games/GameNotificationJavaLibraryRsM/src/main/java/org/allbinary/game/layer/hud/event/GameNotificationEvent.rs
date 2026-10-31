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
// package org.allbinary.game.layer.hud.event
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GameNotificationEvent {
    pub base: AllBinaryEventObject,
    pub string: String,
    pub seconds: Option<i32>,
    pub basicColor: Option<Rc<RefCell<BasicColor>>>,
    pub removeable: Option<bool>,
}


impl GameNotificationEvent {
    

    pub fn new_struct(mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut string: String, mut seconds: Option<i32>, mut basicColor: Option<Rc<RefCell<BasicColor>>>, mut permanent: Option<bool>) -> Self {
        let mut this = Self {
            base: AllBinaryEventObject::new_struct(anyType.clone()),
            string: String::new(),
            seconds: None,
            basicColor: None,
            removeable: None,
        };
        this.string = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        this.basicColor = jcall!(BasicColorFactory::getInstance()).BLACK.clone();
        // super(...) is in the struct literal above
        this.setString(string.clone());
        this.seconds = seconds.clone();
        this.setBasicColorP(basicColor.clone());
        this.removeable = permanent.clone();
        this
    }

    pub fn new(mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut string: String, mut seconds: Option<i32>, mut basicColor: Option<Rc<RefCell<BasicColor>>>, mut permanent: Option<bool>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(anyType, string, seconds, basicColor, permanent))))
    }
    

    pub fn getString(&mut self) -> String {
        return self.string.clone();
    }
    

    pub fn getSeconds(&mut self) -> Option<i32> {
        return self.seconds.clone();
    }
    

    pub fn getBasicColorP(&mut self) -> Option<Rc<RefCell<BasicColor>>> {
        return self.basicColor.clone();
    }
    

    pub fn getPermanent(&mut self) -> Option<bool> {
        return self.removeable.clone();
    }
    

    pub fn setString(&mut self, mut string: String) {
        self.string = string.clone();
    }
    

    pub fn setBasicColorP(&mut self, mut basicColor: Option<Rc<RefCell<BasicColor>>>) {
        self.basicColor = basicColor.clone();
    }
}
impl std::ops::Deref for GameNotificationEvent {
    type Target = AllBinaryEventObject;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for GameNotificationEvent {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn for GameNotificationEvent {
    fn as_AllBinaryEventObject(&self) -> &crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject { &self.base }
    fn as_AllBinaryEventObject_mut(&mut self) -> &mut crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject { &mut self.base }
}


