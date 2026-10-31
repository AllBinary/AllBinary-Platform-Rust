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
// use crate::javax::microedition::lcdui::CommandListener::CommandListener;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
// use crate::javax::microedition::lcdui::NullCommandListener::NullCommandListener;
use crate::org::allbinary::game::init::BasicBuildGameInitializerFactory::BasicBuildGameInitializerFactory;
use crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManager;
use crate::org::allbinary::game::score::NoHighScoresFactory::NoHighScoresFactory;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::AllBinaryGameCanvas::AllBinaryGameCanvas;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NullGameCanvas {
    pub base: AllBinaryGameCanvas,
    pub runningNull: bool,
}

pub static mut NullGameCanvas_instance: std::cell::LazyCell<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = std::cell::LazyCell::new(|| jcall!(NullUtil::getInstance()).NULL_OBJECT.clone());
thread_local! { pub static NullGameCanvas_NO_GAME: String = String::from("No Background Game"); }
thread_local! { pub static NullGameCanvas_TYPE: i32 = 1; }
impl crate::org::allbinary::game::displayable::canvas::MenuListener::MenuListener for NullGameCanvas {
    fn open(&mut self) { self.base.open() }
    fn close(&mut self) { self.base.close() }
}
impl crate::org::allbinary::game::input::GameKeyEventSourceInterface::GameKeyEventSourceInterface for NullGameCanvas {
    fn getSourceId(&mut self) -> i32 { self.base.getSourceId() }
}
impl crate::Runnable for NullGameCanvas {
    fn run(&mut self) { Self::run(self) }
}

impl NullGameCanvas {
    

