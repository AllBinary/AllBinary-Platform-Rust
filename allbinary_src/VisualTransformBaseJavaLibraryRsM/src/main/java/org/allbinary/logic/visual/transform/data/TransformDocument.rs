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
use crate::org::allbinary::data::tree::dom::document::DomDocumentHelper::DomDocumentHelper;
use crate::org::allbinary::data::tree::dom::document::XmlDocumentHelper::XmlDocumentHelper;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
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

pub struct TransformDocument {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub baseNode: Option<Rc<RefCell<Node>>>,
    pub document: Option<Rc<RefCell<Document>>>,
}

impl TransformDocumentInterface for TransformDocument {
    fn getBaseNode(&mut self) -> Option<Rc<RefCell<Node>>> { Self::getBaseNode(self) }
    fn getDoc(&mut self) -> Option<Rc<RefCell<Document>>> { Self::getDoc(self) }
}

impl TransformDocument {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            baseNode: None,
            document: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            this.document = DomDocumentHelper::create();
            let mut allbinaryNode: Option<Rc<RefCell<Node>>> = jcall!(this.document).createElement(String::from("allbinary"));
            this.baseNode = allbinaryNode.clone();
            jcall!(this.document).appendChild(this.baseNode.clone());
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

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
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

