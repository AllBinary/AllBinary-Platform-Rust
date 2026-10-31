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
// use crate::java::io::ByteArrayOutputStream::ByteArrayOutputStream;
// use crate::java::io::PrintStream::PrintStream;
// use crate::java::util::HashMap::HashMap;
// use crate::java::util::Properties::Properties;
// use crate::javax::mail::Address::Address;
// use crate::javax::mail::Authenticator::Authenticator;
// use crate::javax::mail::Message::Message;
// use crate::javax::mail::MessagingException::MessagingException;
// use crate::javax::mail::Session::Session;
// use crate::javax::mail::internet::InternetAddress::InternetAddress;
// use crate::javax::mail::internet::InternetHeaders::InternetHeaders;
// use crate::javax::mail::internet::MimeBodyPart::MimeBodyPart;
// use crate::javax::mail::internet::MimeMessage::MimeMessage;
// use crate::javax::mail::internet::MimeMultipart::MimeMultipart;
use crate::org::allbinary::data::tree::dom::ModDomHelper::ModDomHelper;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use super::EmailInterface::EmailInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct Email {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub msg: Option<Rc<RefCell<MimeMessage>>>,
    pub properties: Option<Rc<RefCell<Properties>>>,
    pub bs: Vec<u8>,
    pub isDebug: bool,
}

thread_local! { pub static Email_DEBUG: String = String::from("mail.debug"); }
thread_local! { pub static Email_SMTP_HOST: String = String::from("mail.smtp.host"); }
thread_local! { pub static Email_SMTP_PORT: String = String::from("mail.smtp.port"); }
thread_local! { pub static Email_SMTP_USER: String = String::from("mail.smtp.user"); }
thread_local! { pub static Email_SMTP_RETURN_ADDRESS: String = String::from("mail.from"); }
thread_local! { pub static Email_SMTP_LOCAL_HOST: String = String::from("mail.smtp.localhost"); }
impl EmailInterface for Email {
    fn getDebugInfo(&mut self) -> String { Self::getDebugInfo(self) }
    fn getMimeMessage(&mut self) -> Option<Rc<RefCell<MimeMessage>>> { Self::getMimeMessage(self) }
    fn log(&mut self) -> String { Self::log(self) }
}
impl crate::org::allbinary::data::tree::dom::DomNodeInterface::DomNodeInterface for Email {
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}
impl crate::org::allbinary::data::tree::dom::document::mapping::DomDocumentMappingInterface::DomDocumentMappingInterface for Email {
    fn toXmlDoc(&mut self) -> Option<Rc<RefCell<Document>>> { Self::toXmlDoc(self) }
}

impl Email {
    

