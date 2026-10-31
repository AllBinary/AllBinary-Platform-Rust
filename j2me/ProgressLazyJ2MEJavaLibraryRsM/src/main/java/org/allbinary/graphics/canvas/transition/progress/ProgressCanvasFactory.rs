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
// package org.allbinary.graphics.canvas.transition.progress
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::graphics::paint::PaintableInterface::PaintableInterface;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use super::LazyProgressCanvas::LazyProgressCanvas;
use super::ProgressCanvas::ProgressCanvas;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ProgressCanvasFactory {
}

pub static mut ProgressCanvasFactory_PROGRESS_FORM_SCREEN: std::cell::LazyCell<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = std::cell::LazyCell::new(|| jcall!(NullUtil::getInstance()).NULL_OBJECT.clone());

impl ProgressCanvasFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvasDyn>>> {
        if match (&unsafe { ProgressCanvasFactory_PROGRESS_FORM_SCREEN.clone() }, &jcall!(NullUtil::getInstance()).NULL_OBJECT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            unsafe { *ProgressCanvasFactory_PROGRESS_FORM_SCREEN = LazyProgressCanvas::new(jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), jcall!(BasicColorFactory::getInstance()).BLACK.clone(), jcall!(BasicColorFactory::getInstance()).WHITE.clone()) };
        }
        return (unsafe { ProgressCanvasFactory_PROGRESS_FORM_SCREEN.clone() }).clone() /* as ProgressCanvas */;
    }
    

    pub fn getLazyInstance() -> Option<Rc<RefCell<dyn PaintableInterface>>> {
        return ProgressCanvasFactory::getInstance().map(|reference| reference as Rc<RefCell<dyn PaintableInterface>>);
    }
}

