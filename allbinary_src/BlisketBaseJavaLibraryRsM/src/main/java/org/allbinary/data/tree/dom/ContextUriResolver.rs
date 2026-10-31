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
use crate::org::allbinary::logic::io::path::AbPathData::AbPathData;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::visual::transform::info::template::TransformInfoTemplateData::TransformInfoTemplateData;
use super::URIResolverStrings::URIResolverStrings;
use super::BasicUriResolver::BasicUriResolver;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ContextUriResolver {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub uriResolverStrings: Option<Rc<RefCell<URIResolverStrings>>>,
    pub basicURIResolver: Option<Rc<RefCell<BasicUriResolver>>>,
}

// impl URIResolver for ContextUriResolver with the methods of the trait

impl ContextUriResolver {
    

    pub fn new_struct(mut basicURIResolver: Option<Rc<RefCell<BasicUriResolver>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            uriResolverStrings: None,
            basicURIResolver: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.uriResolverStrings = URIResolverStrings::getInstance();
        this.basicURIResolver = basicURIResolver.clone();
        this
    }

    pub fn new(mut basicURIResolver: Option<Rc<RefCell<BasicUriResolver>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(basicURIResolver))))
    }
    

    // throws TransformerException
    pub fn resolve(&mut self, mut href: String, mut base: String) -> Option<Rc<RefCell<Source>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(URLGLOBALS::getMainPath());
            jcall!(stringBuffer).append(jcall!(FREEBLISKET_PATH_GLOBALS::getInstance()).XSLPATH.clone());
            jcall!(stringBuffer).append(jcall!(FREEBLISKET_PATH_GLOBALS::getInstance()).INSTALLPATH.clone());
            jcall!(stringBuffer).append(jcall!(AbPathData::getInstance()).SEPARATOR.clone());
            jcall!(stringBuffer).append(href.clone());
            let mut fileAbPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> = AbFilePath::new(jcall!(stringBuffer).toString()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).XMLLOGGING.clone()) {
                jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
                jcall!(stringBuffer).append(jcall!(self.uriResolverStrings).ATTEMPT.clone());
                jcall!(stringBuffer).append(href.clone());
                jcall!(stringBuffer).append(jcall!(self.uriResolverStrings).BASE.clone());
                jcall!(stringBuffer).append(base.clone());
                jcall!(stringBuffer).append(jcall!(self.uriResolverStrings).NEW_PATH.clone());
                jcall!(stringBuffer).append(jcall!(fileAbPath).toString());
                jcall!(stringBuffer).append(jcall!(self.uriResolverStrings).NOTE.clone());
                jcall!(stringBuffer).append(jcall!(FREEBLISKET_PATH_GLOBALS::getInstance()).XSLPATH.clone());
                jcall!(stringBuffer).append(jcall!(self.uriResolverStrings).URL_GLOBAL.clone());
                jcall!(stringBuffer).append(jcall!(self.uriResolverStrings).REQUIRED_EXTENSION.clone());
                jcall!(stringBuffer).append(jcall!(self.basicURIResolver).getExtension());
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.uriResolverStrings).RESOLVE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            return StreamSource::new(jcall!(CryptFileReader::new(jcall!(TransformInfoTemplateData::getInstance()).UNCRYPTED_EXTENSION.clone(), jcall!(TransformInfoTemplateData::getInstance()).ENCRYPTED_EXTENSION.clone())).getInputStream(fileAbPath.clone()));
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            panic!("{:?}", e);
            }
        }
    }
    

    pub fn toString(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            return format!("{}{}", format!("{}{}", URLGLOBALS::getMainPath(), jcall!(FREEBLISKET_PATH_GLOBALS::getInstance()).INSTALLPATH), jcall!(self.uriResolverStrings).IMPORT_URL);
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            return String::from("ContextUriResolver - Does not work without webapp path should be changed");
            }
        }
    }
}

