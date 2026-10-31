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
// use crate::javax::microedition::lcdui::ItemStateListener::ItemStateListener;
use crate::org::allbinary::game::state::GameStateCompositeInterface::GameStateCompositeInterface;
use crate::org::allbinary::graphics::paint::PaintableInterface::PaintableInterface;
use crate::org::allbinary::thread::RunnableInterface::RunnableInterface;
use crate::org::allbinary::util::ABHashtable::ABHashtable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait GameCanvasRunnableInterface: RunnableInterface + PaintableInterface + ItemStateListener + GameStateCompositeInterface {
    

    fn getTitle(&mut self) -> String;
    

    fn isInitialized(&mut self) -> bool;
    

    fn isPaused(&mut self) -> bool;
    

    fn unPause(&mut self);
    

    fn pause(&mut self);
    

    fn isHighScoreSubmitted(&mut self) -> bool;
    

    // throws Exception
    fn setHighScoreSubmitted(&mut self, highScoreSubmitted: bool);
    

    // throws Exception
    fn processGameOver(&mut self);
    

    fn isGameOver(&mut self) -> bool;
    

    fn initCommands(&mut self, cmdListener: Option<Rc<RefCell<CommandListener>>>);
    

    // throws Exception
    fn getCurrentStateHashtable(&mut self) -> Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>;
    

    // throws Exception
    fn getLoadStateHashtable(&mut self) -> Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>;
    

    // throws Exception
    fn setLoadStateHashtable(&mut self, hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>);
    

    fn getType(&mut self) -> i32;
}


