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
use crate::org::allbinary::logic::communication::smtp::info::BasicEmailInfo::BasicEmailInfo;
use crate::org::allbinary::logic::communication::smtp::info::EmailInfo::EmailInfo;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use super::EmailInterface::EmailInterface;
use super::AbEmail::AbEmail;
use super::Email::Email;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicEmail {
    pub email: Option<Rc<RefCell<AbEmail>>>,
}

impl EmailInterface for BasicEmail {
    fn getDebugInfo(&mut self) -> String { Self::getDebugInfo(self) }
    fn getMimeMessage(&mut self) -> Option<Rc<RefCell<MimeMessage>>> { Self::getMimeMessage(self) }
    fn log(&mut self) -> String { Self::log(self) }
}
impl crate::org::allbinary::data::tree::dom::DomNodeInterface::DomNodeInterface for BasicEmail {
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}
impl crate::org::allbinary::data::tree::dom::document::mapping::DomDocumentMappingInterface::DomDocumentMappingInterface for BasicEmail {
    fn toXmlDoc(&mut self) -> Option<Rc<RefCell<Document>>> { Self::toXmlDoc(self) }
}

impl BasicEmail {
    

    pub fn new_struct(mut emailInfo: Option<Rc<RefCell<EmailInfo>>>, mut to: String) -> Self {
        let mut this = Self {
            email: None,
        };
        let mut basicEmailInfo: Option<Rc<RefCell<dyn crate::org::allbinary::logic::communication::smtp::info::BasicEmailInfo::BasicEmailInfoDyn>>> = jcall!(emailInfo).getBasicEmailInfo();
        this.email = AbEmail::new(jcall!(jcall!(basicEmailInfo).getEmailServerConfigurationInterface()).getSmtpServer(), jcall!(jcall!(basicEmailInfo).getEmailServerConfigurationInterface()).getAccountName(), to.clone(), jcall!(basicEmailInfo).getSubject(), jcall!(basicEmailInfo).getTextBody(), jcall!(emailInfo).getHtmlAttachment(), jcall!(emailInfo).getContentBase());
        this
    }

    pub fn new(mut emailInfo: Option<Rc<RefCell<EmailInfo>>>, mut to: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(emailInfo, to))))
    }
    

    pub fn getEmail(&mut self) -> Option<Rc<RefCell<Email>>> {
        return (jcall!(self.email).getEmail()).clone() /* as Email */;
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
    

    // throws Exception
    pub fn toXmlDoc(&mut self) -> Option<Rc<RefCell<Document>>> {
        return jcall!(self.email).toXmlDoc();
    }
}

