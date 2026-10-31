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
// package org.allbinary.input.motion.button
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::input::event::DownGameKeyEventHandler::DownGameKeyEventHandler;
use crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent;
use crate::org::allbinary::game::input::event::UpGameKeyEventHandler::UpGameKeyEventHandler;
use crate::org::allbinary::graphics::GPoint::GPoint;
use crate::org::allbinary::graphics::Rectangle::Rectangle;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::math::RectangleCollisionUtil::RectangleCollisionUtil;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use super::TouchButtonInput::TouchButtonInput;
use super::CurrentlyPressedTouchButtonSingleton::CurrentlyPressedTouchButtonSingleton;
use super::ReleaseHelper::ReleaseHelper;
use super::MultitouchReleaseHelper::MultitouchReleaseHelper;
use super::TouchButton::TouchButton;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TouchButtonRecognizer {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub rectangleCollisionUtil: Option<Rc<RefCell<RectangleCollisionUtil>>>,
    pub upGameKeyEventHandler: Option<Rc<RefCell<UpGameKeyEventHandler>>>,
    pub downGameKeyEventHandler: Option<Rc<RefCell<DownGameKeyEventHandler>>>,
    pub lastPressedTouchButtonInput: Option<Rc<RefCell<TouchButtonInput>>>,
    pub currentlyPressedTouchButtonSingleton: Option<Rc<RefCell<dyn crate::org::allbinary::input::motion::button::CurrentlyPressedTouchButtonSingleton::CurrentlyPressedTouchButtonSingletonDyn>>>,
    pub releaseHelper: Option<Rc<RefCell<ReleaseHelper>>>,
}


