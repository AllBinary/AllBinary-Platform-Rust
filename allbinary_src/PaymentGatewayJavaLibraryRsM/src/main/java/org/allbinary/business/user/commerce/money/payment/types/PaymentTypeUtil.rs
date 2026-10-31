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
// package org.allbinary.business.user.commerce.money.payment.types
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::business::entry::EntryData::EntryData;
use crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewaysData::PaymentGatewaysData;
use crate::org::allbinary::data::tree::dom::DomNodeHelper::DomNodeHelper;
use crate::org::allbinary::data::tree::dom::DomSearchHelper::DomSearchHelper;
use crate::org::allbinary::data::tree::dom::document::DomDocumentHelper::DomDocumentHelper;
use crate::org::allbinary::globals::FREEBLISKET_PATH_GLOBALS::FREEBLISKET_PATH_GLOBALS;
use crate::org::allbinary::globals::URLGLOBALS::URLGLOBALS;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::control::crypt::file::CryptFileReader::CryptFileReader;
use crate::org::allbinary::logic::io::path::AbPath::AbPath;
use crate::org::allbinary::logic::io::path::AbPathData::AbPathData;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use crate::org::allbinary::logic::visual::transform::info::objectConfig::TransformInfoObjectConfigData::TransformInfoObjectConfigData;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use crate::org::w3c::dom::NodeList::NodeList;
use super::PaymentType::PaymentType;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PaymentTypeUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub PAYMENTGATEWAYFILEPATHSTRING: String,
    pub SHIPPINGMETHODSFILENAME: String,
    pub defaultName: String,
    pub paymentTypeVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}

thread_local! { pub static PaymentTypeUtil_instance: Option<Rc<RefCell<PaymentTypeUtil>>> = PaymentTypeUtil::new(); }

impl PaymentTypeUtil {
    

