//
// AllBinary Open License Version 1
// Copyright (c) 2022 AllBinary
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
// package org.allbinary.data.tree.dom
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::io::ByteArrayOutputStream::ByteArrayOutputStream;
// use crate::javax::xml::transform::stream::StreamResult::StreamResult;
// use crate::javax::xml::transform::stream::StreamSource::StreamSource;
use crate::org::allbinary::logic::io::file::AbFile::AbFile;
use crate::org::allbinary::logic::io::path::AbPath::AbPath;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AbXslHelper {
}

thread_local! { pub static AbXslHelper_instance: Option<Rc<RefCell<AbXslHelper>>> = AbXslHelper::new(); }

impl AbXslHelper {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<AbXslHelper>>> {
        return AbXslHelper_instance.with(Clone::clone).clone();
    }
    

    // throws Exception
    pub fn translate(&mut self, mut xslFile: Option<Rc<RefCell<AbFile>>>, mut xmlFile: Option<Rc<RefCell<AbFile>>>) -> String {
        return { let java_arguments = (jcall!(xslFile).getAbsolutePath(), jcall!(xmlFile).getAbsolutePath(),); self.translate(java_arguments.0, java_arguments.1) };
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn translate(&mut self, mut xslPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>, mut xmlPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>) -> String {
        return { let java_arguments = (jcall!(xslPath).getPath(), jcall!(xmlPath).getPath(),); self.translate(java_arguments.0, java_arguments.1) };
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn translate(&mut self, mut xsltFilePath: String, mut xmlFilePath: String) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut outputStream: Vec<u8> = Vec::new();
            return jcall!(jcall!(XslHelper::XslHelper::getInstance()).translate3(StreamSource::new(xsltFilePath.clone()), StreamSource::new(xmlFilePath.clone()), StreamResult::new(outputStream.clone()))).toString();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            panic!("{:?}", e);
            }
        }
    }
}

