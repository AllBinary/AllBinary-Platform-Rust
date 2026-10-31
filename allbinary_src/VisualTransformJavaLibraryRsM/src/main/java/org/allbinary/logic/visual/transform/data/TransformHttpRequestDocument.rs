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
// package org.allbinary.logic.visual.transform.data
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::servlet::http::HttpServletRequest::HttpServletRequest;
// use crate::javax::servlet::jsp::PageContext::PageContext;
use crate::org::allbinary::data::tree::dom::document::DomDocumentHelper::DomDocumentHelper;
use crate::org::allbinary::data::tree::dom::document::XmlDocumentHelper::XmlDocumentHelper;
use crate::org::allbinary::logic::communication::http::AcceptableResponseGenerator::AcceptableResponseGenerator;
use crate::org::allbinary::logic::communication::http::request::session::WeblisketSession::WeblisketSession;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::control::search::SearchRequest::SearchRequest;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use super::TransformDocumentInterface::TransformDocumentInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TransformHttpRequestDocument {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub baseNode: Option<Rc<RefCell<Node>>>,
    pub document: Option<Rc<RefCell<Document>>>,
}

thread_local! { pub static TransformHttpRequestDocument_language: String = String::from("en"); }
thread_local! { pub static TransformHttpRequestDocument_local: String = String::from("US"); }
impl TransformDocumentInterface for TransformHttpRequestDocument {
    fn getBaseNode(&mut self) -> Option<Rc<RefCell<Node>>> { Self::getBaseNode(self) }
    fn getDoc(&mut self) -> Option<Rc<RefCell<Document>>> { Self::getDoc(self) }
}

impl TransformHttpRequestDocument {
    

    pub fn new_struct2(mut pageContext: Option<Rc<RefCell<PageContext>>>, mut weblisketSession: Option<Rc<RefCell<WeblisketSession>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            baseNode: None,
            document: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut contentType: String = jcall!(AcceptableResponseGenerator::getInstance()).get((jcall!(pageContext).getRequest()).clone() /* as HttpServletRequest */);
            this.document = DomDocumentHelper::create();
            let mut contentTypeNode: Option<Rc<RefCell<Node>>> = jcall!(this.document).createElement(contentType.clone());
            let mut languageTypeNode: Option<Rc<RefCell<Node>>> = jcall!(this.document).createElement(TransformHttpRequestDocument_language.with(Clone::clone).clone());
            let mut localTypeNode: Option<Rc<RefCell<Node>>> = jcall!(this.document).createElement(TransformHttpRequestDocument_local.with(Clone::clone).clone());
            this.baseNode = localTypeNode.clone();
            jcall!(languageTypeNode).appendChild(localTypeNode.clone());
            jcall!(contentTypeNode).appendChild(languageTypeNode.clone());
            jcall!(this.document).appendChild(contentTypeNode.clone());
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGSERROR.clone()) {
                { let java_receiver = this.logUtil.clone(); let java_arguments = (this.log(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGSERROR.clone()) {
                jcall!(this.logUtil).put(jcall!(this.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone(), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", e);
            }
        }
        this
    }

    pub fn new2(mut pageContext: Option<Rc<RefCell<PageContext>>>, mut weblisketSession: Option<Rc<RefCell<WeblisketSession>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2(pageContext, weblisketSession))))
    }
    

    pub fn new_struct1(mut searchRequest: Option<Rc<RefCell<SearchRequest>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            baseNode: None,
            document: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut contentType: String = jcall!(searchRequest).getContentType();
            this.document = DomDocumentHelper::create();
            let mut contentTypeNode: Option<Rc<RefCell<Node>>> = jcall!(this.document).createElement(contentType.clone());
            let mut languageTypeNode: Option<Rc<RefCell<Node>>> = jcall!(this.document).createElement(TransformHttpRequestDocument_language.with(Clone::clone).clone());
            let mut localTypeNode: Option<Rc<RefCell<Node>>> = jcall!(this.document).createElement(TransformHttpRequestDocument_local.with(Clone::clone).clone());
            this.baseNode = localTypeNode.clone();
            jcall!(languageTypeNode).appendChild(localTypeNode.clone());
            jcall!(contentTypeNode).appendChild(languageTypeNode.clone());
            jcall!(this.document).appendChild(contentTypeNode.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGSERROR.clone()) {
                jcall!(this.logUtil).put(jcall!(this.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone(), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", e);
            }
        }
        this
    }

    pub fn new1(mut searchRequest: Option<Rc<RefCell<SearchRequest>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(searchRequest))))
    }
    

    pub fn getBaseNode(&mut self) -> Option<Rc<RefCell<Node>>> {
        return self.baseNode.clone();
    }
    

    pub fn getDoc(&mut self) -> Option<Rc<RefCell<Document>>> {
        return self.document.clone();
    }
    

    // throws Exception
    pub fn log(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        if self.baseNode.is_some() {
            jcall!(stringBuffer).append(String::from("BaseNode: "));
            jcall!(stringBuffer).append(jcall!(self.baseNode).getNodeName());
        } else {
            jcall!(stringBuffer).append(String::from("Log-Error: BaseNode is Null"));
        }
        jcall!(stringBuffer).append(String::from("\nDocument: "));
        jcall!(stringBuffer).append(XmlDocumentHelper::toString(self.document.clone()));
        return jcall!(stringBuffer).toString();
    }
}
pub trait TransformHttpRequestDocumentDyn {
    fn as_TransformHttpRequestDocument(&self) -> &TransformHttpRequestDocument;
    fn as_TransformHttpRequestDocument_mut(&mut self) -> &mut TransformHttpRequestDocument;
    fn getBaseNode(&mut self) -> Option<Rc<RefCell<Node>>> { TransformHttpRequestDocument::getBaseNode(self.as_TransformHttpRequestDocument_mut()) }
    fn getDoc(&mut self) -> Option<Rc<RefCell<Document>>> { TransformHttpRequestDocument::getDoc(self.as_TransformHttpRequestDocument_mut()) }
    fn log(&mut self) -> String { TransformHttpRequestDocument::log(self.as_TransformHttpRequestDocument_mut()) }
}
impl TransformHttpRequestDocumentDyn for TransformHttpRequestDocument {
    fn as_TransformHttpRequestDocument(&self) -> &TransformHttpRequestDocument { self }
    fn as_TransformHttpRequestDocument_mut(&mut self) -> &mut TransformHttpRequestDocument { self }
}


