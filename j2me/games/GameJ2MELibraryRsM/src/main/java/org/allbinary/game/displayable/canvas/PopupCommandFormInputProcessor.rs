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
use crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas;
use crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionForm;
use crate::org::allbinary::input::motion::gesture::observer::MotionGestureEvent::MotionGestureEvent;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use super::CommandFormInputProcessor::CommandFormInputProcessor;
use super::PopupMenuInputProcessor::PopupMenuInputProcessor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PopupCommandFormInputProcessor {
    pub base: CommandFormInputProcessor,
}

impl crate::org::allbinary::game::input::event::GameKeyEventListenerInterface::GameKeyEventListenerInterface for PopupCommandFormInputProcessor {
}

impl PopupCommandFormInputProcessor {
    

    pub fn new_struct(mut gameKeyEventList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut playerInputId: i32, mut gameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn>>>, mut form: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionFormDyn>>>, mut popupMenuInputProcessor: Option<Rc<RefCell<PopupMenuInputProcessor>>>) -> Self {
        let mut this = Self {
            base: CommandFormInputProcessor::new_struct(gameKeyEventList.clone(), playerInputId.clone(), gameCanvas.clone(), form.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut gameKeyEventList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut playerInputId: i32, mut gameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn>>>, mut form: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionFormDyn>>>, mut popupMenuInputProcessor: Option<Rc<RefCell<PopupMenuInputProcessor>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(gameKeyEventList, playerInputId, gameCanvas, form, popupMenuInputProcessor))))
    }
    

    // throws Exception
    pub fn processInput(&mut self, mut key: i32) -> i32 {
        return self.base.processInput(key.clone());
    }
    

    // throws Exception
    pub fn processMotionInput(&mut self, mut motionGestureEvent: Option<Rc<RefCell<MotionGestureEvent>>>) {
        self.base.processMotionInput(motionGestureEvent.clone());
    }
}
impl std::ops::Deref for PopupCommandFormInputProcessor {
    type Target = CommandFormInputProcessor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for PopupCommandFormInputProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::displayable::canvas::BasicMenuInputProcessor::BasicMenuInputProcessorDyn for PopupCommandFormInputProcessor {
    fn as_BasicMenuInputProcessor(&self) -> &crate::org::allbinary::game::displayable::canvas::BasicMenuInputProcessor::BasicMenuInputProcessor { &self.base.base }
    fn as_BasicMenuInputProcessor_mut(&mut self) -> &mut crate::org::allbinary::game::displayable::canvas::BasicMenuInputProcessor::BasicMenuInputProcessor { &mut self.base.base }
    fn processInputList(&mut self) -> i32 { crate::org::allbinary::game::displayable::canvas::CommandFormInputProcessor::CommandFormInputProcessor::processInputList(crate::org::allbinary::game::displayable::canvas::CommandFormInputProcessor::CommandFormInputProcessorDyn::as_CommandFormInputProcessor_mut(self)) }
}
impl crate::org::allbinary::game::displayable::canvas::CommandFormInputProcessor::CommandFormInputProcessorDyn for PopupCommandFormInputProcessor {
    fn as_CommandFormInputProcessor(&self) -> &crate::org::allbinary::game::displayable::canvas::CommandFormInputProcessor::CommandFormInputProcessor { &self.base }
    fn as_CommandFormInputProcessor_mut(&mut self) -> &mut crate::org::allbinary::game::displayable::canvas::CommandFormInputProcessor::CommandFormInputProcessor { &mut self.base }
    fn processInput(&mut self, mut key: i32) -> i32 { Self::processInput(self, key) }
    fn processMotionInput(&mut self, mut motionGestureEvent: Option<Rc<RefCell<MotionGestureEvent>>>) { Self::processMotionInput(self, motionGestureEvent) }
}
impl crate::org::allbinary::game::input::GameInput::GameInputDyn for PopupCommandFormInputProcessor {
    fn as_GameInput(&self) -> &crate::org::allbinary::game::input::GameInput::GameInput { &self.base.base.base.base }
    fn as_GameInput_mut(&mut self) -> &mut crate::org::allbinary::game::input::GameInput::GameInput { &mut self.base.base.base.base }
}
impl crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInputDyn for PopupCommandFormInputProcessor {
    fn as_PlayerGameInput(&self) -> &crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInput { &self.base.base.base }
    fn as_PlayerGameInput_mut(&mut self) -> &mut crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInput { &mut self.base.base.base }
}


