// 
// AllBinary Open License Version 1 
// Copyright (c) 2025 AllBinary 
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
// package org.allbinary.logic.io.file
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::string::CommonSeps::CommonSeps;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct FileUnamedUtil {
    pub commonSeps: Option<Rc<RefCell<CommonSeps>>>,
}

thread_local! { pub static FileUnamedUtil_instance: Option<Rc<RefCell<FileUnamedUtil>>> = FileUnamedUtil::new(); }

impl FileUnamedUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            commonSeps: None,
        };
        this.commonSeps = CommonSeps::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<FileUnamedUtil>>> {
        return FileUnamedUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn process(&mut self, mut string: String) -> String {
        let mut simplifiedString: String = string.to_lowercase();
        simplifiedString = simplifiedString.replace(&jcall!(self.commonSeps).SPACE, &jcall!(self.commonSeps).UNDERSCORE);
        simplifiedString = simplifiedString.replace(&jcall!(self.commonSeps).DASH, &jcall!(self.commonSeps).UNDERSCORE);
        return simplifiedString.clone();
    }
}

