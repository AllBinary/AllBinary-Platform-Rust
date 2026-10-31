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
use crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent;
use crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionForm;
use crate::org::allbinary::input::motion::gesture::observer::MotionGestureEvent::MotionGestureEvent;
use crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelper;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use super::CommandFormInputProcessor::CommandFormInputProcessor;
use super::GameCommandCanvas::GameCommandCanvas;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImmediateCommandFormInputProcessor {
    pub base: CommandFormInputProcessor,
    pub timeHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>>,
    pub gameCommandCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn>>>,
}

impl crate::org::allbinary::game::input::event::DownGameKeyEventListenerInterface::DownGameKeyEventListenerInterface for ImmediateCommandFormInputProcessor {
    fn onDownGameKeyEvent(&mut self, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) { Self::onDownGameKeyEvent(self, gameKeyEvent) }
}
impl crate::org::allbinary::game::input::event::GameKeyEventListenerInterface::GameKeyEventListenerInterface for ImmediateCommandFormInputProcessor {
}
impl crate::org::allbinary::game::input::event::PressGameKeyEventListenerInterface::PressGameKeyEventListenerInterface for ImmediateCommandFormInputProcessor {
    fn onPressGameKeyEvent(&mut self, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) { Self::onPressGameKeyEvent(self, gameKeyEvent) }
}
impl crate::org::allbinary::game::input::event::UpGameKeyEventListenerInterface::UpGameKeyEventListenerInterface for ImmediateCommandFormInputProcessor {
    fn onUpGameKeyEvent(&mut self, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) { Self::onUpGameKeyEvent(self, gameKeyEvent) }
}

impl ImmediateCommandFormInputProcessor {
    

    pub fn new_struct(mut gameKeyEventList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut playerInputId: i32, mut gameCommandCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn>>>, mut form: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionFormDyn>>>) -> Self {
        let mut this = Self {
            base: CommandFormInputProcessor::new_struct(gameKeyEventList.clone(), playerInputId.clone(), gameCommandCanvas.clone(), form.clone()),
            timeHelper: None,
            gameCommandCanvas: None,
        };
        this.timeHelper = TimeDelayHelper::new(25).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>);
        // super(...) is in the struct literal above
        this.gameCommandCanvas = gameCommandCanvas.clone();
        this
    }

    pub fn new(mut gameKeyEventList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut playerInputId: i32, mut gameCommandCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn>>>, mut form: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionFormDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(gameKeyEventList, playerInputId, gameCommandCanvas, form))))
    }
    

    // synchronized
    pub fn onPressGameKeyEvent(&mut self, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.base.onPressGameKeyEvent(gameKeyEvent.clone());
            if jcall!(self.timeHelper).isTimeTNT() {
                self.processInputList();
                jcall!(jcall!(self.gameCommandCanvas).as_GameCommandCanvas_mut().repaintBehavior).onChangeRepaint(self.getCanvas());
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.gameInputStrings).ON_PRESS_GAME_KEY.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    // synchronized
    pub fn onDownGameKeyEvent(&mut self, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.base.onDownGameKeyEvent(gameKeyEvent.clone());
            if jcall!(self.timeHelper).isTimeTNT() {
                self.processInputList();
                jcall!(jcall!(self.gameCommandCanvas).as_GameCommandCanvas_mut().repaintBehavior).onChangeRepaint(self.getCanvas());
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.gameInputStrings).ON_DOWN_GAME_KEY.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    // synchronized
    pub fn onUpGameKeyEvent(&mut self, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.base.onUpGameKeyEvent(gameKeyEvent.clone());
            if jcall!(self.timeHelper).isTimeTNT() {
                self.processInputList();
                jcall!(jcall!(self.gameCommandCanvas).as_GameCommandCanvas_mut().repaintBehavior).onChangeRepaint(self.getCanvas());
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.gameInputStrings).ON_UP_GAME_KEY.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn onMotionGestureEvent(&mut self, mut motionGestureEvent: Option<Rc<RefCell<MotionGestureEvent>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.base.onMotionGestureEvent(motionGestureEvent.clone());
            if jcall!(self.timeHelper).isTimeTNT() {
                self.processInputList();
                jcall!(jcall!(self.gameCommandCanvas).as_GameCommandCanvas_mut().repaintBehavior).onChangeRepaint(self.getCanvas());
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("onMotionGestureEvent"), crate::java_into_object(e.clone()));
            }
        }
    }
}
impl std::ops::Deref for ImmediateCommandFormInputProcessor {
    type Target = CommandFormInputProcessor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ImmediateCommandFormInputProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::displayable::canvas::BasicMenuInputProcessor::BasicMenuInputProcessorDyn for ImmediateCommandFormInputProcessor {
    fn as_BasicMenuInputProcessor(&self) -> &crate::org::allbinary::game::displayable::canvas::BasicMenuInputProcessor::BasicMenuInputProcessor { &self.base.base }
    fn as_BasicMenuInputProcessor_mut(&mut self) -> &mut crate::org::allbinary::game::displayable::canvas::BasicMenuInputProcessor::BasicMenuInputProcessor { &mut self.base.base }
    fn onMotionGestureEvent(&mut self, mut motionGestureEvent: Option<Rc<RefCell<MotionGestureEvent>>>) { Self::onMotionGestureEvent(self, motionGestureEvent) }
    fn processInputList(&mut self) -> i32 { crate::org::allbinary::game::displayable::canvas::CommandFormInputProcessor::CommandFormInputProcessor::processInputList(crate::org::allbinary::game::displayable::canvas::CommandFormInputProcessor::CommandFormInputProcessorDyn::as_CommandFormInputProcessor_mut(self)) }
}
impl crate::org::allbinary::game::displayable::canvas::CommandFormInputProcessor::CommandFormInputProcessorDyn for ImmediateCommandFormInputProcessor {
    fn as_CommandFormInputProcessor(&self) -> &crate::org::allbinary::game::displayable::canvas::CommandFormInputProcessor::CommandFormInputProcessor { &self.base }
    fn as_CommandFormInputProcessor_mut(&mut self) -> &mut crate::org::allbinary::game::displayable::canvas::CommandFormInputProcessor::CommandFormInputProcessor { &mut self.base }
}
impl crate::org::allbinary::game::input::GameInput::GameInputDyn for ImmediateCommandFormInputProcessor {
    fn as_GameInput(&self) -> &crate::org::allbinary::game::input::GameInput::GameInput { &self.base.base.base.base }
    fn as_GameInput_mut(&mut self) -> &mut crate::org::allbinary::game::input::GameInput::GameInput { &mut self.base.base.base.base }
}
impl crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInputDyn for ImmediateCommandFormInputProcessor {
    fn as_PlayerGameInput(&self) -> &crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInput { &self.base.base.base }
    fn as_PlayerGameInput_mut(&mut self) -> &mut crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInput { &mut self.base.base.base }
    fn onPressGameKeyEvent(&mut self, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) { Self::onPressGameKeyEvent(self, gameKeyEvent) }
    fn onDownGameKeyEvent(&mut self, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) { Self::onDownGameKeyEvent(self, gameKeyEvent) }
    fn onUpGameKeyEvent(&mut self, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) { Self::onUpGameKeyEvent(self, gameKeyEvent) }
}


