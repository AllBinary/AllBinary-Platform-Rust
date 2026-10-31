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
// use crate::javax::microedition::lcdui::NullCanvas::NullCanvas;
use crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent;
use crate::org::allbinary::input::motion::gesture::observer::MotionGestureEvent::MotionGestureEvent;
use crate::org::allbinary::util::BasicArrayListUtil::BasicArrayListUtil;
use super::BasicMenuInputProcessor::BasicMenuInputProcessor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NoMenuInputProcessor {
    pub base: BasicMenuInputProcessor,
}

thread_local! { pub static NoMenuInputProcessor_SINGLETON: Option<Rc<RefCell<NoMenuInputProcessor>>> = NoMenuInputProcessor::new(); }
impl crate::org::allbinary::game::input::event::DownGameKeyEventListenerInterface::DownGameKeyEventListenerInterface for NoMenuInputProcessor {
    fn onDownGameKeyEvent(&mut self, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) { Self::onDownGameKeyEvent(self, gameKeyEvent) }
}
impl crate::org::allbinary::game::input::event::GameKeyEventListenerInterface::GameKeyEventListenerInterface for NoMenuInputProcessor {
}
impl crate::org::allbinary::game::input::event::UpGameKeyEventListenerInterface::UpGameKeyEventListenerInterface for NoMenuInputProcessor {
    fn onUpGameKeyEvent(&mut self, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) { Self::onUpGameKeyEvent(self, gameKeyEvent) }
}

impl NoMenuInputProcessor {
    

    pub fn getInstance() -> Option<Rc<RefCell<NoMenuInputProcessor>>> {
        return NoMenuInputProcessor_SINGLETON.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: BasicMenuInputProcessor::new_struct(jcall!(BasicArrayListUtil::getInstance()).getImmutableInstance(), -1, crate::javax::microedition::lcdui::NullCanvas::NullCanvas_NULL_CANVAS.with(Clone::clone).clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // synchronized
    pub fn onDownGameKeyEvent(&mut self, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) {
    }
    

    // synchronized
    pub fn onUpGameKeyEvent(&mut self, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) {
    }
    

    pub fn onMotionGestureEvent(&mut self, mut motionGestureEvent: Option<Rc<RefCell<MotionGestureEvent>>>) {
    }
}
impl std::ops::Deref for NoMenuInputProcessor {
    type Target = BasicMenuInputProcessor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for NoMenuInputProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::displayable::canvas::BasicMenuInputProcessor::BasicMenuInputProcessorDyn for NoMenuInputProcessor {
    fn as_BasicMenuInputProcessor(&self) -> &crate::org::allbinary::game::displayable::canvas::BasicMenuInputProcessor::BasicMenuInputProcessor { &self.base }
    fn as_BasicMenuInputProcessor_mut(&mut self) -> &mut crate::org::allbinary::game::displayable::canvas::BasicMenuInputProcessor::BasicMenuInputProcessor { &mut self.base }
    fn onMotionGestureEvent(&mut self, mut motionGestureEvent: Option<Rc<RefCell<MotionGestureEvent>>>) { Self::onMotionGestureEvent(self, motionGestureEvent) }
}
impl crate::org::allbinary::game::input::GameInput::GameInputDyn for NoMenuInputProcessor {
    fn as_GameInput(&self) -> &crate::org::allbinary::game::input::GameInput::GameInput { &self.base.base.base }
    fn as_GameInput_mut(&mut self) -> &mut crate::org::allbinary::game::input::GameInput::GameInput { &mut self.base.base.base }
}
impl crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInputDyn for NoMenuInputProcessor {
    fn as_PlayerGameInput(&self) -> &crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInput { &self.base.base }
    fn as_PlayerGameInput_mut(&mut self) -> &mut crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInput { &mut self.base.base }
    fn onDownGameKeyEvent(&mut self, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) { Self::onDownGameKeyEvent(self, gameKeyEvent) }
    fn onUpGameKeyEvent(&mut self, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) { Self::onUpGameKeyEvent(self, gameKeyEvent) }
}


