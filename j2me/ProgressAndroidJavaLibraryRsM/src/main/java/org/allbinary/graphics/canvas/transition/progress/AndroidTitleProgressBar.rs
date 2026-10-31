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
use crate::Runnable;
use super::*;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::android::activity::NullProgressActivity::NullProgressActivity;
use crate::org::allbinary::android::activity::ProgressActivityInterface::ProgressActivityInterface;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::ProgressCanvas::ProgressCanvas;
use super::ShowTitleProgressBarRunnable::ShowTitleProgressBarRunnable;
use super::DismissTitleProgressBarRunnable::DismissTitleProgressBarRunnable;
use super::TitleProgressBarPortionSetProgressRunnable::TitleProgressBarPortionSetProgressRunnable;
use super::TitleProgressBarSetProgressRunnable::TitleProgressBarSetProgressRunnable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AndroidTitleProgressBar {
    pub base: ProgressCanvas,
    pub showTitleProgressBarRunnable: Option<Rc<RefCell<ShowTitleProgressBarRunnable>>>,
    pub dismissTitleProgressBarRunnable: Option<Rc<RefCell<DismissTitleProgressBarRunnable>>>,
    pub progressDialogPortionSetProgressRunnable: Option<Rc<RefCell<TitleProgressBarPortionSetProgressRunnable>>>,
    pub progressDialogSetProgressRunnable: Option<Rc<RefCell<TitleProgressBarSetProgressRunnable>>>,
    pub progressActivity: Option<Rc<RefCell<dyn ProgressActivityInterface>>>,
    pub portion: i32,
}

impl Runnable for AndroidTitleProgressBar {
    fn run(&mut self) { self.base.run() }
}

impl AndroidTitleProgressBar {
    

