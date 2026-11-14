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
use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::javax::microedition::lcdui::Image::Image;
use crate::javax::microedition::lcdui::NullImage::NullImage;
use crate::android::app::Activity::Activity;
use crate::org::allbinary::AndroidUtil::AndroidUtil;
use crate::org::allbinary::image::GameFeatureImageCacheFactory::GameFeatureImageCacheFactory;
use crate::org::allbinary::image::PreResourceImageUtil::PreResourceImageUtil;
use crate::org::allbinary::logic::communication::log::ForcedLogUtil::ForcedLogUtil;
use crate::org::allbinary::logic::java::exception::ExceptionUtil::ExceptionUtil;
use crate::org::allbinary::animation::Animation::Animation;
use crate::org::allbinary::animation::NullAnimationFactory::NullAnimationFactory;
use crate::org::allbinary::game::configuration::feature::Features::Features;
use crate::org::allbinary::game::configuration::feature::MainFeatureFactory::MainFeatureFactory;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::displayable::DisplayInfoSingleton::DisplayInfoSingleton;
use crate::org::allbinary::graphics::displayable::event::DisplayChangeEvent::DisplayChangeEvent;
use crate::org::allbinary::graphics::displayable::event::DisplayChangeEventHandler::DisplayChangeEventHandler;
use crate::org::allbinary::graphics::displayable::event::DisplayChangeEventListener::DisplayChangeEventListener;
use crate::org::allbinary::image::ImageCacheFactory::ImageCacheFactory;
use crate::org::allbinary::media::image::ImageScaleUtil::ImageScaleUtil;
use crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject;
use crate::org::allbinary::animation::AnimationBehavior::AnimationBehavior;
use crate::org::allbinary::animation::image::ImageAnimation::ImageAnimation;
use crate::org::allbinary::data::resource::ResourceUtil::ResourceUtil;
use crate::org::allbinary::logic::util::event::EventStrings::EventStrings;
use crate::org::allbinary::thread::ARunnable::ARunnable;
use crate::org::allbinary::thread::NullRunnable::NullRunnable;
use crate::java::lang::Exception::*;
use super::ProgressCanvas::ProgressCanvas;
use super::ShowTitleProgressBarRunnable::ShowTitleProgressBarRunnable;
use super::DismissTitleProgressBarRunnable::DismissTitleProgressBarRunnable;
use super::TitleProgressBarSetProgressRunnable::TitleProgressBarSetProgressRunnable;
use super::TitleProgressBarPortionSetProgressRunnable::TitleProgressBarPortionSetProgressRunnable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AndroidBasicTitleProgressBar {
    pub base: ProgressCanvas,
    pub showTitleProgressBarRunnable: Option<Rc<RefCell<dyn crate::org::allbinary::thread::ARunnable::ARunnableDyn>>>,
    pub dismissTitleProgressBarRunnable: Option<Rc<RefCell<dyn crate::org::allbinary::thread::ARunnable::ARunnableDyn>>>,
    pub titleProgressDialogPortionSetProgressRunnable: Option<Rc<RefCell<dyn crate::org::allbinary::thread::ARunnable::ARunnableDyn>>>,
    pub titleProgressDialogSetProgressRunnable: Option<Rc<RefCell<dyn crate::org::allbinary::thread::ARunnable::ARunnableDyn>>>,
    pub midletActivity: Option<Rc<RefCell<Activity>>>,
    pub portion: i32,
    pub IMAGE: Vec<Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>>,
    pub image: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>,
    pub animation: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>,
}

thread_local! { pub static AndroidBasicTitleProgressBar_RESOURCE: String = String::from("ProgressImage"); }
pub static mut AndroidBasicTitleProgressBar_background: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| 0);
impl DisplayChangeEventListener for AndroidBasicTitleProgressBar {
    fn onDisplayChangeEvent(&mut self, mut displayChangeEvent: Option<Rc<RefCell<DisplayChangeEvent>>>) { Self::onDisplayChangeEvent(self, displayChangeEvent) }
}
impl crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface for AndroidBasicTitleProgressBar {
    fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { Self::onEvent(self, eventObject) }
}
impl crate::org::allbinary::thread::RunnableInterface::RunnableInterface for AndroidBasicTitleProgressBar {
    fn isRunning(&mut self) -> bool { crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvas::isRunning(crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvasDyn::as_RunnableCanvas_mut(self)) }
    fn setRunning(&mut self, p1: bool) { crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvas::setRunning(crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvasDyn::as_RunnableCanvas_mut(self), p1) }
    fn setThread(&mut self, p1: Option<Rc<RefCell<crate::java::lang::Thread::Thread>>>) { crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvas::setThread(crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvasDyn::as_RunnableCanvas_mut(self), p1) }
}
impl crate::Runnable for AndroidBasicTitleProgressBar {
    fn run(&mut self) { crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvas::run(crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvasDyn::as_RunnableCanvas_mut(self)) }
}

