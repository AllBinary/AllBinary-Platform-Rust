// Generated Code Do Not Modify
// package org.allbinary.graphics.displayable
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::javax::microedition::lcdui::Image::Image;
use super::DisplayInfoSingleton::DisplayInfoSingleton;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ScreenRelationalUtil {
}

pub static ScreenRelationalUtil_instance: crate::JavaStatic<Option<Rc<RefCell<ScreenRelationalUtil>>>> = crate::JavaStatic::new(|| ScreenRelationalUtil::new());

impl ScreenRelationalUtil {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<ScreenRelationalUtil>>> {
        return ScreenRelationalUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn getScaleImage(&mut self, mut image: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>) -> f32 {
        return { let java_arguments = (jcall!(image).getWidth(), jcall!(image).getHeight(),); self.getScaleWH(java_arguments.0, java_arguments.1) };
    }
    

    pub fn getScaleWH(&mut self, mut aWidth: i32, mut aHeight: i32) -> f32 {
        return self.getScaleWHA(aWidth.clone(), aHeight.clone(), 20);
    }
    

    pub fn getScaleWHA(&mut self, mut aWidth: i32, mut aHeight: i32, mut add: i32) -> f32 {
        let mut displayInfoSingleton: Option<Rc<RefCell<DisplayInfoSingleton>>> = DisplayInfoSingleton::getInstance();
        let mut last: Vec<i32> = jcall!(displayInfoSingleton).getLast();
        let mut width: f32 = ((last[({ let java_field = jcall!(displayInfoSingleton).WIDTH.clone(); java_field }) as usize]) as f32);
        let mut height: f32 = ((last[({ let java_field = jcall!(displayInfoSingleton).HEIGHT.clone(); java_field }) as usize]) as f32);
        let mut largestSize: f32 = width.clone();
        if largestSize < height {
            largestSize = height.clone();
        }
        let mut scale: f32 = (largestSize) / ((aWidth) as f32);
        return scale.clone();
    }
    

    pub fn getScaledHeight(&mut self, mut aWidth: i32, mut aHeight: i32) -> i32 {
        return (((self.getScaleWH(aWidth.clone(), aHeight.clone()) * ((aHeight) as f32))) as i32);
    }
    

    pub fn getScaledHeightAdd(&mut self, mut aWidth: i32, mut aHeight: i32, mut add: i32) -> i32 {
        return (((self.getScaleWHA(aWidth.clone(), aHeight.clone(), add.clone()) * ((aHeight) as f32))) as i32);
    }
}

