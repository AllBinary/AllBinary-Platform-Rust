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
// package org.allbinary.input.motion.button
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::input::mapping::InputMappingInterface::InputMappingInterface;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::paint::ProcessPaintable::ProcessPaintable;
use super::TouchButtonsPaintableComposite::TouchButtonsPaintableComposite;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TouchButtonsPaintableCompositeFactory {
}


impl TouchButtonsPaintableCompositeFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance(mut inputMappingInterface: Option<Rc<RefCell<dyn InputMappingInterface>>>, mut basicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::ProcessPaintable::ProcessPaintableDyn>>> {
        return TouchButtonsPaintableComposite::new(inputMappingInterface.clone(), basicColor.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::paint::ProcessPaintable::ProcessPaintableDyn>>);
    }
}

