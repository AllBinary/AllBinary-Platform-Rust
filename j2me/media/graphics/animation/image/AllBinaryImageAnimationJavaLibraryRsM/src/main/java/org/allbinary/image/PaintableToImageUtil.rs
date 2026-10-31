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
// package org.allbinary.image
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
// use crate::javax::microedition::lcdui::Image::Image;
use crate::org::allbinary::graphics::paint::PaintableInterface::PaintableInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PaintableToImageUtil {
}

thread_local! { pub static PaintableToImageUtil_instance: Option<Rc<RefCell<PaintableToImageUtil>>> = PaintableToImageUtil::new(); }

impl PaintableToImageUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn getImage(mut paintableInterface: Option<Rc<RefCell<dyn PaintableInterface>>>, mut width: i32, mut height: i32) -> Option<Rc<RefCell<Image>>> {
        let mut image: Option<Rc<RefCell<Image>>> = jcall!(GameFeatureImageCacheFactory::GameFeatureImageCacheFactory::getInstance()).get(std::any::type_name_of_val(&PaintableToImageUtil_instance.with(Clone::clone)).to_string(), width.clone(), height.clone());
        let mut graphics: Option<Rc<RefCell<Graphics>>> = jcall!(image).getGraphics();
        jcall!(paintableInterface).paint(graphics.clone());
        return image.clone();
    }
}

