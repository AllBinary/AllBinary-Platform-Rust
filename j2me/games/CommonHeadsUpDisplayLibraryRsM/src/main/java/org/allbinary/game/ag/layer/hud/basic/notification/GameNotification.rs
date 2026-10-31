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
// package org.allbinary.game.ag.layer.hud.basic.notification
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::layer::hud::event::GameNotificationEvent::GameNotificationEvent;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::java::bool::BooleanFactory::BooleanFactory;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GameNotification {
    pub stringList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub timeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub colorList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}

thread_local! { pub static GameNotification_NULL_GAME_NOTIFICATION_EVENT: Option<Rc<RefCell<GameNotificationEvent>>> = GameNotificationEvent::new(jcall!(NullUtil::getInstance()).NULL_OBJECT.clone(), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), Some((0) as i32), jcall!(BasicColorFactory::getInstance()).RED.clone(), jcall!(BooleanFactory::getInstance()).FALSE.clone()); }

impl GameNotification {

    pub fn new_struct() -> Self {
        let mut this = Self {
            stringList: None,
            timeList: None,
            colorList: None,
        };
        this.stringList = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.timeList = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.colorList = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn add(&mut self, mut string: String, mut seconds: Option<i32>, mut basicColor: Option<Rc<RefCell<BasicColor>>>) {
        if !jcall!(self.stringList).contains(crate::java_into_object(string.clone())) {
            jcall!(self.stringList).add(crate::java_into_object(string.clone()));
            jcall!(self.timeList).add(crate::java_into_object((seconds.clone()).expect("NullPointerException")));
            jcall!(self.colorList).add(crate::java_into_object(basicColor.clone()));
        }
    }
    

    pub fn clear(&mut self) {
        jcall!(self.stringList).clear();
        jcall!(self.timeList).clear();
        jcall!(self.colorList).clear();
    }
    

    pub fn getSize(&mut self) -> i32 {
        return jcall!(self.stringList).size();
    }
}