impl TouchButtonRecognizer {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            rectangleCollisionUtil: None,
            upGameKeyEventHandler: None,
            downGameKeyEventHandler: None,
            lastPressedTouchButtonInput: None,
            currentlyPressedTouchButtonSingleton: None,
            releaseHelper: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.rectangleCollisionUtil = RectangleCollisionUtil::getInstance();
        this.upGameKeyEventHandler = UpGameKeyEventHandler::getInstance();
        this.downGameKeyEventHandler = DownGameKeyEventHandler::getInstance();
        this.lastPressedTouchButtonInput = jcall!(BasicTouchInputFactory::BasicTouchInputFactory::getInstance()).NONE.clone();
        this.currentlyPressedTouchButtonSingleton = CurrentlyPressedTouchButtonSingleton::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct())));
            if jcall!(TouchScreenFactory::TouchScreenFactory::getInstance()).isMultiTouch() {
            this.releaseHelper = MultitouchReleaseHelper::new((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()));
        } else {
            this.releaseHelper = ReleaseHelper::new((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()));
        }
        this
    }
    

    // throws Exception
    pub fn processRelease(&mut self, mut touchButtonInput: Option<Rc<RefCell<TouchButtonInput>>>, mut deviceId: i32) {
        jcall!(self.releaseHelper).release(touchButtonInput.clone(), deviceId.clone());
        let mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>> = jcall!(touchButtonInput).getGameKeyEvent();
        jcall!(self.upGameKeyEventHandler).fireEvent(gameKeyEvent.clone());
        jcall!(jcall!(self.upGameKeyEventHandler).getInstanceForDevice(deviceId.clone())).fireEvent(gameKeyEvent.clone());
        jcall!(self.currentlyPressedTouchButtonSingleton).remove(touchButtonInput.clone());
    }
    

    // throws Exception
    // synchronized
    pub fn releaseTouchButtonInput(&mut self, mut x: i32, mut y: i32, mut deviceId: i32) -> bool {
        let mut touchButtonInput: Option<Rc<RefCell<TouchButtonInput>>> = self.lastPressedTouchButtonInput.clone();
        if jcall!(self.currentlyPressedTouchButtonSingleton).contains(touchButtonInput.clone()) {
            self.lastPressedTouchButtonInput = jcall!(BasicTouchInputFactory::BasicTouchInputFactory::getInstance()).NONE.clone();
            self.processRelease(touchButtonInput.clone(), deviceId.clone());
            return true;
        } else {
            let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(TouchButtonFactory::TouchButtonFactory::getInstance()).getList();
            let mut rectangle: Option<Rc<RefCell<Rectangle>>> = None;
            let mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = None;
            let mut touchButton: Option<Rc<RefCell<dyn crate::org::allbinary::input::motion::button::TouchButton::TouchButtonDyn>>> = None;
            for index in (0..=jcall!(list).size() - 1).rev() {
                touchButton = { let java_object = (jcall!(list).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::input::motion::button::TouchButton::TouchButtonDyn>>>>().expect("ClassCastException").clone()) };
                rectangle = jcall!(touchButton).getRectangle();
                point = jcall!(rectangle).getPoint();
                if { let java_receiver = self.rectangleCollisionUtil.clone(); let java_arguments = (jcall!(point).getX(), jcall!(point).getY(), jcall!(rectangle).getMaxX(), jcall!(rectangle).getMaxY(), x.clone(), y.clone(),); let java_result = jcall!(java_receiver).isInside(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3, java_arguments.4, java_arguments.5); java_result } {
                    touchButtonInput = jcall!(touchButton).getTouchButtonInput();
                    self.processRelease(touchButtonInput.clone(), deviceId.clone());
                    return true;
                }
            }
        }
        return false;
    }
    

    // throws Exception
    // synchronized
    pub fn pressTouchButtonInput(&mut self, mut x: i32, mut y: i32, mut deviceId: i32) -> bool {
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(TouchButtonFactory::TouchButtonFactory::getInstance()).getList();
        let mut rectangle: Option<Rc<RefCell<Rectangle>>> = None;
        let mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = None;
        let mut touchButton: Option<Rc<RefCell<dyn crate::org::allbinary::input::motion::button::TouchButton::TouchButtonDyn>>> = None;
        let mut touchButtonInput: Option<Rc<RefCell<TouchButtonInput>>> = None;
        let mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>> = None;
        for index in (0..=jcall!(list).size() - 1).rev() {
            touchButton = { let java_object = (jcall!(list).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::input::motion::button::TouchButton::TouchButtonDyn>>>>().expect("ClassCastException").clone()) };
            rectangle = jcall!(touchButton).getRectangle();
            point = jcall!(rectangle).getPoint();
            if { let java_receiver = self.rectangleCollisionUtil.clone(); let java_arguments = (jcall!(point).getX(), jcall!(point).getY(), jcall!(rectangle).getMaxX(), jcall!(rectangle).getMaxY(), x.clone(), y.clone(),); let java_result = jcall!(java_receiver).isInside(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3, java_arguments.4, java_arguments.5); java_result } {
                touchButtonInput = jcall!(touchButton).getTouchButtonInput();
                if !jcall!(self.currentlyPressedTouchButtonSingleton).contains(touchButtonInput.clone()) {
                    jcall!(self.releaseHelper).release(touchButtonInput.clone(), deviceId.clone());
                    self.lastPressedTouchButtonInput = touchButtonInput.clone();
                    jcall!(self.currentlyPressedTouchButtonSingleton).add(touchButtonInput.clone());
                    gameKeyEvent = jcall!(touchButtonInput).getGameKeyEvent();
                    jcall!(self.downGameKeyEventHandler).fireEvent(gameKeyEvent.clone());
                    jcall!(jcall!(self.downGameKeyEventHandler).getInstanceForDevice(deviceId.clone())).fireEvent(gameKeyEvent.clone());
                }
                return true;
            }
        }
        return false;
    }
}

//CurrentlyPressedTouchButtonSingletonDebug.getInstance();
pub struct ReleaseHelper {
    pub touchButtonRecognizer: Option<Rc<RefCell<TouchButtonRecognizer>>>,
}


impl ReleaseHelper {
        

