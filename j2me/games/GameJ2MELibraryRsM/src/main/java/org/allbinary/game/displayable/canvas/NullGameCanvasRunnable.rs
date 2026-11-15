// 
// AllBinary Open License Version 1 
// Copyright (c) 2025 AllBinary 
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
use crate::javax::microedition::lcdui::CommandListener::CommandListener;
use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::javax::microedition::lcdui::Item::Item;
use crate::org::allbinary::game::state::GameState::GameState;
use crate::org::allbinary::game::state::GameStateFactory::GameStateFactory;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use crate::java::lang::Thread::*;
use crate::java::lang::Exception::*;
use super::GameCanvasRunnableInterface::GameCanvasRunnableInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NullGameCanvasRunnable {
    pub gameStateFactory: Option<Rc<RefCell<GameStateFactory>>>,
}

pub static NullGameCanvasRunnable_NULL_GAME_CANVAS_RUNNABLE: crate::JavaStatic<Option<Rc<RefCell<NullGameCanvasRunnable>>>> = crate::JavaStatic::new(|| NullGameCanvasRunnable::new());
impl GameCanvasRunnableInterface for NullGameCanvasRunnable {
    fn getTitle(&mut self) -> String { Self::getTitle(self) }
    fn isInitialized(&mut self) -> bool { Self::isInitialized(self) }
    fn isPaused(&mut self) -> bool { Self::isPaused(self) }
    fn unPause(&mut self) { Self::unPause(self) }
    fn pause(&mut self) { Self::pause(self) }
    fn isHighScoreSubmitted(&mut self) -> bool { Self::isHighScoreSubmitted(self) }
    fn setHighScoreSubmitted(&mut self, mut highScoreSubmitted: bool) { Self::setHighScoreSubmitted(self, highScoreSubmitted) }
    fn processGameOver(&mut self) { Self::processGameOver(self) }
    fn isGameOver(&mut self) -> bool { Self::isGameOver(self) }
    fn initCommands(&mut self, mut cmdListener: Option<Rc<RefCell<dyn CommandListener>>>) { Self::initCommands(self, cmdListener) }
    fn getCurrentStateHashtable(&mut self) -> Option<Rc<RefCell<ABHashtable<crate::JavaObject, crate::JavaObject>>>> { Self::getCurrentStateHashtable(self) }
    fn getLoadStateHashtable(&mut self) -> Option<Rc<RefCell<ABHashtable<crate::JavaObject, crate::JavaObject>>>> { Self::getLoadStateHashtable(self) }
    fn setLoadStateHashtable(&mut self, mut hashtable: Option<Rc<RefCell<ABHashtable<crate::JavaObject, crate::JavaObject>>>>) { Self::setLoadStateHashtable(self, hashtable) }
    fn getType(&mut self) -> i32 { Self::getType(self) }
}
impl crate::javax::microedition::lcdui::ItemStateListener::ItemStateListener for NullGameCanvasRunnable {
    fn itemStateChanged(&mut self, mut item: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Item::ItemDyn>>>) { Self::itemStateChanged(self, item) }
}
impl crate::org::allbinary::game::state::GameStateCompositeInterface::GameStateCompositeInterface for NullGameCanvasRunnable {
    fn setGameState(&mut self, mut gameState: Option<Rc<RefCell<GameState>>>) { Self::setGameState(self, gameState) }
    fn getGameState(&mut self) -> Option<Rc<RefCell<GameState>>> { Self::getGameState(self) }
}
impl crate::org::allbinary::graphics::paint::PaintableInterface::PaintableInterface for NullGameCanvasRunnable {
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::paint(self, graphics) }
    fn paintThreed(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::paintThreed(self, graphics) }
}
impl crate::org::allbinary::thread::RunnableInterface::RunnableInterface for NullGameCanvasRunnable {
    fn isRunning(&mut self) -> bool { Self::isRunning(self) }
    fn setRunning(&mut self, mut isRunning: bool) { Self::setRunning(self, isRunning) }
    fn setThread(&mut self, mut thread: Option<Rc<RefCell<Thread>>>) { Self::setThread(self, thread) }
}
impl crate::Runnable for NullGameCanvasRunnable {
    fn run(&mut self) { Self::run(self) }
}

impl NullGameCanvasRunnable {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }

    pub fn new_struct() -> Self {
        let mut this = Self {
            gameStateFactory: None,
        };
        this.gameStateFactory = GameStateFactory::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getTitle(&mut self) -> String {
        return { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone();
    }
    

    pub fn isInitialized(&mut self) -> bool {
        return false;
    }
    

    pub fn isPaused(&mut self) -> bool {
        return false;
    }
    

    pub fn unPause(&mut self) {
    }
    

    pub fn pause(&mut self) {
    }
    

    pub fn isHighScoreSubmitted(&mut self) -> bool {
        return false;
    }
    

    // throws Exception
    pub fn setHighScoreSubmitted(&mut self, mut highScoreSubmitted: bool) {
    }
    

    // throws Exception
    pub fn processGameOver(&mut self) {
    }
    

    pub fn isGameOver(&mut self) -> bool {
        return false;
    }
    

    pub fn initCommands(&mut self, mut cmdListener: Option<Rc<RefCell<dyn CommandListener>>>) {
    }
    

    // throws Exception
    pub fn getCurrentStateHashtable(&mut self) -> Option<Rc<RefCell<ABHashtable<crate::JavaObject, crate::JavaObject>>>> {
        return { let java_field = jcall!(StdUtil::getInstance()).NULL_TABLE.clone(); java_field }.clone();
    }
    

    // throws Exception
    pub fn getLoadStateHashtable(&mut self) -> Option<Rc<RefCell<ABHashtable<crate::JavaObject, crate::JavaObject>>>> {
        return { let java_field = jcall!(StdUtil::getInstance()).NULL_TABLE.clone(); java_field }.clone();
    }
    

    // throws Exception
    pub fn setLoadStateHashtable(&mut self, mut hashtable: Option<Rc<RefCell<ABHashtable<crate::JavaObject, crate::JavaObject>>>>) {
    }
    

    pub fn getType(&mut self) -> i32 {
        return -1;
    }
    

    // throws Exception
    pub fn setGameState(&mut self, mut gameState: Option<Rc<RefCell<GameState>>>) {
    }
    

    pub fn getGameState(&mut self) -> Option<Rc<RefCell<GameState>>> {
        return { let java_field = jcall!(self.gameStateFactory).NO_GAME_STATE.clone(); java_field }.clone();
    }
    

    pub fn itemStateChanged(&mut self, mut item: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Item::ItemDyn>>>) {
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
    }
    

    pub fn paintThreed(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
    }
    

    pub fn isRunning(&mut self) -> bool {
        return false;
    }
    

    pub fn setRunning(&mut self, mut isRunning: bool) {
    }
    

    // throws Exception
    pub fn setThread(&mut self, mut thread: Option<Rc<RefCell<Thread>>>) {
    }
    

    pub fn run(&mut self) {
    }
}

