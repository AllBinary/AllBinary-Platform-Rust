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
// use crate::javax::microedition::lcdui::Command::Command;
// use crate::javax::microedition::lcdui::CommandListener::CommandListener;
use crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
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
    pub command: Option<Rc<RefCell<Command>>>,
}

impl Runnable for CommandRunnable {
    fn run(&mut self) { Self::run(self) }
}

impl CommandRunnable {
    

    pub fn new_struct(mut commandFormInputProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::CommandFormInputProcessor::CommandFormInputProcessorDyn>>>, mut command: Option<Rc<RefCell<Command>>>) -> Self {
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

    pub fn new(mut commandFormInputProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::CommandFormInputProcessor::CommandFormInputProcessorDyn>>>, mut command: Option<Rc<RefCell<Command>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(commandFormInputProcessor, command))))
    }
    

    pub fn run(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).START_RUNNABLE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).RUN.clone());
            let mut canvas: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn>>> = jcall!(self.commandFormInputProcessor).getCanvas();
            let mut commandListener: Option<Rc<RefCell<CommandListener>>> = jcall!(canvas).getCustomCommandListener();
            jcall!(commandListener).commandAction(self.command.clone(), canvas.clone());
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).END_RUNNABLE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).RUN.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).RUN.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
}

