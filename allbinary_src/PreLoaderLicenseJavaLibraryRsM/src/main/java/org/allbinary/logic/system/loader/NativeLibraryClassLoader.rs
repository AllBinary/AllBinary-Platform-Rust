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
// package org.allbinary.logic.system.loader
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::ClassLoader::ClassLoader;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NativeLibraryClassLoader {
    pub base: ClassLoader,
    pub message: String,
}


impl NativeLibraryClassLoader {
    

    pub fn new_struct(mut parent: Option<Rc<RefCell<ClassLoader>>>) -> Self {
        let mut this = Self {
            base: ClassLoader::new_struct1(parent.clone()),
            message: String::new(),
        };
        this.message = String::from("This Class Loader Does Not Load Classes - Only For Loading and Unloading of Native Libraries");
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut parent: Option<Rc<RefCell<ClassLoader>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(parent))))
    }
}
impl std::ops::Deref for NativeLibraryClassLoader {
    type Target = ClassLoader;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for NativeLibraryClassLoader {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



