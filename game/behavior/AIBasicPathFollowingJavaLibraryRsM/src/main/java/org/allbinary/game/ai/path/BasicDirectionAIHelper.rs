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
// package org.allbinary.game.ai.path
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Canvas::Canvas;
use crate::org::allbinary::direction::Direction::Direction;
use crate::org::allbinary::direction::DirectionFactory::DirectionFactory;
use crate::org::allbinary::math::AngleIncrementInfo::AngleIncrementInfo;
use crate::org::allbinary::math::AngleInfo::AngleInfo;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicDirectionAIHelper {
    pub name: String,
    pub angleInfo: Option<Rc<RefCell<AngleInfo>>>,
}


impl BasicDirectionAIHelper {
    

    pub fn new_struct(mut name: String, mut angleInfo: Option<Rc<RefCell<AngleInfo>>>) -> Self {
        let mut this = Self {
            name: String::new(),
            angleInfo: None,
        };
        this.name = name.clone();
        this.angleInfo = angleInfo.clone();
        this
    }

    pub fn new(mut name: String, mut angleInfo: Option<Rc<RefCell<AngleInfo>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(name, angleInfo))))
    }
    

    pub fn turnAI(&mut self, mut frame: i32) -> i32 {
        let mut keyDirection: i32 = -1;
        let mut directionAngle: i32 = jcall!(jcall!(self.angleInfo).getAngleIncrementInfo()).getFrameAngle(frame.clone());
        let mut angle: i16 = jcall!(self.angleInfo).getAngle();
        let mut degrees: i32 = (directionAngle - ((angle) as i32)).abs();
        if degrees < 180 {
            if ((angle) as i32) > directionAngle {
                keyDirection = unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_LEFT.clone() }.clone();
            } else if ((angle) as i32) < directionAngle {
                keyDirection = unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_RIGHT.clone() }.clone();
            }
        } else {
            if ((angle) as i32) > directionAngle {
                keyDirection = unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_RIGHT.clone() }.clone();
            } else if ((angle) as i32) < directionAngle {
                keyDirection = unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_LEFT.clone() }.clone();
            }
        }
        return keyDirection.clone();
    }
    

    pub fn getAIKeyPressed(&mut self, mut frame: Option<i32>) -> i32 {
        let mut angle: i16 = ((jcall!(jcall!(self.angleInfo).getAngleIncrementInfo()).getFrameAngle(((frame).unwrap() as i32))) as i16);
        if ((jcall!(self.angleInfo).getAngle()) as i32) != ((angle) as i32) {
            return { let java_arguments = (((frame).unwrap() as i32),); self.turnAI(java_arguments.0) };
        } else {
            return unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_UP.clone() }.clone();
        }
    }
    

    pub fn getAIKeyPressedFromDirection(&mut self, mut geographicMapDirectionData: Option<Rc<RefCell<Direction>>>) -> i32 {
        let mut keyDirection: i32 = -1;
        let mut angleIncrementInfo: Option<Rc<RefCell<AngleIncrementInfo>>> = jcall!(self.angleInfo).getAngleIncrementInfo();
        let mut directionFactory: Option<Rc<RefCell<DirectionFactory>>> = DirectionFactory::getInstance();
        if match (&geographicMapDirectionData, &jcall!(directionFactory).DOWN) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            keyDirection = self.getAIKeyPressed(jcall!(angleIncrementInfo).DOWN_FRAME.clone());
        } else if match (&geographicMapDirectionData, &jcall!(directionFactory).UP) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            keyDirection = self.getAIKeyPressed(jcall!(angleIncrementInfo).UP_FRAME.clone());
        } else if match (&geographicMapDirectionData, &jcall!(directionFactory).LEFT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            keyDirection = self.getAIKeyPressed(jcall!(angleIncrementInfo).LEFT_FRAME.clone());
        } else if match (&geographicMapDirectionData, &jcall!(directionFactory).RIGHT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            keyDirection = self.getAIKeyPressed(jcall!(angleIncrementInfo).RIGHT_FRAME.clone());
        }
        return keyDirection.clone();
    }
    

    pub fn getAngleInfoP(&mut self) -> Option<Rc<RefCell<AngleInfo>>> {
        return self.angleInfo.clone();
    }
    

    pub fn getName(&mut self) -> String {
        return self.name.clone();
    }
}

