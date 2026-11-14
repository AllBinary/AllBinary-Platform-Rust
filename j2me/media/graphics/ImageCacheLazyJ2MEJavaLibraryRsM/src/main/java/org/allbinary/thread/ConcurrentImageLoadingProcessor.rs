// 
// AllBinary Open License Version 1 
// Copyright (c) 2022 AllBinary 
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
// package org.allbinary.thread
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvas;
use crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvasFactory::ProgressCanvasFactory;
use crate::org::allbinary::image::ImageCache::ImageCache;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::java::lang::Exception::*;
use super::BaseImageLoadingProcessor::BaseImageLoadingProcessor;
use super::ABRunnable::ABRunnable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ConcurrentImageLoadingProcessor {
    pub base: BaseImageLoadingProcessor,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub runnable: Option<Rc<RefCell<dyn crate::org::allbinary::thread::ABRunnable::ABRunnableDyn>>>,
}


impl ConcurrentImageLoadingProcessor {
    

    pub fn new_struct(mut imageCache: Option<Rc<RefCell<dyn crate::org::allbinary::image::ImageCache::ImageCacheDyn>>>) -> Self {
        let mut this = Self {
            base: BaseImageLoadingProcessor::new_struct(),
            logUtil: None,
            runnable: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.runnable = ImageCacheRunnable::new(imageCache.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::thread::ABRunnable::ABRunnableDyn>>);
        this
    }

    pub fn new(mut imageCache: Option<Rc<RefCell<dyn crate::org::allbinary::image::ImageCache::ImageCacheDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(imageCache))))
    }
    

    pub fn runTask(&mut self) {
        if !jcall!(self.runnable).isRunning() {
            jcall!(ImageThreadPool::ImageThreadPool::getInstance()).runTask(self.runnable.clone().map(|_java_reference| -> Rc<RefCell<dyn crate::Runnable>> { panic!("ClassCastException: the Rust trait of ABRunnable does not have Runnable as a base trait") }));
        }
    }
}
impl std::ops::Deref for ConcurrentImageLoadingProcessor {
    type Target = BaseImageLoadingProcessor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ConcurrentImageLoadingProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::thread::BaseImageLoadingProcessor::BaseImageLoadingProcessorDyn for ConcurrentImageLoadingProcessor {
    fn as_BaseImageLoadingProcessor(&self) -> &crate::org::allbinary::thread::BaseImageLoadingProcessor::BaseImageLoadingProcessor { &self.base }
    fn as_BaseImageLoadingProcessor_mut(&mut self) -> &mut crate::org::allbinary::thread::BaseImageLoadingProcessor::BaseImageLoadingProcessor { &mut self.base }
    fn runTask(&mut self) { Self::runTask(self) }
}


pub struct ImageCacheRunnable {
    pub base: ABRunnable,
    pub imageCache: Option<Rc<RefCell<dyn crate::org::allbinary::image::ImageCache::ImageCacheDyn>>>,
}

impl crate::org::allbinary::thread::RunnableInterface::RunnableInterface for ImageCacheRunnable {
    fn isRunning(&mut self) -> bool { crate::org::allbinary::thread::ABRunnable::ABRunnable::isRunning(crate::org::allbinary::thread::ABRunnable::ABRunnableDyn::as_ABRunnable_mut(self)) }
    fn setRunning(&mut self, p1: bool) { crate::org::allbinary::thread::ABRunnable::ABRunnable::setRunning(crate::org::allbinary::thread::ABRunnable::ABRunnableDyn::as_ABRunnable_mut(self), p1) }
    fn setThread(&mut self, p1: Option<Rc<RefCell<crate::java::lang::Thread::Thread>>>) { crate::org::allbinary::thread::ABRunnable::ABRunnable::setThread(crate::org::allbinary::thread::ABRunnable::ABRunnableDyn::as_ABRunnable_mut(self), p1) }
}
impl crate::Runnable for ImageCacheRunnable {
    fn run(&mut self) { Self::run(self) }
}

impl ImageCacheRunnable {
        

        pub fn new_struct(mut imageCache: Option<Rc<RefCell<dyn crate::org::allbinary::image::ImageCache::ImageCacheDyn>>>) -> Self {
            let mut this = Self {
            base: ABRunnable::new_struct(),
            imageCache: None,
            };
            this.imageCache = imageCache.clone();
            this
        }

        pub fn new(mut imageCache: Option<Rc<RefCell<dyn crate::org::allbinary::image::ImageCache::ImageCacheDyn>>>) -> Option<Rc<RefCell<Self>>> {
            Some(Rc::new(RefCell::new(Self::new_struct(imageCache))))
        }
        

        pub fn run(&mut self) {
            let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                self.setRunning(true);
                jcall!(self.imageCache).waitForLoadNow();
                jcall!(self.imageCache).loadImages();
                jcall!(self.imageCache).loadRemainingAnimations();
                self.setRunning(false);
                let mut progressCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvasDyn>>> = ProgressCanvasFactory::getInstance();
                if !{ let java_field = jcall!(progressCanvas).as_ProgressCanvas_mut().inProgress.clone(); java_field } {
                    jcall!(progressCanvas).endFromInitialLazyLoadingComplete();
                }
            })) {
                Ok(_) => {}
                Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

                self.setRunning(false);
                let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
                jcall!(logUtil).put({ let java_field = jcall!(commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(commonStrings).RUN.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
                }
            }
        }
}
impl std::ops::Deref for ImageCacheRunnable {
    type Target = ABRunnable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ImageCacheRunnable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::thread::ABRunnable::ABRunnableDyn for ImageCacheRunnable {
    fn as_ABRunnable(&self) -> &crate::org::allbinary::thread::ABRunnable::ABRunnable { &self.base }
    fn as_ABRunnable_mut(&mut self) -> &mut crate::org::allbinary::thread::ABRunnable::ABRunnable { &mut self.base }
    fn run(&mut self) { Self::run(self) }
}


