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
// package org.allbinary.graphics
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

pub struct ScreenCapture {
}


impl ScreenCapture {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn saveFrame(&mut self) {
    }
    

    pub fn startRecording(&mut self) {
    }
    

    pub fn endRecording(&mut self) {
    }
}
pub trait ScreenCaptureDyn {
    fn as_ScreenCapture(&self) -> &ScreenCapture;
    fn as_ScreenCapture_mut(&mut self) -> &mut ScreenCapture;
    fn saveFrame(&mut self) { ScreenCapture::saveFrame(self.as_ScreenCapture_mut()) }
    fn startRecording(&mut self) { ScreenCapture::startRecording(self.as_ScreenCapture_mut()) }
    fn endRecording(&mut self) { ScreenCapture::endRecording(self.as_ScreenCapture_mut()) }
}
impl ScreenCaptureDyn for ScreenCapture {
    fn as_ScreenCapture(&self) -> &ScreenCapture { self }
    fn as_ScreenCapture_mut(&mut self) -> &mut ScreenCapture { self }
}


