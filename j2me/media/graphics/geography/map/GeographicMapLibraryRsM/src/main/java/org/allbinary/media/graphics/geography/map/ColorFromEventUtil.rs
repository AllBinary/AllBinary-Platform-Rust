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
// package org.allbinary.media.graphics.geography.map
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use super::GeographicMapCompositeInterface::GeographicMapCompositeInterface;
use super::BasicGeographicMap::BasicGeographicMap;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ColorFromEventUtil {
    pub YELLOW: Option<Rc<RefCell<BasicColor>>>,
    pub COLOR_INT: i32,
}

pub static ColorFromEventUtil_instance: crate::JavaStatic<Option<Rc<RefCell<ColorFromEventUtil>>>> = crate::JavaStatic::new(|| ColorFromEventUtil::new());

impl ColorFromEventUtil {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }

    pub fn new_struct() -> Self {
        let mut this = Self {
            YELLOW: None,
            COLOR_INT: 0,
        };
        this.YELLOW = { let java_field = jcall!(BasicColorFactory::getInstance()).YELLOW.clone(); java_field }.clone();
        this.COLOR_INT = jcall!(this.YELLOW).intValue();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<ColorFromEventUtil>>> {
        return ColorFromEventUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn getForegroundColor(&mut self, mut gameCanvasInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) -> Option<Rc<RefCell<BasicColor>>> {
        let mut color: Option<Rc<RefCell<BasicColor>>> = self.YELLOW.clone();
        let mut geographicMapCompositeInterface: Option<Rc<RefCell<dyn GeographicMapCompositeInterface>>> = (|| -> Option<Rc<RefCell<dyn GeographicMapCompositeInterface>>> { panic!("ClassCastException: a Rust dyn AllBinaryGameLayerManager cannot be cast to GeographicMapCompositeInterface") })();
        let mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>> = jcall!(geographicMapCompositeInterface).getGeographicMapInterface()[(0) as usize].clone();
        if geographicMapInterface.is_some() {
            let mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>> = jcall!(geographicMapInterface).getForegroundBasicColor();
            color = foregroundBasicColor.clone();
        }
        return color.clone();
    }
}

