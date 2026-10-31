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
// use crate::javax::microedition::lcdui::Command::Command;
// use crate::javax::microedition::lcdui::CommandListener::CommandListener;
use crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManager;
use super::AllBinaryGameCanvasInterface::AllBinaryGameCanvasInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NullAllBinaryGameCanvas {
}

thread_local! { pub static NullAllBinaryGameCanvas_NULL_ALLBINARY_GAME_CANVAS: Option<Rc<RefCell<NullAllBinaryGameCanvas>>> = NullAllBinaryGameCanvas::new(); }
impl AllBinaryGameCanvasInterface for NullAllBinaryGameCanvas {
    fn getLayerManager(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>> { Self::getLayerManager(self) }
}
impl crate::org::allbinary::graphics::displayable::DisplayableInterface::DisplayableInterface for NullAllBinaryGameCanvas {
    fn addCommand(&mut self, mut cmd: Option<Rc<RefCell<Command>>>) { Self::addCommand(self, cmd) }
    fn getHeight(&mut self) -> i32 { Self::getHeight(self) }
    fn getWidth(&mut self) -> i32 { Self::getWidth(self) }
    fn removeCommand(&mut self, mut cmd: Option<Rc<RefCell<Command>>>) { Self::removeCommand(self, cmd) }
    fn setCommandListener(&mut self, mut l: Option<Rc<RefCell<CommandListener>>>) { Self::setCommandListener(self, l) }
}

impl NullAllBinaryGameCanvas {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getLayerManager(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn addCommand(&mut self, mut cmd: Option<Rc<RefCell<Command>>>) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getHeight(&mut self) -> i32 {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getWidth(&mut self) -> i32 {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn removeCommand(&mut self, mut cmd: Option<Rc<RefCell<Command>>>) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn setCommandListener(&mut self, mut l: Option<Rc<RefCell<CommandListener>>>) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
}

