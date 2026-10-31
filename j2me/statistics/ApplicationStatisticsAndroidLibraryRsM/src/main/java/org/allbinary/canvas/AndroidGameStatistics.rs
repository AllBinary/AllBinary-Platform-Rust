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
use crate::org::allbinary::android::view::BaseViewWrapper::BaseViewWrapper;
use crate::org::allbinary::android::view::ViewWrapper::ViewWrapper;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use super::BaseGameStatistics::BaseGameStatistics;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AndroidGameStatistics {
    pub base: BaseGameStatistics,
    pub view: Option<Rc<RefCell<dyn crate::org::allbinary::android::view::BaseViewWrapper::BaseViewWrapperDyn>>>,
    pub totalOnDraws: i64,
    pub ANDROID_STRING_ARRAY: Vec<String>,
    pub TOTAL_ONDRAWS: String,
    pub ONDRAWS_RATE: String,
}


impl AndroidGameStatistics {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: BaseGameStatistics::new_struct(),
            view: None,
            totalOnDraws: 0_i64,
            ANDROID_STRING_ARRAY: Vec::new(),
            TOTAL_ONDRAWS: String::new(),
            ONDRAWS_RATE: String::new(),
        };
        this.view = crate::org::allbinary::android::view::BaseViewWrapper::BaseViewWrapper_NULL_VIEW_WRAPPER.with(Clone::clone).clone();
        this.ANDROID_STRING_ARRAY = vec![String::new(); (14) as usize];
        this.TOTAL_ONDRAWS = String::from(" Total onDraws: ");
        this.ONDRAWS_RATE = String::from(" onDraws(/10) Sec: ");
        let mut stringUtil: Option<Rc<RefCell<StringUtil>>> = StringUtil::getInstance();
        for index in (0..(this.ANDROID_STRING_ARRAY.len() as i32)) {
            this.ANDROID_STRING_ARRAY[(index) as usize] = jcall!(stringUtil).EMPTY_STRING.clone();
        }
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn initView(&mut self, mut view: Option<Rc<RefCell<View>>>) {
        self.base.init();
        self.view = ViewWrapper::new(view.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::android::view::BaseViewWrapper::BaseViewWrapperDyn>>);
        self.totalOnDraws = 0_i64;
    }
    

    pub fn process(&mut self) {
        jcall!(self.view).postInvalidate();
    }
    

    pub fn nextOnDraw(&mut self) {
        self.totalOnDraws += 1;
    }
    

    pub fn toStringArray(&mut self) -> Vec<String> {
        let mut stringArray: Vec<String> = self.base.toStringArray();
        for index in (0..(stringArray.len() as i32)) {
            self.ANDROID_STRING_ARRAY[(index) as usize] = stringArray[(index) as usize].clone();
        }
        let mut totalTime: i64 = jcall!(self.getTimeDelayHelper()).getElapsed(jcall!(self.gameTickTimeDelayHelper).startTime.clone());
        totalTime = (totalTime / 10000_i64);
        self.ANDROID_STRING_ARRAY[(10) as usize] = self.TOTAL_ONDRAWS.clone();
        self.ANDROID_STRING_ARRAY[(11) as usize] = (self.totalOnDraws.clone()).to_string();
        self.ANDROID_STRING_ARRAY[(12) as usize] = self.ONDRAWS_RATE.clone();
        self.ANDROID_STRING_ARRAY[(13) as usize] = (self.totalOnDraws / totalTime).to_string();
        return self.ANDROID_STRING_ARRAY.clone();
    }
    

    pub fn toString(&mut self) -> String {
        let mut totalTime: i64 = jcall!(self.getTimeDelayHelper()).getElapsed(jcall!(self.gameTickTimeDelayHelper).startTime.clone());
        totalTime = (totalTime / 1000_i64);
        if totalTime > 0_i64 {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(self.base.toStringAt(totalTime.clone()));
            if self.totalOnDraws > 0_i64 {
                jcall!(stringBuffer).append(self.TOTAL_ONDRAWS.clone());
                jcall!(stringBuffer).append((self.totalOnDraws.clone()).to_string());
                jcall!(stringBuffer).append(self.ONDRAWS_RATE.clone());
                let mut drawsOverTime: String = (self.totalOnDraws / totalTime).to_string();
                jcall!(stringBuffer).append(drawsOverTime.clone());
            }
            return jcall!(stringBuffer).toString();
        } else {
            return self.NOT.clone();
        }
    }
}
impl std::ops::Deref for AndroidGameStatistics {
    type Target = BaseGameStatistics;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AndroidGameStatistics {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait AndroidGameStatisticsDyn: crate::org::allbinary::canvas::BaseGameStatistics::BaseGameStatisticsDyn {
    fn as_AndroidGameStatistics(&self) -> &AndroidGameStatistics;
    fn as_AndroidGameStatistics_mut(&mut self) -> &mut AndroidGameStatistics;
    fn initView(&mut self, mut view: Option<Rc<RefCell<View>>>) { AndroidGameStatistics::initView(self.as_AndroidGameStatistics_mut(), view) }
    fn nextOnDraw(&mut self) { AndroidGameStatistics::nextOnDraw(self.as_AndroidGameStatistics_mut()) }
}
impl AndroidGameStatisticsDyn for AndroidGameStatistics {
    fn as_AndroidGameStatistics(&self) -> &AndroidGameStatistics { self }
    fn as_AndroidGameStatistics_mut(&mut self) -> &mut AndroidGameStatistics { self }
}

impl crate::org::allbinary::canvas::BaseGameStatistics::BaseGameStatisticsDyn for AndroidGameStatistics {
    fn as_BaseGameStatistics(&self) -> &crate::org::allbinary::canvas::BaseGameStatistics::BaseGameStatistics { &self.base }
    fn as_BaseGameStatistics_mut(&mut self) -> &mut crate::org::allbinary::canvas::BaseGameStatistics::BaseGameStatistics { &mut self.base }
    fn process(&mut self) { Self::process(self) }
    fn toStringArray(&mut self) -> Vec<String> { Self::toStringArray(self) }
    fn toString(&mut self) -> String { Self::toString(self) }
}


