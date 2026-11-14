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
// package org.allbinary.data.tree.dom.document
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::io::ByteArrayOutputStream::ByteArrayOutputStream;
// use crate::java::io::StringReader::StringReader;
// use crate::java::io::StringWriter::StringWriter;
use crate::java::io::Writer::Writer;
// use crate::javax::xml::parsers::DocumentBuilderFactory::DocumentBuilderFactory;
// use crate::javax::xml::transform::OutputKeys::OutputKeys;
// use crate::javax::xml::transform::Transformer::Transformer;
// use crate::javax::xml::transform::TransformerFactory::TransformerFactory;
// use crate::javax::xml::transform::dom::DOMSource::DOMSource;
// use crate::javax::xml::transform::stream::StreamResult::StreamResult;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::java::bool::BooleanFactory::BooleanFactory;
use crate::org::allbinary::logic::java::characters::CharacterSetData::CharacterSetData;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::w3c::dom::Document::Document;
use crate::org::xml::sax::InputSource::InputSource;
use crate::java::lang::Exception::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct XmlDocumentHelper {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub INDENT_NUMBER: String,
    pub transformer: Option<Rc<RefCell<Transformer>>>,
}

thread_local! { pub static XmlDocumentHelper_instance: Option<Rc<RefCell<XmlDocumentHelper>>> = XmlDocumentHelper::new(); }

impl XmlDocumentHelper {
    

    pub fn getInstance() -> Option<Rc<RefCell<XmlDocumentHelper>>> {
        return XmlDocumentHelper_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            INDENT_NUMBER: String::new(),
            transformer: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.INDENT_NUMBER = String::from("indent-number");
        this.init(4, true);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn init(&mut self, mut indent: i32, mut ignoreDeclaration: bool) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut booleanFactory: Option<Rc<RefCell<BooleanFactory>>> = BooleanFactory::getInstance();
            let mut transformerFactory: Option<Rc<RefCell<TransformerFactory>>> = TransformerFactory::newInstance();
            jcall!(transformerFactory).setAttribute(self.INDENT_NUMBER.clone(), indent.clone());
            self.transformer = { let java_value = (jcall!(transformerFactory).newTransformer()).clone(); java_value };
            { let java_receiver = self.transformer.clone(); let java_arguments = (unsafe { crate::javax::xml::transform::OutputKeys::OutputKeys_ENCODING.clone() }.clone(), { let java_field = jcall!(CharacterSetData::getInstance()).UTF_8.clone(); java_field }.clone(),); jcall!(java_receiver).setOutputProperty(java_arguments.0, java_arguments.1); };
            jcall!(self.transformer).setOutputProperty(unsafe { crate::javax::xml::transform::OutputKeys::OutputKeys_OMIT_XML_DECLARATION.clone() }.clone(), (if ignoreDeclaration { { let java_field = jcall!(booleanFactory).YES.clone(); java_field }.clone() } else { { let java_field = jcall!(booleanFactory).NO.clone(); java_field }.clone() }));
            jcall!(self.transformer).setOutputProperty(unsafe { crate::javax::xml::transform::OutputKeys::OutputKeys_INDENT.clone() }.clone(), { let java_field = jcall!(booleanFactory).YES.clone(); java_field }.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(self.logUtil).put({ let java_field = jcall!(commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(commonStrings).INIT.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    // throws Exception
    pub fn format(&mut self, mut xmlString: String) -> String {
        let mut src: Option<Rc<RefCell<InputSource>>> = InputSource::new(StringReader::new(xmlString.clone()));
        let mut document: Option<Rc<RefCell<Document>>> = jcall!(jcall!(DocumentBuilderFactory::newInstance()).newDocumentBuilder()).parse(src.clone());
        let mut out: Option<Rc<RefCell<dyn crate::java::io::Writer::WriterDyn>>> = StringWriter::new();
        jcall!(self.transformer).transform(DOMSource::new(document.clone()), StreamResult::new(out.clone()));
        return jcall!(out).as_Writer_mut().toString();
    }
    

    // throws Exception
    pub fn toString(mut document: Option<Rc<RefCell<Document>>>) -> String {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut domSource: Option<Rc<RefCell<DOMSource>>> = DOMSource::new(document.clone());
            let mut byteArrayOutputStream: Option<Rc<RefCell<crate::java::io::OutputStream::OutputStream>>> = crate::java::io::OutputStream::OutputStream::new();
            let mut streamResult: Option<Rc<RefCell<StreamResult>>> = StreamResult::new(byteArrayOutputStream.clone());
            let mut copyTransformerFactory: Option<Rc<RefCell<TransformerFactory>>> = TransformerFactory::newInstance();
            let mut copyTransformer: Option<Rc<RefCell<Transformer>>> = jcall!(copyTransformerFactory).newTransformer();
            jcall!(copyTransformer).transform(domSource.clone(), streamResult.clone());
            return Some(jcall!(byteArrayOutputStream).toString());
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

