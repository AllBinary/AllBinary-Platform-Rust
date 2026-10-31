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
use crate::android::app::Activity::Activity;
use crate::org::allbinary::android::activity::MidletActivityBase::MidletActivityBase;
use crate::org::allbinary::android::activity::SimpleProgressActivityInterface::SimpleProgressActivityInterface;
use crate::org::allbinary::thread::ARunnable::ARunnable;
use super::ProgressCanvas::ProgressCanvas;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ProgressRunnable {
    pub base: ARunnable,
    pub midletActivity: Option<Rc<RefCell<dyn SimpleProgressActivityInterface>>>,
    pub progressCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvasDyn>>>,
}


impl ProgressRunnable {
    

    pub fn new_struct(mut midletActivity: Option<Rc<RefCell<Activity>>>, mut progressCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvasDyn>>>) -> Self {
        let mut this = Self {
            base: ARunnable::new_struct(),
            midletActivity: None,
            progressCanvas: None,
        };
        let mut midletActivity2: Option<Rc<RefCell<dyn crate::org::allbinary::android::activity::MidletActivityBase::MidletActivityBaseDyn>>> = (midletActivity).clone() /* as MidletActivityBase */;
        this.midletActivity = (midletActivity2).clone() /* as SimpleProgressActivityInterface */;
        this.progressCanvas = progressCanvas.clone();
        this
    }

    pub fn new(mut midletActivity: Option<Rc<RefCell<Activity>>>, mut progressCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvasDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(midletActivity, progressCanvas))))
    }
    

    pub fn run(&mut self) {
    }
}
impl std::ops::Deref for ProgressRunnable {
    type Target = ARunnable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ProgressRunnable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait ProgressRunnableDyn: crate::org::allbinary::thread::ARunnable::ARunnableDyn {
    fn as_ProgressRunnable(&self) -> &ProgressRunnable;
    fn as_ProgressRunnable_mut(&mut self) -> &mut ProgressRunnable;
}
impl ProgressRunnableDyn for ProgressRunnable {
    fn as_ProgressRunnable(&self) -> &ProgressRunnable { self }
    fn as_ProgressRunnable_mut(&mut self) -> &mut ProgressRunnable { self }
}

impl crate::org::allbinary::thread::ARunnable::ARunnableDyn for ProgressRunnable {
    fn as_ARunnable(&self) -> &crate::org::allbinary::thread::ARunnable::ARunnable { &self.base }
    fn as_ARunnable_mut(&mut self) -> &mut crate::org::allbinary::thread::ARunnable::ARunnable { &mut self.base }
}
impl crate::Runnable for ProgressRunnable {
    fn run(&mut self) { Self::run(self) }
}


