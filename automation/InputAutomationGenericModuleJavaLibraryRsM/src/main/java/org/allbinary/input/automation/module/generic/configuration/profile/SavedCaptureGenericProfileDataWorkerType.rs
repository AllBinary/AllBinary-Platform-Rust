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
// package org.allbinary.input.automation.module.generic.configuration.profile
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::input::media::image::InputImageType::InputImageType;
use super::GenericProfileDataWorkerType::GenericProfileDataWorkerType;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SavedCaptureGenericProfileDataWorkerType {
    pub base: GenericProfileDataWorkerType,
    pub path: String,
    pub lastFrame: i32,
}

pub static mut SavedCaptureGenericProfileDataWorkerType_SAVED_CAPTURE: std::cell::LazyCell<Option<Rc<RefCell<SavedCaptureGenericProfileDataWorkerType>>>> = std::cell::LazyCell::new(|| SavedCaptureGenericProfileDataWorkerType::new(format!("{}{}", String::from("Saved "), jcall!(unsafe { crate::org::allbinary::input::media::image::InputImageType::InputImageType_CAPTURE.clone() }).getName()), String::from("c:\\share\\captures\\2\\"), 100));

impl SavedCaptureGenericProfileDataWorkerType {
    

    pub fn new_struct(mut name: String, mut path: String, mut lastFrame: i32) -> Self {
        let mut this = Self {
            base: GenericProfileDataWorkerType::new_struct(name.clone()),
            path: String::new(),
            lastFrame: 0,
        };
        // super(...) is in the struct literal above
        this.setPath(path.clone());
        this.setLastFrame(lastFrame.clone());
        this
    }

    pub fn new(mut name: String, mut path: String, mut lastFrame: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(name, path, lastFrame))))
    }
    

    pub fn getPath(&mut self) -> String {
        return self.path.clone();
    }
    

    pub fn setPath(&mut self, mut path: String) {
        self.path = path.clone();
    }
    

    pub fn getLastFrame(&mut self) -> i32 {
        return self.lastFrame.clone();
    }
    

    pub fn setLastFrame(&mut self, mut lastFrame: i32) {
        self.lastFrame = lastFrame.clone();
    }
}
impl std::ops::Deref for SavedCaptureGenericProfileDataWorkerType {
    type Target = GenericProfileDataWorkerType;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for SavedCaptureGenericProfileDataWorkerType {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::input::automation::module::generic::configuration::profile::GenericProfileDataWorkerType::GenericProfileDataWorkerTypeDyn for SavedCaptureGenericProfileDataWorkerType {
    fn as_GenericProfileDataWorkerType(&self) -> &crate::org::allbinary::input::automation::module::generic::configuration::profile::GenericProfileDataWorkerType::GenericProfileDataWorkerType { &self.base }
    fn as_GenericProfileDataWorkerType_mut(&mut self) -> &mut crate::org::allbinary::input::automation::module::generic::configuration::profile::GenericProfileDataWorkerType::GenericProfileDataWorkerType { &mut self.base }
}