    pub fn new_struct7(mut from: String, mut to: String, mut subject: String, mut server: String, mut textBody: String, mut htmlAttachment: String, mut contentBase: String) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            msg: None,
            properties: None,
            bs: None,
            isDebug: false,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.isDebug = false;
        this.init(from.clone(), to.clone(), subject.clone(), server.clone(), textBody.clone(), htmlAttachment.clone(), contentBase.clone());
        this
    }

    pub fn new7(mut from: String, mut to: String, mut subject: String, mut server: String, mut textBody: String, mut htmlAttachment: String, mut contentBase: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct7(from, to, subject, server, textBody, htmlAttachment, contentBase))))
    }
    

    pub fn new_struct8(mut from: String, mut to: String, mut subject: String, mut server: String, mut textBody: String, mut htmlAttachment: String, mut contentBase: String, mut isDebug: bool) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            msg: None,
            properties: None,
            bs: None,
            isDebug: false,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.isDebug = isDebug.clone();
        this.init(from.clone(), to.clone(), subject.clone(), server.clone(), textBody.clone(), htmlAttachment.clone(), contentBase.clone());
        this
    }

    pub fn new8(mut from: String, mut to: String, mut subject: String, mut server: String, mut textBody: String, mut htmlAttachment: String, mut contentBase: String, mut isDebug: bool) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct8(from, to, subject, server, textBody, htmlAttachment, contentBase, isDebug))))
    }
    

    // throws Exception
    pub fn init(&mut self, mut from: String, mut to: String, mut subject: String, mut server: String, mut textBody: String, mut htmlAttachment: String, mut contentBase: String) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut stringValidationUtil: Option<Rc<RefCell<StringValidationUtil>>> = StringValidationUtil::getInstance();
            let mut mimeBodyParts: Vec<Option<Rc<RefCell<MimeBodyPart>>>> = vec![None; ((if !jcall!(stringValidationUtil).isEmpty(htmlAttachment.clone()) { 2 } else { 1 })) as usize];
            mimeBodyParts[(0) as usize] = MimeBodyPart::new();
            jcall!(mimeBodyParts[(0) as usize]).setText(textBody.clone());
            if !jcall!(stringValidationUtil).isEmpty(htmlAttachment.clone()) {
                let mut internetHeaders: Option<Rc<RefCell<InternetHeaders>>> = InternetHeaders::new();
                jcall!(internetHeaders).addHeader(String::from("Content-Type"), String::from("text/html"));
                if !jcall!(stringValidationUtil).isEmpty(contentBase.clone()) {
                    jcall!(internetHeaders).addHeader(String::from("Content-Base"), contentBase.clone());
                }
                mimeBodyParts[(1) as usize] = MimeBodyPart::new(internetHeaders.clone(), htmlAttachment.bytes().map(|value| value as i8).collect::<Vec<i8>>());
            }
            self.init8(server.clone(), (None).clone() /* as Authenticator */, vec![InternetAddress::new(from.clone())], vec![InternetAddress::new(to.clone())], (None).clone() /* as  */, (None).clone() /* as  */, subject.clone(), mimeBodyParts.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).EMAILLOGGINGERROR.clone()) {
                let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
                jcall!(self.logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("emailConstructor"), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", e);
            }
        }
    }
    

    // throws MessagingException
    // Java overloaded method: Rust needs a different name
    pub fn init8(&mut self, mut server: String, mut authenticator: Option<Rc<RefCell<Authenticator>>>, mut addresses_from: Vec<Option<Rc<RefCell<InternetAddress>>>>, mut addresses_to: Vec<Option<Rc<RefCell<InternetAddress>>>>, mut addresses_cc: Vec<Option<Rc<RefCell<InternetAddress>>>>, mut addresses_bcc: Vec<Option<Rc<RefCell<InternetAddress>>>>, mut subject: String, mut mimeBodyParts: Vec<Option<Rc<RefCell<MimeBodyPart>>>>) {
        self.properties = Properties::new();
        jcall!(self.properties).put(Email_SMTP_HOST.with(Clone::clone).clone(), server.clone());
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut hostName: String = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
            if jcall!(StringValidationUtil::getInstance()).isEmpty(hostName.clone()) {
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).EMAILLOGGINGERROR.clone()) {
                    jcall!(self.logUtil).putF(String::from("Unable to get HostName so using fake"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("init()"));
                }
                jcall!(self.properties).put(Email_SMTP_LOCAL_HOST.with(Clone::clone).clone(), String::from("FakeHostName"));
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).EMAILLOGGINGERROR.clone()) {
                jcall!(self.logUtil).put(String::from("Continuing on Exception: Unable to get HostName"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("init()"), crate::java_into_object(e.clone()));
            }
            jcall!(self.properties).put(Email_SMTP_LOCAL_HOST.with(Clone::clone).clone(), String::from("FakeHostName"));
            }
        }
        if self.isDebug {
            jcall!(self.properties).put(Email_DEBUG.with(Clone::clone).clone(), String::from("true"));
        }
        let mut session: Option<Rc<RefCell<Session>>> = Session::getInstance(self.properties.clone(), authenticator.clone());
        if self.isDebug {
            jcall!(session).setDebug(true);
            self.bs = Vec::new();
            let mut printStream: () = ();
            jcall!(session).setDebugOut(printStream.clone());
        }
        self.msg = MimeMessage::new(session.clone());
        jcall!(self.msg).addFrom(addresses_from.clone());
        jcall!(self.msg).setRecipients(jcall!(unsafe { crate::javax::mail::Message::Message_RecipientType.clone() }).TO.clone(), addresses_to.clone());
        jcall!(self.msg).setRecipients(jcall!(unsafe { crate::javax::mail::Message::Message_RecipientType.clone() }).CC.clone(), addresses_cc.clone());
        jcall!(self.msg).setRecipients(jcall!(unsafe { crate::javax::mail::Message::Message_RecipientType.clone() }).BCC.clone(), addresses_bcc.clone());
        jcall!(self.msg).setSubject(subject.clone());
        let mut mimeMultipart: Option<Rc<RefCell<MimeMultipart>>> = MimeMultipart::new();
        for i in (0..(mimeBodyParts.len() as i32)) {
            jcall!(mimeMultipart).addBodyPart(mimeBodyParts[(i) as usize].clone());
        }
        jcall!(self.msg).setContent(mimeMultipart.clone());
    }
    

    pub fn getMimeMessage(&mut self) -> Option<Rc<RefCell<MimeMessage>>> {
        return self.msg.clone();
    }
    

    pub fn getDebugInfo(&mut self) -> String {
        if self.isDebug {
            return format!("{}{}", String::from("More Logging Before This To Standard Out: "), String::from_utf8_lossy(&self.bs).into_owned().clone());
        } else {
            return String::from("Debugging Is Off");
        }
    }
    

    // throws Exception
    pub fn log(&mut self) -> String {
        return format!("{}{}", String::from("\nHashMap: "), jcall!(self.toHashMap()).toString());
    }
    

    // throws Exception
    pub fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut hashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
            jcall!(hashMap).put(EmailData_SERVER.with(Clone::clone).clone(), (jcall!(self.properties).get(Email_SMTP_HOST.with(Clone::clone).clone())).clone() /* as String */);
            let mut addresses: Vec<Option<Rc<RefCell<Address>>>> = jcall!(self.msg).getFrom();
            if addresses.is_some() {
                for index in (0..(addresses.len() as i32)) {
                    jcall!(hashMap).put(EmailData_FROM.with(Clone::clone).clone(), jcall!(addresses[(index) as usize]).toString());
                }
            }
            addresses = jcall!(self.msg).getRecipients(jcall!(unsafe { crate::javax::mail::Message::Message_RecipientType.clone() }).TO.clone());
            if addresses.is_some() {
                for index in (0..(addresses.len() as i32)) {
                    jcall!(hashMap).put(EmailData_TO.with(Clone::clone).clone(), jcall!(addresses[(index) as usize]).toString());
                }
            }
            addresses = jcall!(self.msg).getRecipients(jcall!(unsafe { crate::javax::mail::Message::Message_RecipientType.clone() }).CC.clone());
            if addresses.is_some() {
                for index in (0..(addresses.len() as i32)) {
                    jcall!(hashMap).put(EmailData_CC.with(Clone::clone).clone(), jcall!(addresses[(index) as usize]).toString());
                }
            }
            addresses = jcall!(self.msg).getRecipients(jcall!(unsafe { crate::javax::mail::Message::Message_RecipientType.clone() }).BCC.clone());
            if addresses.is_some() {
                for index in (0..(addresses.len() as i32)) {
                    jcall!(hashMap).put(EmailData_BCC.with(Clone::clone).clone(), jcall!(addresses[(index) as usize]).toString());
                }
            }
            jcall!(hashMap).put(EmailData_SUBJECT.with(Clone::clone).clone(), jcall!(self.msg).getSubject());
            let mut mimeMultipart: Option<Rc<RefCell<MimeMultipart>>> = (jcall!(self.msg).getContent()).clone() /* as MimeMultipart */;
            if mimeMultipart.is_some() {
                for index in (0..jcall!(mimeMultipart).getCount()) {
                    let mut content: String = (jcall!(jcall!(mimeMultipart).getBodyPart(index.clone())).getContent()).clone() /* as String */;
                    jcall!(hashMap).put(EmailData_CONTENT.with(Clone::clone).clone(), content.clone());
                }
            }
            return hashMap.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).EMAILLOGGINGERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("toHashMap()"), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", e);
            }
        }
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        let mut node: Option<Rc<RefCell<Node>>> = ModDomHelper::createNameValueNodes(document.clone(), EmailData_NAME.with(Clone::clone).clone(), self.toHashMap());
        return node.clone();
    }
    

    // throws Exception
    pub fn toXmlDoc(&mut self) -> Option<Rc<RefCell<Document>>> {
        panic!("{:?}", format!("{}: {:?}", "Exception", (jcall!(CommonStrings::getInstance()).NOT_IMPLEMENTED.clone())));
    }
}

