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
// package org.allbinary.android.input.api3
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use crate::Runnable;
use super::*;
use crate::android::app::Activity::Activity;
use crate::android::content::Context::Context;
use crate::android::os::IBinder::IBinder;
use crate::android::view::View::View;
use crate::android::view::inputmethod::InputMethodManager::InputMethodManager;
use crate::org::allbinary::android::activity::ViewCompositeInterface::ViewCompositeInterface;
use crate::org::allbinary::logic::communication::log::ForcedLogUtil::ForcedLogUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct HideVirtualKeyboardRunnable {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub activity: Option<Rc<RefCell<Activity>>>,
}

impl Runnable for HideVirtualKeyboardRunnable {
    fn run(&mut self) { Self::run(self) }
}

impl HideVirtualKeyboardRunnable {
    

    pub fn new_struct(mut activity: Option<Rc<RefCell<Activity>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            activity: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.activity = activity.clone();
        this
    }

    pub fn new(mut activity: Option<Rc<RefCell<Activity>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(activity))))
    }
    

    pub fn run(&mut self) {
        let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut inputMethodManager: Option<Rc<RefCell<InputMethodManager>>> = (jcall!(self.activity).getSystemService(unsafe { crate::android::content::Context::Context_INPUT_METHOD_SERVICE.clone() }.clone())).clone() /* as InputMethodManager */;
            let mut viewCompositeInterface: Option<Rc<RefCell<dyn ViewCompositeInterface>>> = (self.activity).clone() /* as ViewCompositeInterface */;
            if viewCompositeInterface.is_none() {
                ForcedLogUtil::log(String::from("Activity Null"), crate::java_into_object(String::from(std::any::type_name::<Self>())));
            }
            let mut view: Option<Rc<RefCell<View>>> = jcall!(viewCompositeInterface).getView();
            if jcall!(view).getWindowToken().is_none() {
                jcall!(self.logUtil).putF(String::from("Unable to hide virtual keyboard as window token is null - This usually happens during loading when the view is not attached to a window"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).RUN.clone());
                return;
            }
            let mut token: Option<Rc<RefCell<IBinder>>> = jcall!(view).getWindowToken();
            jcall!(inputMethodManager).hideSoftInputFromWindow(token.clone(), 0);
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).RUN.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
}

