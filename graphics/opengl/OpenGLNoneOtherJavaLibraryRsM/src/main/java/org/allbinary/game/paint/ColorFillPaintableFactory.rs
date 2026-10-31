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
// package org.allbinary.game.paint
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystem;
use crate::org::allbinary::logic::system::os::OperatingSystemFactory::OperatingSystemFactory;
use super::ColorFillPaintable::ColorFillPaintable;
use super::OverScanColorFillPaintable::OverScanColorFillPaintable;
use super::ColorFillBasePaintable::ColorFillBasePaintable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ColorFillPaintableFactory {
}

thread_local! { pub static ColorFillPaintableFactory_instance: Option<Rc<RefCell<ColorFillPaintableFactory>>> = ColorFillPaintableFactory::new(); }

impl ColorFillPaintableFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<ColorFillPaintableFactory>>> {
        return ColorFillPaintableFactory_instance.with(Clone::clone).clone();
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn getInstance2(&mut self, mut basicColor: Option<Rc<RefCell<BasicColor>>>, mut forThreedCanvas: bool) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::paint::ColorFillBasePaintable::ColorFillBasePaintableDyn>>> {
        let mut operatingSystem: Option<Rc<RefCell<dyn crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystemDyn>>> = jcall!(OperatingSystemFactory::getInstance()).getOperatingSystemInstance();
        if jcall!(operatingSystem).isOverScan() {
            return OverScanColorFillPaintable::new(basicColor.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::paint::ColorFillBasePaintable::ColorFillBasePaintableDyn>>);
        } else {
            return ColorFillPaintable::new(basicColor.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::paint::ColorFillBasePaintable::ColorFillBasePaintableDyn>>);
        }
    }
}

