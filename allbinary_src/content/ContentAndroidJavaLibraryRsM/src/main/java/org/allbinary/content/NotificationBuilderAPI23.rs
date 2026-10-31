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
// use crate::javax::microedition::lcdui::Command::Command;
use crate::android::app::Notification::Notification;
use crate::android::app::Notification::Builder::Builder;
use crate::android::app::PendingIntent::PendingIntent;
use crate::android::content::Context::Context;
use crate::android::os::Parcelable::Parcelable;
use super::NotificationBuilder::NotificationBuilder;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NotificationBuilderAPI23 {
    pub base: NotificationBuilder,
}


impl NotificationBuilderAPI23 {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: NotificationBuilder::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn build(&mut self, mut context: Option<Rc<RefCell<Context>>>, mut command: Option<Rc<RefCell<Command>>>, mut message: String, mut integer: Option<i32>, mut pendingIntent: Option<Rc<RefCell<PendingIntent>>>) -> Option<Rc<RefCell<Parcelable>>> {
        let mut notification: Option<Rc<RefCell<Notification>>> = jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(Builder::new(context.clone())).setSmallIcon(((integer).unwrap() as i32))).setTicker(message.clone())).setWhen((std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as i64))).setContentTitle(jcall!(command).getLabel())).setContentText(message.clone())).setContentIntent(pendingIntent.clone())).build();
        return notification.clone();
    }
}
impl std::ops::Deref for NotificationBuilderAPI23 {
    type Target = NotificationBuilder;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for NotificationBuilderAPI23 {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::content::NotificationBuilder::NotificationBuilderDyn for NotificationBuilderAPI23 {
    fn as_NotificationBuilder(&self) -> &crate::org::allbinary::content::NotificationBuilder::NotificationBuilder { &self.base }
    fn as_NotificationBuilder_mut(&mut self) -> &mut crate::org::allbinary::content::NotificationBuilder::NotificationBuilder { &mut self.base }
    fn build(&mut self, mut context: Option<Rc<RefCell<Context>>>, mut command: Option<Rc<RefCell<Command>>>, mut message: String, mut integer: Option<i32>, mut pendingIntent: Option<Rc<RefCell<PendingIntent>>>) -> Option<Rc<RefCell<Parcelable>>> { Self::build(self, context, command, message, integer, pendingIntent) }
}


