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
use super::*;
use crate::org::allbinary::game::displayable::canvas::GameCanvasRunnableInterface::GameCanvasRunnableInterface;
use crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManager;
use crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvas;
use crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvasFactory::ProgressCanvasFactory;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::system::security::licensing::ClientInformationFactory::ClientInformationFactory;
use crate::org::allbinary::thread::PrimaryThreadPool::PrimaryThreadPool;
use crate::org::allbinary::game::displayable::canvas::NullGameCanvasRunnable::NullGameCanvasRunnable;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use super::GameMidlet::GameMidlet;
use super::DemoRunnable::DemoRunnable;
use super::CreateGameRunnable::CreateGameRunnable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct DemoGameMidlet {
    pub base: GameMidlet,
}


impl DemoGameMidlet {
    

    pub fn new_struct(mut clientInformationFactory: Option<Rc<RefCell<ClientInformationFactory>>>) -> Self {
        let mut this = Self {
            base: GameMidlet::new_struct(clientInformationFactory.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut clientInformationFactory: Option<Rc<RefCell<ClientInformationFactory>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(clientInformationFactory))))
    }
    

    // throws Exception
    pub fn isContinue(&mut self) -> bool {
        let mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> = self.getStartStateHashtable();
        if hashtable.is_some() && jcall!(hashtable).size() > 0 {
            return true;
        } else {
            return false;
        }
    }
    

    pub fn getHighestLevel(&mut self) -> i32 {
        return 1;
    }
    

    // throws Exception
    pub fn createDemoGameCanvasRunnableInterface(&mut self) -> Option<Rc<RefCell<dyn GameCanvasRunnableInterface>>> {
        jcall!(self.logUtil).putF(jcall!(self.commonStrings).NOT_IMPLEMENTED.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).CREATE.clone());
        return crate::org::allbinary::game::displayable::canvas::NullGameCanvasRunnable::NullGameCanvasRunnable_NULL_GAME_CANVAS_RUNNABLE.with(Clone::clone).clone();
    }
    

    // throws Exception
    pub fn createGameCanvasRunnable(&mut self, mut allBinaryGameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>) -> Option<Rc<RefCell<dyn GameCanvasRunnableInterface>>> {
        panic!("{:?}", format!("{}: {:?}", "Exception", (jcall!(self.commonStrings).NOT_IMPLEMENTED.clone())));
    }
    

    // throws Exception
    pub fn createGameCanvasRunnableInterface(&mut self) -> Option<Rc<RefCell<dyn GameCanvasRunnableInterface>>> {
        return { let java_arguments = (self.createGameLayerManager(),); self.createGameCanvasRunnable(java_arguments.0) };
    }
    

    pub fn demoSetup(&mut self) {
    }
    

    pub fn postDemoSetup(&mut self) {
    }
    

    // throws Exception
    // synchronized
    pub fn setDemo(&mut self) {
        jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setDemo"));
        let mut progressCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvasDyn>>> = ProgressCanvasFactory::getInstance();
        jcall!(progressCanvas).start();
        jcall!(PrimaryThreadPool::getInstance()).runTask(DemoRunnable::new(None /* Java this: a &mut self method has no Rc of itself */));
    }
    

    // throws Exception
    // synchronized
    pub fn createGame(&mut self) {
        let mut CREATE_GAME: String = String::from("createGame");
        jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), CREATE_GAME.clone());
        let mut progressCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvasDyn>>> = ProgressCanvasFactory::getInstance();
        jcall!(progressCanvas).start();
        let mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> = self.getStartStateHashtable();
        { let java_arguments = (jcall!(StdUtil::getInstance()).NULL_TABLE.clone(),); self.setStartStateHashtable(java_arguments.0) };
        jcall!(PrimaryThreadPool::getInstance()).runTask(CreateGameRunnable::new(None /* Java this: a &mut self method has no Rc of itself */, hashtable.clone()));
        jcall!(self.logUtil).putF(jcall!(self.commonStrings).END.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), CREATE_GAME.clone());
    }
    

    pub fn isReady(&mut self) -> bool {
        return true;
    }
}
impl std::ops::Deref for DemoGameMidlet {
    type Target = GameMidlet;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for DemoGameMidlet {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait DemoGameMidletDyn: crate::org::allbinary::game::midlet::GameMidlet::GameMidletDyn {
    fn as_DemoGameMidlet(&self) -> &DemoGameMidlet;
    fn as_DemoGameMidlet_mut(&mut self) -> &mut DemoGameMidlet;
    fn isContinue(&mut self) -> bool { DemoGameMidlet::isContinue(self.as_DemoGameMidlet_mut()) }
    fn getHighestLevel(&mut self) -> i32 { DemoGameMidlet::getHighestLevel(self.as_DemoGameMidlet_mut()) }
    fn createDemoGameCanvasRunnableInterface(&mut self) -> Option<Rc<RefCell<dyn GameCanvasRunnableInterface>>> { DemoGameMidlet::createDemoGameCanvasRunnableInterface(self.as_DemoGameMidlet_mut()) }
    fn createGameCanvasRunnable(&mut self, mut allBinaryGameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>) -> Option<Rc<RefCell<dyn GameCanvasRunnableInterface>>> { DemoGameMidlet::createGameCanvasRunnable(self.as_DemoGameMidlet_mut(), allBinaryGameLayerManager) }
    fn createGameCanvasRunnableInterface(&mut self) -> Option<Rc<RefCell<dyn GameCanvasRunnableInterface>>> { DemoGameMidlet::createGameCanvasRunnableInterface(self.as_DemoGameMidlet_mut()) }
    fn demoSetup(&mut self) { DemoGameMidlet::demoSetup(self.as_DemoGameMidlet_mut()) }
    fn postDemoSetup(&mut self) { DemoGameMidlet::postDemoSetup(self.as_DemoGameMidlet_mut()) }
    fn isReady(&mut self) -> bool { DemoGameMidlet::isReady(self.as_DemoGameMidlet_mut()) }
}
impl DemoGameMidletDyn for DemoGameMidlet {
    fn as_DemoGameMidlet(&self) -> &DemoGameMidlet { self }
    fn as_DemoGameMidlet_mut(&mut self) -> &mut DemoGameMidlet { self }
}

impl crate::org::allbinary::game::midlet::GameMidlet::GameMidletDyn for DemoGameMidlet {
    fn as_GameMidlet(&self) -> &crate::org::allbinary::game::midlet::GameMidlet::GameMidlet { &self.base }
    fn as_GameMidlet_mut(&mut self) -> &mut crate::org::allbinary::game::midlet::GameMidlet::GameMidlet { &mut self.base }
    fn setDemo(&mut self) { Self::setDemo(self) }
    fn createGame(&mut self) { Self::createGame(self) }
}
impl crate::org::allbinary::midlet::AllBinaryMidlet::AllBinaryMidletDyn for DemoGameMidlet {
    fn as_AllBinaryMidlet(&self) -> &crate::org::allbinary::midlet::AllBinaryMidlet::AllBinaryMidlet { &self.base.base.base }
    fn as_AllBinaryMidlet_mut(&mut self) -> &mut crate::org::allbinary::midlet::AllBinaryMidlet::AllBinaryMidlet { &mut self.base.base.base }
    fn startApp(&mut self) { crate::org::allbinary::game::midlet::GameMidlet::GameMidlet::startApp(crate::org::allbinary::game::midlet::GameMidlet::GameMidletDyn::as_GameMidlet_mut(self)) }
    fn pauseApp(&mut self) { crate::org::allbinary::game::midlet::GameMidlet::GameMidlet::pauseApp(crate::org::allbinary::game::midlet::GameMidlet::GameMidletDyn::as_GameMidlet_mut(self)) }
    fn destroyAppInRunnable(&mut self, p1: bool, p2: bool) { crate::org::allbinary::game::midlet::GameMidlet::GameMidlet::destroyAppInRunnable(crate::org::allbinary::game::midlet::GameMidlet::GameMidletDyn::as_GameMidlet_mut(self), p1, p2) }
    fn destroyApp(&mut self, p1: bool) { crate::org::allbinary::game::midlet::GameMidlet::GameMidlet::destroyApp(crate::org::allbinary::game::midlet::GameMidlet::GameMidletDyn::as_GameMidlet_mut(self), p1) }
}
impl crate::org::allbinary::midlet::ProgressMidlet::ProgressMidletDyn for DemoGameMidlet {
    fn as_ProgressMidlet(&self) -> &crate::org::allbinary::midlet::ProgressMidlet::ProgressMidlet { &self.base.base }
    fn as_ProgressMidlet_mut(&mut self) -> &mut crate::org::allbinary::midlet::ProgressMidlet::ProgressMidlet { &mut self.base.base }
}


