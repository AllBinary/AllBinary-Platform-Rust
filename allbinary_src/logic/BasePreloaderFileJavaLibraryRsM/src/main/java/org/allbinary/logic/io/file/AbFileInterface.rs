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
// package org.allbinary.logic.io.file
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::io::FilenameFilter::FilenameFilter;
// use crate::java::io::IOException::IOException;
// use crate::java::net::URI::URI;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait AbFileInterface {
    

    fn canRead(&mut self) -> bool;
    

    fn canWrite(&mut self) -> bool;
    

    // throws IOException
    fn createNewFile(&mut self) -> bool;
    

    // throws IOException
    fn delete(&mut self) -> bool;
    

    fn deleteOnExit(&mut self);
    

    fn exists(&mut self) -> bool;
    

    fn getAbsolutePath(&mut self) -> String;
    

    // throws IOException
    fn getCanonicalPath(&mut self) -> String;
    

    fn getParent(&mut self) -> String;
    

    fn getPath(&mut self) -> String;
    

    fn isAbsolute(&mut self) -> bool;
    

    fn isDirectory(&mut self) -> bool;
    

    fn isFile(&mut self) -> bool;
    

    fn isHidden(&mut self) -> bool;
    

    fn lastModified(&mut self) -> i64;
    

    fn length(&mut self) -> i64;
    

    fn list(&mut self) -> Vec<String>;
    

    fn listWithFilter(&mut self, filter: Option<Rc<RefCell<FilenameFilter>>>) -> Vec<String>;
    

    fn mkdir(&mut self) -> bool;
    

    fn mkdirs(&mut self) -> bool;
    

    fn setLastModified(&mut self, time: i64) -> bool;
    

    fn setReadOnly(&mut self) -> bool;
    

    fn toString(&mut self) -> String;
    

    fn toURI(&mut self) -> Option<Rc<RefCell<URI>>>;
}


