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
// package org.allbinary.canvas
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::configuration::feature::Features::Features;
use crate::org::allbinary::graphics::opengles::OpenGLFeatureFactory::OpenGLFeatureFactory;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::system::Memory::Memory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SpecialMessageUtil {
}

thread_local! { pub static SpecialMessageUtil_SINGLETON: Option<Rc<RefCell<SpecialMessageUtil>>> = SpecialMessageUtil::new(); }

impl SpecialMessageUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<SpecialMessageUtil>>> {
        return SpecialMessageUtil_SINGLETON.with(Clone::clone).clone();
    }
    

    pub fn get(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(Memory::getInfo());
        jcall!(stringBuffer).append(jcall!(CommonSeps::getInstance()).SPACE.clone());
        jcall!(stringBuffer).append(jcall!(GameStatisticsFactory::GameStatisticsFactory::getInstance()).toString());
        jcall!(stringBuffer).append(jcall!(CommonSeps::getInstance()).SPACE.clone());
        jcall!(stringBuffer).append(String::from(" Option: "));
        if jcall!(Features::getInstance()).isDefault(jcall!(OpenGLFeatureFactory::getInstance()).OPENGL_AUTO_SELECT.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
            jcall!(stringBuffer).append(jcall!(jcall!(OpenGLFeatureFactory::getInstance()).OPENGL_AUTO_SELECT).getName());
        } else {
            jcall!(stringBuffer).append(jcall!(jcall!(OpenGLFeatureFactory::getInstance()).OPENGL_MINIMUM).getName());
        }
        return jcall!(stringBuffer).toString();
    }
}

