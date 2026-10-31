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
// package org.allbinary.servlet
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::servlet::http::HttpServletRequest::HttpServletRequest;
use crate::org::allbinary::globals::AppUrlGlobals::AppUrlGlobals;
use crate::org::allbinary::globals::URLGLOBALS::URLGLOBALS;
use crate::org::allbinary::logic::io::path::AbPathData::AbPathData;
use crate::org::allbinary::logic::system::loader::WebappClassLoaderInfo::WebappClassLoaderInfo;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BlisketServletUtil {
}

thread_local! { pub static BlisketServletUtil_instance: Option<Rc<RefCell<BlisketServletUtil>>> = BlisketServletUtil::new(); }

impl BlisketServletUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<BlisketServletUtil>>> {
        return BlisketServletUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn init(&mut self, mut request: Option<Rc<RefCell<HttpServletRequest>>>) {
        let mut urlGlobals: Option<Rc<RefCell<AppUrlGlobals>>> = AppUrlGlobals::new();
        jcall!(urlGlobals).setWebappPath(jcall!(jcall!(request).getServletContext()).getRealPath(jcall!(AbPathData::getInstance()).SEPARATOR.clone()));
        URLGLOBALS::init(urlGlobals.clone());
        WebappClassLoaderInfo::setLoader(jcall!(self.getClass()).getClassLoader());
    }
}

