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
// package org.allbinary.media.image.cache
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use super::BufferedImageInfo::BufferedImageInfo;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BufferedImageInfoFactory {
}

pub static mut BufferedImageInfoFactory_hashMap: std::cell::LazyCell<Option<Rc<RefCell<HashMap>>>> = std::cell::LazyCell::new(|| jcall!(StdUtil::getInstance()).createHashMap());

impl BufferedImageInfoFactory {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance(mut width: i32, mut height: i32, mut r#type: i32) -> Option<Rc<RefCell<BufferedImageInfo>>> {
        let mut stringbuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringbuffer).appendint(width.clone());
        jcall!(stringbuffer).appendint(height.clone());
        jcall!(stringbuffer).appendint(r#type.clone());
        let mut key: String = jcall!(stringbuffer).toString();
        let mut bufferedImageInfo: Option<Rc<RefCell<BufferedImageInfo>>> = (jcall!(unsafe { BufferedImageInfoFactory_hashMap.clone() }).get(key.clone())).clone() /* as BufferedImageInfo */;
        if bufferedImageInfo.is_none() {
            bufferedImageInfo = BufferedImageInfo::new(width.clone(), height.clone(), r#type.clone());
            jcall!(unsafe { BufferedImageInfoFactory_hashMap.clone() }).put(key.clone(), bufferedImageInfo.clone());
        }
        return bufferedImageInfo.clone();
    }
}

