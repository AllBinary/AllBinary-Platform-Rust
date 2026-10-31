// 
// AllBinary Open License Version 1 
// Copyright (c) 2002 AllBinary 
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
// package org.allbinary.logic.io.file.directory
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::io::File::File;
// use crate::java::io::FileFilter::FileFilter;
use crate::org::allbinary::logic::util::visitor::VisitorInterface::VisitorInterface;
use super::io::io;
use super::java::java;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct DirectoryFileVisitor {
}

impl VisitorInterface for DirectoryFileVisitor {
    fn visit(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { Self::visit(self, anyType) }
}
// impl FileFilter for DirectoryFileVisitor with the methods of the trait

impl DirectoryFileVisitor {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn accept(&mut self, mut arg0: Option<Rc<RefCell<File>>>) -> bool {
        return ({ let java_object = self.visit((arg0).clone() /* as Any */); java_object.as_ref().map(|object| { let object = object.borrow(); object.downcast_ref::<bool>().copied().or_else(|| object.downcast_ref::<Option<bool>>().copied().flatten()).expect("ClassCastException") }) }).unwrap();
    }
    

    pub fn visit(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        let mut file: Option<Rc<RefCell<File>>> = (anyType).clone() /* as File */;
        let mut index: i32 = 1;
        let mut aFile: Option<Rc<RefCell<File>>> = jcall!(file).getParentFile();
        while ({ aFile = jcall!(aFile).getParentFile(); aFile.clone() }).is_some() {
            index += 1;
        }
        if jcall!(file).isDirectory() && index < 5 {
            return true;
        }
        return false;
    }
}

