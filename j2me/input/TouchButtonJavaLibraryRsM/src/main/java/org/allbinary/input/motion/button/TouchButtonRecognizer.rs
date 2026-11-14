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
use crate::java::lang::Exception::*;
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
    pub releaseHelper: Option<Rc<RefCell<dyn crate::org::allbinary::input::motion::button::ReleaseHelper::ReleaseHelperDyn>>>,
}


impl TouchButtonRecognizer {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

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
        this.lastPressedTouchButtonInput = { let java_field = jcall!(BasicTouchInputFactory::BasicTouchInputFactory::getInstance()).NONE.clone(); java_field }.clone();
        this.currentlyPressedTouchButtonSingleton = CurrentlyPressedTouchButtonSingleton::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct())));
            if jcall!(TouchScreenFactory::TouchScreenFactory::getInstance()).isMultiTouch() {
            jcall!(this).releaseHelper = MultitouchReleaseHelper::new(this.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::input::motion::button::ReleaseHelper::ReleaseHelperDyn>>);
        } else {
            jcall!(this).releaseHelper = ReleaseHelper::new(this.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::input::motion::button::ReleaseHelper::ReleaseHelperDyn>>);
        }
        this
    }
    

    // throws Exception
    pub fn processRelease(&mut self, mut touchButtonInput: Option<Rc<RefCell<TouchButtonInput>>>, mut deviceId: i32) {
        jcall!(self.releaseHelper).release(touchButtonInput.clone(), deviceId.clone());
        let mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>> = jcall!(touchButtonInput).getGameKeyEvent();
        jcall!(self.upGameKeyEventHandler).fireEvent(gameKeyEvent.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>));
        jcall!(jcall!(self.upGameKeyEventHandler).getInstanceForDevice(deviceId.clone())).as_UpGameKeyEventHandlerBase_mut().fireEvent(gameKeyEvent.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>));
        jcall!(self.currentlyPressedTouchButtonSingleton).remove(touchButtonInput.clone());
    }
    

    // throws Exception
    // synchronized
    pub fn releaseTouchButtonInput(&mut self, mut x: i32, mut y: i32, mut deviceId: i32) -> bool {
        let mut touchButtonInput: Option<Rc<RefCell<TouchButtonInput>>> = self.lastPressedTouchButtonInput.clone();
        if jcall!(self.currentlyPressedTouchButtonSingleton).contains(touchButtonInput.clone()) {
            self.lastPressedTouchButtonInput = { let java_value = ({ let java_field = jcall!(BasicTouchInputFactory::BasicTouchInputFactory::getInstance()).NONE.clone(); java_field }).clone(); java_value }.clone();
            self.processRelease(touchButtonInput.clone(), deviceId.clone());
            return true;
        } else {
            let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(TouchButtonFactory::TouchButtonFactory::getInstance()).getList();
            let mut rectangle: Option<Rc<RefCell<Rectangle>>> = None;
            let mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = None;
            let mut touchButton: Option<Rc<RefCell<dyn crate::org::allbinary::input::motion::button::TouchButton::TouchButtonDyn>>> = None;
            for index in (0..={ let java_bound = jcall!(list).size() - 1; java_bound }).rev() {
                touchButton = { let java_value = ({ let java_object = (jcall!(list).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::input::motion::button::TouchButton::TouchButtonDyn>>>>().expect("ClassCastException").clone()) }).clone(); java_value };
                rectangle = { let java_value = (jcall!(touchButton).getRectangle()).clone(); java_value };
                point = { let java_value = (jcall!(rectangle).getPoint()).clone(); java_value };
                if { let java_receiver = self.rectangleCollisionUtil.clone(); let java_arguments = (jcall!(point).getX(), jcall!(point).getY(), jcall!(rectangle).getMaxX(), jcall!(rectangle).getMaxY(), x.clone(), y.clone(),); let java_result = jcall!(java_receiver).isInside(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3, java_arguments.4, java_arguments.5); java_result } {
                    touchButtonInput = { let java_value = (jcall!(touchButton).getTouchButtonInput()).clone(); java_value };
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
        for index in (0..={ let java_bound = jcall!(list).size() - 1; java_bound }).rev() {
            touchButton = { let java_value = ({ let java_object = (jcall!(list).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::input::motion::button::TouchButton::TouchButtonDyn>>>>().expect("ClassCastException").clone()) }).clone(); java_value };
            rectangle = { let java_value = (jcall!(touchButton).getRectangle()).clone(); java_value };
            point = { let java_value = (jcall!(rectangle).getPoint()).clone(); java_value };
            if { let java_receiver = self.rectangleCollisionUtil.clone(); let java_arguments = (jcall!(point).getX(), jcall!(point).getY(), jcall!(rectangle).getMaxX(), jcall!(rectangle).getMaxY(), x.clone(), y.clone(),); let java_result = jcall!(java_receiver).isInside(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3, java_arguments.4, java_arguments.5); java_result } {
                touchButtonInput = { let java_value = (jcall!(touchButton).getTouchButtonInput()).clone(); java_value };
                if !jcall!(self.currentlyPressedTouchButtonSingleton).contains(touchButtonInput.clone()) {
                    jcall!(self.releaseHelper).release(touchButtonInput.clone(), deviceId.clone());
                    self.lastPressedTouchButtonInput = touchButtonInput.clone();
                    jcall!(self.currentlyPressedTouchButtonSingleton).add(touchButtonInput.clone());
                    gameKeyEvent = { let java_value = (jcall!(touchButtonInput).getGameKeyEvent()).clone(); java_value };
                    jcall!(self.downGameKeyEventHandler).fireEvent(gameKeyEvent.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>));
                    jcall!(jcall!(self.downGameKeyEventHandler).getInstanceForDevice(deviceId.clone())).as_DownGameKeyEventHandlerBase_mut().fireEvent(gameKeyEvent.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>));
                }
                return true;
            }
        }
        return false;
    }
}

