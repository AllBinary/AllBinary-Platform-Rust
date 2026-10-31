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
use crate::org::allbinary::game::configuration::feature::Features::Features;
use crate::org::allbinary::game::configuration::feature::GameFeatureChoiceGroups::GameFeatureChoiceGroups;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use super::OpenGLFeatureFactory::OpenGLFeatureFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct OpenGLOptions {
}


impl OpenGLOptions {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn init(&mut self) {
        let mut openGLFeatureFactory: Option<Rc<RefCell<OpenGLFeatureFactory>>> = OpenGLFeatureFactory::getInstance();
        if jcall!(Features::getInstance()).isFeature(jcall!(openGLFeatureFactory).OPENGL_OPTIONS.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
            let mut openGLMultipleList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            jcall!(openGLMultipleList).add(crate::java_into_object(jcall!(openGLFeatureFactory).OPENGL.clone()));
            jcall!(GameFeatureChoiceGroups::getMultipleInstance()).add(String::from("OpenGL (Complete Restart)"), openGLMultipleList.clone());
            let mut openGLVersionSelectorMultipleList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            jcall!(openGLVersionSelectorMultipleList).add(crate::java_into_object(jcall!(openGLFeatureFactory).OPENGL_AUTO_SELECT.clone()));
            jcall!(openGLVersionSelectorMultipleList).add(crate::java_into_object(jcall!(openGLFeatureFactory).OPENGL_MINIMUM.clone()));
            jcall!(GameFeatureChoiceGroups::getExclusiveInstance()).add(String::from("OpenGL Version Selector"), openGLVersionSelectorMultipleList.clone());
            let mut openGLImageColorMultipleList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            jcall!(openGLImageColorMultipleList).add(crate::java_into_object(jcall!(openGLFeatureFactory).IMAGE_COLOR_DEPTH_4444.clone()));
            jcall!(openGLImageColorMultipleList).add(crate::java_into_object(jcall!(openGLFeatureFactory).IMAGE_COLOR_DEPTH_8888.clone()));
            jcall!(GameFeatureChoiceGroups::getExclusiveInstance()).add(String::from("OpenGL Image Color RGBA"), openGLImageColorMultipleList.clone());
        }
    }
}

