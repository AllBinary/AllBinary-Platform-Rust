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
// package org.allbinary.graphics.canvas.transition.progress
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::android::app::Activity::Activity;
use super::ProgressRunnable::ProgressRunnable;
use super::ProgressCanvas::ProgressCanvas;
use super::AndroidBasicTitleProgressBar::AndroidBasicTitleProgressBar;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TitleProgressBarPortionSetProgressRunnable {
    pub base: ProgressRunnable,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}


impl TitleProgressBarPortionSetProgressRunnable {
    

    pub fn new_struct(mut midletActivity: Option<Rc<RefCell<Activity>>>, mut progressCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvasDyn>>>) -> Self {
        let mut this = Self {
            base: ProgressRunnable::new_struct(midletActivity.clone(), progressCanvas.clone()),
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut midletActivity: Option<Rc<RefCell<Activity>>>, mut progressCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvasDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(midletActivity, progressCanvas))))
    }
    

    pub fn run(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut progressCanvas: Option<Rc<RefCell<AndroidBasicTitleProgressBar>>> = (self.progressCanvas).clone() /* as AndroidBasicTitleProgressBar */;
            { let java_receiver = self.midletActivity.clone(); let java_arguments = ((((jcall!(self.progressCanvas).getValue() + jcall!(self.progressCanvas).getMaxValue() / jcall!(progressCanvas).as_ProgressCanvas_mut().getPortion())) as i32), jcall!(self.progressCanvas).getText(),); jcall!(java_receiver).onSetProgress(java_arguments.0, java_arguments.1); };
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(self.logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).RUN.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
}
impl std::ops::Deref for TitleProgressBarPortionSetProgressRunnable {
    type Target = ProgressRunnable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for TitleProgressBarPortionSetProgressRunnable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::canvas::transition::progress::ProgressRunnable::ProgressRunnableDyn for TitleProgressBarPortionSetProgressRunnable {
    fn as_ProgressRunnable(&self) -> &crate::org::allbinary::graphics::canvas::transition::progress::ProgressRunnable::ProgressRunnable { &self.base }
    fn as_ProgressRunnable_mut(&mut self) -> &mut crate::org::allbinary::graphics::canvas::transition::progress::ProgressRunnable::ProgressRunnable { &mut self.base }
}
impl crate::org::allbinary::thread::ARunnable::ARunnableDyn for TitleProgressBarPortionSetProgressRunnable {
    fn as_ARunnable(&self) -> &crate::org::allbinary::thread::ARunnable::ARunnable { &self.base.base }
    fn as_ARunnable_mut(&mut self) -> &mut crate::org::allbinary::thread::ARunnable::ARunnable { &mut self.base.base }
}
impl crate::Runnable for TitleProgressBarPortionSetProgressRunnable {
    fn run(&mut self) { Self::run(self) }
}


