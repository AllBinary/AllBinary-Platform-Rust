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
use crate::org::allbinary::game::input::GameInputStrings::GameInputStrings;
use crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent;
use crate::org::allbinary::graphics::GPoint::GPoint;
use crate::org::allbinary::graphics::Rectangle::Rectangle;
use crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas;
use crate::org::allbinary::input::motion::gesture::MotionGestureInput::MotionGestureInput;
use crate::org::allbinary::input::motion::gesture::TouchMotionGestureFactory::TouchMotionGestureFactory;
use crate::org::allbinary::input::motion::gesture::observer::MotionGestureEvent::MotionGestureEvent;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::math::RectangleCollisionUtil::RectangleCollisionUtil;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelper;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use super::BasicMenuInputProcessor::BasicMenuInputProcessor;
use super::AllBinaryGameCanvas::AllBinaryGameCanvas;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PopupMenuInputProcessor {
    pub base: BasicMenuInputProcessor,
    pub rectangleCollisionUtil: Option<Rc<RefCell<RectangleCollisionUtil>>>,
    pub CLICK_DELAY: i32,
    pub clickTimeHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>>,
    pub rectangle: Option<Rc<RefCell<Rectangle>>>,
}

impl crate::org::allbinary::game::input::event::GameKeyEventListenerInterface::GameKeyEventListenerInterface for PopupMenuInputProcessor {
}

impl PopupMenuInputProcessor {
    

    pub fn new_struct(mut gameKeyEventList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut playerInputId: i32, mut gameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn>>>, mut rectangle: Option<Rc<RefCell<Rectangle>>>) -> Self {
        let mut this = Self {
            base: BasicMenuInputProcessor::new_struct(gameKeyEventList.clone(), playerInputId.clone(), gameCanvas.clone()),
            rectangleCollisionUtil: None,
            CLICK_DELAY: 0,
            clickTimeHelper: None,
            rectangle: None,
        };
        this.rectangleCollisionUtil = RectangleCollisionUtil::getInstance();
        this.CLICK_DELAY = 120;
        this.clickTimeHelper = TimeDelayHelper::new(this.CLICK_DELAY.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>);
        // super(...) is in the struct literal above
        this.rectangle = rectangle.clone();
        this
    }

    pub fn new(mut gameKeyEventList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut playerInputId: i32, mut gameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn>>>, mut rectangle: Option<Rc<RefCell<Rectangle>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(gameKeyEventList, playerInputId, gameCanvas, rectangle))))
    }
    

    pub fn init(&mut self, mut rectangle: Option<Rc<RefCell<Rectangle>>>) {
        self.rectangle = rectangle.clone();
    }
    

    // throws Exception
    pub fn processInput(&mut self, mut key: i32) -> i32 {
        if key == unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_STAR.clone() } {
            let mut gameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>> = (self.getCanvas()).clone() /* as AllBinaryGameCanvas */;
            jcall!(gameCanvas).as_MyCanvas_mut().toggleMenu();
            return 1;
        }
        return 0;
    }
    

    // throws Exception
    pub fn processInputList(&mut self) -> i32 {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut motionInputsIndex: i32 = self.processMotionInputs();
            let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.getGameKeyEventList();
            let mut size: i32 = jcall!(list).size();
            let mut key: i32 = 0;
            let mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>> = None;
            for index in (0..size) {
                gameKeyEvent = { let java_object = (jcall!(list).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<GameKeyEvent>>>>().expect("ClassCastException").clone()) };
                if gameKeyEvent.is_some() {
                    key = jcall!(gameKeyEvent).getKey();
                    if self.processInput(key.clone()) == 1 {
                        break;
                    }
                }
            }
            self.clear();
            if size > 0 || motionInputsIndex >= 0 {
                return 1;
            } else {
                return -1;
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(GameInputStrings::getInstance()).PROCESS_INPUT.clone(), crate::java_into_object(e.clone()),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3); };
            return -1;
            }
        }
    }
    

    // throws Exception
    pub fn processMotionInputs(&mut self) -> i32 {
        let mut lastIndex: i32 = jcall!(self.motionGestureEventList).size() - 1;
        if lastIndex >= 0 {
            let mut motionGestureEvent: Option<Rc<RefCell<MotionGestureEvent>>> = { let java_object = (jcall!(self.motionGestureEventList).as_BasicArrayList_mut().objectArray[(lastIndex) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<MotionGestureEvent>>>>().expect("ClassCastException").clone()) };
            self.processMotionInput(motionGestureEvent.clone());
        }
        jcall!(self.motionGestureEventList).clear();
        return lastIndex.clone();
    }
    

    // throws Exception
    pub fn processMotionInput(&mut self, mut motionGestureEvent: Option<Rc<RefCell<MotionGestureEvent>>>) {
        if motionGestureEvent.is_none() {
            jcall!(self.logUtil).putF(String::from("Exception: Bug"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.gameInputStrings).PROCESS_MOTION_INPUT.clone());
            return;
        }
        let mut touchMotionGestureFactory: Option<Rc<RefCell<TouchMotionGestureFactory>>> = TouchMotionGestureFactory::getInstance();
        let mut motionGestureInput: Option<Rc<RefCell<MotionGestureInput>>> = jcall!(motionGestureEvent).getMotionGesture();
        if match (&motionGestureInput, &jcall!(touchMotionGestureFactory).RELEASED) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            let mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = jcall!(motionGestureEvent).getCurrentPoint();
            let mut rectPoint: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = jcall!(self.rectangle).getPoint();
            if { let java_receiver = self.rectangleCollisionUtil.clone(); let java_arguments = (jcall!(rectPoint).getX(), jcall!(rectPoint).getY(), jcall!(self.rectangle).getMaxX() + 20, jcall!(self.rectangle).getMaxY(), jcall!(point).getX(), jcall!(point).getY(),); let java_result = jcall!(java_receiver).isInside(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3, java_arguments.4, java_arguments.5); java_result } {
                if jcall!(self.clickTimeHelper).isTimeTNT() {
                    let mut gameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>> = (self.getCanvas()).clone() /* as AllBinaryGameCanvas */;
                    jcall!(gameCanvas).as_MyCanvas_mut().toggleMenu();
                }
            }
        }
    }
}
impl std::ops::Deref for PopupMenuInputProcessor {
    type Target = BasicMenuInputProcessor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for PopupMenuInputProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::displayable::canvas::BasicMenuInputProcessor::BasicMenuInputProcessorDyn for PopupMenuInputProcessor {
    fn as_BasicMenuInputProcessor(&self) -> &crate::org::allbinary::game::displayable::canvas::BasicMenuInputProcessor::BasicMenuInputProcessor { &self.base }
    fn as_BasicMenuInputProcessor_mut(&mut self) -> &mut crate::org::allbinary::game::displayable::canvas::BasicMenuInputProcessor::BasicMenuInputProcessor { &mut self.base }
    fn processInputList(&mut self) -> i32 { Self::processInputList(self) }
}
impl crate::org::allbinary::game::input::GameInput::GameInputDyn for PopupMenuInputProcessor {
    fn as_GameInput(&self) -> &crate::org::allbinary::game::input::GameInput::GameInput { &self.base.base.base }
    fn as_GameInput_mut(&mut self) -> &mut crate::org::allbinary::game::input::GameInput::GameInput { &mut self.base.base.base }
}
impl crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInputDyn for PopupMenuInputProcessor {
    fn as_PlayerGameInput(&self) -> &crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInput { &self.base.base }
    fn as_PlayerGameInput_mut(&mut self) -> &mut crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInput { &mut self.base.base }
}


