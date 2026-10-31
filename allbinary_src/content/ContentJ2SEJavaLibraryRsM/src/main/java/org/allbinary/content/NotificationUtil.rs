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
use crate::org::allbinary::data::resource::ResourceUtil::ResourceUtil;
use crate::org::allbinary::TsUtil::TsUtil;
use super::NotificationManager::NotificationManager;
use super::Context::Context;
use super::Intent::Intent;
use super::Notification::Notification;
use super::PendingIntent::PendingIntent;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

fn java_string_hash(text: &str) -> i32 {
    text.chars().fold(0i32, |hash, character| hash.wrapping_mul(31).wrapping_add(character as i32))
}

pub struct NotificationUtil {
    pub notificationManager: Option<Rc<RefCell<NotificationManager>>>,
}

thread_local! { pub static NotificationUtil_SINGLETON: Option<Rc<RefCell<NotificationUtil>>> = NotificationUtil::new(); }

impl NotificationUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            notificationManager: None,
        };
        this.notificationManager = (jcall!(jcall!(ResourceUtil::getInstance()).getContext()).getSystemService(unsafe { Context_NOTIFICATION_SERVICE.clone() }.clone())).clone() /* as NotificationManager */;
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<NotificationUtil>>> {
        return NotificationUtil_SINGLETON.with(Clone::clone).clone();
    }
    

    pub fn notify(&mut self, mut command: Option<Rc<RefCell<Command>>>, mut resource: String, mut message: String) {
        let mut context: Option<Rc<RefCell<Context>>> = jcall!(ResourceUtil::getInstance()).getContext();
        let mut intent: Option<Rc<RefCell<Intent>>> = jcall!(CommandUriAction::CommandUriAction::getInstance()).getIntent(command.clone());
        let mut integer: Option<i32> = jcall!(ResourceUtil::getInstance()).getResourceId(resource.clone());
        let mut notification: Option<Rc<RefCell<Notification>>> = Notification::new(((integer).unwrap() as i32), message.clone(), (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as i64));
        let mut pendingIntent: Option<Rc<RefCell<PendingIntent>>> = PendingIntent::getActivity(context.clone(), 0, intent.clone(), 0);
        jcall!(notification).setLatestEventInfo(context.clone(), jcall!(command).getLabel(), message.clone(), pendingIntent.clone());
        { let java_receiver = self.notificationManager.clone(); let java_arguments = (jcall!(TsUtil::getInstance()).hashCode(crate::java_into_object(command.clone())), notification.clone(),); jcall!(java_receiver).notify(java_arguments.0, java_arguments.1); };
    }
}