impl AndroidBasicTitleProgressBar {
    

    pub fn new_struct(mut title: String, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Self {
        let mut this = Self {
            base: ProgressCanvas::new_struct(title.clone(), backgroundBasicColor.clone(), foregroundBasicColor.clone()),
            showTitleProgressBarRunnable: None,
            dismissTitleProgressBarRunnable: None,
            titleProgressDialogPortionSetProgressRunnable: None,
            titleProgressDialogSetProgressRunnable: None,
            midletActivity: None,
            portion: 0,
            IMAGE: Vec::new(),
            image: None,
            animation: None,
        };
        this.showTitleProgressBarRunnable = NullRunnable::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::thread::ARunnable::ARunnableDyn>>);
        this.dismissTitleProgressBarRunnable = NullRunnable::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::thread::ARunnable::ARunnableDyn>>);
        this.titleProgressDialogPortionSetProgressRunnable = NullRunnable::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::thread::ARunnable::ARunnableDyn>>);
        this.titleProgressDialogSetProgressRunnable = NullRunnable::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::thread::ARunnable::ARunnableDyn>>);
        this.midletActivity = crate::org::allbinary::AndroidUtil::AndroidUtil_NULL_ACTIVITY.with(Clone::clone).clone();
        this.portion = 0;
        this.IMAGE = vec![None; (4) as usize];
        this.image = crate::javax::microedition::lcdui::NullImage::NullImage_NULL_IMAGE.with(Clone::clone).clone();
        this.animation = jcall!(NullAnimationFactory::getFactoryInstance()).getInstance(0);
        // super(...) is in the struct literal above
        this.init();
        this
    }

    pub fn new(mut title: String, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(title.clone(), backgroundBasicColor.clone(), foregroundBasicColor.clone()))));
            jcall!(DisplayChangeEventHandler::getInstance()).addListenerInterface(this.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface>>));
            let mut size: i32 = (jcall!(this).IMAGE.len() as i32).clone();
            for index in (0..size) {
            { let java_index = (index) as usize; jcall!(this).IMAGE[java_index] = crate::javax::microedition::lcdui::NullImage::NullImage_NULL_IMAGE.with(Clone::clone).clone(); };
        }
        this
    }
    

    pub fn init(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if unsafe { AndroidBasicTitleProgressBar_background.clone() } != 0 {
                jcall!(ResourceUtil::getInstance()).addResource(AndroidBasicTitleProgressBar_RESOURCE.with(Clone::clone).clone(), Some((unsafe { AndroidBasicTitleProgressBar_background.clone() }.clone()) as i32));
                GameFeatureImageCacheFactory::init();
                self.image = { let java_value = (jcall!(ImageCacheFactory::getInstance()).getWithKey(crate::java_into_object(AndroidBasicTitleProgressBar_RESOURCE.with(Clone::clone).clone()))).clone(); java_value };
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).INIT.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn initActivity(&mut self, mut activity: Option<Rc<RefCell<Activity>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if !match (&self.midletActivity, &activity) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                self.midletActivity = activity.clone();
                self.showTitleProgressBarRunnable = ShowTitleProgressBarRunnable::new(self.midletActivity.clone(), None /* Java this: a &mut self method has no Rc of itself */).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::thread::ARunnable::ARunnableDyn>>);
                self.dismissTitleProgressBarRunnable = DismissTitleProgressBarRunnable::new(self.midletActivity.clone(), None /* Java this: a &mut self method has no Rc of itself */).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::thread::ARunnable::ARunnableDyn>>);
                self.titleProgressDialogSetProgressRunnable = TitleProgressBarSetProgressRunnable::new(self.midletActivity.clone(), None /* Java this: a &mut self method has no Rc of itself */).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::thread::ARunnable::ARunnableDyn>>);
                self.titleProgressDialogPortionSetProgressRunnable = TitleProgressBarPortionSetProgressRunnable::new(self.midletActivity.clone(), None /* Java this: a &mut self method has no Rc of itself */).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::thread::ARunnable::ARunnableDyn>>);
                self.loadProgressImages();
            }
            self.updateCurrent();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).INIT.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn updateCurrent(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if !self.isBackground() {
                self.animation = { let java_value = (jcall!(NullAnimationFactory::getFactoryInstance()).getInstance(0)).clone(); java_value };
            } else if unsafe { AndroidBasicTitleProgressBar_background.clone() } != 0 {
                let mut displayInfo: Option<Rc<RefCell<DisplayInfoSingleton>>> = DisplayInfoSingleton::getInstance();
                let mut currentImage: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> = None;
                if jcall!(displayInfo).isPortrait() {
                    currentImage = self.getImage(0);
                } else {
                    currentImage = self.getImage(2);
                }
                if currentImage.is_none() {
                    self.animation = { let java_value = (jcall!(NullAnimationFactory::getFactoryInstance()).getInstance(0)).clone(); java_value };
                } else {
                    self.animation = { let java_value = (ImageAnimation::new(currentImage.clone(), AnimationBehavior::getInstance()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>)).clone(); java_value };
                }
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).UPDATE.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) {
        ForcedLogUtil::log({ let java_field = jcall!(EventStrings::getInstance()).PERFORMANCE_MESSAGE.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())));
    }
    

    pub fn onDisplayChangeEvent(&mut self, mut displayChangeEvent: Option<Rc<RefCell<DisplayChangeEvent>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.loadProgressImages();
            self.updateCurrent();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", { let java_field = jcall!(self.commonStrings).EXCEPTION_LABEL.clone(); java_field }, jcall!(ExceptionUtil::getInstance()).getStackTrace(e.clone())), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.canvasStrings).ON_DISPLAY_CHANGE_EVENT.clone(); java_field }.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            self.animation = { let java_value = (jcall!(NullAnimationFactory::getFactoryInstance()).getInstance(0)).clone(); java_value };
            }
        }
    }
    

    pub fn loadProgressImages(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut displayInfo: Option<Rc<RefCell<DisplayInfoSingleton>>> = DisplayInfoSingleton::getInstance();
            let mut lastWidth: i32 = jcall!(displayInfo).getLastWidth();
            let mut lastHeight: i32 = jcall!(displayInfo).getLastHeight();
            if jcall!(displayInfo).isPortraitWH(lastWidth.clone(), lastHeight.clone()) {
                self.setImages(0, lastWidth.clone(), lastHeight.clone());
            } else {
                self.setImages(2, lastWidth.clone(), lastHeight.clone());
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("IllegalArgumentException "), jcall!(ExceptionUtil::getInstance()).getStackTrace(e.clone())), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("loadProgressImages"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            self.animation = { let java_value = (jcall!(NullAnimationFactory::getFactoryInstance()).getInstance(0)).clone(); java_value };
            }
        }
    }
    

    pub fn isInitialized(&mut self) -> bool {
        if !match (&self.midletActivity, &crate::org::allbinary::AndroidUtil::AndroidUtil_NULL_ACTIVITY.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return true;
        } else {
            return false;
        }
    }
    

    pub fn start(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).putF({ let java_field = jcall!(self.commonStrings).START.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).START_METHOD_NAME.clone(); java_field }.clone());
            self.base.start();
            jcall!(self.midletActivity).runOnUiThread(self.showTitleProgressBarRunnable.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).START_METHOD_NAME.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn end(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).putF({ let java_field = jcall!(self.commonStrings).START.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).END_METHOD_NAME.clone(); java_field }.clone());
            jcall!(self.midletActivity).runOnUiThread(self.dismissTitleProgressBarRunnable.clone());
            self.base.end();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).END_METHOD_NAME.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn addEarlyPortion(&mut self, mut value: i32, mut text: String, mut index: i32) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.portion = value.clone();
            { let java_arguments = (value.clone(), text.clone(), index.clone(),); self.base.addEarlyPortion(java_arguments.0, java_arguments.1, java_arguments.2) };
            if !match (&self.midletActivity, &crate::org::allbinary::AndroidUtil::AndroidUtil_NULL_ACTIVITY.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                jcall!(self.midletActivity).runOnUiThread(self.titleProgressDialogPortionSetProgressRunnable.clone());
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.ADD_PORTION.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn addPortion(&mut self, mut value: i32, mut text: String, mut index: i32) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.portion = value.clone();
            { let java_arguments = (value.clone(), text.clone(), index.clone(),); self.base.addPortion(java_arguments.0, java_arguments.1, java_arguments.2) };
            jcall!(self.midletActivity).runOnUiThread(self.titleProgressDialogPortionSetProgressRunnable.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.ADD_PORTION.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn addNormalPortion(&mut self, mut value: i32, mut text: String) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.portion = value.clone();
            { let java_arguments = (value.clone(), text.clone(),); self.base.addNormalPortion(java_arguments.0, java_arguments.1) };
            jcall!(self.midletActivity).runOnUiThread(self.titleProgressDialogPortionSetProgressRunnable.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.ADD_PORTION.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn setValue(&mut self, mut value: i32) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            { let java_arguments = (value.clone(),); self.base.setValue(java_arguments.0) };
            jcall!(self.midletActivity).runOnUiThread(self.titleProgressDialogSetProgressRunnable.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setValue"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn waitUntilDisplayed(&mut self) {
    }
    

    // throws Exception
    pub fn setImages(&mut self, mut index: i32, mut lastWidth: i32, mut lastHeight: i32) {
        let mut image: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> = self.image.clone();
        if !match (&image, &crate::javax::microedition::lcdui::NullImage::NullImage_NULL_IMAGE.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            if jcall!(Features::getInstance()).isFeature({ let java_field = jcall!(MainFeatureFactory::getInstance()).FULL_SCREEN.clone(); java_field }.clone()) {
                if match (&self.IMAGE[(index) as usize], &crate::javax::microedition::lcdui::NullImage::NullImage_NULL_IMAGE.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                    { let java_index = (index) as usize; self.IMAGE[java_index] = jcall!(ImageScaleUtil::getInstance()).createImage2(ImageCacheFactory::getInstance(), image.clone(), ((lastWidth) as f32), ((jcall!(image).getWidth()) as f32), ((lastHeight) as f32) - 20.0_f32, ((jcall!(image).getHeight()) as f32), false); };
                }
            } else {
                let mut nextIndex: i32 = index + 1;
                if match (&self.IMAGE[(nextIndex) as usize], &crate::javax::microedition::lcdui::NullImage::NullImage_NULL_IMAGE.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                    { let java_index = (nextIndex) as usize; self.IMAGE[java_index] = jcall!(ImageScaleUtil::getInstance()).createImage2(ImageCacheFactory::getInstance(), image.clone(), ((lastWidth) as f32), ((jcall!(image).getWidth()) as f32), ((lastHeight) as f32) - 28.0_f32, ((jcall!(image).getHeight()) as f32), false); };
                }
            }
        }
    }
    

    // throws Exception
    pub fn initOpenGL(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
        jcall!(self.logUtil).putF({ let java_field = jcall!(self.commonStrings).START.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).INIT.clone(); java_field }.clone());
        self.image = { let java_value = (jcall!(GameFeatureImageCacheFactory::getInstance()).getWithKey(crate::java_into_object(AndroidBasicTitleProgressBar_RESOURCE.with(Clone::clone).clone()))).clone(); java_value };
        let mut preResourceImageUtil: Option<Rc<RefCell<PreResourceImageUtil>>> = PreResourceImageUtil::getInstance();
        let mut size: i32 = (self.IMAGE.len() as i32).clone();
        for index in (0..size) {
            if !match (&self.IMAGE[(index) as usize], &crate::javax::microedition::lcdui::NullImage::NullImage_NULL_IMAGE.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                { let java_index = (index) as usize; self.IMAGE[java_index] = jcall!(preResourceImageUtil).encapsulate(self.IMAGE[(index) as usize].clone()); };
            }
        }
        self.updateCurrent();
    }
    

    // throws Exception
    pub fn update(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
        jcall!(self.logUtil).putF({ let java_field = jcall!(self.commonStrings).START.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).UPDATE.clone(); java_field }.clone());
        self.initOpenGL(graphics.clone());
        self.image = { let java_value = (jcall!(GameFeatureImageCacheFactory::getInstance()).getWithKey(crate::java_into_object(AndroidBasicTitleProgressBar_RESOURCE.with(Clone::clone).clone()))).clone(); java_value };
        let mut preResourceImageUtil: Option<Rc<RefCell<PreResourceImageUtil>>> = PreResourceImageUtil::getInstance();
        let mut size: i32 = (self.IMAGE.len() as i32).clone();
        for index in (0..size) {
            if !match (&self.IMAGE[(index) as usize], &crate::javax::microedition::lcdui::NullImage::NullImage_NULL_IMAGE.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                jcall!(preResourceImageUtil).update(graphics.clone(), self.IMAGE[(index) as usize].clone());
            }
        }
    }
    

    // throws Exception
    pub fn getImage(&mut self, mut index: i32) -> Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> {
        let mut image: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> = crate::javax::microedition::lcdui::NullImage::NullImage_NULL_IMAGE.with(Clone::clone).clone();
        if jcall!(Features::getInstance()).isFeature({ let java_field = jcall!(MainFeatureFactory::getInstance()).FULL_SCREEN.clone(); java_field }.clone()) {
            image = { let java_value = (self.IMAGE[(index) as usize]).clone(); java_value }.clone();
        } else {
            image = { let java_value = (self.IMAGE[(index + 1) as usize]).clone(); java_value }.clone();
        }
        return image.clone();
    }
    

    pub fn paint2(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.animation).paintXY(graphics.clone(), 0, 20);
            { let java_arguments = (graphics.clone(),); self.base.paint2(java_arguments.0) };
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.canvasStrings).PAINT.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn setBackgroundResource(mut background: i32) {
        unsafe { *AndroidBasicTitleProgressBar_background = background.clone() };
    }
    

    pub fn setBackground(&mut self, mut background: bool) {
        { let java_arguments = (background.clone(),); self.base.setBackground(java_arguments.0) };
        self.updateCurrent();
    }
    

    pub fn setPortion(&mut self, mut portion: i32) {
        self.portion = portion.clone();
    }
    

    pub fn getPortion(&mut self) -> i32 {
        return self.portion.clone();
    }
}
impl std::ops::Deref for AndroidBasicTitleProgressBar {
    type Target = ProgressCanvas;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AndroidBasicTitleProgressBar {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvasDyn for AndroidBasicTitleProgressBar {
    fn as_ProgressCanvas(&self) -> &crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvas { &self.base }
    fn as_ProgressCanvas_mut(&mut self) -> &mut crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvas { &mut self.base }
    fn update(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::update(self, graphics) }
    fn start(&mut self) { Self::start(self) }
    fn end(&mut self) { Self::end(self) }
    fn addEarlyPortion(&mut self, mut value: i32, mut text: String, mut index: i32) { Self::addEarlyPortion(self, value, text, index) }
    fn addPortion(&mut self, mut value: i32, mut text: String, mut index: i32) { Self::addPortion(self, value, text, index) }
    fn addNormalPortion(&mut self, mut value: i32, mut text: String) { Self::addNormalPortion(self, value, text) }
    fn setValue(&mut self, mut value: i32) { Self::setValue(self, value) }
    fn paint2(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::paint2(self, graphics) }
    fn setBackground(&mut self, mut background: bool) { Self::setBackground(self, background) }
}
impl crate::javax::microedition::lcdui::Canvas::CanvasDyn for AndroidBasicTitleProgressBar {
    fn as_Canvas(&self) -> &crate::javax::microedition::lcdui::Canvas::Canvas { &self.base.base.base.base }
    fn as_Canvas_mut(&mut self) -> &mut crate::javax::microedition::lcdui::Canvas::Canvas { &mut self.base.base.base.base }
}
impl crate::javax::microedition::lcdui::Displayable::DisplayableDyn for AndroidBasicTitleProgressBar {
    fn as_Displayable(&self) -> &crate::javax::microedition::lcdui::Displayable::Displayable { &self.base.base.base.base.base }
    fn as_Displayable_mut(&mut self) -> &mut crate::javax::microedition::lcdui::Displayable::Displayable { &mut self.base.base.base.base.base }
    fn addCommand(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::addCommand(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn removeCommand(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::removeCommand(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn getWidth(&mut self) -> i32 { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::getWidth(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self)) }
    fn getHeight(&mut self) -> i32 { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::getHeight(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self)) }
    fn setCommandListener(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::CommandListener::CommandListener>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::setCommandListener(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn hideNotify(&mut self) { crate::javax::microedition::lcdui::Canvas::Canvas::hideNotify(crate::javax::microedition::lcdui::Canvas::CanvasDyn::as_Canvas_mut(self)) }
    fn keyPressed(&mut self, p1: i32) { crate::javax::microedition::lcdui::Canvas::Canvas::keyPressed(crate::javax::microedition::lcdui::Canvas::CanvasDyn::as_Canvas_mut(self), p1) }
    fn keyRepeated(&mut self, p1: i32) { crate::javax::microedition::lcdui::Canvas::Canvas::keyRepeated(crate::javax::microedition::lcdui::Canvas::CanvasDyn::as_Canvas_mut(self), p1) }
    fn keyReleased(&mut self, p1: i32) { crate::javax::microedition::lcdui::Canvas::Canvas::keyReleased(crate::javax::microedition::lcdui::Canvas::CanvasDyn::as_Canvas_mut(self), p1) }
    fn pointerPressed(&mut self, p1: i32, p2: i32) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::pointerPressed(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1, p2) }
    fn pointerReleased(&mut self, p1: i32, p2: i32) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::pointerReleased(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1, p2) }
    fn pointerDragged(&mut self, p1: i32, p2: i32) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::pointerDragged(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1, p2) }
    fn paintThreed(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvas::paintThreed(crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvasDyn::as_ProgressCanvas_mut(self), p1) }
    fn paint(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvas::paint(crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvasDyn::as_ProgressCanvas_mut(self), p1) }
    fn repaint(&mut self) { crate::javax::microedition::lcdui::Canvas::Canvas::repaint(crate::javax::microedition::lcdui::Canvas::CanvasDyn::as_Canvas_mut(self)) }
    fn repaint4(&mut self, p1: i32, p2: i32, p3: i32, p4: i32) { crate::javax::microedition::lcdui::Canvas::Canvas::repaint4(crate::javax::microedition::lcdui::Canvas::CanvasDyn::as_Canvas_mut(self), p1, p2, p3, p4) }
    fn sizeChanged2(&mut self, p1: i32, p2: i32) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::sizeChanged2(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1, p2) }
    fn showNotify(&mut self) { crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvas::showNotify(crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvasDyn::as_RunnableCanvas_mut(self)) }
    fn setFullScreenMode(&mut self, p1: bool) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::setFullScreenMode(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn getTypeAsInt(&mut self) -> i32 { crate::javax::microedition::lcdui::Canvas::Canvas::getTypeAsInt(crate::javax::microedition::lcdui::Canvas::CanvasDyn::as_Canvas_mut(self)) }
    fn repaintXYWH(&mut self, p1: i32, p2: i32, p3: i32, p4: i32) { crate::javax::microedition::lcdui::Canvas::Canvas::repaintXYWH(crate::javax::microedition::lcdui::Canvas::CanvasDyn::as_Canvas_mut(self), p1, p2, p3, p4) }
    fn getDisplayableType(&mut self) -> i32 { crate::javax::microedition::lcdui::Canvas::Canvas::getDisplayableType(crate::javax::microedition::lcdui::Canvas::CanvasDyn::as_Canvas_mut(self)) }
}
impl crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn for AndroidBasicTitleProgressBar {
    fn as_MyCanvas(&self) -> &crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas { &self.base.base.base }
    fn as_MyCanvas_mut(&mut self) -> &mut crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas { &mut self.base.base.base }
}
impl crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvasDyn for AndroidBasicTitleProgressBar {
    fn as_RunnableCanvas(&self) -> &crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvas { &self.base.base }
    fn as_RunnableCanvas_mut(&mut self) -> &mut crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvas { &mut self.base.base }
    fn initCommands(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::CommandListener::CommandListener>>>) { crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvas::initCommands(crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvasDyn::as_ProgressCanvas_mut(self), p1) }
}
impl crate::org::allbinary::graphics::displayable::DisplayableInterface::DisplayableInterface for AndroidBasicTitleProgressBar {
    fn addCommand(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::addCommand(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn getHeight(&mut self) -> i32 { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::getHeight(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self)) }
    fn getWidth(&mut self) -> i32 { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::getWidth(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self)) }
    fn removeCommand(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::removeCommand(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn setCommandListener(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::CommandListener::CommandListener>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::setCommandListener(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
}
impl crate::org::allbinary::graphics::paint::PaintableInterface::PaintableInterface for AndroidBasicTitleProgressBar {
    fn paint(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvas::paint(crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvasDyn::as_ProgressCanvas_mut(self), p1) }
    fn paintThreed(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvas::paintThreed(crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvasDyn::as_ProgressCanvas_mut(self), p1) }
}
impl crate::org::allbinary::graphics::font::UpdateMyFontInterface::UpdateMyFontInterface for AndroidBasicTitleProgressBar {
    fn updateMeasurement(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvas::updateMeasurement(crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvasDyn::as_ProgressCanvas_mut(self), p1) }
}
impl crate::org::allbinary::graphics::displayable::command::MyCommandInterface::MyCommandInterface for AndroidBasicTitleProgressBar {
    fn addCommand(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::addCommand(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn removeAllCommands(&mut self) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::removeAllCommands(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self)) }
}


