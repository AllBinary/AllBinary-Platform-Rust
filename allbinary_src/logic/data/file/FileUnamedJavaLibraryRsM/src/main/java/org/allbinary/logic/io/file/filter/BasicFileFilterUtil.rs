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
// package org.allbinary.logic.io.file.filter
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::io::FileFilter::FileFilter;
use crate::org::allbinary::logic::io::file::visitor::IncludeFileExtensionsBooleanFileVisitor::IncludeFileExtensionsBooleanFileVisitor;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use super::VisitorFileFilter::VisitorFileFilter;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicFileFilterUtil {
}


impl BasicFileFilterUtil {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance(mut extension: String) -> Option<Rc<RefCell<FileFilter>>> {
        let mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        jcall!(vector).add(crate::java_into_object(extension.clone()));
        let mut includeFileExtensionsBooleanFileVisitor: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::file::visitor::IncludeFileExtensionsBooleanFileVisitor::IncludeFileExtensionsBooleanFileVisitorDyn>>> = IncludeFileExtensionsBooleanFileVisitor::new(vector.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::file::visitor::IncludeFileExtensionsBooleanFileVisitor::IncludeFileExtensionsBooleanFileVisitorDyn>>);
        return (VisitorFileFilter::new(includeFileExtensionsBooleanFileVisitor.clone())).clone() /* as FileFilter */;
    }
}

