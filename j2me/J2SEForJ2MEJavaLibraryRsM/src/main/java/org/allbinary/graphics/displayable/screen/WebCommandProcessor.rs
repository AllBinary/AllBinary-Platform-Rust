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
// use crate::java::awt::Desktop::Desktop;
// use crate::java::net::URI::URI;
// use crate::javax::microedition::lcdui::Canvas::Canvas;
// use crate::javax::microedition::lcdui::Command::Command;
// use crate::javax::microedition::lcdui::CommandListener::CommandListener;
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
    pub list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}

thread_local! { pub static WebCommandProcessor_instance: Option<Rc<RefCell<WebCommandProcessor>>> = WebCommandProcessor::new(); }

impl WebCommandProcessor {

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            aboutCommandProcessor: None,
            list: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.aboutCommandProcessor = AboutCommandProcessor::getInstance();
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
            if Desktop::isDesktopSupported() && jcall!(Desktop::getDesktop()).isSupported(jcall!(unsafe { crate::java::awt::Desktop::Desktop_Action.clone() }).BROWSE.clone()) {
                jcall!(Desktop::getDesktop()).browse(URI::new((jcall!(self.list).removeAt(0)).clone() /* as String */));
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(self.logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).PROCESS.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
}

