//
// AllBinary Open License Version 1
// Copyright (c) 2022 AllBinary
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
// package org.allbinary.graphics.displayable.screen
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::android::content::Context::Context;
// use crate::javax::microedition::lcdui::Canvas::Canvas;
// use crate::javax::microedition::lcdui::Command::Command;
// use crate::javax::microedition::lcdui::CommandListener::CommandListener;
use crate::android::content::Intent::Intent;
use crate::org::allbinary::TsUtil::TsUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use super::AboutCommandProcessor::AboutCommandProcessor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct WebCommandProcessor {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub aboutCommandProcessor: Option<Rc<RefCell<AboutCommandProcessor>>>,
    pub WEB_VIEW_ACTIVITY: String,
    pub list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}

thread_local! { pub static WebCommandProcessor_instance: Option<Rc<RefCell<WebCommandProcessor>>> = WebCommandProcessor::new(); }

impl WebCommandProcessor {

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            aboutCommandProcessor: None,
            WEB_VIEW_ACTIVITY: String::new(),
            list: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.aboutCommandProcessor = AboutCommandProcessor::getInstance();
        this.WEB_VIEW_ACTIVITY = String::from("org.allbinary.android.activity.WebViewActivity");
        this.list = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<WebCommandProcessor>>> {
        return WebCommandProcessor_instance.with(Clone::clone).clone();
    }
    

    pub fn process(&mut self, mut midletCommandListener: Option<Rc<RefCell<CommandListener>>>, mut command: Option<Rc<RefCell<Command>>>, mut canvas: Option<Rc<RefCell<Canvas>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut context: Option<Rc<RefCell<Context>>> = (jcall!(self.aboutCommandProcessor).getContext()).clone() /* as Context */;
            let mut intent: Option<Rc<RefCell<Intent>>> = Intent::new(context.clone(), jcall!(jcall!(TsUtil::getInstance()).getClassClassLoader(None /* Java this: a &mut self method has no Rc of itself */)).loadClass(self.WEB_VIEW_ACTIVITY.clone()));
            jcall!(intent).putExtra(jcall!(self.aboutCommandProcessor).URL.clone(), (jcall!(self.list).removeAt(0)).clone() /* as String */);
            jcall!(intent).setFlags(unsafe { crate::android::content::Intent::Intent_FLAG_ACTIVITY_NEW_TASK.clone() }.clone());
            jcall!(context).startActivity(intent.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(self.logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("process"), crate::java_into_object(e.clone()));
            }
        }
    }
}

