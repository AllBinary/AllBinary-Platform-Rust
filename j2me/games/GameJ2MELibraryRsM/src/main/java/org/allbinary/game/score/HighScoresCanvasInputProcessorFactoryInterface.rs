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
// package org.allbinary.game.score
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::score::displayable::HighScoresCanvas::HighScoresCanvas;
use super::HighScoresCanvasInputProcessor::HighScoresCanvasInputProcessor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait HighScoresCanvasInputProcessorFactoryInterface {
    

    fn getInstance(&mut self, highScoresCanvas: Option<Rc<RefCell<HighScoresCanvas>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::score::HighScoresCanvasInputProcessor::HighScoresCanvasInputProcessorDyn>>>;

    fn toString(&mut self) -> String { String::from(std::any::type_name::<Self>()) }
}


