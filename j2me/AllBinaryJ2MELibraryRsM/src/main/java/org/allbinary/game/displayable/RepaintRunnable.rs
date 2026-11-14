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
// package org.allbinary.game.displayable
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::javax::microedition::lcdui::Canvas::Canvas;
use crate::org::allbinary::graphics::displayable::DisplayInfoSingleton::DisplayInfoSingleton;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::thread::ARunnable::ARunnable;
use crate::java::lang::Exception::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RepaintRunnable {
    pub base: ARunnable,
    pub canvas: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Canvas::CanvasDyn>>>,
}


impl RepaintRunnable {
    

    pub fn new_struct(mut canvas: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Canvas::CanvasDyn>>>) -> Self {
        let mut this = Self {
            base: ARunnable::new_struct(),
            canvas: None,
        };
        this.canvas = canvas.clone();
        this
    }

    pub fn new(mut canvas: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Canvas::CanvasDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(canvas))))
    }
    

    pub fn run(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.canvas).repaint();
            jcall!(DisplayInfoSingleton::getInstance()).process();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
            jcall!(logUtil).put({ let java_field = jcall!(commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(commonStrings).RUN.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
}
impl std::ops::Deref for RepaintRunnable {
    type Target = ARunnable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for RepaintRunnable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::thread::ARunnable::ARunnableDyn for RepaintRunnable {
    fn as_ARunnable(&self) -> &crate::org::allbinary::thread::ARunnable::ARunnable { &self.base }
    fn as_ARunnable_mut(&mut self) -> &mut crate::org::allbinary::thread::ARunnable::ARunnable { &mut self.base }
}
impl crate::Runnable for RepaintRunnable {
    fn run(&mut self) { Self::run(self) }
}