    pub fn new_struct(mut title: String, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Self {
        let mut this = Self {
            base: ProgressCanvas::new_struct(title.clone(), backgroundBasicColor.clone(), foregroundBasicColor.clone()),
            showTitleProgressBarRunnable: None,
            dismissTitleProgressBarRunnable: None,
            progressDialogPortionSetProgressRunnable: None,
            progressDialogSetProgressRunnable: None,
            progressActivity: None,
            portion: 0,
        };
        this.showTitleProgressBarRunnable = ShowTitleProgressBarRunnable::new(this);
        this.dismissTitleProgressBarRunnable = DismissTitleProgressBarRunnable::new(this);
        this.progressDialogPortionSetProgressRunnable = TitleProgressBarPortionSetProgressRunnable::new(this);
        this.progressDialogSetProgressRunnable = TitleProgressBarSetProgressRunnable::new(this);
        this.progressActivity = crate::org::allbinary::android::activity::NullProgressActivity::NullProgressActivity_NULL_PROGRESS_ACTIVITY.with(Clone::clone).clone();
        this.portion = 0;
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut title: String, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(title, backgroundBasicColor, foregroundBasicColor))))
    }
    

    pub fn init(&mut self, mut activity: Option<Rc<RefCell<dyn ProgressActivityInterface>>>) {
        self.progressActivity = activity.clone();
    }
    

    pub fn isInitialized(&mut self) -> bool {
        if !match (&self.progressActivity, &crate::org::allbinary::android::activity::NullProgressActivity::NullProgressActivity_NULL_PROGRESS_ACTIVITY.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return true;
        } else {
            return false;
        }
    }
    

    pub fn start(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).START_METHOD_NAME.clone());
            self.base.start();
            jcall!(self.progressActivity).runOnUiThread(self.showTitleProgressBarRunnable.clone().map(|reference| reference as Rc<RefCell<dyn Runnable>>));
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).START_METHOD_NAME.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn end(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).END_METHOD_NAME.clone());
            jcall!(self.progressActivity).runOnUiThread(self.dismissTitleProgressBarRunnable.clone().map(|reference| reference as Rc<RefCell<dyn Runnable>>));
            self.base.end();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).END_METHOD_NAME.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn addPortion(&mut self, mut value: i32, mut text: String, mut index: i32) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.base.addPortion(value.clone(), text.clone(), index.clone());
            self.portion = value.clone();
            jcall!(self.progressActivity).runOnUiThread(self.progressDialogPortionSetProgressRunnable.clone().map(|reference| reference as Rc<RefCell<dyn Runnable>>));
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.ADD_PORTION.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn addNormalPortion(&mut self, mut value: i32, mut text: String) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.base.addNormalPortion(value.clone(), text.clone());
            self.portion = value.clone();
            jcall!(self.progressActivity).runOnUiThread(self.progressDialogPortionSetProgressRunnable.clone().map(|reference| reference as Rc<RefCell<dyn Runnable>>));
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.ADD_PORTION.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn setValue(&mut self, mut value: i32) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.base.setValue(value.clone());
            jcall!(self.progressActivity).runOnUiThread(self.progressDialogSetProgressRunnable.clone().map(|reference| reference as Rc<RefCell<dyn Runnable>>));
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setValue"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn waitUntilDisplayed(&mut self) {
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
    }
}
impl std::ops::Deref for AndroidTitleProgressBar {
    type Target = ProgressCanvas;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AndroidTitleProgressBar {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvasDyn for AndroidTitleProgressBar {
    fn as_ProgressCanvas(&self) -> &crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvas { &self.base }
    fn as_ProgressCanvas_mut(&mut self) -> &mut crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvas { &mut self.base }
    fn start(&mut self) { Self::start(self) }
    fn end(&mut self) { Self::end(self) }
    fn addPortion(&mut self, mut value: i32, mut text: String, mut index: i32) { Self::addPortion(self, value, text, index) }
    fn addNormalPortion(&mut self, mut value: i32, mut text: String) { Self::addNormalPortion(self, value, text) }
    fn setValue(&mut self, mut value: i32) { Self::setValue(self, value) }
}
impl crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn for AndroidTitleProgressBar {
    fn as_MyCanvas(&self) -> &crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas { &self.base.base.base }
    fn as_MyCanvas_mut(&mut self) -> &mut crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas { &mut self.base.base.base }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
}
impl crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvasDyn for AndroidTitleProgressBar {
    fn as_RunnableCanvas(&self) -> &crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvas { &self.base.base }
    fn as_RunnableCanvas_mut(&mut self) -> &mut crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvas { &mut self.base.base }
}


pub struct TitleProgressBarSetProgressRunnable {
    pub androidTitleProgressBar: Option<Rc<RefCell<AndroidTitleProgressBar>>>,
}

impl Runnable for TitleProgressBarSetProgressRunnable {
    fn run(&mut self) { Self::run(self) }
}

impl TitleProgressBarSetProgressRunnable {
        

        pub fn new_struct(mut androidTitleProgressBar: Option<Rc<RefCell<AndroidTitleProgressBar>>>) -> Self {
            let mut this = Self {
            androidTitleProgressBar: None,
            };
            this.androidTitleProgressBar = androidTitleProgressBar.clone();
            this
        }

        pub fn new(mut androidTitleProgressBar: Option<Rc<RefCell<AndroidTitleProgressBar>>>) -> Option<Rc<RefCell<Self>>> {
            Some(Rc::new(RefCell::new(Self::new_struct(androidTitleProgressBar))))
        }
        

        pub fn run(&mut self) {
            let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let mut value: i32 = ((jcall!(self.androidTitleProgressBar).getValue()) as i32);
                jcall!(jcall!(self.androidTitleProgressBar).progressActivity).onTitleProgressBarSetProgress(value.clone());
            })) {
                Ok(_) => {}
                Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

                jcall!(logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).RUN.clone(), crate::java_into_object(e.clone()));
                }
            }
        }
}

pub struct TitleProgressBarPortionSetProgressRunnable {
    pub androidTitleProgressBar: Option<Rc<RefCell<AndroidTitleProgressBar>>>,
}

