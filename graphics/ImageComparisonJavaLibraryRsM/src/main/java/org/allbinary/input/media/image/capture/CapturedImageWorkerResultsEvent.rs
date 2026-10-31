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
// package org.allbinary.input.media.image.capture
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::awt::image::BufferedImage::BufferedImage;
use crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CapturedImageWorkerResultsEvent {
    pub base: AllBinaryEventObject,
    pub frame: Option<i64>,
    pub bufferedImage: Option<Rc<RefCell<BufferedImage>>>,
}


impl CapturedImageWorkerResultsEvent {
    

    pub fn new_struct(mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut frame: Option<i64>, mut bufferedImage: Option<Rc<RefCell<BufferedImage>>>) -> Self {
        let mut this = Self {
            base: AllBinaryEventObject::new_struct(anyType.clone()),
            frame: None,
            bufferedImage: None,
        };
        // super(...) is in the struct literal above
        this.frame = frame.clone();
        this.setBufferedImage(bufferedImage.clone());
        this
    }

    pub fn new(mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut frame: Option<i64>, mut bufferedImage: Option<Rc<RefCell<BufferedImage>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(anyType, frame, bufferedImage))))
    }
    

    pub fn getBufferedImage(&mut self) -> Option<Rc<RefCell<BufferedImage>>> {
        return self.bufferedImage.clone();
    }
    

    pub fn setBufferedImage(&mut self, mut bufferedImage: Option<Rc<RefCell<BufferedImage>>>) {
        self.bufferedImage = bufferedImage.clone();
    }
    

    pub fn getFrame(&mut self) -> Option<i64> {
        return self.frame.clone();
    }
}
impl std::ops::Deref for CapturedImageWorkerResultsEvent {
    type Target = AllBinaryEventObject;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for CapturedImageWorkerResultsEvent {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn for CapturedImageWorkerResultsEvent {
    fn as_AllBinaryEventObject(&self) -> &crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject { &self.base }
    fn as_AllBinaryEventObject_mut(&mut self) -> &mut crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject { &mut self.base }
}


