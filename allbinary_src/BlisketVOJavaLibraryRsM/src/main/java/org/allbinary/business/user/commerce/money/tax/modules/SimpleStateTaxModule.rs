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
// package org.allbinary.business.user.commerce.money.tax.modules
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontInterface::StoreFrontInterface;
use crate::org::allbinary::business::user::address::StreetAddress::StreetAddress;
use crate::org::allbinary::business::user::address::StreetAddressData::StreetAddressData;
use crate::org::allbinary::business::user::commerce::money::tax::components::TaxData::TaxData;
use crate::org::allbinary::data::tree::dom::DomNodeHelper::DomNodeHelper;
use crate::org::allbinary::data::tree::dom::DomSearchHelper::DomSearchHelper;
use crate::org::allbinary::data::tree::dom::document::DomDocumentHelper::DomDocumentHelper;
use crate::org::allbinary::data::tree::dom::document::XmlDocumentHelper::XmlDocumentHelper;
use crate::org::allbinary::globals::FREEBLISKET_PATH_GLOBALS::FREEBLISKET_PATH_GLOBALS;
use crate::org::allbinary::globals::URLGLOBALS::URLGLOBALS;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::control::crypt::file::CryptFileReader::CryptFileReader;
use crate::org::allbinary::logic::io::path::AbPath::AbPath;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use crate::org::allbinary::logic::visual::transform::info::objectConfig::TransformInfoObjectConfigData::TransformInfoObjectConfigData;
use crate::org::allbinary::string::CommonPhoneStrings::CommonPhoneStrings;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use crate::org::w3c::dom::NodeList::NodeList;
use super::TaxModuleInterface::TaxModuleInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SimpleStateTaxModule {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub streetAddress: Option<Rc<RefCell<StreetAddress>>>,
    pub storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>>,
    pub ALL: String,
    pub STATETAXPATH: String,
    pub STATETAXFILE: String,
    pub document: Option<Rc<RefCell<Document>>>,
}

impl TaxModuleInterface for SimpleStateTaxModule {
    fn getTaxRate(&mut self, mut streetAddress: Option<Rc<RefCell<StreetAddress>>>, mut storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>>) -> Option<f32> { Self::getTaxRate(self, streetAddress, storeFrontInterface) }
    fn isValid(&mut self, mut streetAddress: Option<Rc<RefCell<StreetAddress>>>, mut storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>>) -> Option<bool> { Self::isValid(self, streetAddress, storeFrontInterface) }
}

impl SimpleStateTaxModule {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            streetAddress: None,
            storeFrontInterface: None,
            ALL: String::new(),
            STATETAXPATH: String::new(),
            STATETAXFILE: String::new(),
            document: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.ALL = jcall!(CommonPhoneStrings::getInstance()).STAR.clone();
        this.STATETAXPATH = String::from("/generic/taxes/");
        this.STATETAXFILE = String::from("stateTaxation.xml");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn getTaxRate(&mut self, mut streetAddress: Option<Rc<RefCell<StreetAddress>>>, mut storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>>) -> Option<f32> {
        self.streetAddress = streetAddress.clone();
        self.storeFrontInterface = storeFrontInterface.clone();
        let mut fileAbPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> = AbPath::new(format!("{}{}", format!("{}{}", format!("{}{}", URLGLOBALS::getMainPath(), jcall!(FREEBLISKET_PATH_GLOBALS::getInstance()).XSLPATH), jcall!(self.storeFrontInterface).getName()), self.STATETAXPATH), self.STATETAXFILE.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
        let mut data: String = jcall!(CryptFileReader::new(jcall!(TransformInfoObjectConfigData::getInstance()).UNCRYPTED_EXTENSION.clone(), jcall!(TransformInfoObjectConfigData::getInstance()).ENCRYPTED_EXTENSION.clone())).get(fileAbPath.clone());
        self.document = DomDocumentHelper::create1(data.clone());
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).TAX.clone()) {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Tax Doc: "), XmlDocumentHelper::toString(self.document.clone())), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getTaxRate"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        let mut state: String = jcall!(self.streetAddress).getState();
        let mut country: String = jcall!(self.streetAddress).getCountry();
        state = state.to_uppercase();
        let mut nodeList: Option<Rc<RefCell<NodeList>>> = jcall!(self.document).getElementsByTagName(crate::org::allbinary::business::user::commerce::money::tax::components::TaxData::TaxData_NAME.with(Clone::clone).clone());
        for index in (0..jcall!(nodeList).getLength()) {
            let mut node: Option<Rc<RefCell<Node>>> = jcall!(nodeList).item(index.clone());
            let mut taxNodeChildren: Option<Rc<RefCell<NodeList>>> = jcall!(node).getChildNodes();
            let mut streetAddressNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(crate::org::allbinary::business::user::address::StreetAddressData::StreetAddressData_NAME.with(Clone::clone).clone(), taxNodeChildren.clone());
            let mut taxableStreetAddress: Option<Rc<RefCell<StreetAddress>>> = StreetAddress::new1(streetAddressNode.clone());
            let mut stringValidationUtil: Option<Rc<RefCell<StringValidationUtil>>> = StringValidationUtil::getInstance();
            if !jcall!(stringValidationUtil).isEmpty(country.clone()) && !jcall!(stringValidationUtil).isEmpty(jcall!(taxableStreetAddress).getCountry()) && (country.cmp(&jcall!(taxableStreetAddress).getCountry()) as i32) == 0 {
                if true && jcall!(taxableStreetAddress).getState().is_some() && (state.cmp(&jcall!(taxableStreetAddress).getState()) as i32) == 0 {
                    let mut rateNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(crate::org::allbinary::business::user::commerce::money::tax::components::TaxData::TaxData_RATE.with(Clone::clone).clone(), taxNodeChildren.clone());
                    let mut taxRate: String = DomNodeHelper::getTextNodeValue(rateNode.clone());
                    let mut taxRateFloat: Option<f32> = Some((((((Some((taxRate.clone()) as f32)).unwrap() as f32)) as f64) * .01) as f32);
                    return taxRateFloat.clone();
                }
            }
        }
        return Some((0) as f32);
    }
    

