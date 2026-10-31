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
// package org.allbinary.input.automation.robot
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::awt::*;
// use crate::java::awt::image::BufferedImage::BufferedImage;
// use crate::javax::help::HelpSet::HelpSet;
use crate::org::allbinary::input::automation::PointHelper::PointHelper;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InputRobot {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub robot: Option<Rc<RefCell<Robot>>>,
}

thread_local! { pub static InputRobot_NAME: String = String::from("Java Robot"); }
// impl InputRobotInterface for InputRobot with the methods of the trait

impl InputRobot {
    

    pub fn new_struct(mut graphicsDevice: Option<Rc<RefCell<GraphicsDevice>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            robot: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.robot = Robot::new(graphicsDevice.clone());
        this
    }

    pub fn new(mut graphicsDevice: Option<Rc<RefCell<GraphicsDevice>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(graphicsDevice))))
    }
    

    pub fn getName(&mut self) -> String {
        return InputRobot_NAME.with(Clone::clone).clone();
    }
    

    pub fn getHelpSet(&mut self) -> Option<Rc<RefCell<HelpSet>>> {
        return None;
    }
    

    pub fn getMousePoint(&mut self) -> Option<Rc<RefCell<Point>>> {
        let mut point: Option<Rc<RefCell<Point>>> = jcall!(MouseInfo::MouseInfo::getPointerInfo()).getLocation();
        return point.clone();
    }
    

    // throws Exception
    pub fn mouseMoveToTarget3(&mut self, mut rectangle: Option<Rc<RefCell<Rectangle>>>, mut x: Option<i32>, mut y: Option<i32>) {
        let mut point: Option<Rc<RefCell<Point>>> = PointHelper::getCenterPoint(rectangle.clone());
        jcall!(self.robot).mouseMove(jcall!(point).x + x, jcall!(point).y + y);
        let mut message: String = jcall!(StringMaker::new()).append(String::from("Moved Mouse To: x: ")).appendint(((jcall!(point).x) as i32).clone()).append(String::from(" y: ")).appendint(((jcall!(point).y) as i32).clone()).append(String::from(" in the middle of: ")).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(rectangle.clone()))).toString();
        jcall!(self.logUtil).putF(message.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("moveMouseToTarget"));
    }
    

    pub fn mouseMove(&mut self, mut point: Option<Rc<RefCell<Point>>>) {
        self.mouseMove2(jcall!(point).x.clone(), jcall!(point).y.clone());
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn mouseMove2(&mut self, mut x: i32, mut y: i32) {
        jcall!(self.robot).mouseMove(x.clone(), y.clone());
        let mut message: String = jcall!(StringMaker::new()).append(String::from("Moved Mouse To: x: ")).appendint(x.clone()).append(String::from(" y: ")).appendint(y.clone()).toString();
        jcall!(self.logUtil).putF(message.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("moveMouse"));
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn mouseMoveToTarget(&mut self, mut rectangle: Option<Rc<RefCell<Rectangle>>>) {
        self.mouseMoveToTarget3(rectangle.clone(), 0, 0);
    }
    

    pub fn createScreenCapture(&mut self, mut screenRect: Option<Rc<RefCell<Rectangle>>>) -> Option<Rc<RefCell<BufferedImage>>> {
        return jcall!(self.robot).createScreenCapture(screenRect.clone());
    }
    

    pub fn delay(&mut self, mut ms: Option<i32>) {
        jcall!(self.robot).delay(ms.clone());
    }
    

    pub fn getAutoDelay(&mut self) -> Option<i32> {
        return jcall!(self.robot).getAutoDelay();
    }
    

    pub fn getPixelColor(&mut self, mut x: Option<i32>, mut y: Option<i32>) -> Option<Rc<RefCell<Color>>> {
        return jcall!(self.robot).getPixelColor(x.clone(), y.clone());
    }
    

    pub fn isAutoWaitForIdle(&mut self) -> bool {
        return jcall!(self.robot).isAutoWaitForIdle();
    }
    

    pub fn keyPress(&mut self, mut keycode: Option<i32>) {
        jcall!(self.robot).keyPress(keycode.clone());
    }
    

    pub fn keyRelease(&mut self, mut keycode: Option<i32>) {
        jcall!(self.robot).keyRelease(keycode.clone());
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn mouseMove2(&mut self, mut x: Option<i32>, mut y: Option<i32>) {
        jcall!(self.robot).mouseMove(x.clone(), y.clone());
    }
    

    pub fn mousePress(&mut self, mut buttons: Option<i32>) {
        jcall!(self.robot).mousePress(buttons.clone());
    }
    

    pub fn mouseRelease(&mut self, mut buttons: Option<i32>) {
        jcall!(self.robot).mouseRelease(buttons.clone());
    }
    

    pub fn mouseWheel(&mut self, mut wheelAmt: Option<i32>) {
        jcall!(self.robot).mouseWheel(wheelAmt.clone());
    }
    

    pub fn setAutoDelay(&mut self, mut ms: Option<i32>) {
        jcall!(self.robot).setAutoDelay(ms.clone());
    }
    

    pub fn setAutoWaitForIdle(&mut self, mut isOn: bool) {
        jcall!(self.robot).setAutoWaitForIdle(isOn.clone());
    }
    

    pub fn toString(&mut self) -> String {
        return jcall!(self.robot).toString();
    }
    

    pub fn waitForIdle(&mut self) {
        jcall!(self.robot).waitForIdle();
    }
    

    pub fn on(&mut self) {
    }
    

    pub fn off(&mut self) {
    }
}

