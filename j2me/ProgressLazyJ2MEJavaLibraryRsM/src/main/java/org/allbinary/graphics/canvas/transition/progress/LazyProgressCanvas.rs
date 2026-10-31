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
// package org.allbinary.graphics.canvas.transition.progress
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::canvas::Processor::Processor;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::paint::NullPaintable::NullPaintable;
use crate::org::allbinary::image::ImageCacheFactory::ImageCacheFactory;
use super::ProgressCanvas::ProgressCanvas;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LazyProgressCanvas {
    pub base: ProgressCanvas,
}

impl crate::Runnable for LazyProgressCanvas {
    fn run(&mut self) { self.base.run() }
}

impl LazyProgressCanvas {
    

    pub fn new_struct(mut title: String, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Self {
        let mut this = Self {
            base: ProgressCanvas::new_struct(title.clone(), backgroundBasicColor.clone(), foregroundBasicColor.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut title: String, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(title, backgroundBasicColor, foregroundBasicColor))))
    }
    

    pub fn start(&mut self) {
        self.base.start();
        self.hasPainted = false;
    }
    

    pub fn end(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).END_METHOD_NAME.clone());
            self.endActual();
            self.paintable = self.GAUGE_PAINTABLE.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::paint::PaintableInterface::PaintableInterface>>);
            jcall!(ImageCacheFactory::getInstance()).runTask();
            jcall!(ImageCacheFactory::getInstance()).progressEnded();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).putF(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).END_METHOD_NAME.clone());
            }
        }
    }
    

    pub fn inGame(&mut self) {
        self.inGameProcessor = Processor::getInstance();
    }
    

    pub fn endFromInitialLazyLoadingComplete(&mut self) {
        self.base.endFromInitialLazyLoadingComplete();
        self.paintable = NullPaintable::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::paint::PaintableInterface::PaintableInterface>>);
    }
    

    pub fn endIfPaintedSinceStart(&mut self) {
        if match (&self.paintable, &self.GAUGE_PAINTABLE) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } && self.hasPainted {
            self.endFromInitialLazyLoadingComplete();
        }
    }
}
impl std::ops::Deref for LazyProgressCanvas {
    type Target = ProgressCanvas;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for LazyProgressCanvas {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvasDyn for LazyProgressCanvas {
    fn as_ProgressCanvas(&self) -> &crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvas { &self.base }
    fn as_ProgressCanvas_mut(&mut self) -> &mut crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvas { &mut self.base }
    fn start(&mut self) { Self::start(self) }
    fn inGame(&mut self) { Self::inGame(self) }
    fn end(&mut self) { Self::end(self) }
    fn endFromInitialLazyLoadingComplete(&mut self) { Self::endFromInitialLazyLoadingComplete(self) }
    fn endIfPaintedSinceStart(&mut self) { Self::endIfPaintedSinceStart(self) }
}
impl crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvasDyn for LazyProgressCanvas {
    fn as_RunnableCanvas(&self) -> &crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvas { &self.base.base }
    fn as_RunnableCanvas_mut(&mut self) -> &mut crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvas { &mut self.base.base }
}
impl crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn for LazyProgressCanvas {
    fn as_MyCanvas(&self) -> &crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas { &self.base.base.base }
    fn as_MyCanvas_mut(&mut self) -> &mut crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas { &mut self.base.base.base }
}


