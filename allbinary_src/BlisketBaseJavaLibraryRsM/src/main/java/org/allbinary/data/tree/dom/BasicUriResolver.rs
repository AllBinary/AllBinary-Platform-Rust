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
// use crate::javax::xml::transform::Source::Source;
// use crate::javax::xml::transform::TransformerException::TransformerException;
// use crate::javax::xml::transform::URIResolver::URIResolver;
// use crate::javax::xml::transform::stream::StreamSource::StreamSource;
use crate::org::allbinary::globals::FREEBLISKET_PATH_GLOBALS::FREEBLISKET_PATH_GLOBALS;
use crate::org::allbinary::globals::URLGLOBALS::URLGLOBALS;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::control::crypt::file::CryptFileReader::CryptFileReader;
use crate::org::allbinary::logic::io::path::AbFilePath::AbFilePath;
use crate::org::allbinary::logic::io::path::AbPath::AbPath;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::visual::transform::info::template::TransformInfoTemplateData::TransformInfoTemplateData;
use crate::java::lang::Exception::*;
use super::URIResolverStrings::URIResolverStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicUriResolver {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub uriResolverStrings: Option<Rc<RefCell<URIResolverStrings>>>,
    pub extension: String,
}

// impl URIResolver for BasicUriResolver with the methods of the trait

impl BasicUriResolver {
    

    pub fn new_struct(mut extension: String) -> Self {
        let mut this = Self {
            logUtil: None,
            uriResolverStrings: None,
            extension: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.uriResolverStrings = URIResolverStrings::getInstance();
        this.extension = extension.clone();
        this
    }

    pub fn new(mut extension: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(extension))))
    }
    

    pub fn getExtension(&mut self) -> String {
        return self.extension.clone();
    }
    

    // throws TransformerException
    pub fn resolve(&mut self, mut href: String, mut base: String) -> Option<Rc<RefCell<Source>>> {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(URLGLOBALS::getMainPath());
            jcall!(stringBuffer).append({ let java_field = jcall!(FREEBLISKET_PATH_GLOBALS::getInstance()).XSLPATH.clone(); java_field }.clone());
            jcall!(stringBuffer).append(href.clone());
            let mut abPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> = (AbFilePath::new(jcall!(stringBuffer).toString())).clone() /* as AbPath */;
            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).XMLLOGGING.clone(); java_field }.clone()) {
                jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
                jcall!(stringBuffer).append({ let java_field = jcall!(self.uriResolverStrings).ATTEMPT.clone(); java_field }.clone());
                jcall!(stringBuffer).append(href.clone());
                jcall!(stringBuffer).append({ let java_field = jcall!(self.uriResolverStrings).BASE.clone(); java_field }.clone());
                jcall!(stringBuffer).append(base.clone());
                jcall!(stringBuffer).append({ let java_field = jcall!(self.uriResolverStrings).NEW_PATH.clone(); java_field }.clone());
                jcall!(stringBuffer).append(jcall!(abPath).toString());
                jcall!(stringBuffer).append({ let java_field = jcall!(self.uriResolverStrings).NOTE.clone(); java_field }.clone());
                jcall!(stringBuffer).append({ let java_field = jcall!(FREEBLISKET_PATH_GLOBALS::getInstance()).XSLPATH.clone(); java_field }.clone());
                jcall!(stringBuffer).append({ let java_field = jcall!(self.uriResolverStrings).URL_GLOBAL.clone(); java_field }.clone());
                jcall!(stringBuffer).append({ let java_field = jcall!(self.uriResolverStrings).REQUIRED_EXTENSION.clone(); java_field }.clone());
                jcall!(stringBuffer).append(self.extension.clone());
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.uriResolverStrings).RESOLVE.clone(); java_field }.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            return Some(StreamSource::new(jcall!(CryptFileReader::new({ let java_field = jcall!(TransformInfoTemplateData::getInstance()).UNCRYPTED_EXTENSION.clone(); java_field }.clone(), { let java_field = jcall!(TransformInfoTemplateData::getInstance()).ENCRYPTED_EXTENSION.clone(); java_field }.clone())).getInputStream(abPath.clone())));
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            panic!("{:?}", e);
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    pub fn toString(&mut self) -> String {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(URLGLOBALS::getMainPath());
            jcall!(stringBuffer).append({ let java_field = jcall!(FREEBLISKET_PATH_GLOBALS::getInstance()).XSLPATH.clone(); java_field }.clone());
            jcall!(stringBuffer).append({ let java_field = jcall!(self.uriResolverStrings).IMPORT_URL.clone(); java_field }.clone());
            return Some(jcall!(stringBuffer).toString());
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            return String::from("BasicUriResolver - Does not work without webapp path should be changed");
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
}