    // Java static initializer: call this once at startup
    pub fn static_init() {
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<PaymentTypeUtil>>> {
        return PaymentTypeUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            PAYMENTGATEWAYFILEPATHSTRING: String::new(),
            SHIPPINGMETHODSFILENAME: String::new(),
            defaultName: String::new(),
            paymentTypeVector: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.SHIPPINGMETHODSFILENAME = String::from("paymentGatewayTypes.xml");
        this.defaultName = String::new();
        this.paymentTypeVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut sep: String = jcall!(AbPathData::getInstance()).SEPARATOR.clone();
        jcall!(stringBuffer).append(sep.clone());
        jcall!(stringBuffer).append(String::from("generic"));
        jcall!(stringBuffer).append(sep.clone());
        jcall!(stringBuffer).append(String::from("user"));
        jcall!(stringBuffer).append(sep.clone());
        jcall!(stringBuffer).append(String::from("commerce"));
        jcall!(stringBuffer).append(sep.clone());
        jcall!(stringBuffer).append(String::from("money"));
        jcall!(stringBuffer).append(sep.clone());
        jcall!(stringBuffer).append(String::from("payment"));
        jcall!(stringBuffer).append(sep.clone());
        jcall!(stringBuffer).append(String::from("gateway"));
        jcall!(stringBuffer).append(sep.clone());
        this.PAYMENTGATEWAYFILEPATHSTRING = jcall!(stringBuffer).toString();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn add(&mut self, mut paymentType: Option<Rc<RefCell<PaymentType>>>) {
        jcall!(self.paymentTypeVector).add(crate::java_into_object(paymentType.clone()));
    }
    

    // throws Exception
    pub fn get(&mut self, mut paymentTypeString: String) -> Option<Rc<RefCell<PaymentType>>> {
        let mut size: i32 = jcall!(self.paymentTypeVector).size();
        for i in (0..size) {
            let mut paymentType: Option<Rc<RefCell<PaymentType>>> = { let java_object = (jcall!(self.paymentTypeVector).get(i.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<PaymentType>>>>().expect("ClassCastException").clone()) };
            if jcall!(jcall!(jcall!(paymentType).getBasicPaymentType()).getName()).compareTo(paymentTypeString.clone()) == 0 {
                return paymentType.clone();
            }
        }
        panic!("{:?}", format!("{}: {:?}", "Exception", (format!("{}{}", String::from("Unknown PaymentType: "), paymentTypeString))));
    }
    

    // throws Exception
    pub fn initDefault(&mut self, mut storeName: String) {
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).PAYMENT.clone()) {
            jcall!(self.logUtil).putF(format!("{}{}", String::from("initDefault Payment: "), self.defaultName), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("initDefault"));
        }
        if jcall!(StringValidationUtil::getInstance()).isEmpty(self.defaultName.clone()) {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(URLGLOBALS::getMainPath());
            jcall!(stringBuffer).append(jcall!(FREEBLISKET_PATH_GLOBALS::getInstance()).XSLPATH.clone());
            jcall!(stringBuffer).append(storeName.clone());
            jcall!(stringBuffer).append(jcall!(AbPathData::getInstance()).SEPARATOR.clone());
            jcall!(stringBuffer).append(self.PAYMENTGATEWAYFILEPATHSTRING.clone());
            let mut abPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> = (AbPath::new(jcall!(stringBuffer).toString(), self.SHIPPINGMETHODSFILENAME.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>)).clone() /* as AbPath */;
            let mut data: String = jcall!(CryptFileReader::new(jcall!(TransformInfoObjectConfigData::getInstance()).UNCRYPTED_EXTENSION.clone(), jcall!(TransformInfoObjectConfigData::getInstance()).ENCRYPTED_EXTENSION.clone())).get(abPath.clone());
            let mut document: Option<Rc<RefCell<Document>>> = DomDocumentHelper::create1(data.clone());
            let mut nodeList: Option<Rc<RefCell<NodeList>>> = jcall!(document).getElementsByTagName(unsafe { crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewaysData::PaymentGatewaysData_NAME.clone() }.clone());
            for index in (0..jcall!(nodeList).getLength()) {
                let mut node: Option<Rc<RefCell<Node>>> = jcall!(nodeList).item(index.clone());
                let mut defaultPaymentGatewayNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(jcall!(EntryData::getInstance()).DEFAULT.clone(), jcall!(node).getChildNodes());
                let mut defaultPaymentGatewayNodeValue: String = DomNodeHelper::getTextNodeValue(defaultPaymentGatewayNode.clone());
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).PAYMENT.clone()) {
                    jcall!(self.logUtil).putF(format!("{}{}", String::from("DefaultPaymentGatewayNodeValue: "), defaultPaymentGatewayNodeValue), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("initDefault"));
                }
                if !jcall!(StringValidationUtil::getInstance()).isEmpty(defaultPaymentGatewayNodeValue.clone()) {
                    self.defaultName = defaultPaymentGatewayNodeValue.clone();
                }
            }
        }
    }
    

    // throws Exception
    pub fn getDefault(&mut self, mut storeName: String) -> Option<Rc<RefCell<PaymentType>>> {
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).PAYMENT.clone()) {
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getDefault"));
        }
        self.initDefault(storeName.clone());
        if !jcall!(StringValidationUtil::getInstance()).isEmpty(self.defaultName.clone()) {
            return self.get(self.defaultName.clone());
        }
        panic!("{:?}", format!("{}: {:?}", "Exception", (format!("{}{}", String::from("No Default: "), self.defaultName))));
    }
    

    pub fn isContain(&mut self, mut paymentType: Option<Rc<RefCell<PaymentType>>>) -> bool {
        return jcall!(self.paymentTypeVector).contains(crate::java_into_object(paymentType.clone()));
    }
    

    pub fn difference(&mut self, mut a_PaymentTypeVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut diff: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut size: i32 = jcall!(self.paymentTypeVector).size();
        for i in (0..size) {
            let mut paymentType: Option<Rc<RefCell<PaymentType>>> = { let java_object = (jcall!(self.paymentTypeVector).get(i.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<PaymentType>>>>().expect("ClassCastException").clone()) };
            if !jcall!(a_PaymentTypeVector).contains(crate::java_into_object(paymentType.clone())) {
                jcall!(diff).add(crate::java_into_object(paymentType.clone()));
            }
        }
        return diff.clone();
    }
}