impl Runnable for TitleProgressBarPortionSetProgressRunnable {
    fn run(&mut self) { Self::run(self) }
}

impl TitleProgressBarPortionSetProgressRunnable {
        

        pub fn new_struct(mut androidTitleProgressBar: Option<Rc<RefCell<AndroidTitleProgressBar>>>) -> Self {
            let mut this = Self {
            androidTitleProgressBar: None,
            };
            this.androidTitleProgressBar = androidTitleProgressBar.clone();
            this
        }

        pub fn new(mut androidTitleProgressBar: Option<Rc<RefCell<AndroidTitleProgressBar>>>) -> Option<Rc<RefCell<Self>>> {
            Some(Rc::new(RefCell::new(Self::new_struct(androidTitleProgressBar))))
        }
        

        pub fn run(&mut self) {
            let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let mut value: i32 = (((jcall!(self.androidTitleProgressBar).getValue() + jcall!(self.androidTitleProgressBar).getMaxValue() / ((jcall!(self.androidTitleProgressBar).portion) as f32))) as i32);
                jcall!(jcall!(self.androidTitleProgressBar).progressActivity).onTitleProgressBarSetProgress(value.clone());
            })) {
                Ok(_) => {}
                Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

                jcall!(logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).RUN.clone(), crate::java_into_object(e.clone()));
                }
            }
        }
}

pub struct ShowTitleProgressBarRunnable {
    pub androidTitleProgressBar: Option<Rc<RefCell<AndroidTitleProgressBar>>>,
}

impl Runnable for ShowTitleProgressBarRunnable {
    fn run(&mut self) { Self::run(self) }
}

impl ShowTitleProgressBarRunnable {
        

        pub fn new_struct(mut androidTitleProgressBar: Option<Rc<RefCell<AndroidTitleProgressBar>>>) -> Self {
            let mut this = Self {
            androidTitleProgressBar: None,
            };
            this.androidTitleProgressBar = androidTitleProgressBar.clone();
            this
        }

        pub fn new(mut androidTitleProgressBar: Option<Rc<RefCell<AndroidTitleProgressBar>>>) -> Option<Rc<RefCell<Self>>> {
            Some(Rc::new(RefCell::new(Self::new_struct(androidTitleProgressBar))))
        }
        

        pub fn run(&mut self) {
            let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let mut maxValue: i32 = ((jcall!(self.androidTitleProgressBar).getMaxValue()) as i32);
                jcall!(jcall!(self.androidTitleProgressBar).progressActivity).onShowTitleProgressBar(maxValue.clone(), false);
            })) {
                Ok(_) => {}
                Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

                jcall!(logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).RUN.clone(), crate::java_into_object(e.clone()));
                }
            }
        }
}

pub struct DismissTitleProgressBarRunnable {
    pub androidTitleProgressBar: Option<Rc<RefCell<AndroidTitleProgressBar>>>,
}

impl Runnable for DismissTitleProgressBarRunnable {
    fn run(&mut self) { Self::run(self) }
}

impl DismissTitleProgressBarRunnable {
        

        pub fn new_struct(mut androidTitleProgressBar: Option<Rc<RefCell<AndroidTitleProgressBar>>>) -> Self {
            let mut this = Self {
            androidTitleProgressBar: None,
            };
            this.androidTitleProgressBar = androidTitleProgressBar.clone();
            this
        }

        pub fn new(mut androidTitleProgressBar: Option<Rc<RefCell<AndroidTitleProgressBar>>>) -> Option<Rc<RefCell<Self>>> {
            Some(Rc::new(RefCell::new(Self::new_struct(androidTitleProgressBar))))
        }
        

        pub fn run(&mut self) {
            let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                jcall!(jcall!(self.androidTitleProgressBar).progressActivity).onDismissTitleProgressBar();
            })) {
                Ok(_) => {}
                Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

                jcall!(logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).RUN.clone(), crate::java_into_object(e.clone()));
                }
            }
        }
}