    pub fn isValid(&mut self, mut streetAddress: Option<Rc<RefCell<StreetAddress>>>, mut storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>>) -> Option<bool> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.streetAddress = streetAddress.clone();
            self.storeFrontInterface = storeFrontInterface.clone();
            let mut fileAbPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> = AbPath::new(format!("{}{}", format!("{}{}", format!("{}{}", URLGLOBALS::getMainPath(), jcall!(FREEBLISKET_PATH_GLOBALS::getInstance()).XSLPATH), jcall!(self.storeFrontInterface).getName()), self.STATETAXPATH), self.STATETAXFILE.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
            let mut data: String = jcall!(CryptFileReader::new(jcall!(TransformInfoObjectConfigData::getInstance()).UNCRYPTED_EXTENSION.clone(), jcall!(TransformInfoObjectConfigData::getInstance()).ENCRYPTED_EXTENSION.clone())).get(fileAbPath.clone());
            self.document = DomDocumentHelper::create1(data.clone());
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).TAX.clone()) {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Tax Doc: "), XmlDocumentHelper::toString(self.document.clone())), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).IS_VALID.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            let mut country: String = jcall!(self.streetAddress).getCountry();
            let mut rootNode: Option<Rc<RefCell<Node>>> = jcall!({ let java_receiver = self.document.clone(); let java_arguments = (jcall!(TransformInfoObjectConfigData::getInstance()).NAME.clone(),); let java_result = jcall!(java_receiver).getElementsByTagName(java_arguments.0); java_result }).item(0);
            let mut nodeList: Option<Rc<RefCell<NodeList>>> = jcall!(self.document).getElementsByTagName(crate::org::allbinary::business::user::commerce::money::tax::components::TaxData::TaxData_NAME.with(Clone::clone).clone());
            for index in (0..jcall!(nodeList).getLength()) {
                let mut node: Option<Rc<RefCell<Node>>> = jcall!(nodeList).item(index.clone());
                let mut taxNodeChildren: Option<Rc<RefCell<NodeList>>> = jcall!(node).getChildNodes();
                let mut streetAddressNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(crate::org::allbinary::business::user::address::StreetAddressData::StreetAddressData_NAME.with(Clone::clone).clone(), taxNodeChildren.clone());
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).TAX.clone()) {
                    let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                    jcall!(stringBuffer).append(String::from("StreetAddress Node: "));
                    jcall!(stringBuffer).append(jcall!(streetAddressNode).getNodeName());
                    jcall!(stringBuffer).append(String::from(" Value: "));
                    jcall!(stringBuffer).append(jcall!(streetAddressNode).getNodeValue());
                    jcall!(stringBuffer).append(String::from(" Number Of Children: "));
                    jcall!(stringBuffer).appendint(((jcall!(jcall!(streetAddressNode).getChildNodes()).getLength()) as i32));
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).IS_VALID.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                }
                let mut taxableStreetAddress: Option<Rc<RefCell<StreetAddress>>> = StreetAddress::new1(streetAddressNode.clone());
                let mut stringValidationUtil: Option<Rc<RefCell<StringValidationUtil>>> = StringValidationUtil::getInstance();
                if !jcall!(stringValidationUtil).isEmpty(country.clone()) && !jcall!(stringValidationUtil).isEmpty(jcall!(taxableStreetAddress).getCountry()) && jcall!(jcall!(taxableStreetAddress).getCountry()).compareTo(self.ALL.clone()) == 0 {
                    return true;
                }
                if !jcall!(stringValidationUtil).isEmpty(country.clone()) && !jcall!(stringValidationUtil).isEmpty(jcall!(taxableStreetAddress).getCountry()) && (country.cmp(&jcall!(taxableStreetAddress).getCountry()) as i32) == 0 {
                    let mut state: String = jcall!(self.streetAddress).getState();
                    if true {
                        return true;
                    }
                }
            }
            return false;
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEWERROR.clone()) {
                jcall!(self.logUtil).put(String::from("Failed to validate"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).IS_VALID.clone(), crate::java_into_object(e.clone()));
            }
            return false;
            }
        }
    }
}

