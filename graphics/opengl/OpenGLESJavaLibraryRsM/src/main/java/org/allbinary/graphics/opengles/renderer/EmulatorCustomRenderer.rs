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
// package org.allbinary.graphics.opengles.renderer
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::khronos::egl::EGLConfig::EGLConfig;
// use crate::javax::microedition::khronos::opengles::GL10::GL10;
use crate::org::allbinary::emulator::InitEmulatorFactory::InitEmulatorFactory;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::java::lang::Thread::*;
use super::RendererStrings::RendererStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct EmulatorCustomRenderer {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub renderStrings: Option<Rc<RefCell<RendererStrings>>>,
}


impl EmulatorCustomRenderer {

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            renderStrings: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.renderStrings = RendererStrings::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn onSurfaceCreated(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut eglConfig: Option<Rc<RefCell<EGLConfig>>>) {
        jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.renderStrings).ON_SURFACE_CREATED.clone());
        let mut initEmulatorFactory: Option<Rc<RefCell<InitEmulatorFactory>>> = InitEmulatorFactory::getInstance();
        if !jcall!(initEmulatorFactory).isInitEmulator() {
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let mut WAIT_FOR_EMULATOR: String = String::from("Waiting on Emulator Initialization");
                while !jcall!(initEmulatorFactory).isInitEmulator() {
                    PreLogUtil::put(WAIT_FOR_EMULATOR.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.renderStrings).ON_SURFACE_CREATED.clone());
                    std::thread::sleep(std::time::Duration::from_millis((180) as u64));
                }
            })) {
                Ok(_) => {}
                Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.renderStrings).ON_SURFACE_CREATED.clone(), crate::java_into_object(e.clone()));
                }
            }
        }
    }
}

