// 
// AllBinary Open License Version 1 
// Copyright (c) 2022 AllBinary 
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
use crate::javax::microedition::lcdui::Image::Image;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct OpenGLESImageExclusionUtil {
    pub EXCLUSION: String,
    pub list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}

thread_local! { pub static OpenGLESImageExclusionUtil_instance: Option<Rc<RefCell<OpenGLESImageExclusionUtil>>> = OpenGLESImageExclusionUtil::new(); }

impl OpenGLESImageExclusionUtil {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<OpenGLESImageExclusionUtil>>> {
        return OpenGLESImageExclusionUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            EXCLUSION: String::new(),
            list: None,
        };
        this.EXCLUSION = String::from("font");
        this.list = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn isCustomScaling(&mut self, mut image: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>) -> bool {
        let mut imageName: String = jcall!(image).getName();
        let mut isNormalScaling: bool = jcall!(self.list).size() == 0 || imageName.starts_with(&self.EXCLUSION.clone());
        if isNormalScaling {
            return false;
        } else {
            let mut size: i32 = jcall!(self.list).size();
            let mut name: String = String::new();
            for index in (0..size) {
                name = { let java_value = ({ let java_object = (jcall!(self.list).get(index.clone())).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
                if imageName.starts_with(&name.clone()) {
                    isNormalScaling = true;
                }
            }
        }
        if isNormalScaling {
            return false;
        } else {
            return true;
        }
    }
}

