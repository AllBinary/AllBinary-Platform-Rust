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
// package org.allbinary.logic.communication.http.request
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::io::path::AbPathData::AbPathData;
use crate::java::lang::Exception::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct HttpRequestUtil {
}

pub static HttpRequestUtil_instance: crate::JavaStatic<Option<Rc<RefCell<HttpRequestUtil>>>> = crate::JavaStatic::new(|| HttpRequestUtil::new());

impl HttpRequestUtil {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<HttpRequestUtil>>> {
        return HttpRequestUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getLastSeparatorIndex(&mut self, mut requestPath: String) -> i32 {
        let mut beginIndex: i32 = requestPath.rfind(&{ let java_field = jcall!(AbPathData::getInstance()).SEPARATOR.clone(); java_field }.clone()).map(|index| index as i32).unwrap_or(-1);
        if beginIndex < 0 {
            beginIndex = { let java_value = (requestPath.rfind(&String::from("\\")).map(|index| index as i32).unwrap_or(-1)).clone(); java_value };
        }
        return beginIndex.clone();
    }
    

    // throws Exception
    pub fn generateFileName(&mut self, mut requestPath: String) -> String {
        let mut beginIndex: i32 = self.getLastSeparatorIndex(requestPath.clone());
        if beginIndex < 0 {
            return requestPath.clone();
        } else {
            return requestPath[(beginIndex + 1) as usize..].to_string();
        }
    }
}

