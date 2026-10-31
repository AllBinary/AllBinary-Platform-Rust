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
use crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvas;
use crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvasFactory::ProgressCanvasFactory;
use crate::org::allbinary::graphics::displayable::command::MyCommandsFactory::MyCommandsFactory;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use super::DemoGameMidlet::DemoGameMidlet;
use super::DemoGameMidletEvent::DemoGameMidletEvent;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CreateGameRunnable {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub demoGameMidlet: Option<Rc<RefCell<dyn crate::org::allbinary::game::midlet::DemoGameMidlet::DemoGameMidletDyn>>>,
    pub hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>,
    pub startGameMidletEvent: Option<Rc<RefCell<DemoGameMidletEvent>>>,
}

impl Runnable for CreateGameRunnable {
    fn run(&mut self) { Self::run(self) }
}

impl CreateGameRunnable {
    

    pub fn new_struct(mut demoGameMidlet: Option<Rc<RefCell<dyn crate::org::allbinary::game::midlet::DemoGameMidlet::DemoGameMidletDyn>>>, mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            demoGameMidlet: None,
            hashtable: None,
            startGameMidletEvent: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.demoGameMidlet = demoGameMidlet.clone();
        this.hashtable = hashtable.clone();
        this
    }

    pub fn new(mut demoGameMidlet: Option<Rc<RefCell<dyn crate::org::allbinary::game::midlet::DemoGameMidlet::DemoGameMidletDyn>>>, mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(demoGameMidlet, hashtable))));
            this.startGameMidletEvent = DemoGameMidletEvent::new((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()), jcall!(DemoGameMidletStateFactory::DemoGameMidletStateFactory::getInstance()).START_GAME.clone());
        this
    }
    

    pub fn run(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).START_RUNNABLE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).RUN.clone());
            let mut progressCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvasDyn>>> = ProgressCanvasFactory::getInstance();
            { let java_receiver = self.demoGameMidlet.clone(); let java_arguments = (jcall!(MyCommandsFactory::getInstance()).SET_DISPLAYABLE.clone(), progressCanvas.clone(),); jcall!(java_receiver).commandAction(java_arguments.0, java_arguments.1); };
            jcall!(self.demoGameMidlet).stopGameCanvasRunnableInterface();
            { let java_receiver = self.demoGameMidlet.clone(); let java_arguments = (jcall!(self.demoGameMidlet).createGameCanvasRunnableInterface(),); jcall!(java_receiver).setGameCanvasRunnableInterface(java_arguments.0); };
            jcall!(jcall!(self.demoGameMidlet).getGameCanvasRunnableInterface()).setLoadStateHashtable(self.hashtable.clone());
            jcall!(self.demoGameMidlet).startGameCanvasRunnableInterface();
            jcall!(DemoGameMidletEventHandler::DemoGameMidletEventHandler::getInstance()).fireEvent(self.startGameMidletEvent.clone());
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

