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
// package org.allbinary.logic.visual.media
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MediaTypeData {
    pub RESIZABLE_MEDIA: String,
    pub NOT_RESIZABLE_MEDIA: String,
}

thread_local! { pub static MediaTypeData_instance: Option<Rc<RefCell<MediaTypeData>>> = MediaTypeData::new(); }

impl MediaTypeData {
    

    pub fn getInstance() -> Option<Rc<RefCell<MediaTypeData>>> {
        return MediaTypeData_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            RESIZABLE_MEDIA: String::new(),
            NOT_RESIZABLE_MEDIA: String::new(),
        };
        this.RESIZABLE_MEDIA = String::from("RESIZABLE_MEDIA");
        this.NOT_RESIZABLE_MEDIA = String::from("NOT_RESIZABLE_MEDIA");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

