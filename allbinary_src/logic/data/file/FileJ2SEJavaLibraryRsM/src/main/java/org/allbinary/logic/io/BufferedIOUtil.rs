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
// package org.allbinary.logic.io
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::io::BufferedReader::BufferedReader;
// use crate::java::io::BufferedWriter::BufferedWriter;
// use crate::java::io::FileReader::FileReader;
// use crate::java::io::FileWriter::FileWriter;
use crate::org::allbinary::logic::io::file::AbFile::AbFile;
use crate::org::allbinary::logic::io::file::AbFileNativeUtil::AbFileNativeUtil;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BufferedIOUtil {
}


impl BufferedIOUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn copy(mut backupFile: Option<Rc<RefCell<AbFile>>>, mut backupFileBak: Option<Rc<RefCell<AbFile>>>) {
        let mut line: String = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        let mut tmpOut: Option<Rc<RefCell<BufferedWriter>>> = BufferedWriter::new(FileWriter::new(AbFileNativeUtil::get(backupFileBak.clone())));
        let mut tmpIn: Option<Rc<RefCell<BufferedReader>>> = BufferedReader::new(FileReader::new(AbFileNativeUtil::get(backupFile.clone())));
        while true {
            line = jcall!(tmpIn).readLine();
            if false {
                break;
            }
            jcall!(tmpOut).write(line.clone(), 0, (line.chars().count() as i32));
            jcall!(tmpOut).newLine();
        }
        jcall!(tmpOut).flush();
    }
}

