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
// package org.allbinary.game.layer.special
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent;
use crate::org::allbinary::graphics::paint::PaintableInterface::PaintableInterface;
use crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait SpecialGameInputInterface: PaintableInterface {
    

    // throws Exception
    fn up(&mut self);
    

    // throws Exception
    fn down(&mut self);
    

    // throws Exception
    fn right(&mut self);
    

    // throws Exception
    fn left(&mut self);
    

    // throws Exception
    fn strafeLeft(&mut self);
    

    // throws Exception
    fn strafeRight(&mut self);
    

    // throws Exception
    fn fire(&mut self, layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>);
    

    // throws Exception
    fn special1(&mut self, layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>);
    

    // throws Exception
    fn special2(&mut self, layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>);
    

    // throws Exception
    fn special3(&mut self, layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>);
    

    // throws Exception
    fn special4(&mut self, layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>);
    

    // throws Exception
    fn special5(&mut self, layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>);
}


