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
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::util::cache::CacheableInterface::CacheableInterface;
use super::ImageComparisonResult::ImageComparisonResult;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageComparisonResultFrameCacheable {
    pub frame: Option<i64>,
    pub imageComparisonResult: Option<Rc<RefCell<ImageComparisonResult>>>,
}

impl CacheableInterface for ImageComparisonResultFrameCacheable {
    fn getKey(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { Self::getKey(self) }
}

impl ImageComparisonResultFrameCacheable {
    

    pub fn new_struct(mut imageComparisonResult: Option<Rc<RefCell<ImageComparisonResult>>>, mut frame: Option<i64>) -> Self {
        let mut this = Self {
            frame: None,
            imageComparisonResult: None,
        };
        this.setImageComparisonResult(imageComparisonResult.clone());
        this.frame = frame.clone();
        this
    }

    pub fn new(mut imageComparisonResult: Option<Rc<RefCell<ImageComparisonResult>>>, mut frame: Option<i64>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(imageComparisonResult, frame))))
    }
    

    pub fn getKey(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return self.frame.clone();
    }
    

    pub fn getFrame(&mut self) -> Option<i64> {
        return self.frame.clone();
    }
    

    pub fn setFrame(&mut self, mut frame: Option<i64>) {
        self.frame = frame.clone();
    }
    

    pub fn getImageComparisonResult(&mut self) -> Option<Rc<RefCell<ImageComparisonResult>>> {
        return self.imageComparisonResult.clone();
    }
    

    pub fn setImageComparisonResult(&mut self, mut imageComparisonResult: Option<Rc<RefCell<ImageComparisonResult>>>) {
        self.imageComparisonResult = imageComparisonResult.clone();
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(std::any::type_name_of_val(&self).to_string());
        jcall!(stringBuffer).append(String::from(" Frame: "));
        jcall!(stringBuffer).appendlong(self.getFrame());
        return jcall!(stringBuffer).toString();
    }
}

