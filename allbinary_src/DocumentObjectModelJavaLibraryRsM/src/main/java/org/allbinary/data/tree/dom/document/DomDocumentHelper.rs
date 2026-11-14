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
// package org.allbinary.data.tree.dom.document
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::java::io::InputStream::InputStream;
// use crate::java::io::StringBufferInputStream::StringBufferInputStream;
// use crate::javax::xml::parsers::DocumentBuilder::DocumentBuilder;
// use crate::javax::xml::parsers::DocumentBuilderFactory::DocumentBuilderFactory;
use crate::org::w3c::dom::Document::Document;
use crate::java::lang::Exception::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct DomDocumentHelper {
}


impl DomDocumentHelper {

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
    

    pub fn init() {
        System::System::setProperty(String::from("jdk.xml.maxElementDepth"), String::from("0"));
        System::System::setProperty(String::from("jdk.xml.xpathExprOpLimit"), String::from("0"));
        System::System::setProperty(String::from("jdk.xml.xpathTotalOpLimit"), String::from("0"));
        let mut LIMIT: String = String::from("500000");
        System::System::setProperty(String::from("jdk.xml.maxGeneralEntitySizeLimit"), LIMIT.clone());
        System::System::setProperty(String::from("jdk.xml.totalEntitySizeLimit"), LIMIT.clone());
    }
    

    pub fn create() -> Option<Rc<RefCell<Document>>> {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut factory: Option<Rc<RefCell<DocumentBuilderFactory>>> = DocumentBuilderFactory::newInstance();
            let mut builder: Option<Rc<RefCell<DocumentBuilder>>> = jcall!(factory).newDocumentBuilder();
            let mut document: Option<Rc<RefCell<Document>>> = jcall!(builder).newDocument();
            return Some(document.clone());
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            return None;
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn create1(mut inputStream: Option<Rc<RefCell<dyn crate::java::io::InputStream::InputStreamDyn>>>) -> Option<Rc<RefCell<Document>>> {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut factory: Option<Rc<RefCell<DocumentBuilderFactory>>> = DocumentBuilderFactory::newInstance();
            let mut builder: Option<Rc<RefCell<DocumentBuilder>>> = jcall!(factory).newDocumentBuilder();
            let mut document: Option<Rc<RefCell<Document>>> = jcall!(builder).parse(inputStream.clone());
            return Some(document.clone());
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            panic!("{:?}", e);
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn create1_String(mut xmlString: String) -> Option<Rc<RefCell<Document>>> {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut factory: Option<Rc<RefCell<DocumentBuilderFactory>>> = DocumentBuilderFactory::newInstance();
            let mut builder: Option<Rc<RefCell<DocumentBuilder>>> = jcall!(factory).newDocumentBuilder();
            let mut document: Option<Rc<RefCell<Document>>> = jcall!(builder).parse(StringBufferInputStream::new(xmlString.clone()));
            return Some(document.clone());
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            panic!("{:?}", e);
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
}

