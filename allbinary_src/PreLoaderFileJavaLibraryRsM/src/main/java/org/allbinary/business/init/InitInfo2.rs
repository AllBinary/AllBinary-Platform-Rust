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
// package org.allbinary.business.init
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::io::file::directory::Directory::Directory;
use crate::org::allbinary::logic::io::path::AbPath::AbPath;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InitInfo2 {
    pub directory: Option<Rc<RefCell<Directory>>>,
}

thread_local! { pub static InitInfo2_instance: Option<Rc<RefCell<InitInfo2>>> = InitInfo2::new(); }

impl InitInfo2 {

    pub fn new_struct() -> Self {
        let mut this = Self {
            directory: None,
        };
        this.directory = Directory::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<InitInfo2>>> {
        return InitInfo2_instance.with(Clone::clone).clone();
    }
    

    pub fn isTestHtmlPathValid(&mut self, mut value: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>) -> bool {
        return jcall!(self.directory).create(value.clone());
    }
    

    pub fn isMainPathValid(&mut self, mut value: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>) -> bool {
        return jcall!(self.directory).create(value.clone());
    }
}

