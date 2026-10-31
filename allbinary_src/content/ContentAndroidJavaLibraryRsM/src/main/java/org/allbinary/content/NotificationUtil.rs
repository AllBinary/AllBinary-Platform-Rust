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
use crate::android::app::NotificationManager::NotificationManager;
use crate::android::app::PendingIntent::PendingIntent;
use crate::android::content::Context::Context;
use crate::android::content::Intent::Intent;
use crate::org::allbinary::android::AndroidInfoFactory::AndroidInfoFactory;
use crate::org::allbinary::data::resource::ResourceUtil::ResourceUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::NotificationBuilder::NotificationBuilder;
use super::NotificationBuilderAPI23::NotificationBuilderAPI23;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NotificationUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub notificationManager: Option<Rc<RefCell<NotificationManager>>>,
    pub notificationBuilder: Option<Rc<RefCell<dyn crate::org::allbinary::content::NotificationBuilder::NotificationBuilderDyn>>>,
}

thread_local! { pub static NotificationUtil_SINGLETON: Option<Rc<RefCell<NotificationUtil>>> = NotificationUtil::new(); }

impl NotificationUtil {
    

    pub fn getInstance() -> Option<Rc<RefCell<NotificationUtil>>> {
        return NotificationUtil_SINGLETON.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            notificationManager: None,
            notificationBuilder: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.notificationManager = (jcall!(jcall!(ResourceUtil::getInstance()).getContext()).getSystemService(unsafe { crate::android::content::Context::Context_NOTIFICATION_SERVICE.clone() }.clone())).clone() /* as NotificationManager */;
        let mut SDK_VERSION: i32 = jcall!(AndroidInfoFactory::getInstance()).getVersion();
        if SDK_VERSION > 22 {
            this.notificationBuilder = NotificationBuilderAPI23::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::content::NotificationBuilder::NotificationBuilderDyn>>);
        } else {
            this.notificationBuilder = NotificationBuilder_NULL_NOTIFICATION_BUILDER.with(Clone::clone).clone();
        }
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn notify(&mut self, mut command: Option<Rc<RefCell<Command>>>, mut resource: String, mut message: String) {
        let mut context: Option<Rc<RefCell<Context>>> = jcall!(ResourceUtil::getInstance()).getContext();
        let mut intent: Option<Rc<RefCell<Intent>>> = jcall!(CommandUriAction::CommandUriAction::getInstance()).getIntent(command.clone());
        let mut integer: Option<i32> = jcall!(ResourceUtil::getInstance()).getResourceId(resource.clone());
        let mut FLAG_IMMUTABLE: i32 = 1 << 26;
        let mut SDK_VERSION: i32 = jcall!(AndroidInfoFactory::getInstance()).getVersion();
        let mut pendingIntent: Option<Rc<RefCell<PendingIntent>>> = PendingIntent::getActivity(context.clone(), 0, intent.clone(), (if SDK_VERSION > 22 { FLAG_IMMUTABLE } else { 0 }));
        if self.notificationBuilder.is_none() {
            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(self.logUtil).putF(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).NOT_IMPLEMENTED.clone());
            return;
        }
        jcall!(self.notificationBuilder).build(context.clone(), command.clone(), message.clone(), integer.clone(), pendingIntent.clone());
    }
}

