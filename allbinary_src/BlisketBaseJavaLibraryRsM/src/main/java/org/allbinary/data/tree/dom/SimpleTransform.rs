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
// use crate::java::io::StringBufferInputStream::StringBufferInputStream;
// use crate::javax::xml::transform::stream::StreamSource::StreamSource;
use crate::org::allbinary::data::tree::dom::document::DomDocumentHelper::DomDocumentHelper;
use crate::org::allbinary::data::tree::dom::document::XmlDocumentHelper::XmlDocumentHelper;
use crate::org::allbinary::logic::io::AbFileLocalInputStream::AbFileLocalInputStream;
use crate::org::allbinary::logic::io::file::AbFile::AbFile;
use crate::org::allbinary::logic::io::path::AbPath::AbPath;
use crate::org::w3c::dom::Document::Document;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SimpleTransform {
    pub xsltFilePath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>,
    pub dataFilePath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>,
}


impl SimpleTransform {
    

    pub fn new_struct(mut xsltFilePath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>, mut dataFilePath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>) -> Self {
        let mut this = Self {
            xsltFilePath: None,
            dataFilePath: None,
        };
        this.xsltFilePath = xsltFilePath.clone();
        this.dataFilePath = dataFilePath.clone();
        this
    }

    pub fn new(mut xsltFilePath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>, mut dataFilePath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(xsltFilePath, dataFilePath))))
    }
    

    // throws Exception
    pub fn transform(&mut self) -> String {
        let mut inputStream: Option<Rc<RefCell<AbFileLocalInputStream>>> = AbFileLocalInputStream::new(AbFile::createAbFileFromAbPath(self.xsltFilePath.clone()));
        let mut document: Option<Rc<RefCell<Document>>> = DomDocumentHelper::create1(AbFileLocalInputStream::new(AbFile::createAbFileFromAbPath(self.dataFilePath.clone())));
        let mut result: String = jcall!(XslHelper::XslHelper::getInstance()).translate(StreamSource::new(inputStream.clone()), StreamSource::new(StringBufferInputStream::new(XmlDocumentHelper::toString(document.clone()))));
        return result.clone();
    }
}

