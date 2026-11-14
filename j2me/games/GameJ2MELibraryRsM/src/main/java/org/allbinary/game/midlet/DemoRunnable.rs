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
// package org.allbinary.game.midlet
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use crate::Runnable;
use super::*;
use crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvasFactory::ProgressCanvasFactory;
use crate::org::allbinary::graphics::displayable::command::MyCommandsFactory::MyCommandsFactory;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::java::lang::Exception::*;
use super::DemoGameMidlet::DemoGameMidlet;
use super::DemoGameMidletEvent::DemoGameMidletEvent;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct DemoRunnable {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub demoGameMidlet: Option<Rc<RefCell<dyn crate::org::allbinary::game::midlet::DemoGameMidlet::DemoGameMidletDyn>>>,
    pub startDemoGameMidletEvent: Option<Rc<RefCell<DemoGameMidletEvent>>>,
}

impl Runnable for DemoRunnable {
    fn run(&mut self) { Self::run(self) }
}

impl DemoRunnable {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn new_struct(mut demoGameMidlet: Option<Rc<RefCell<dyn crate::org::allbinary::game::midlet::DemoGameMidlet::DemoGameMidletDyn>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            demoGameMidlet: None,
            startDemoGameMidletEvent: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.demoGameMidlet = demoGameMidlet.clone();
        this.startDemoGameMidletEvent = { let java_value = (DemoGameMidletEvent::new(crate::java_into_object(this.demoGameMidlet.clone()), { let java_field = jcall!(DemoGameMidletStateFactory::DemoGameMidletStateFactory::getInstance()).START_DEMO.clone(); java_field }.clone())).clone(); java_value };
        this
    }

    pub fn new(mut demoGameMidlet: Option<Rc<RefCell<dyn crate::org::allbinary::game::midlet::DemoGameMidlet::DemoGameMidletDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(demoGameMidlet))))
    }
    

    pub fn run(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append({ let java_field = jcall!(CommonLabels::getInstance()).START_LABEL.clone(); java_field }.clone()).append(String::from("GameCanvasRunnableInterface")).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).RUN.clone(); java_field }.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            { let java_receiver = self.demoGameMidlet.clone(); let java_arguments = ({ let java_field = jcall!(MyCommandsFactory::getInstance()).SET_DISPLAYABLE.clone(); java_field }.clone(), ProgressCanvasFactory::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Displayable::DisplayableDyn>>),); jcall!(java_receiver).as_DemoGameMidlet_mut().commandAction(java_arguments.0, java_arguments.1); };
            { let java_receiver = self.demoGameMidlet.clone(); let java_arguments = (jcall!(self.demoGameMidlet).createDemoGameCanvasRunnableInterface(),); jcall!(java_receiver).setGameCanvasRunnableInterface(java_arguments.0); };
            jcall!(self.demoGameMidlet).demoSetup();
            jcall!(DemoGameMidletEventHandler::DemoGameMidletEventHandler::getInstance()).fireEvent(self.startDemoGameMidletEvent.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>));
            jcall!(self.demoGameMidlet).startGameCanvasRunnableInterface();
            jcall!(self.demoGameMidlet).postDemoSetup();
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

