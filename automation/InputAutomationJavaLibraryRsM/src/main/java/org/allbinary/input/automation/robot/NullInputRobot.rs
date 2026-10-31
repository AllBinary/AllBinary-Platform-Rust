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
use crate::org::allbinary::input::automation::AbstractInputRobot::AbstractInputRobot;
use crate::org::allbinary::input::automation::PointFactory::PointFactory;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NullInputRobot {
    pub base: AbstractInputRobot,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

thread_local! { pub static NullInputRobot_LIBRARY_NAME: String = String::from("null"); }
thread_local! { pub static NullInputRobot_NAME: String = String::from("NULL"); }
// impl InputRobotInterface for NullInputRobot with the methods of the trait

impl NullInputRobot {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: AbstractInputRobot::new_struct(None),
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getName(&mut self) -> String {
        return NullInputRobot_NAME.with(Clone::clone).clone();
    }
    

    pub fn getHelpSet(&mut self) -> Option<Rc<RefCell<HelpSet>>> {
        return None;
    }
    

    pub fn getMousePoint(&mut self) -> Option<Rc<RefCell<Point>>> {
        return PointFactory::getInstance(0, 0);
    }
    

    pub fn keyPress(&mut self, mut keycode: Option<i32>) {
        jcall!(self.logUtil).putF(format!("{}{}", String::from("KeyCode: "), keycode), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("keyPress"));
    }
    

    pub fn keyRelease(&mut self, mut keycode: Option<i32>) {
        jcall!(self.logUtil).putF(format!("{}{}", String::from("KeyCode: "), keycode), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("keyRelease"));
    }
    

    pub fn mouseMove2(&mut self, mut x: Option<i32>, mut y: Option<i32>) {
        jcall!(self.logUtil).putF(format!("{}{}", format!("{}{}", format!("{}{}", String::from("X: "), x), String::from(" Y: ")), y), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("mouseMove"));
    }
    

    pub fn mousePress(&mut self, mut buttons: Option<i32>) {
        jcall!(self.logUtil).putF(format!("{}{}", String::from("Buttons: "), buttons), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("mousePress"));
    }
    

    pub fn mouseRelease(&mut self, mut buttons: Option<i32>) {
        jcall!(self.logUtil).putF(format!("{}{}", String::from("Buttons: "), buttons), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("mouseRelease"));
    }
    

    pub fn createScreenCapture(&mut self, mut screenRect: Option<Rc<RefCell<Rectangle>>>) -> Option<Rc<RefCell<BufferedImage>>> {
        return None;
    }
    

    pub fn delay(&mut self, mut ms: Option<i32>) {
    }
    

    pub fn getAutoDelay(&mut self) -> Option<i32> {
        return -1;
    }
    

    pub fn getPixelColor(&mut self, mut x: Option<i32>, mut y: Option<i32>) -> Option<Rc<RefCell<Color>>> {
        return None;
    }
    

    pub fn isAutoWaitForIdle(&mut self) -> bool {
        return false;
    }
    

    pub fn mouseWheel(&mut self, mut wheelAmt: Option<i32>) {
    }
    

    pub fn setAutoDelay(&mut self, mut ms: Option<i32>) {
    }
    

    pub fn setAutoWaitForIdle(&mut self, mut isOn: bool) {
    }
    

    pub fn toString(&mut self) -> String {
        return String::from("DirectXWindowsInputRobot");
    }
    

    pub fn waitForIdle(&mut self) {
    }
}
impl std::ops::Deref for NullInputRobot {
    type Target = AbstractInputRobot;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for NullInputRobot {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::input::automation::AbstractInputRobot::AbstractInputRobotDyn for NullInputRobot {
    fn as_AbstractInputRobot(&self) -> &crate::org::allbinary::input::automation::AbstractInputRobot::AbstractInputRobot { &self.base }
    fn as_AbstractInputRobot_mut(&mut self) -> &mut crate::org::allbinary::input::automation::AbstractInputRobot::AbstractInputRobot { &mut self.base }
    fn getHelpSet(&mut self) -> Option<Rc<RefCell<HelpSet>>> { Self::getHelpSet(self) }
    fn mouseMove2(&mut self, mut x: Option<i32>, mut y: Option<i32>) { Self::mouseMove2(self, x, y) }
}


