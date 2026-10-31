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
// package org.allbinary.logic.io.path
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use super::AbPath::AbPath;
use super::AbPathData::AbPathData;
use super::PathUtil::PathUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AbFilePath {
    pub base: AbPath,
}


impl AbFilePath {
    

    pub fn new_struct(mut aPath: String) -> Self {
        let mut this = Self {
            base: AbPath::new_struct(jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone()),
        };
        // super(...) is in the struct literal above
        let mut pathData: Option<Rc<RefCell<AbPathData>>> = AbPathData::getInstance();
        let mut abPathUtil: Option<Rc<RefCell<PathUtil>>> = PathUtil::getInstance();
        this.schema = this.getSchema(aPath.clone());
        { let java_arguments = (jcall!(pathData).getNameFromPath(aPath.clone()),); this.setName(java_arguments.0) };
        { let java_arguments = (jcall!(abPathUtil).adjust({ let java_arguments = (jcall!(jcall!(abPathUtil).removeNameFromPath(aPath.clone())).toString(),); this.getPathFromPath(java_arguments.0) }),); this.setPath(java_arguments.0) };
        this
    }

    pub fn new(mut aPath: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(aPath))))
    }
}
impl std::ops::Deref for AbFilePath {
    type Target = AbPath;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AbFilePath {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::io::path::AbPath::AbPathDyn for AbFilePath {
    fn as_AbPath(&self) -> &crate::org::allbinary::logic::io::path::AbPath::AbPath { &self.base }
    fn as_AbPath_mut(&mut self) -> &mut crate::org::allbinary::logic::io::path::AbPath::AbPath { &mut self.base }
}


