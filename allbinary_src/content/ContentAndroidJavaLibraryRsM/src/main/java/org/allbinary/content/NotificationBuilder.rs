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
// package org.allbinary.content
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::android::NullParcelable::NullParcelable;
// use crate::javax::microedition::lcdui::Command::Command;
use crate::android::app::PendingIntent::PendingIntent;
use crate::android::content::Context::Context;
use crate::android::os::Parcelable::Parcelable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NotificationBuilder {
}

thread_local! { pub static NotificationBuilder_NULL_NOTIFICATION_BUILDER: Option<Rc<RefCell<dyn NotificationBuilderDyn>>> = NotificationBuilder::new().map(|reference| reference as Rc<RefCell<dyn NotificationBuilderDyn>>); }

impl NotificationBuilder {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn build(&mut self, mut context: Option<Rc<RefCell<Context>>>, mut command: Option<Rc<RefCell<Command>>>, mut message: String, mut integer: Option<i32>, mut pendingIntent: Option<Rc<RefCell<PendingIntent>>>) -> Option<Rc<RefCell<Parcelable>>> {
        return crate::org::allbinary::android::NullParcelable::NullParcelable_NULL_PARCELABLE.with(Clone::clone).clone();
    }
}
pub trait NotificationBuilderDyn {
    fn as_NotificationBuilder(&self) -> &NotificationBuilder;
    fn as_NotificationBuilder_mut(&mut self) -> &mut NotificationBuilder;
    fn build(&mut self, mut context: Option<Rc<RefCell<Context>>>, mut command: Option<Rc<RefCell<Command>>>, mut message: String, mut integer: Option<i32>, mut pendingIntent: Option<Rc<RefCell<PendingIntent>>>) -> Option<Rc<RefCell<Parcelable>>> { NotificationBuilder::build(self.as_NotificationBuilder_mut(), context, command, message, integer, pendingIntent) }
}
impl NotificationBuilderDyn for NotificationBuilder {
    fn as_NotificationBuilder(&self) -> &NotificationBuilder { self }
    fn as_NotificationBuilder_mut(&mut self) -> &mut NotificationBuilder { self }
}


