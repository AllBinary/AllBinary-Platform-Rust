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
use super::*;
use crate::org::allbinary::graphics::displayable::GameTickDisplayInfoSingleton::GameTickDisplayInfoSingleton;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::time::GameTickTimeDelayHelper::GameTickTimeDelayHelper;
use crate::org::allbinary::time::GameTickTimeDelayHelperFactory::GameTickTimeDelayHelperFactory;
use super::GameRunnable::GameRunnable;
use super::AllBinaryGameCanvas::AllBinaryGameCanvas;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GameCanvasRunnable {
    pub base: GameRunnable,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub gameTickDisplayInfoSingleton: Option<Rc<RefCell<GameTickDisplayInfoSingleton>>>,
    pub allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>,
    pub gameTickTimeDelayHelper: Option<Rc<RefCell<GameTickTimeDelayHelper>>>,
}


impl GameCanvasRunnable {
    

    pub fn new_struct(mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) -> Self {
        let mut this = Self {
            base: GameRunnable::new_struct(),
            logUtil: None,
            gameTickDisplayInfoSingleton: None,
            allBinaryGameCanvas: None,
            gameTickTimeDelayHelper: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.gameTickDisplayInfoSingleton = GameTickDisplayInfoSingleton::getInstance();
        this.gameTickTimeDelayHelper = GameTickTimeDelayHelperFactory::getInstance();
        this.allBinaryGameCanvas = allBinaryGameCanvas.clone();
        this
    }

    pub fn new(mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(allBinaryGameCanvas))))
    }
    

    pub fn run(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(jcall!(self.allBinaryGameCanvas).getLoopTimeHelperP()).setStartTime(jcall!(self.gameTickTimeDelayHelper).setStartTime());
            jcall!(self.gameTickDisplayInfoSingleton).update();
            jcall!(self.allBinaryGameCanvas).processGame();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(self.logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).RUN.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    // throws Exception
    pub fn processLoopSleep(&mut self) {
        jcall!(self.allBinaryGameCanvas).processLoopSleep();
    }
}
impl std::ops::Deref for GameCanvasRunnable {
    type Target = GameRunnable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for GameCanvasRunnable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::displayable::canvas::GameRunnable::GameRunnableDyn for GameCanvasRunnable {
    fn as_GameRunnable(&self) -> &crate::org::allbinary::game::displayable::canvas::GameRunnable::GameRunnable { &self.base }
    fn as_GameRunnable_mut(&mut self) -> &mut crate::org::allbinary::game::displayable::canvas::GameRunnable::GameRunnable { &mut self.base }
    fn processLoopSleep(&mut self) { Self::processLoopSleep(self) }
}
impl crate::Runnable for GameCanvasRunnable {
    fn run(&mut self) { Self::run(self) }
}


