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
use crate::org::allbinary::canvas::Processor::Processor;
use super::AllBinaryGameCanvas::AllBinaryGameCanvas;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct EndLevelIntermissionProcessor {
    pub base: Processor,
    pub gameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>,
}


impl EndLevelIntermissionProcessor {
    

    pub fn new_struct(mut gameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) -> Self {
        let mut this = Self {
            base: Processor::new_struct(),
            gameCanvas: None,
        };
        this.gameCanvas = gameCanvas.clone();
        this
    }

    pub fn new(mut gameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(gameCanvas))))
    }
    

    // throws Exception
    pub fn process(&mut self) {
        jcall!(self.gameCanvas).processEndLevelIntermissionGameState();
    }
}
impl std::ops::Deref for EndLevelIntermissionProcessor {
    type Target = Processor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for EndLevelIntermissionProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::canvas::Processor::ProcessorDyn for EndLevelIntermissionProcessor {
    fn as_Processor(&self) -> &crate::org::allbinary::canvas::Processor::Processor { &self.base }
    fn as_Processor_mut(&mut self) -> &mut crate::org::allbinary::canvas::Processor::Processor { &mut self.base }
    fn process(&mut self) { Self::process(self) }
}