        pub fn new_struct(mut touchButtonRecognizer: Option<Rc<RefCell<TouchButtonRecognizer>>>) -> Self {
            let mut this = Self {
            touchButtonRecognizer: None,
            };
            this.touchButtonRecognizer = touchButtonRecognizer.clone();
            this
        }

        pub fn new(mut touchButtonRecognizer: Option<Rc<RefCell<TouchButtonRecognizer>>>) -> Option<Rc<RefCell<Self>>> {
            Some(Rc::new(RefCell::new(Self::new_struct(touchButtonRecognizer))))
        }
        

        // throws Exception
        pub fn release(&mut self, mut touchButtonInput: Option<Rc<RefCell<TouchButtonInput>>>, mut deviceId: i32) {
            let mut nextTouchButtonInput: Option<Rc<RefCell<TouchButtonInput>>> = None;
            let mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>> = None;
            for index in (0..=jcall!(jcall!(self.touchButtonRecognizer).currentlyPressedTouchButtonSingleton).size() - 1).rev() {
                nextTouchButtonInput = jcall!(jcall!(self.touchButtonRecognizer).currentlyPressedTouchButtonSingleton).get(index.clone());
                gameKeyEvent = jcall!(nextTouchButtonInput).getGameKeyEvent();
                jcall!(jcall!(self.touchButtonRecognizer).upGameKeyEventHandler).fireEvent(gameKeyEvent.clone());
                jcall!(jcall!(jcall!(self.touchButtonRecognizer).upGameKeyEventHandler).getInstanceForDevice(deviceId.clone())).fireEvent(gameKeyEvent.clone());
                jcall!(jcall!(self.touchButtonRecognizer).currentlyPressedTouchButtonSingleton).removeAt(index.clone());
            }
        }
}

pub struct MultitouchReleaseHelper {
    pub base: ReleaseHelper,
}


impl MultitouchReleaseHelper {
        

        pub fn new_struct(mut touchButtonRecognizer: Option<Rc<RefCell<TouchButtonRecognizer>>>) -> Self {
            let mut this = Self {
            base: ReleaseHelper::new_struct1(touchButtonRecognizer.clone()),
            };
            // super(...) is in the struct literal above
            this
        }

        pub fn new(mut touchButtonRecognizer: Option<Rc<RefCell<TouchButtonRecognizer>>>) -> Option<Rc<RefCell<Self>>> {
            Some(Rc::new(RefCell::new(Self::new_struct(touchButtonRecognizer))))
        }
        

        // throws Exception
        pub fn release(&mut self, mut touchButtonInput: Option<Rc<RefCell<TouchButtonInput>>>, mut deviceId: i32) {
            let mut cancelTouchButtonInput: Option<Rc<RefCell<TouchButtonInput>>> = jcall!(CancelTouchButtonInputFactory::CancelTouchButtonInputFactory::getInstance()).getCancel(touchButtonInput.clone());
            let mut nextTouchButtonInput: Option<Rc<RefCell<TouchButtonInput>>> = None;
            let mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>> = None;
            for index in (0..=jcall!(jcall!(self.touchButtonRecognizer).currentlyPressedTouchButtonSingleton).size() - 1).rev() {
                nextTouchButtonInput = jcall!(jcall!(self.touchButtonRecognizer).currentlyPressedTouchButtonSingleton).get(index.clone());
                if match (&cancelTouchButtonInput, &nextTouchButtonInput) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                    gameKeyEvent = jcall!(nextTouchButtonInput).getGameKeyEvent();
                    jcall!(jcall!(self.touchButtonRecognizer).upGameKeyEventHandler).fireEvent(gameKeyEvent.clone());
                    jcall!(jcall!(jcall!(self.touchButtonRecognizer).upGameKeyEventHandler).getInstanceForDevice(deviceId.clone())).fireEvent(gameKeyEvent.clone());
                    jcall!(jcall!(self.touchButtonRecognizer).currentlyPressedTouchButtonSingleton).removeAt(index.clone());
                }
            }
        }
}
impl std::ops::Deref for MultitouchReleaseHelper {
    type Target = ReleaseHelper;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for MultitouchReleaseHelper {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



