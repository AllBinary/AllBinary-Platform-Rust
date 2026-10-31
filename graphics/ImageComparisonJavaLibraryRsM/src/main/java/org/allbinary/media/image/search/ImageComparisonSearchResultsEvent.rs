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
// package org.allbinary.media.image.search
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject;
use super::ImageComparisonSearch::ImageComparisonSearch;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageComparisonSearchResultsEvent {
    pub base: AllBinaryEventObject,
    pub imageComparisonSearch: Option<Rc<RefCell<ImageComparisonSearch>>>,
}


impl ImageComparisonSearchResultsEvent {
    

    pub fn new_struct(mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut imageComparisonSearch: Option<Rc<RefCell<ImageComparisonSearch>>>) -> Self {
        let mut this = Self {
            base: AllBinaryEventObject::new_struct(anyType.clone()),
            imageComparisonSearch: None,
        };
        // super(...) is in the struct literal above
        this.setImageComparisonSearch(imageComparisonSearch.clone());
        this
    }

    pub fn new(mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut imageComparisonSearch: Option<Rc<RefCell<ImageComparisonSearch>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(anyType, imageComparisonSearch))))
    }
    

    pub fn getImageComparisonSearch(&mut self) -> Option<Rc<RefCell<ImageComparisonSearch>>> {
        return self.imageComparisonSearch.clone();
    }
    

    pub fn setImageComparisonSearch(&mut self, mut imageComparisonSearch: Option<Rc<RefCell<ImageComparisonSearch>>>) {
        self.imageComparisonSearch = imageComparisonSearch.clone();
    }
}
impl std::ops::Deref for ImageComparisonSearchResultsEvent {
    type Target = AllBinaryEventObject;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ImageComparisonSearchResultsEvent {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn for ImageComparisonSearchResultsEvent {
    fn as_AllBinaryEventObject(&self) -> &crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject { &self.base }
    fn as_AllBinaryEventObject_mut(&mut self) -> &mut crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject { &mut self.base }
}


