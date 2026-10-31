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
// package org.allbinary.media.image.comparison
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject;
use super::ImageComparisonResult::ImageComparisonResult;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageComparisonResultsEvent {
    pub base: AllBinaryEventObject,
    pub imageComparisonResult: Option<Rc<RefCell<ImageComparisonResult>>>,
}


impl ImageComparisonResultsEvent {
    

    pub fn new_struct(mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut imageComparisonInfo: Option<Rc<RefCell<ImageComparisonResult>>>) -> Self {
        let mut this = Self {
            base: AllBinaryEventObject::new_struct(anyType.clone()),
            imageComparisonResult: None,
        };
        // super(...) is in the struct literal above
        this.setImageComparisonResult(imageComparisonInfo.clone());
        this
    }

    pub fn new(mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut imageComparisonInfo: Option<Rc<RefCell<ImageComparisonResult>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(anyType, imageComparisonInfo))))
    }
    

    pub fn getImageComparisonResult(&mut self) -> Option<Rc<RefCell<ImageComparisonResult>>> {
        return self.imageComparisonResult.clone();
    }
    

    pub fn setImageComparisonResult(&mut self, mut imageComparisonResult: Option<Rc<RefCell<ImageComparisonResult>>>) {
        self.imageComparisonResult = imageComparisonResult.clone();
    }
}
impl std::ops::Deref for ImageComparisonResultsEvent {
    type Target = AllBinaryEventObject;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ImageComparisonResultsEvent {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn for ImageComparisonResultsEvent {
    fn as_AllBinaryEventObject(&self) -> &crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject { &self.base }
    fn as_AllBinaryEventObject_mut(&mut self) -> &mut crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject { &mut self.base }
}


