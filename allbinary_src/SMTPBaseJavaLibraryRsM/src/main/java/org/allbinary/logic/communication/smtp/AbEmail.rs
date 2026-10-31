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
// package org.allbinary.logic.communication.smtp
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
// use crate::javax::mail::internet::MimeMessage::MimeMessage;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use super::EmailInterface::EmailInterface;
use super::Email::Email;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AbEmail {
    pub email: Option<Rc<RefCell<Email>>>,
}

impl EmailInterface for AbEmail {
    fn getDebugInfo(&mut self) -> String { Self::getDebugInfo(self) }
    fn getMimeMessage(&mut self) -> Option<Rc<RefCell<MimeMessage>>> { Self::getMimeMessage(self) }
    fn log(&mut self) -> String { Self::log(self) }
}
impl crate::org::allbinary::data::tree::dom::DomNodeInterface::DomNodeInterface for AbEmail {
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}
impl crate::org::allbinary::data::tree::dom::document::mapping::DomDocumentMappingInterface::DomDocumentMappingInterface for AbEmail {
    fn toXmlDoc(&mut self) -> Option<Rc<RefCell<Document>>> { Self::toXmlDoc(self) }
}

impl AbEmail {
    

    pub fn new_struct(mut server: String, mut from: String, mut to: String, mut subject: String, mut textBody: String, mut htmlAttachment: String, mut contentBase: String) -> Self {
        let mut this = Self {
            email: None,
        };
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).EMAILLOGGINGERROR.clone()) {
            this.email = Email::new8(from.clone(), to.clone(), subject.clone(), server.clone(), textBody.clone(), htmlAttachment.clone(), contentBase.clone(), true);
        } else {
            this.email = Email::new7(from.clone(), to.clone(), subject.clone(), server.clone(), textBody.clone(), htmlAttachment.clone(), contentBase.clone());
        }
        this
    }

    pub fn new(mut server: String, mut from: String, mut to: String, mut subject: String, mut textBody: String, mut htmlAttachment: String, mut contentBase: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(server, from, to, subject, textBody, htmlAttachment, contentBase))))
    }
    

    pub fn getMimeMessage(&mut self) -> Option<Rc<RefCell<MimeMessage>>> {
        return jcall!(self.email).getMimeMessage();
    }
    

    pub fn getDebugInfo(&mut self) -> String {
        return jcall!(self.email).getDebugInfo();
    }
    

    // throws Exception
    pub fn log(&mut self) -> String {
        return jcall!(self.email).log();
    }
    

    // throws Exception
    pub fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> {
        return jcall!(self.email).toHashMap();
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        return jcall!(self.email).toXmlNode(document.clone());
    }
    

    pub fn getEmail(&mut self) -> Option<Rc<RefCell<Email>>> {
        return (self.email).clone() /* as Email */;
    }
    

    // throws Exception
    pub fn toXmlDoc(&mut self) -> Option<Rc<RefCell<Document>>> {
        return jcall!(self.email).toXmlDoc();
    }
}

