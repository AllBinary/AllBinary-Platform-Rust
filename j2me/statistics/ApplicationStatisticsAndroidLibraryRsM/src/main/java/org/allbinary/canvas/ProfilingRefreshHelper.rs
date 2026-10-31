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
// package org.allbinary.canvas
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::android::view::View::View;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use super::AndroidGameStatistics::AndroidGameStatistics;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ProfilingRefreshHelper {
    pub base: AndroidGameStatistics,
    pub firstTime: bool,
    pub bestFrameProcessingTime: i32,
    pub worstFrameProcessingTime: i32,
    pub frameProcessingTimeElapsed: i64,
}

thread_local! { pub static ProfilingRefreshHelper_instance: Option<Rc<RefCell<ProfilingRefreshHelper>>> = ProfilingRefreshHelper::new(); }

impl ProfilingRefreshHelper {
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn crate::org::allbinary::canvas::AndroidGameStatistics::AndroidGameStatisticsDyn>>> {
        return ProfilingRefreshHelper_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: AndroidGameStatistics::new_struct(),
            firstTime: false,
            bestFrameProcessingTime: 0,
            worstFrameProcessingTime: 0,
            frameProcessingTimeElapsed: 0_i64,
        };
        this.firstTime = true;
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn initView(&mut self, mut view: Option<Rc<RefCell<View>>>) {
        self.base.initView(view.clone());
        self.frameProcessingTimeElapsed = jcall!(self.getTimeDelayHelper()).getStartTime();
        self.bestFrameProcessingTime = i32::MAX.clone();
        self.worstFrameProcessingTime = 0;
    }
    

    pub fn nextFrame(&mut self) {
        self.frameProcessingTimeElapsed = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as i64) - self.frameProcessingTimeElapsed;
        if self.firstTime != true {
            if self.frameProcessingTimeElapsed > ((self.worstFrameProcessingTime) as i64) {
                self.worstFrameProcessingTime = ((self.frameProcessingTimeElapsed) as i32);
            }
            if self.frameProcessingTimeElapsed < ((self.bestFrameProcessingTime) as i64) {
                self.bestFrameProcessingTime = ((self.frameProcessingTimeElapsed) as i32);
            }
        } else {
            self.firstTime = false;
        }
        self.base.nextFrame();
    }
    

    pub fn toString(&mut self) -> String {
        return jcall!(StringMaker::new()).append(self.base.toString()).append(String::from(" Worst: ")).appendint(self.worstFrameProcessingTime.clone()).append(String::from(" Best: ")).appendint(self.bestFrameProcessingTime.clone()).toString();
    }
}
impl std::ops::Deref for ProfilingRefreshHelper {
    type Target = AndroidGameStatistics;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ProfilingRefreshHelper {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::canvas::AndroidGameStatistics::AndroidGameStatisticsDyn for ProfilingRefreshHelper {
    fn as_AndroidGameStatistics(&self) -> &crate::org::allbinary::canvas::AndroidGameStatistics::AndroidGameStatistics { &self.base }
    fn as_AndroidGameStatistics_mut(&mut self) -> &mut crate::org::allbinary::canvas::AndroidGameStatistics::AndroidGameStatistics { &mut self.base }
    fn initView(&mut self, mut view: Option<Rc<RefCell<View>>>) { Self::initView(self, view) }
}
impl crate::org::allbinary::canvas::BaseGameStatistics::BaseGameStatisticsDyn for ProfilingRefreshHelper {
    fn as_BaseGameStatistics(&self) -> &crate::org::allbinary::canvas::BaseGameStatistics::BaseGameStatistics { &self.base.base }
    fn as_BaseGameStatistics_mut(&mut self) -> &mut crate::org::allbinary::canvas::BaseGameStatistics::BaseGameStatistics { &mut self.base.base }
    fn process(&mut self) { crate::org::allbinary::canvas::AndroidGameStatistics::AndroidGameStatistics::process(crate::org::allbinary::canvas::AndroidGameStatistics::AndroidGameStatisticsDyn::as_AndroidGameStatistics_mut(self)) }
    fn nextFrame(&mut self) { Self::nextFrame(self) }
    fn toString(&mut self) -> String { Self::toString(self) }
}