    pub fn createNull() -> Option<Rc<RefCell<NullGameCanvas>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            return NullGameCanvas::new(AllBinaryGameLayerManager::getNullInstance());
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(LogUtil::getInstance()).put(jcall!(CommonStrings::getInstance()).EXCEPTION.clone(), crate::java_into_object(String::from("NullGameCanvas")), jcall!(CommonStrings::getInstance()).GET_INSTANCE.clone(), crate::java_into_object(e.clone()));
            panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
            }
        }
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<NullGameCanvas>>> {
        if match (&unsafe { NullGameCanvas_instance.clone() }, &jcall!(NullUtil::getInstance()).NULL_OBJECT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            unsafe { *NullGameCanvas_instance = NullGameCanvas::createNull() };
        }
        return (unsafe { NullGameCanvas_instance.clone() }).clone() /* as NullGameCanvas */;
    }
    

    // throws Exception
    pub fn createCanvas(mut gameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>) -> Option<Rc<RefCell<NullGameCanvas>>> {
        let mut nullGameCanvas: Option<Rc<RefCell<NullGameCanvas>>> = NullGameCanvas::new(gameLayerManager.clone());
        jcall!(nullGameCanvas).setInitialized(true);
        jcall!(nullGameCanvas).setTitle(NullGameCanvas_NO_GAME.with(Clone::clone).clone());
        return nullGameCanvas.clone();
    }
    

    pub fn new_struct(mut gameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>) -> Self {
        let mut this = Self {
            base: AllBinaryGameCanvas::new_struct(crate::javax::microedition::lcdui::NullCommandListener::NullCommandListener_NULL_COMMAND_LISTENER.with(Clone::clone).clone(), gameLayerManager.clone(), NoHighScoresFactory::getInstance(), unsafe { crate::org::allbinary::game::init::BasicBuildGameInitializerFactory::BasicBuildGameInitializerFactory_NULL_BASE_BUILD_GMAE_INITIALIZER_FACTORY.clone() }.clone(), false),
            runningNull: false,
        };
        this.runningNull = true;
        // super(...) is in the struct literal above
        this.base.setWait(1200);
        this
    }

    pub fn new(mut gameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(gameLayerManager))))
    }
    

    // synchronized
    pub fn isGameOver(&mut self) -> bool {
        return false;
    }
    

    pub fn setRunning(&mut self, mut running: bool) {
        self.runningNull = running.clone();
    }
    

    // synchronized
    pub fn isRunning(&mut self) -> bool {
        return self.runningNull.clone();
    }
    

    // synchronized
    pub fn pause(&mut self) {
        self.setPaused(true);
    }
    

    // synchronized
    pub fn unPause(&mut self) {
        self.setPaused(false);
    }
    

    pub fn initCommands(&mut self, mut cmdListener: Option<Rc<RefCell<CommandListener>>>) {
        self.removeAllCommands();
        self.setCommandListener(cmdListener.clone());
    }
    

    // throws Exception
    pub fn buildGameInit(&mut self, mut isProgress: bool) {
    }
    

    pub fn draw(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
    }
    

    pub fn run(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).START_RUNNABLE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).RUN.clone());
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).END_RUNNABLE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).RUN.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).RUN.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn getType(&mut self) -> i32 {
        return NullGameCanvas_TYPE.with(Clone::clone).clone();
    }
}
impl std::ops::Deref for NullGameCanvas {
    type Target = AllBinaryGameCanvas;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for NullGameCanvas {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn for NullGameCanvas {
    fn as_AllBinaryGameCanvas(&self) -> &crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas { &self.base }
    fn as_AllBinaryGameCanvas_mut(&mut self) -> &mut crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas { &mut self.base }
    fn isGameOver(&mut self) -> bool { Self::isGameOver(self) }
    fn buildGameInit(&mut self, mut isProgress: bool) { Self::buildGameInit(self, isProgress) }
}
impl crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvasDyn for NullGameCanvas {
    fn as_RunnableCanvas(&self) -> &crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvas { &self.base.base }
    fn as_RunnableCanvas_mut(&mut self) -> &mut crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvas { &mut self.base.base }
    fn initCommands(&mut self, mut cmdListener: Option<Rc<RefCell<CommandListener>>>) { Self::initCommands(self, cmdListener) }
    fn setRunning(&mut self, mut running: bool) { Self::setRunning(self, running) }
    fn isRunning(&mut self) -> bool { Self::isRunning(self) }
    fn setCurrentThread(&mut self) { crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas::setCurrentThread(crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn::as_AllBinaryGameCanvas_mut(self)) }
    fn processSleep(&mut self) { crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas::processSleep(crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn::as_AllBinaryGameCanvas_mut(self)) }
    fn isPausable(&mut self) -> bool { crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas::isPausable(crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn::as_AllBinaryGameCanvas_mut(self)) }
    fn isSingleThread(&mut self) -> bool { crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas::isSingleThread(crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn::as_AllBinaryGameCanvas_mut(self)) }
    fn run(&mut self) { Self::run(self) }
    fn getType(&mut self) -> i32 { Self::getType(self) }
}
impl crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn for NullGameCanvas {
    fn as_MyCanvas(&self) -> &crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas { &self.base.base.base }
    fn as_MyCanvas_mut(&mut self) -> &mut crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas { &mut self.base.base.base }
    fn pause(&mut self) { Self::pause(self) }
    fn unPause(&mut self) { Self::unPause(self) }
    fn draw(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::draw(self, graphics) }
    fn keyPressedByDevice(&mut self, p1: i32, p2: i32) { crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas::keyPressedByDevice(crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn::as_AllBinaryGameCanvas_mut(self), p1, p2) }
    fn keyRepeatedByDevice(&mut self, p1: i32, p2: i32) { crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas::keyRepeatedByDevice(crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn::as_AllBinaryGameCanvas_mut(self), p1, p2) }
    fn keyReleasedByDevice(&mut self, p1: i32, p2: i32) { crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas::keyReleasedByDevice(crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn::as_AllBinaryGameCanvas_mut(self), p1, p2) }
}
impl crate::org::allbinary::game::displayable::canvas::PopupMenuInterface::PopupMenuInterface for NullGameCanvas {
    fn popupMenu(&mut self) { crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas::popupMenu(crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn::as_AllBinaryGameCanvas_mut(self)) }
    fn closeMenu(&mut self) { crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas::closeMenu(crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn::as_AllBinaryGameCanvas_mut(self)) }
    fn toggleMenu(&mut self) { crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas::toggleMenu(crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn::as_AllBinaryGameCanvas_mut(self)) }
}
impl crate::org::allbinary::game::IntermissionCompositeInterface::IntermissionCompositeInterface for NullGameCanvas {
    fn getStartIntermissionInterface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::IntermissionInterface::IntermissionInterface>>> { crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas::getStartIntermissionInterface(crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn::as_AllBinaryGameCanvas_mut(self)) }
    fn getEndLevelIntermissionInterface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::IntermissionInterface::IntermissionInterface>>> { crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas::getEndLevelIntermissionInterface(crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn::as_AllBinaryGameCanvas_mut(self)) }
}
impl crate::org::allbinary::game::IntermissionEnableListenerInterface::IntermissionEnableListenerInterface for NullGameCanvas {
    fn notifyIntermission(&mut self, p1: bool) { crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas::notifyIntermission(crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn::as_AllBinaryGameCanvas_mut(self), p1) }
}


