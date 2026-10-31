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
// package org.allbinary.graphics.displayable.screen
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::graphics::paint::NullPaintable::NullPaintable;
use crate::org::allbinary::graphics::paint::Paintable::Paintable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AboutPaintableFactory {
    pub paintableArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>>,
}

thread_local! { pub static AboutPaintableFactory_instance: Option<Rc<RefCell<AboutPaintableFactory>>> = AboutPaintableFactory::new(); }

impl AboutPaintableFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            paintableArray: Vec::new(),
        };
        this.paintableArray = vec![NullPaintable::getInstance()];
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<AboutPaintableFactory>>> {
        return AboutPaintableFactory_instance.with(Clone::clone).clone();
    }
}

