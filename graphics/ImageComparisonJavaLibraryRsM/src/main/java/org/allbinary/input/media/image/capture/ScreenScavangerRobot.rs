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
// package org.allbinary.input.media.image.capture
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::awt::Dimension::Dimension;
// use crate::java::awt::Rectangle::Rectangle;
// use crate::java::awt::Toolkit::Toolkit;
// use crate::java::awt::image::BufferedImage::BufferedImage;
use crate::org::allbinary::input::automation::robot::InputRobotFactory::InputRobotFactory;
use crate::org::allbinary::input::automation::robot::InputRobotInterface::InputRobotInterface;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::ABHashtable::ABHashtable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ScreenScavangerRobot {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
}


impl ScreenScavangerRobot {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn getScreenAsBufferedImages(&mut self) -> Vec<Option<Rc<RefCell<BufferedImage>>>> {
        jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getScreenAsBufferedImages"));
        let mut dimScreenSize: Option<Rc<RefCell<Dimension>>> = jcall!(Toolkit::getDefaultToolkit()).getScreenSize();
        let mut rectScreenSize: Option<Rc<RefCell<Rectangle>>> = Rectangle::new(dimScreenSize.clone());
        let mut robotHashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> = jcall!(InputRobotFactory::getInstance()).get();
        let mut bufferedImageArray: Vec<Option<Rc<RefCell<BufferedImage>>>> = vec![None; (1) as usize];
        let mut index: i32 = 0;
        let mut inputRobotInterface: Option<Rc<RefCell<dyn InputRobotInterface>>> = { let java_object = jcall!(robotHashtable).get(crate::java_into_object(String::from("Java Robot"))); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn InputRobotInterface>>>>().expect("ClassCastException").clone()) };
        bufferedImageArray[(index) as usize] = jcall!(inputRobotInterface).createScreenCapture(rectScreenSize.clone());
        jcall!(self.logUtil).putF(String::from("Finish"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getScreenAsBufferedImages"));
        return bufferedImageArray.clone();
    }
}

