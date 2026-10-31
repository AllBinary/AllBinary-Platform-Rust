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
// package org.allbinary.graphics.opengles
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::configuration::feature::Feature::Feature;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct OpenGLFeature {
    pub base: Feature,
}


impl OpenGLFeature {
    

    pub fn new_struct(mut name: String) -> Self {
        let mut this = Self {
            base: Feature::new_struct(name.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut name: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(name))))
    }
}
impl std::ops::Deref for OpenGLFeature {
    type Target = Feature;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for OpenGLFeature {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn for OpenGLFeature {
    fn as_Feature(&self) -> &crate::org::allbinary::game::configuration::feature::Feature::Feature { &self.base }
    fn as_Feature_mut(&mut self) -> &mut crate::org::allbinary::game::configuration::feature::Feature::Feature { &mut self.base }
}


