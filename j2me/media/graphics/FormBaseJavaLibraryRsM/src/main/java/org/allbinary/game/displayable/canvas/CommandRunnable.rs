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
use crate::javax::microedition::lcdui::Command::Command;
use crate::javax::microedition::lcdui::CommandListener::CommandListener;
use crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::java::lang::Exception::*;
use super::CommandFormInputProcessor::CommandFormInputProcessor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CommandRunnable {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub commandFormInputProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::CommandFormInputProcessor::CommandFormInputProcessorDyn>>>,
    pub command: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>,
}

impl Runnable for CommandRunnable {
    fn run(&mut self) { Self::run(self) }
}

impl CommandRunnable {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn new_struct(mut commandFormInputProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::CommandFormInputProcessor::CommandFormInputProcessorDyn>>>, mut command: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            commandFormInputProcessor: None,
            command: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.commandFormInputProcessor = commandFormInputProcessor.clone();
        this.command = command.clone();
        this
    }

    pub fn new(mut commandFormInputProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::CommandFormInputProcessor::CommandFormInputProcessorDyn>>>, mut command: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(commandFormInputProcessor, command))))
    }
    

    pub fn run(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).putF({ let java_field = jcall!(self.commonStrings).START_RUNNABLE.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).RUN.clone(); java_field }.clone());
            let mut canvas: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn>>> = jcall!(self.commandFormInputProcessor).getCanvas();
            let mut commandListener: Option<Rc<RefCell<dyn CommandListener>>> = jcall!(canvas).getCustomCommandListener();
            jcall!(commandListener).commandAction(self.command.clone(), canvas.clone().map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Displayable::DisplayableDyn>>));
            jcall!(self.logUtil).putF({ let java_field = jcall!(self.commonStrings).END_RUNNABLE.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).RUN.clone(); java_field }.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).RUN.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
}

