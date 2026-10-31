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
// use crate::javax::xml::transform::Transformer::Transformer;
// use crate::javax::xml::transform::TransformerFactory::TransformerFactory;
// use crate::javax::xml::transform::dom::DOMSource::DOMSource;
// use crate::javax::xml::transform::stream::StreamResult::StreamResult;
use crate::org::allbinary::logic::io::AbDataOutputStream::AbDataOutputStream;
use crate::org::allbinary::logic::io::AbFileInputStream::AbFileInputStream;
use crate::org::allbinary::logic::io::DataOutputStreamFactory::DataOutputStreamFactory;
use crate::org::allbinary::logic::io::StreamUtil::StreamUtil;
use crate::org::allbinary::logic::io::file::AbFile::AbFile;
use crate::org::w3c::dom::Document::Document;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct DomDocumentFileHelper {
}


impl DomDocumentFileHelper {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn createDocument(mut xmlFile: Option<Rc<RefCell<AbFile>>>) -> Option<Rc<RefCell<Document>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            return DomDocumentHelper::DomDocumentHelper::create1(AbFileInputStream::new1(xmlFile.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::AbFileInputStream::AbFileInputStreamDyn>>));
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            panic!("{:?}", e);
            }
        }
    }
    

    // throws Exception
    pub fn save(mut file: Option<Rc<RefCell<AbFile>>>, mut document: Option<Rc<RefCell<Document>>>) {
        let mut dataOutputStream: Option<Rc<RefCell<AbDataOutputStream>>> = None;
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut copyTransformerFactory: Option<Rc<RefCell<TransformerFactory>>> = TransformerFactory::newInstance();
            let mut copyTransformer: Option<Rc<RefCell<Transformer>>> = jcall!(copyTransformerFactory).newTransformer();
            let mut domSource: Option<Rc<RefCell<DOMSource>>> = DOMSource::new(document.clone());
            if jcall!(file).isFile() {
                jcall!(file).delete();
            }
            jcall!(file).createNewFile();
            dataOutputStream = jcall!(DataOutputStreamFactory::getInstance()).getInstanceForAbFile(file.clone());
            let mut streamResult: Option<Rc<RefCell<StreamResult>>> = StreamResult::new(dataOutputStream.clone());
            jcall!(copyTransformer).transform(domSource.clone(), streamResult.clone());
            jcall!(dataOutputStream).flush();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            panic!("{:?}", e);
            }
        }
        { // finally
            jcall!(StreamUtil::getInstance()).close(dataOutputStream.clone());
        }
    }
}

