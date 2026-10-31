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
// package org.allbinary.media.image.comparison.motion
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use crate::Vector;
use super::*;
// use crate::java::awt::Rectangle::Rectangle;
// use crate::java::util::Vector::Vector;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use super::Cloneable::Cloneable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MotionRectangle {
    pub rectangle: Option<Rc<RefCell<Rectangle>>>,
    pub pixelDeltaVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}

// impl Cloneable for MotionRectangle with the methods of the trait

impl MotionRectangle {
    

    pub fn new_struct(mut rectangle: Option<Rc<RefCell<Rectangle>>>) -> Self {
        let mut this = Self {
            rectangle: None,
            pixelDeltaVector: None,
        };
        this.setRectangle(rectangle.clone());
        this.setPixelDeltaVector(BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayListD::BasicArrayListDDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>));
        this
    }

    pub fn new(mut rectangle: Option<Rc<RefCell<Rectangle>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(rectangle))))
    }
    

    pub fn getRectangle(&mut self) -> Option<Rc<RefCell<Rectangle>>> {
        return self.rectangle.clone();
    }
    

    pub fn setRectangle(&mut self, mut rectangle: Option<Rc<RefCell<Rectangle>>>) {
        self.rectangle = rectangle.clone();
    }
    

    pub fn getPixelDeltaVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.pixelDeltaVector.clone();
    }
    

    pub fn setPixelDeltaVector(&mut self, mut pixelDeltaVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        self.pixelDeltaVector = pixelDeltaVector.clone();
    }
    

    pub fn clone(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        let mut motionRectangle: Option<Rc<RefCell<MotionRectangle>>> = MotionRectangle::new((jcall!(self.getRectangle()).clone()).clone() /* as Rectangle */);
        jcall!(motionRectangle).setPixelDeltaVector((jcall!(self.getPixelDeltaVector()).clone()).clone() /* as BasicArrayList */);
        return (|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(motionRectangle.clone());
    }
}

