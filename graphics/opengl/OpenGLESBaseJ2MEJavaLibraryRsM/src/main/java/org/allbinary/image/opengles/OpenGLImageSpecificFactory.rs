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
// package org.allbinary.image.opengles
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use super::OpenGLImageFactory::OpenGLImageFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct OpenGLImageSpecificFactory {
    pub imageFactory: Option<Rc<RefCell<dyn crate::org::allbinary::image::opengles::OpenGLImageFactory::OpenGLImageFactoryDyn>>>,
    pub updating: bool,
}

thread_local! { pub static OpenGLImageSpecificFactory_instance: Option<Rc<RefCell<OpenGLImageSpecificFactory>>> = OpenGLImageSpecificFactory::new(); }

impl OpenGLImageSpecificFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            imageFactory: None,
            updating: false,
        };
        this.imageFactory = OpenGLImageFactory_NULL_OPENGL_IMAGE_FACTORY.with(Clone::clone).clone();
        this.updating = false;
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<OpenGLImageSpecificFactory>>> {
        return OpenGLImageSpecificFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn setImageFactory(&mut self, mut imageFactory: Option<Rc<RefCell<dyn crate::org::allbinary::image::opengles::OpenGLImageFactory::OpenGLImageFactoryDyn>>>) {
        self.imageFactory = imageFactory.clone();
        PreLogUtil::put(jcall!(StringMaker::new()).append(String::from("Selected Image Factory: ")).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(self.imageFactory.clone()))).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setImageFactory"));
    }
    

    pub fn getImageFactory(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::image::opengles::OpenGLImageFactory::OpenGLImageFactoryDyn>>> {
        return self.imageFactory.clone();
    }
}

