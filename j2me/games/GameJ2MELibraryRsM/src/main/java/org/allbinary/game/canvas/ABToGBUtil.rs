// 
// AllBinary Open License Version 1 
// Copyright (c) 2022 AllBinary 
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
// package org.allbinary.game.canvas
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Canvas::Canvas;
// use crate::javax::microedition::lcdui::NullCanvas::NullCanvas;
use crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManager;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ABToGBUtil {
    pub abCanvas: Option<Rc<RefCell<Canvas>>>,
    pub allBinaryGameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>,
}

thread_local! { pub static ABToGBUtil_instance: Option<Rc<RefCell<ABToGBUtil>>> = ABToGBUtil::new(); }

impl ABToGBUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            abCanvas: None,
            allBinaryGameLayerManager: None,
        };
        this.abCanvas = crate::javax::microedition::lcdui::NullCanvas::NullCanvas_NULL_CANVAS.with(Clone::clone).clone();
        this.allBinaryGameLayerManager = AllBinaryGameLayerManager::getNullInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<ABToGBUtil>>> {
        return ABToGBUtil_instance.with(Clone::clone).clone();
    }
}

