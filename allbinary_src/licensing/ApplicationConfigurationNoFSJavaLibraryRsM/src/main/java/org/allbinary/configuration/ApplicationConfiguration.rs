// Generated Code Do Not Modify
// package org.allbinary.configuration
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::configuration::feature::Feature::Feature;
use crate::org::allbinary::game::configuration::feature::Features::Features;
use crate::org::allbinary::game::configuration::feature::MainFeatureFactory::MainFeatureFactory;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ApplicationConfiguration {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub fullscreen: bool,
    pub showTitleBar: bool,
    pub progressBarView: bool,
}

pub static mut ApplicationConfiguration_instance: std::cell::LazyCell<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = std::cell::LazyCell::new(|| jcall!(NullUtil::getInstance()).NULL_OBJECT.clone());

impl ApplicationConfiguration {
    

    pub fn getInstance() -> Option<Rc<RefCell<ApplicationConfiguration>>> {
        if match (&unsafe { ApplicationConfiguration_instance.clone() }, &jcall!(NullUtil::getInstance()).NULL_OBJECT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            unsafe { *ApplicationConfiguration_instance = ApplicationConfiguration::new() };
        }
        return (unsafe { ApplicationConfiguration_instance.clone() }).clone() /* as ApplicationConfiguration */;
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            fullscreen: false,
            showTitleBar: false,
            progressBarView: false,
        };
        this.logUtil = LogUtil::getInstance();
        this.fullscreen = true;
        this.showTitleBar = false;
        this.progressBarView = false;
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            this.read();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(this.logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).CONSTRUCTOR.clone(), e.clone());
            }
        }
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn read(&mut self) {
        self.setFullscreen(true);
        self.setProgressBarView(false);
        self.setShowTitleBar(false);
        { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Read Configuration: "), self.toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("read"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    // throws Exception
    pub fn write(&mut self) {
    }
    

    // throws Exception
    pub fn update(&mut self, mut gameFeature: Option<Rc<RefCell<Feature>>>) {
        if match (&gameFeature, &jcall!(MainFeatureFactory::getInstance()).FULL_SCREEN) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            let mut features: Option<Rc<RefCell<Features>>> = Features::getInstance();
            if jcall!(features).isFeature(gameFeature.clone()) {
                if !self.isFullscreen() {
                    self.setFullscreen(true);
                    self.write();
                }
            } else {
                if self.isFullscreen() {
                    self.setFullscreen(false);
                    self.write();
                }
            }
        }
    }
    

    pub fn setFullscreen(&mut self, mut fullscreen: bool) {
        self.fullscreen = fullscreen.clone();
    }
    

    pub fn isFullscreen(&mut self) -> bool {
        return self.fullscreen.clone();
    }
    

    pub fn setShowTitleBar(&mut self, mut showTitleBar: bool) {
        self.showTitleBar = showTitleBar.clone();
    }
    

    pub fn isShowTitleBar(&mut self) -> bool {
        return self.showTitleBar.clone();
    }
    

    pub fn setProgressBarView(&mut self, mut progressBarView: bool) {
        self.progressBarView = progressBarView.clone();
    }
    

    pub fn isProgressBarView(&mut self) -> bool {
        return self.progressBarView.clone();
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(String::from("isFullscreen: "));
        jcall!(stringBuffer).appendboolean(self.isFullscreen());
        jcall!(stringBuffer).append(String::from(" isProgressBarView: "));
        jcall!(stringBuffer).appendboolean(self.isProgressBarView());
        jcall!(stringBuffer).append(String::from(" isShowTitleBar: "));
        jcall!(stringBuffer).appendboolean(self.isShowTitleBar());
        return jcall!(stringBuffer).toString();
    }
}

