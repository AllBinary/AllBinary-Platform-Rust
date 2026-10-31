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
// package org.allbinary.midlet
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use crate::Runnable;
use super::*;
// use crate::javax::microedition::lcdui::NullCanvas::NullCanvas;
use crate::org::allbinary::canvas::Processor::Processor;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::AllBinaryMidlet::AllBinaryMidlet;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ExitRunnable {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub midlet: Option<Rc<RefCell<dyn crate::org::allbinary::midlet::AllBinaryMidlet::AllBinaryMidletDyn>>>,
    pub processor: Option<Rc<RefCell<dyn crate::org::allbinary::canvas::Processor::ProcessorDyn>>>,
    pub isProgress: bool,
}

impl Runnable for ExitRunnable {
    fn run(&mut self) { Self::run(self) }
}

impl ExitRunnable {
    

    pub fn new_struct(mut midlet: Option<Rc<RefCell<dyn crate::org::allbinary::midlet::AllBinaryMidlet::AllBinaryMidletDyn>>>, mut processor: Option<Rc<RefCell<dyn crate::org::allbinary::canvas::Processor::ProcessorDyn>>>, mut isProgress: bool) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            midlet: None,
            processor: None,
            isProgress: false,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.midlet = midlet.clone();
        this.isProgress = isProgress.clone();
        this.processor = processor.clone();
        this
    }

    pub fn new(mut midlet: Option<Rc<RefCell<dyn crate::org::allbinary::midlet::AllBinaryMidlet::AllBinaryMidletDyn>>>, mut processor: Option<Rc<RefCell<dyn crate::org::allbinary::canvas::Processor::ProcessorDyn>>>, mut isProgress: bool) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(midlet, processor, isProgress))))
    }
    

    pub fn run(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).START_RUNNABLE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).RUN.clone());
            jcall!(self.midlet).destroyAppInRunnable(false, self.isProgress.clone());
            jcall!(self.midlet).setDisplay(crate::javax::microedition::lcdui::NullCanvas::NullCanvas_NULL_CANVAS.with(Clone::clone).clone());
            jcall!(self.midlet).as_AllBinaryMidlet_mut().notifyDestroyed();
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).END_RUNNABLE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).RUN.clone());
            jcall!(self.processor).process();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).RUN.clone(), crate::java_into_object(e.clone()));
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                jcall!(self.processor).process();
            })) {
                Ok(_) => {}
                Err(e2) => {
            let e2: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e2 as Box<dyn std::any::Any>)));

                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).RUN.clone(), crate::java_into_object(e.clone()));
                }
            }
            }
        }
    }
}

