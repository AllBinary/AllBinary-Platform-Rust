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
// package org.allbinary.logic.java.help
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::awt::*;
// use crate::java::awt::event::ActionEvent::ActionEvent;
use crate::java::io::File::File;
use crate::java::net::URI::URI;
use crate::java::net::URL::URL;
// use crate::javax::help::CSH::CSH;
// use crate::javax::help::HelpBroker::HelpBroker;
// use crate::javax::help::HelpSet::HelpSet;
// use crate::javax::swing::*;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::java::lang::Exception::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct JavaHelpUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub point: Option<Rc<RefCell<Point>>>,
    pub dimension: Option<Rc<RefCell<Dimension>>>,
    pub contextSensitiveHelpActionEvent: Option<Rc<RefCell<ActionEvent>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
}

thread_local! { pub static JavaHelpUtil_instance: Option<Rc<RefCell<JavaHelpUtil>>> = JavaHelpUtil::new(); }

impl JavaHelpUtil {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<JavaHelpUtil>>> {
        return JavaHelpUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            point: None,
            dimension: None,
            contextSensitiveHelpActionEvent: None,
            commonStrings: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.point = Point::new(0, 0);
        this.dimension = Dimension::new(640, 480);
        this.contextSensitiveHelpActionEvent = ActionEvent::new(JLabel::new(), unsafe { crate::java::awt::event::ActionEvent::ActionEvent_ACTION_FIRST.clone() }.clone(), None);
        this.commonStrings = CommonStrings::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getHelpSet(&mut self, mut filePath: String) -> Option<Rc<RefCell<HelpSet>>> {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut helpSetURI: Option<Rc<RefCell<URI>>> = jcall!(File::new1(filePath.clone())).toURI();
            let mut helpSetURL: Option<Rc<RefCell<URL>>> = jcall!(helpSetURI).toURL();
            return Some(self.getHelpSet_URL(helpSetURL.clone()));
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            { let java_receiver = self.logUtil.clone(); let java_arguments = ({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(Self::getInstance()), String::from("set"), crate::java_into_object(e.clone()),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3); };
            return None;
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn getHelpSet_URL(&mut self, mut url: Option<Rc<RefCell<URL>>>) -> Option<Rc<RefCell<HelpSet>>> {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            return Some(HelpSet::new(None, url.clone()));
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            { let java_receiver = self.logUtil.clone(); let java_arguments = ({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(Self::getInstance()), String::from("set"), crate::java_into_object(e.clone()),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3); };
            return None;
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    pub fn show(&mut self, mut helpSet: Option<Rc<RefCell<HelpSet>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut helpBroker: Option<Rc<RefCell<HelpBroker>>> = jcall!(helpSet).createHelpBroker();
            jcall!(helpBroker).setLocation(self.point.clone());
            jcall!(helpBroker).setSize(self.dimension.clone());
            jcall!(DisplayHelpFromSource::new(helpBroker.clone())).actionPerformed(self.contextSensitiveHelpActionEvent.clone());
            { let java_receiver = self.logUtil.clone(); let java_arguments = (String::from("CSH Action"), crate::java_into_object(Self::getInstance()), String::from("show"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            { let java_receiver = self.logUtil.clone(); let java_arguments = ({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(Self::getInstance()), String::from("show"), crate::java_into_object(e.clone()),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3); };
            }
        }
    }
    

    pub fn main(mut args: Vec<String>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut javaHelpUtil: Option<Rc<RefCell<JavaHelpUtil>>> = JavaHelpUtil::getInstance();
            jcall!(javaHelpUtil).show(jcall!(javaHelpUtil).getHelpSet(String::from("G:/mnt/bc/mydev/working/automation/InputAutomationJavaApplication/AllBinaryInputAutomationHelp/AllBinaryInputAutomation.hs")));
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            }
        }
    }
}

