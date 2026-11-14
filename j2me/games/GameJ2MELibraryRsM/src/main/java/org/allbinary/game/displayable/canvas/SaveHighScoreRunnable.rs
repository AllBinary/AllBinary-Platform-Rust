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
// package org.allbinary.game.displayable.canvas
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use crate::Runnable;
use super::*;
use crate::org::allbinary::game::score::displayable::HighScoreTextBox::HighScoreTextBox;
use crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvas;
use crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvasFactory::ProgressCanvasFactory;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::java::lang::Exception::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SaveHighScoreRunnable {
    pub progressCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvasDyn>>>,
    pub isLast: bool,
    pub textBox: Option<Rc<RefCell<HighScoreTextBox>>>,
}

impl Runnable for SaveHighScoreRunnable {
    fn run(&mut self) { Self::run(self) }
}

impl SaveHighScoreRunnable {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn new_struct(mut isLast: bool, mut textBox: Option<Rc<RefCell<HighScoreTextBox>>>) -> Self {
        let mut this = Self {
            progressCanvas: None,
            isLast: false,
            textBox: None,
        };
        this.progressCanvas = ProgressCanvasFactory::getInstance();
        this.isLast = isLast.clone();
        this.textBox = textBox.clone();
        this
    }

    pub fn new(mut isLast: bool, mut textBox: Option<Rc<RefCell<HighScoreTextBox>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(isLast, textBox))))
    }
    

    pub fn run(&mut self) {
        let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if self.isLast {
                jcall!(self.textBox).submit();
            } else {
                jcall!(self.textBox).saveHighScore();
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(logUtil).put({ let java_field = jcall!(commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("run"), crate::java_into_object(e.clone()));
            jcall!(self.progressCanvas).end();
            }
        }
    }
}

