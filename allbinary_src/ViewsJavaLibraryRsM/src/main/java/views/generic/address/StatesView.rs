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
// package views.generic.address
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::business::user::address::StreetAddressData::StreetAddressData;
use crate::org::allbinary::data::tree::dom::DomNodeInterface::DomNodeInterface;
use crate::org::allbinary::data::tree::dom::ModDomHelper::ModDomHelper;
use crate::org::allbinary::globals::FREEBLISKET_PATH_GLOBALS::FREEBLISKET_PATH_GLOBALS;
use crate::org::allbinary::globals::URLGLOBALS::URLGLOBALS;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::io::LineReader::LineReader;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct StatesView {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
}

impl DomNodeInterface for StatesView {
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}

impl StatesView {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut node: Option<Rc<RefCell<Node>>> = jcall!(document).createElement(crate::org::allbinary::business::user::address::StreetAddressData::StreetAddressData_STATES.with(Clone::clone).clone());
            let mut statesFile: String = format!("{}{}", format!("{}{}", URLGLOBALS::getMainPath(), jcall!(FREEBLISKET_PATH_GLOBALS::getInstance()).LINEDATAPATH), String::from("states.txt"));
            let mut lineReader: Option<Rc<RefCell<LineReader>>> = LineReader::new(statesFile.clone());
            while jcall!(lineReader).hasNext() {
                let mut option: String = jcall!(lineReader).next();
                jcall!(node).appendChild(ModDomHelper::createNameValueNodes(document.clone(), crate::org::allbinary::business::user::address::StreetAddressData::StreetAddressData_STATE.with(Clone::clone).clone(), option.clone()));
            }
            return node.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).FAILURE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("toXmlNode"), crate::java_into_object(e.clone()));
            }
            return None;
            }
        }
    }
}

