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
use crate::java::io::File::File;
use crate::java::io::FileFilter::FileFilter;
use crate::org::allbinary::logic::util::visitor::VisitorInterface::VisitorInterface;

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
impl FileFilter for DirectoryFileVisitor {
    fn accept(&mut self, mut arg0: Option<Rc<RefCell<File>>>) -> bool { Self::accept(self, arg0) }
}

impl DirectoryFileVisitor {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn accept(&mut self, mut arg0: Option<Rc<RefCell<File>>>) -> bool {
        return ({ let java_object = (self.visit(crate::java_into_object((arg0).clone()))).clone(); java_object.as_ref().map(|object| { let object = object.borrow(); object.downcast_ref::<bool>().copied().or_else(|| object.downcast_ref::<Option<bool>>().copied().flatten()).expect("ClassCastException") }) }).unwrap();
    }
    

    pub fn visit(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        let mut file: Option<Rc<RefCell<File>>> = { let java_object = (anyType).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<File>>>>().expect("ClassCastException").clone()) };
        let mut index: i32 = 1;
        let mut aFile: Option<Rc<RefCell<File>>> = jcall!(file).getParentFile();
        while ({ aFile = { let java_value = (jcall!(aFile).getParentFile()).clone(); java_value }; aFile.clone() }).is_some() {
            index += 1;
        }
        if jcall!(file).isDirectory() && index < 5 {
            return crate::java_into_object(true);
        }
        return crate::java_into_object(false);
    }
}

