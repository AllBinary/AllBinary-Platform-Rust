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
// package org.allbinary.game.score.displayable
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::J2MEUtil::J2MEUtil;
use crate::org::allbinary::logic::ABSystemWrapper::ABSystemWrapper;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::thread::ARunnable::ARunnable;
use crate::java::lang::Exception::*;
use super::HighScoresCanvas::HighScoresCanvas;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct HighScoreRunnable {
    pub base: ARunnable,
    pub highScoresCanvas: Option<Rc<RefCell<HighScoresCanvas>>>,
}


impl HighScoreRunnable {
    

    pub fn new_struct(mut highScoresCanvas: Option<Rc<RefCell<HighScoresCanvas>>>) -> Self {
        let mut this = Self {
            base: ARunnable::new_struct(),
            highScoresCanvas: None,
        };
        this.highScoresCanvas = highScoresCanvas.clone();
        this
    }

    pub fn new(mut highScoresCanvas: Option<Rc<RefCell<HighScoresCanvas>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(highScoresCanvas))))
    }
    

    pub fn run(&mut self) {
        let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
        let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
        let mut highScoresCanvas: Option<Rc<RefCell<HighScoresCanvas>>> = self.highScoresCanvas.clone();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut isHTML: bool = J2MEUtil::isHTML();
            if !isHTML {
                while !{ let java_field = jcall!(highScoresCanvas).hasPainted.clone(); java_field } {
                }
                jcall!(highScoresCanvas).hasPainted = false;
            }
            let mut stringMaker: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            let mut systemWrapper: Option<Rc<RefCell<ABSystemWrapper>>> = ABSystemWrapper::getInstance();
            let mut currentTimeMillis: i64 = jcall!(systemWrapper).currentTimeMillis();
            jcall!(logUtil).putF(jcall!(stringMaker).append(String::from("HighScoresCanvas - Request repaint to be sure: ")).appendlong(currentTimeMillis.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(commonStrings).RUN.clone(); java_field }.clone());
            jcall!({ let java_field = jcall!(highScoresCanvas).repaintBehavior.clone(); java_field }).onChangeRepaint(highScoresCanvas.clone().map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Canvas::CanvasDyn>>));
            if !isHTML {
                while !{ let java_field = jcall!(highScoresCanvas).hasPainted.clone(); java_field } {
                }
            }
            jcall!(stringMaker).delete(0, jcall!(stringMaker).length());
            jcall!(logUtil).putF(jcall!(stringMaker).append(String::from("HighScoresCanvas - Now that the canvas has completed repaint go ahead and fetch the scores: ")).appendlong(currentTimeMillis.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(commonStrings).RUN.clone(); java_field }.clone());
            jcall!(highScoresCanvas).executeUpdate();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(logUtil).put({ let java_field = jcall!(commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(commonStrings).RUN.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
}
impl std::ops::Deref for HighScoreRunnable {
    type Target = ARunnable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for HighScoreRunnable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::thread::ARunnable::ARunnableDyn for HighScoreRunnable {
    fn as_ARunnable(&self) -> &crate::org::allbinary::thread::ARunnable::ARunnable { &self.base }
    fn as_ARunnable_mut(&mut self) -> &mut crate::org::allbinary::thread::ARunnable::ARunnable { &mut self.base }
}
impl crate::Runnable for HighScoreRunnable {
    fn run(&mut self) { Self::run(self) }
}


