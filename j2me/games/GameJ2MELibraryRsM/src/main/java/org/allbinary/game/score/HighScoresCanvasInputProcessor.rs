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
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInput;
use crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent;
use crate::org::allbinary::game::score::displayable::HighScoresCanvas::HighScoresCanvas;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct HighScoresCanvasInputProcessor {
    pub base: PlayerGameInput,
    pub highScoresCanvas: Option<Rc<RefCell<HighScoresCanvas>>>,
}

impl crate::org::allbinary::game::input::event::DownGameKeyEventListenerInterface::DownGameKeyEventListenerInterface for HighScoresCanvasInputProcessor {
    fn onDownGameKeyEvent(&mut self, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) { Self::onDownGameKeyEvent(self, gameKeyEvent) }
}
impl crate::org::allbinary::game::input::event::GameKeyEventListenerInterface::GameKeyEventListenerInterface for HighScoresCanvasInputProcessor {
}
impl crate::org::allbinary::game::input::event::PressGameKeyEventListenerInterface::PressGameKeyEventListenerInterface for HighScoresCanvasInputProcessor {
    fn onPressGameKeyEvent(&mut self, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) { Self::onPressGameKeyEvent(self, gameKeyEvent) }
}
impl crate::org::allbinary::game::input::event::UpGameKeyEventListenerInterface::UpGameKeyEventListenerInterface for HighScoresCanvasInputProcessor {
    fn onUpGameKeyEvent(&mut self, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) { Self::onUpGameKeyEvent(self, gameKeyEvent) }
}

impl HighScoresCanvasInputProcessor {
    

    pub fn new_struct(mut highScoresCanvas: Option<Rc<RefCell<HighScoresCanvas>>>) -> Self {
        let mut this = Self {
            base: PlayerGameInput::new_struct(BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayListD::BasicArrayListDDyn>>), BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayListD::BasicArrayListDDyn>>), -1),
            highScoresCanvas: None,
        };
        // super(...) is in the struct literal above
        this.highScoresCanvas = highScoresCanvas.clone();
        this
    }

    pub fn new(mut highScoresCanvas: Option<Rc<RefCell<HighScoresCanvas>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(highScoresCanvas))))
    }
    

    pub fn open(&mut self) {
    }
    

    pub fn close(&mut self) {
    }
    

    // synchronized
    pub fn onPressGameKeyEvent(&mut self, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) {
        self.base.onPressGameKeyEvent(gameKeyEvent.clone());
        self.update();
    }
    

    // synchronized
    pub fn onDownGameKeyEvent(&mut self, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) {
        self.base.onDownGameKeyEvent(gameKeyEvent.clone());
        self.update();
    }
    

    // synchronized
    pub fn onUpGameKeyEvent(&mut self, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) {
        self.base.onUpGameKeyEvent(gameKeyEvent.clone());
        self.update();
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
    }
    

    pub fn getHighScoresCanvas(&mut self) -> Option<Rc<RefCell<HighScoresCanvas>>> {
        return self.highScoresCanvas.clone();
    }
}
impl std::ops::Deref for HighScoresCanvasInputProcessor {
    type Target = PlayerGameInput;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for HighScoresCanvasInputProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait HighScoresCanvasInputProcessorDyn: crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInputDyn {
    fn as_HighScoresCanvasInputProcessor(&self) -> &HighScoresCanvasInputProcessor;
    fn as_HighScoresCanvasInputProcessor_mut(&mut self) -> &mut HighScoresCanvasInputProcessor;
    fn open(&mut self) { HighScoresCanvasInputProcessor::open(self.as_HighScoresCanvasInputProcessor_mut()) }
    fn close(&mut self) { HighScoresCanvasInputProcessor::close(self.as_HighScoresCanvasInputProcessor_mut()) }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { HighScoresCanvasInputProcessor::paint(self.as_HighScoresCanvasInputProcessor_mut(), graphics) }
    fn getHighScoresCanvas(&mut self) -> Option<Rc<RefCell<HighScoresCanvas>>> { HighScoresCanvasInputProcessor::getHighScoresCanvas(self.as_HighScoresCanvasInputProcessor_mut()) }
}
impl HighScoresCanvasInputProcessorDyn for HighScoresCanvasInputProcessor {
    fn as_HighScoresCanvasInputProcessor(&self) -> &HighScoresCanvasInputProcessor { self }
    fn as_HighScoresCanvasInputProcessor_mut(&mut self) -> &mut HighScoresCanvasInputProcessor { self }
}

impl crate::org::allbinary::game::input::GameInput::GameInputDyn for HighScoresCanvasInputProcessor {
    fn as_GameInput(&self) -> &crate::org::allbinary::game::input::GameInput::GameInput { &self.base.base }
    fn as_GameInput_mut(&mut self) -> &mut crate::org::allbinary::game::input::GameInput::GameInput { &mut self.base.base }
}
impl crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInputDyn for HighScoresCanvasInputProcessor {
    fn as_PlayerGameInput(&self) -> &crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInput { &self.base }
    fn as_PlayerGameInput_mut(&mut self) -> &mut crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInput { &mut self.base }
    fn onPressGameKeyEvent(&mut self, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) { Self::onPressGameKeyEvent(self, gameKeyEvent) }
    fn onDownGameKeyEvent(&mut self, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) { Self::onDownGameKeyEvent(self, gameKeyEvent) }
    fn onUpGameKeyEvent(&mut self, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) { Self::onUpGameKeyEvent(self, gameKeyEvent) }
}


