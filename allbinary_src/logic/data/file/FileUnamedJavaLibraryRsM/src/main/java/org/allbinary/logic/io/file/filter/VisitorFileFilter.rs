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
// use crate::java::io::File::File;
// use crate::java::io::FileFilter::FileFilter;
use crate::org::allbinary::logic::io::file::AbFile::AbFile;
use crate::org::allbinary::logic::io::file::FileWrapperUtil::FileWrapperUtil;
use crate::org::allbinary::logic::util::visitor::VisitorInterface::VisitorInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct VisitorFileFilter {
    pub booleanVisitorInterface: Option<Rc<RefCell<dyn VisitorInterface>>>,
}

// impl FileFilter for VisitorFileFilter with the methods of the trait

impl VisitorFileFilter {
    

    pub fn new_struct(mut booleanVisitorInterface: Option<Rc<RefCell<dyn VisitorInterface>>>) -> Self {
        let mut this = Self {
            booleanVisitorInterface: None,
        };
        this.booleanVisitorInterface = booleanVisitorInterface.clone();
        this
    }

    pub fn new(mut booleanVisitorInterface: Option<Rc<RefCell<dyn VisitorInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(booleanVisitorInterface))))
    }
    

    pub fn accept(&mut self, mut file: Option<Rc<RefCell<File>>>) -> bool {
        let mut abFile: Option<Rc<RefCell<AbFile>>> = FileWrapperUtil::wrapFile(file.clone());
        let mut isFileAcceptable: bool = ({ let java_object = jcall!(self.booleanVisitorInterface).visit(crate::java_into_object(abFile.clone())); java_object.as_ref().map(|object| { let object = object.borrow(); object.downcast_ref::<bool>().copied().or_else(|| object.downcast_ref::<Option<bool>>().copied().flatten()).expect("ClassCastException") }) }).unwrap();
        return isFileAcceptable || jcall!(file).isDirectory();
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn accept(&mut self, mut file: Option<Rc<RefCell<AbFile>>>) -> bool {
        let mut isFileAcceptable: bool = ({ let java_object = jcall!(self.booleanVisitorInterface).visit(crate::java_into_object(file.clone())); java_object.as_ref().map(|object| { let object = object.borrow(); object.downcast_ref::<bool>().copied().or_else(|| object.downcast_ref::<Option<bool>>().copied().flatten()).expect("ClassCastException") }) }).unwrap();
        return isFileAcceptable || jcall!(file).isDirectory();
    }
}

