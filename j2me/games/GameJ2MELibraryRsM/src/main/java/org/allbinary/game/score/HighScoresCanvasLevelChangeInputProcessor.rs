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
// use crate::javax::microedition::lcdui::Canvas::Canvas;
// use crate::javax::microedition::lcdui::Font::Font;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::game::input::event::GameKeyEventHandler::GameKeyEventHandler;
use crate::org::allbinary::game::input::event::GameKeyEventUtil::GameKeyEventUtil;
use crate::org::allbinary::game::score::displayable::HighScoresCanvas::HighScoresCanvas;
use crate::org::allbinary::graphics::Anchor::Anchor;
use crate::org::allbinary::graphics::displayable::DisplayInfoSingleton::DisplayInfoSingleton;
use crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessor;
use crate::org::allbinary::graphics::font::UpdateMyFontInterface::UpdateMyFontInterface;
use crate::org::allbinary::graphics::font::UpdateMyFontProcessor::UpdateMyFontProcessor;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use super::HighScoresCanvasInputProcessor::HighScoresCanvasInputProcessor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct HighScoresCanvasLevelChangeInputProcessor {
    pub base: HighScoresCanvasInputProcessor,
    pub displayInfoSingleton: Option<Rc<RefCell<DisplayInfoSingleton>>>,
    pub INSTRUCTIONS: String,
    pub myFontProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>>,
    pub anchor: i32,
    pub fontHeight: i32,
}

impl UpdateMyFontInterface for HighScoresCanvasLevelChangeInputProcessor {
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::updateMeasurement(self, graphics) }
}
impl crate::org::allbinary::game::input::event::GameKeyEventListenerInterface::GameKeyEventListenerInterface for HighScoresCanvasLevelChangeInputProcessor {
}

impl HighScoresCanvasLevelChangeInputProcessor {
    

    pub fn new_struct(mut highScoresCanvas: Option<Rc<RefCell<HighScoresCanvas>>>) -> Self {
        let mut this = Self {
            base: HighScoresCanvasInputProcessor::new_struct(highScoresCanvas.clone()),
            displayInfoSingleton: None,
            INSTRUCTIONS: String::new(),
            myFontProcessor: None,
            anchor: 0,
            fontHeight: 0,
        };
        this.displayInfoSingleton = DisplayInfoSingleton::getInstance();
        this.INSTRUCTIONS = String::from("(Right = Next Track, Left = Previous Track)");
        this.myFontProcessor = UpdateMyFontProcessor::new(this).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>);
        this.anchor = unsafe { crate::org::allbinary::graphics::Anchor::Anchor_TOP_LEFT.clone() }.clone();
        this.fontHeight = 0;
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut highScoresCanvas: Option<Rc<RefCell<HighScoresCanvas>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(highScoresCanvas))))
    }
    

    pub fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        let mut font: Option<Rc<RefCell<Font>>> = jcall!(graphics).getFont();
        self.fontHeight = jcall!(font).getHeight();
        self.myFontProcessor = MyFontProcessor::getInstance();
    }
    

    pub fn open(&mut self) {
        jcall!(GameKeyEventHandler::getInstance()).addListener(None /* Java this: a &mut self method has no Rc of itself */);
    }
    

    pub fn close(&mut self) {
        jcall!(GameKeyEventHandler::getInstance()).removeListener(None /* Java this: a &mut self method has no Rc of itself */);
    }
    

    // synchronized
    pub fn update(&mut self) {
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.getGameKeyEventList();
        let mut size: i32 = jcall!(list).size();
        for index in (0..size) {
            let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(list).as_BasicArrayList_mut().objectArray[(index) as usize].clone();
            let mut key: i32 = GameKeyEventUtil::getKey(anyType.clone());
            if key == unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_RIGHT.clone() } {
                jcall!(jcall!(self.getHighScoresCanvas()).getGameInfo()).nextGameLevel();
            } else if key == unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_LEFT.clone() } {
                jcall!(jcall!(self.getHighScoresCanvas()).getGameInfo()).previousGameLevel();
            }
            jcall!(self.getHighScoresCanvas()).executeUpdate();
        }
        jcall!(list).clear();
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        jcall!(self.myFontProcessor).process(graphics.clone());
        let mut width: i32 = jcall!(self.displayInfoSingleton).getLastWidth();
        let mut topScoresWidth: i32 = (jcall!(jcall!(graphics).getFont()).stringWidth(self.INSTRUCTIONS.clone()) >> 1);
        jcall!(graphics).drawString(self.INSTRUCTIONS.clone(), (width >> 1) - topScoresWidth, self.fontHeight * 2, self.anchor.clone());
    }
}
impl std::ops::Deref for HighScoresCanvasLevelChangeInputProcessor {
    type Target = HighScoresCanvasInputProcessor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for HighScoresCanvasLevelChangeInputProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::score::HighScoresCanvasInputProcessor::HighScoresCanvasInputProcessorDyn for HighScoresCanvasLevelChangeInputProcessor {
    fn as_HighScoresCanvasInputProcessor(&self) -> &crate::org::allbinary::game::score::HighScoresCanvasInputProcessor::HighScoresCanvasInputProcessor { &self.base }
    fn as_HighScoresCanvasInputProcessor_mut(&mut self) -> &mut crate::org::allbinary::game::score::HighScoresCanvasInputProcessor::HighScoresCanvasInputProcessor { &mut self.base }
    fn open(&mut self) { Self::open(self) }
    fn close(&mut self) { Self::close(self) }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
}
impl crate::org::allbinary::game::input::GameInput::GameInputDyn for HighScoresCanvasLevelChangeInputProcessor {
    fn as_GameInput(&self) -> &crate::org::allbinary::game::input::GameInput::GameInput { &self.base.base.base }
    fn as_GameInput_mut(&mut self) -> &mut crate::org::allbinary::game::input::GameInput::GameInput { &mut self.base.base.base }
    fn update(&mut self) { Self::update(self) }
}
impl crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInputDyn for HighScoresCanvasLevelChangeInputProcessor {
    fn as_PlayerGameInput(&self) -> &crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInput { &self.base.base }
    fn as_PlayerGameInput_mut(&mut self) -> &mut crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInput { &mut self.base.base }
    fn onPressGameKeyEvent(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent>>>) { crate::org::allbinary::game::score::HighScoresCanvasInputProcessor::HighScoresCanvasInputProcessor::onPressGameKeyEvent(crate::org::allbinary::game::score::HighScoresCanvasInputProcessor::HighScoresCanvasInputProcessorDyn::as_HighScoresCanvasInputProcessor_mut(self), p1) }
    fn onDownGameKeyEvent(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent>>>) { crate::org::allbinary::game::score::HighScoresCanvasInputProcessor::HighScoresCanvasInputProcessor::onDownGameKeyEvent(crate::org::allbinary::game::score::HighScoresCanvasInputProcessor::HighScoresCanvasInputProcessorDyn::as_HighScoresCanvasInputProcessor_mut(self), p1) }
    fn onUpGameKeyEvent(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent>>>) { crate::org::allbinary::game::score::HighScoresCanvasInputProcessor::HighScoresCanvasInputProcessor::onUpGameKeyEvent(crate::org::allbinary::game::score::HighScoresCanvasInputProcessor::HighScoresCanvasInputProcessorDyn::as_HighScoresCanvasInputProcessor_mut(self), p1) }
}


