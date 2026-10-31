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

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait InputRobotInterface {
    

    fn getName(&mut self) -> String;
    

    fn getHelpSet(&mut self) -> Option<Rc<RefCell<HelpSet>>>;
    

    fn createScreenCapture(&mut self, screenRect: Option<Rc<RefCell<Rectangle>>>) -> Option<Rc<RefCell<BufferedImage>>>;
    

    fn delay(&mut self, ms: Option<i32>);
    

    fn getAutoDelay(&mut self) -> Option<i32>;
    

    fn getPixelColor(&mut self, x: Option<i32>, y: Option<i32>) -> Option<Rc<RefCell<Color>>>;
    

    fn isAutoWaitForIdle(&mut self) -> bool;
    

    fn keyPress(&mut self, keycode: Option<i32>);
    

    fn keyRelease(&mut self, keycode: Option<i32>);
    

    fn getMousePoint(&mut self) -> Option<Rc<RefCell<Point>>>;
    

    fn mouseMove2(&mut self, x: Option<i32>, y: Option<i32>);
    

    fn mousePress(&mut self, buttons: Option<i32>);
    

    fn mouseRelease(&mut self, buttons: Option<i32>);
    

    fn mouseWheel(&mut self, wheelAmt: Option<i32>);
    

    fn setAutoDelay(&mut self, ms: Option<i32>);
    

    fn setAutoWaitForIdle(&mut self, isOn: bool);
    

    fn toString(&mut self) -> String;
    

    fn waitForIdle(&mut self);
    

    // Java overloaded method: Rust needs a different name
    fn mouseMove(&mut self, point: Option<Rc<RefCell<Point>>>);
    

    // throws Exception
    fn mouseMoveToTarget(&mut self, rectangle: Option<Rc<RefCell<Rectangle>>>);
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    fn mouseMoveToTarget3(&mut self, rectangle: Option<Rc<RefCell<Rectangle>>>, x: Option<i32>, y: Option<i32>);
}


