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
// package org.allbinary.media.image.filter
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::io::File::File;
// use crate::javax::swing::filechooser::FileFilter::FileFilter;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//TWB - replace with file visitor version
pub struct ImageFileFilter {
    pub base: FileFilter,
}


impl ImageFileFilter {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: FileFilter::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn accept(&mut self, mut f: Option<Rc<RefCell<File>>>) -> bool {
        if jcall!(f).isDirectory() {
            return true;
        }
        let mut s: String = jcall!(f).getName();
        let mut i: i32 = s.rfind('.').map(|index| index as i32).unwrap_or(-1);
        if i > 0 && i < (s.chars().count() as i32) - 1 {
            let mut ext: String = s[(i + 1) as usize..].to_string().to_lowercase();
            if (ext == String::from("jpg")) || (ext == String::from("jpeg")) {
                return true;
            }
        }
        return false;
    }
    

    pub fn getDescription(&mut self) -> String {
        return String::from("Accepts .jpg and .jpeg files");
    }
}
impl std::ops::Deref for ImageFileFilter {
    type Target = FileFilter;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ImageFileFilter {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



