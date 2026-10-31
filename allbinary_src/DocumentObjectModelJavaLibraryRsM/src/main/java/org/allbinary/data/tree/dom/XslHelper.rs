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
// package org.allbinary.data.tree.dom
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::io::ByteArrayOutputStream::ByteArrayOutputStream;
// use crate::java::io::File::File;
// use crate::java::io::OutputStream::OutputStream;
// use crate::java::io::StringBufferInputStream::StringBufferInputStream;
// use crate::javax::xml::transform::Transformer::Transformer;
// use crate::javax::xml::transform::TransformerFactory::TransformerFactory;
// use crate::javax::xml::transform::URIResolver::URIResolver;
// use crate::javax::xml::transform::stream::StreamResult::StreamResult;
// use crate::javax::xml::transform::stream::StreamSource::StreamSource;
use crate::org::allbinary::data::tree::dom::document::XmlDocumentHelper::XmlDocumentHelper;
use crate::org::w3c::dom::Document::Document;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//import org.allbinary.data.tree.dom.document.DomDocumentHelper;
pub struct XslHelper {
}

thread_local! { pub static XslHelper_instance: Option<Rc<RefCell<XslHelper>>> = XslHelper::new(); }

impl XslHelper {
    

    pub fn getInstance() -> Option<Rc<RefCell<XslHelper>>> {
        return XslHelper_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn translate3(&mut self, mut xsltStreamSource: Option<Rc<RefCell<StreamSource>>>, mut xmlStreamSource: Option<Rc<RefCell<StreamSource>>>, mut streamResult: Option<Rc<RefCell<StreamResult>>>) -> Option<Rc<RefCell<OutputStream>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut transformerFactory: Option<Rc<RefCell<TransformerFactory>>> = TransformerFactory::newInstance();
            let mut transformer: Option<Rc<RefCell<Transformer>>> = jcall!(transformerFactory).newTransformer(xsltStreamSource.clone());
            jcall!(transformer).transform(xmlStreamSource.clone(), streamResult.clone());
            return jcall!(streamResult).getOutputStream();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            panic!("{:?}", e);
            }
        }
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn translate4(&mut self, mut resolver: Option<Rc<RefCell<URIResolver>>>, mut xsltStreamSource: Option<Rc<RefCell<StreamSource>>>, mut xmlStreamSource: Option<Rc<RefCell<StreamSource>>>, mut streamResult: Option<Rc<RefCell<StreamResult>>>) -> Option<Rc<RefCell<OutputStream>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut tFactory: Option<Rc<RefCell<TransformerFactory>>> = TransformerFactory::newInstance();
            jcall!(tFactory).setURIResolver(resolver.clone());
            let mut transformer: Option<Rc<RefCell<Transformer>>> = jcall!(tFactory).newTransformer(xsltStreamSource.clone());
            jcall!(transformer).transform(xmlStreamSource.clone(), streamResult.clone());
            return jcall!(streamResult).getOutputStream();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            panic!("{:?}", e);
            }
        }
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn translate(&mut self, mut xsltStreamSource: Option<Rc<RefCell<StreamSource>>>, mut xmlDocument: Option<Rc<RefCell<Document>>>) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut outputStream: Vec<u8> = Vec::new();
            return jcall!({ let java_arguments = (xsltStreamSource.clone(), StreamSource::new(StringBufferInputStream::new(XmlDocumentHelper::toString(xmlDocument.clone()))), StreamResult::new(outputStream.clone()),); self.translate3(java_arguments.0, java_arguments.1, java_arguments.2) }).toString();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            panic!("{:?}", e);
            }
        }
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn translate(&mut self, mut xsltStreamSource: Option<Rc<RefCell<StreamSource>>>, mut xmlStreamSource: Option<Rc<RefCell<StreamSource>>>) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut outputStream: Vec<u8> = Vec::new();
            return jcall!(self.translate3(xsltStreamSource.clone(), xmlStreamSource.clone(), StreamResult::new(outputStream.clone()))).toString();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            panic!("{:?}", e);
            }
        }
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn translate3(&mut self, mut resolver: Option<Rc<RefCell<URIResolver>>>, mut xsltStreamSource: Option<Rc<RefCell<StreamSource>>>, mut xmlStreamSource: Option<Rc<RefCell<StreamSource>>>) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut outputStream: Vec<u8> = Vec::new();
            return jcall!(self.translate4(resolver.clone(), xsltStreamSource.clone(), xmlStreamSource.clone(), StreamResult::new(outputStream.clone()))).toString();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            panic!("{:?}", e);
            }
        }
    }
    

    // throws Exception
    pub fn export(&mut self, mut outputFile: Option<Rc<RefCell<File>>>, mut xsltFilePath: String, mut xmlDocument: Option<Rc<RefCell<Document>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(outputFile).createNewFile();
            { let java_arguments = (StreamSource::new(xsltFilePath.clone()), StreamSource::new(StringBufferInputStream::new(XmlDocumentHelper::toString(xmlDocument.clone()))), StreamResult::new(outputFile.clone()),); self.translate3(java_arguments.0, java_arguments.1, java_arguments.2) };
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            panic!("{:?}", e);
            }
        }
    }
}

