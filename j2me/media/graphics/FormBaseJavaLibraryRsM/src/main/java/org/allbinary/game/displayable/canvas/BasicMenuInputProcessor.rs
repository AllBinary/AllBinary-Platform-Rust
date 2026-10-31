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
// use crate::javax::microedition::lcdui::Canvas::Canvas;
// use crate::javax::microedition::lcdui::NullCanvas::NullCanvas;
use crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInput;
use crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas;
use crate::org::allbinary::input::motion::gesture::observer::BaseMotionGestureEventListener::BaseMotionGestureEventListener;
use crate::org::allbinary::input::motion::gesture::observer::MotionGestureEvent::MotionGestureEvent;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicMenuInputProcessor {
    pub base: PlayerGameInput,
    pub motionGestureEventList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub canvas: Option<Rc<RefCell<Canvas>>>,
}

// impl BaseMotionGestureEventListener for BasicMenuInputProcessor with the methods of the trait
impl crate::org::allbinary::game::input::event::GameKeyEventListenerInterface::GameKeyEventListenerInterface for BasicMenuInputProcessor {
}

impl BasicMenuInputProcessor {
    

    pub fn new_struct(mut gameKeyEventList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut playerInputId: i32, mut gameCanvas: Option<Rc<RefCell<Canvas>>>) -> Self {
        let mut this = Self {
            base: PlayerGameInput::new_struct(gameKeyEventList.clone(), BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayListD::BasicArrayListDDyn>>), playerInputId.clone()),
            motionGestureEventList: None,
            canvas: None,
        };
        this.motionGestureEventList = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.canvas = crate::javax::microedition::lcdui::NullCanvas::NullCanvas_NULL_CANVAS.with(Clone::clone).clone();
        // super(...) is in the struct literal above
        this.canvas = gameCanvas.clone();
        this
    }

    pub fn new(mut gameKeyEventList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut playerInputId: i32, mut gameCanvas: Option<Rc<RefCell<Canvas>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(gameKeyEventList, playerInputId, gameCanvas))))
    }
    

    pub fn onMotionGestureEvent(&mut self, mut motionGestureEvent: Option<Rc<RefCell<MotionGestureEvent>>>) {
        jcall!(self.motionGestureEventList).add(crate::java_into_object(motionGestureEvent.clone()));
    }
    

    pub fn onScrolledMotionGestureEvent(&mut self, mut motionGestureEvent: Option<Rc<RefCell<MotionGestureEvent>>>) {
    }
    

    pub fn setCanvas(&mut self, mut canvas: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn>>>) {
        self.canvas = canvas.clone();
    }
    

    pub fn getCanvas(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn>>> {
        return (self.canvas).clone() /* as MyCanvas */;
    }
    

    // throws Exception
    pub fn processInputList(&mut self) -> i32 {
        return -1;
    }
}
impl std::ops::Deref for BasicMenuInputProcessor {
    type Target = PlayerGameInput;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for BasicMenuInputProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait BasicMenuInputProcessorDyn: crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInputDyn {
    fn as_BasicMenuInputProcessor(&self) -> &BasicMenuInputProcessor;
    fn as_BasicMenuInputProcessor_mut(&mut self) -> &mut BasicMenuInputProcessor;
    fn onMotionGestureEvent(&mut self, mut motionGestureEvent: Option<Rc<RefCell<MotionGestureEvent>>>) { BasicMenuInputProcessor::onMotionGestureEvent(self.as_BasicMenuInputProcessor_mut(), motionGestureEvent) }
    fn onScrolledMotionGestureEvent(&mut self, mut motionGestureEvent: Option<Rc<RefCell<MotionGestureEvent>>>) { BasicMenuInputProcessor::onScrolledMotionGestureEvent(self.as_BasicMenuInputProcessor_mut(), motionGestureEvent) }
    fn setCanvas(&mut self, mut canvas: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn>>>) { BasicMenuInputProcessor::setCanvas(self.as_BasicMenuInputProcessor_mut(), canvas) }
    fn getCanvas(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn>>> { BasicMenuInputProcessor::getCanvas(self.as_BasicMenuInputProcessor_mut()) }
    fn processInputList(&mut self) -> i32 { BasicMenuInputProcessor::processInputList(self.as_BasicMenuInputProcessor_mut()) }
}
impl BasicMenuInputProcessorDyn for BasicMenuInputProcessor {
    fn as_BasicMenuInputProcessor(&self) -> &BasicMenuInputProcessor { self }
    fn as_BasicMenuInputProcessor_mut(&mut self) -> &mut BasicMenuInputProcessor { self }
}

impl crate::org::allbinary::game::input::GameInput::GameInputDyn for BasicMenuInputProcessor {
    fn as_GameInput(&self) -> &crate::org::allbinary::game::input::GameInput::GameInput { &self.base.base }
    fn as_GameInput_mut(&mut self) -> &mut crate::org::allbinary::game::input::GameInput::GameInput { &mut self.base.base }
}
impl crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInputDyn for BasicMenuInputProcessor {
    fn as_PlayerGameInput(&self) -> &crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInput { &self.base }
    fn as_PlayerGameInput_mut(&mut self) -> &mut crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInput { &mut self.base }
}


