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
// package org.allbinary.graphics.color
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::BasicColorFactory::BasicColorFactory;
use super::BasicColor::BasicColor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ColorTemperatureUtil {
    pub basicColorFactory: Option<Rc<RefCell<BasicColorFactory>>>,
}

pub static ColorTemperatureUtil_instance: crate::JavaStatic<Option<Rc<RefCell<ColorTemperatureUtil>>>> = crate::JavaStatic::new(|| ColorTemperatureUtil::new());

impl ColorTemperatureUtil {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }

    pub fn new_struct() -> Self {
        let mut this = Self {
            basicColorFactory: None,
        };
        this.basicColorFactory = BasicColorFactory::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<ColorTemperatureUtil>>> {
        return ColorTemperatureUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn getBasicColor(&mut self, mut thirdTemp: i32) -> Option<Rc<RefCell<BasicColor>>> {
        let mut basicColor: Option<Rc<RefCell<BasicColor>>> = { let java_field = jcall!(self.basicColorFactory).RED.clone(); java_field }.clone();
        if thirdTemp > 3700 {
            basicColor = { let java_value = ({ let java_field = jcall!(self.basicColorFactory).PURPLE.clone(); java_field }).clone(); java_value }.clone();
        } else if thirdTemp > 3200 {
            basicColor = { let java_value = ({ let java_field = jcall!(self.basicColorFactory).BLUE.clone(); java_field }).clone(); java_value }.clone();
        } else if thirdTemp > 2700 {
            basicColor = { let java_value = ({ let java_field = jcall!(self.basicColorFactory).PUCE.clone(); java_field }).clone(); java_value }.clone();
        } else if thirdTemp > 2200 {
            basicColor = { let java_value = ({ let java_field = jcall!(self.basicColorFactory).WHITE.clone(); java_field }).clone(); java_value }.clone();
        } else if thirdTemp > 1700 {
            basicColor = { let java_value = ({ let java_field = jcall!(self.basicColorFactory).GREEN.clone(); java_field }).clone(); java_value }.clone();
        } else if thirdTemp > 1200 {
            basicColor = { let java_value = ({ let java_field = jcall!(self.basicColorFactory).YELLOW.clone(); java_field }).clone(); java_value }.clone();
        } else if thirdTemp > 700 {
            basicColor = { let java_value = ({ let java_field = jcall!(self.basicColorFactory).ORANGE.clone(); java_field }).clone(); java_value }.clone();
        }
        return basicColor.clone();
    }
}

