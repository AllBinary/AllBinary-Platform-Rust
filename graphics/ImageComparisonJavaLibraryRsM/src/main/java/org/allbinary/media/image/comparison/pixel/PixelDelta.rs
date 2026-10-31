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
// package org.allbinary.media.image.comparison.pixel
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::graphics::GPoint::GPoint;
use crate::org::allbinary::TsUtil::TsUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::util::cache::CacheableInterface::CacheableInterface;
use crate::org::allbinary::media::image::comparison::color::ColorDelta::ColorDelta;
use crate::org::allbinary::string::CommonSeps::CommonSeps;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

fn java_string_hash(text: &str) -> i32 {
    text.chars().fold(0i32, |hash, character| hash.wrapping_mul(31).wrapping_add(character as i32))
}

pub struct PixelDelta {
    pub point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>,
    pub colorDelta: Option<Rc<RefCell<ColorDelta>>>,
    pub key: Option<Rc<RefCell<Box<dyn std::any::Any>>>>,
}

impl CacheableInterface for PixelDelta {
    fn getKey(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { Self::getKey(self) }
}

impl PixelDelta {
    

    pub fn new_struct(mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>, mut colorDelta: Option<Rc<RefCell<ColorDelta>>>) -> Self {
        let mut this = Self {
            point: None,
            colorDelta: None,
            key: None,
        };
        this.setPoint(point.clone());
        this.setColorDelta(colorDelta.clone());
        this.key = PixelDelta::getKey2(this.getPoint(), this.getColorDelta());
        this
    }

    pub fn new(mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>, mut colorDelta: Option<Rc<RefCell<ColorDelta>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(point, colorDelta))))
    }
    

    pub fn getKey(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return self.key.clone();
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn getKey2(mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>, mut colorDelta: Option<Rc<RefCell<ColorDelta>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return jcall!(StringMaker::new()).appendint(jcall!(TsUtil::getInstance()).hashCode(crate::java_into_object(point.clone()))).append(jcall!(CommonSeps::getInstance()).UNDERSCORE.clone()).append(jcall!(jcall!(colorDelta).getKey()).toString()).toString();
    }
    

    pub fn getPoint(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> {
        return self.point.clone();
    }
    

    pub fn setPoint(&mut self, mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>) {
        self.point = point.clone();
    }
    

    pub fn getColorDelta(&mut self) -> Option<Rc<RefCell<ColorDelta>>> {
        return self.colorDelta.clone();
    }
    

    pub fn setColorDelta(&mut self, mut colorDelta: Option<Rc<RefCell<ColorDelta>>>) {
        self.colorDelta = colorDelta.clone();
    }
    

    pub fn toString(&mut self) -> String {
        return jcall!(StringMaker::new()).append(String::from("PixelDelta: Point: ")).append(jcall!(self.point).toString()).append(jcall!(CommonSeps::getInstance()).SPACE.clone()).append(jcall!(self.colorDelta).toString()).toString();
    }
}

