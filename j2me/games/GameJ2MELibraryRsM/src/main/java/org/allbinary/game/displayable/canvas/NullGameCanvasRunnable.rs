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
// use crate::javax::microedition::lcdui::CommandListener::CommandListener;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
// use crate::javax::microedition::lcdui::Item::Item;
use crate::org::allbinary::game::state::GameState::GameState;
use crate::org::allbinary::game::state::GameStateFactory::GameStateFactory;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use crate::java::lang::Thread::*;
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

thread_local! { pub static NullGameCanvasRunnable_NULL_GAME_CANVAS_RUNNABLE: Option<Rc<RefCell<NullGameCanvasRunnable>>> = NullGameCanvasRunnable::new(); }
// impl GameCanvasRunnableInterface for NullGameCanvasRunnable with the methods of the trait

impl NullGameCanvasRunnable {

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
        return jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
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
    

    pub fn initCommands(&mut self, mut cmdListener: Option<Rc<RefCell<CommandListener>>>) {
    }
    

    // throws Exception
    pub fn getCurrentStateHashtable(&mut self) -> Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> {
        return jcall!(StdUtil::getInstance()).NULL_TABLE.clone();
    }
    

    // throws Exception
    pub fn getLoadStateHashtable(&mut self) -> Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> {
        return jcall!(StdUtil::getInstance()).NULL_TABLE.clone();
    }
    

    // throws Exception
    pub fn setLoadStateHashtable(&mut self, mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>) {
    }
    

    pub fn getType(&mut self) -> i32 {
        return -1;
    }
    

    // throws Exception
    pub fn setGameState(&mut self, mut gameState: Option<Rc<RefCell<GameState>>>) {
    }
    

    pub fn getGameState(&mut self) -> Option<Rc<RefCell<GameState>>> {
        return jcall!(self.gameStateFactory).NO_GAME_STATE.clone();
    }
    

    pub fn itemStateChanged(&mut self, mut item: Option<Rc<RefCell<Item>>>) {
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
    }
    

    pub fn paintThreed(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
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

