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
// package org.allbinary.game.displayable.canvas
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::thread::ABRunnable::ABRunnable;
use crate::java::lang::Exception::*;
use super::DemoCanvas::DemoCanvas;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct DemoGameStartupRunnable {
    pub base: ABRunnable,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub demoCanvas: Option<Rc<RefCell<DemoCanvas>>>,
}

pub static DemoGameStartupRunnable_TYPE: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 4);
impl crate::org::allbinary::thread::RunnableInterface::RunnableInterface for DemoGameStartupRunnable {
    fn isRunning(&mut self) -> bool { crate::org::allbinary::thread::ABRunnable::ABRunnable::isRunning(crate::org::allbinary::thread::ABRunnable::ABRunnableDyn::as_ABRunnable_mut(self)) }
    fn setRunning(&mut self, p1: bool) { crate::org::allbinary::thread::ABRunnable::ABRunnable::setRunning(crate::org::allbinary::thread::ABRunnable::ABRunnableDyn::as_ABRunnable_mut(self), p1) }
    fn setThread(&mut self, p1: Option<Rc<RefCell<crate::java::lang::Thread::Thread>>>) { crate::org::allbinary::thread::ABRunnable::ABRunnable::setThread(crate::org::allbinary::thread::ABRunnable::ABRunnableDyn::as_ABRunnable_mut(self), p1) }
}
impl crate::Runnable for DemoGameStartupRunnable {
    fn run(&mut self) { Self::run(self) }
}

impl DemoGameStartupRunnable {
    

    pub fn new_struct(mut demoCanvas: Option<Rc<RefCell<DemoCanvas>>>) -> Self {
        let mut this = Self {
            base: ABRunnable::new_struct(),
            logUtil: None,
            demoCanvas: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.demoCanvas = demoCanvas.clone();
        this
    }

    pub fn new(mut demoCanvas: Option<Rc<RefCell<DemoCanvas>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(demoCanvas))))
    }
    

    pub fn run(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.setRunning(true);
            jcall!(self.demoCanvas).stopGameDemo();
            jcall!(self.demoCanvas).create();
            jcall!(self.demoCanvas).start();
            self.setRunning(false);
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            self.setRunning(false);
            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(self.logUtil).put({ let java_field = jcall!(commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(commonStrings).RUN.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn getType(&mut self) -> i32 {
        return DemoGameStartupRunnable_TYPE.with(Clone::clone).clone();
    }
}
impl std::ops::Deref for DemoGameStartupRunnable {
    type Target = ABRunnable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for DemoGameStartupRunnable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::thread::ABRunnable::ABRunnableDyn for DemoGameStartupRunnable {
    fn as_ABRunnable(&self) -> &crate::org::allbinary::thread::ABRunnable::ABRunnable { &self.base }
    fn as_ABRunnable_mut(&mut self) -> &mut crate::org::allbinary::thread::ABRunnable::ABRunnable { &mut self.base }
    fn run(&mut self) { Self::run(self) }
    fn getType(&mut self) -> i32 { Self::getType(self) }
}


