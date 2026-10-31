// 
// AllBinary Open License Version 1 
// Copyright (c) 2022 AllBinary 
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
// package org.allbinary.media
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::awt::Dimension::Dimension;
// use crate::java::awt::DisplayMode::DisplayMode;
// use crate::java::awt::GraphicsDevice::GraphicsDevice;
// use crate::java::awt::GraphicsEnvironment::GraphicsEnvironment;
// use crate::java::awt::Toolkit::Toolkit;
use crate::org::allbinary::graphics::PointFactory::PointFactory;
use crate::org::allbinary::graphics::Rectangle::Rectangle;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct DisplaysUtil {
}

thread_local! { pub static DisplaysUtil_instance: Option<Rc<RefCell<DisplaysUtil>>> = DisplaysUtil::new(); }

impl DisplaysUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<DisplaysUtil>>> {
        return DisplaysUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn getPrimaryScreenSize(&mut self) -> Option<Rc<RefCell<Rectangle>>> {
        let mut dimension: Option<Rc<RefCell<Dimension>>> = jcall!(Toolkit::getDefaultToolkit()).getScreenSize();
        return Rectangle::new(jcall!(PointFactory::getInstance()).ZERO_ZERO.clone(), jcall!(dimension).width.clone(), jcall!(dimension).height.clone());
    }
    

    pub fn getScreenSizesAsRectangleArray(&mut self) -> Vec<Option<Rc<RefCell<Rectangle>>>> {
        let mut graphicsEnvironment: Option<Rc<RefCell<GraphicsEnvironment>>> = GraphicsEnvironment::getLocalGraphicsEnvironment();
        let mut graphicsDeviceArray: Vec<Option<Rc<RefCell<GraphicsDevice>>>> = jcall!(graphicsEnvironment).getScreenDevices();
        let mut graphicsDevice: Option<Rc<RefCell<GraphicsDevice>>> = None;
        let mut displayMode: Option<Rc<RefCell<DisplayMode>>> = None;
        let mut rectangle: Option<Rc<RefCell<Rectangle>>> = None;
        let mut size: i32 = (graphicsDeviceArray.len() as i32).clone();
        let mut rectangleArray: Vec<Option<Rc<RefCell<Rectangle>>>> = vec![None; (size) as usize];
        for index in (0..size) {
            graphicsDevice = graphicsDeviceArray[(index) as usize].clone();
            displayMode = jcall!(graphicsDevice).getDisplayMode();
            rectangle = Rectangle::new(jcall!(PointFactory::getInstance()).ZERO_ZERO.clone(), jcall!(displayMode).getWidth(), jcall!(displayMode).getHeight());
            rectangleArray[(index) as usize] = rectangle.clone();
        }
        return rectangleArray.clone();
    }
}

