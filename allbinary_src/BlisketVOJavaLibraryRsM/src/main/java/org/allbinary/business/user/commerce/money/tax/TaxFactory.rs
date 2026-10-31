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
// package org.allbinary.business.user.commerce.money.tax
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::business::DynamicObjectData::DynamicObjectData;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontInterface::StoreFrontInterface;
use crate::org::allbinary::business::user::commerce::money::tax::components::TaxData::TaxData;
use crate::org::allbinary::business::user::commerce::money::tax::modules::SimpleStateTaxModule::SimpleStateTaxModule;
use crate::org::allbinary::business::user::commerce::money::tax::modules::TaxModuleInterface::TaxModuleInterface;
use crate::org::allbinary::data::tree::dom::DomNodeHelper::DomNodeHelper;
use crate::org::allbinary::data::tree::dom::DomSearchHelper::DomSearchHelper;
use crate::org::allbinary::data::tree::dom::document::DomDocumentHelper::DomDocumentHelper;
use crate::org::allbinary::globals::FREEBLISKET_PATH_GLOBALS::FREEBLISKET_PATH_GLOBALS;
use crate::org::allbinary::globals::URLGLOBALS::URLGLOBALS;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::control::crypt::file::CryptFileReader::CryptFileReader;
use crate::org::allbinary::logic::io::path::AbPath::AbPath;
use crate::org::allbinary::logic::io::path::AbPathData::AbPathData;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::system::loader::AbeFactory::AbeFactory;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::logic::visual::transform::info::objectConfig::TransformInfoObjectConfigData::TransformInfoObjectConfigData;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use crate::org::w3c::dom::NodeList::NodeList;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TaxFactory {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub TAXATIONFILENAME: String,
}

thread_local! { pub static TaxFactory_instance: Option<Rc<RefCell<TaxFactory>>> = TaxFactory::new(); }

impl TaxFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<TaxFactory>>> {
        return TaxFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            TAXATIONFILENAME: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.TAXATIONFILENAME = String::from("taxationMethod.xml");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn getInstance2(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>>) -> Option<Rc<RefCell<dyn TaxModuleInterface>>> {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(URLGLOBALS::getMainPath());
        jcall!(stringBuffer).append(jcall!(FREEBLISKET_PATH_GLOBALS::getInstance()).XSLPATH.clone());
        jcall!(stringBuffer).append(jcall!(storeFrontInterface).getName());
        jcall!(stringBuffer).append(jcall!(AbPathData::getInstance()).SEPARATOR.clone());
        jcall!(stringBuffer).append(String::from("generic"));
        jcall!(stringBuffer).append(jcall!(AbPathData::getInstance()).SEPARATOR.clone());
        jcall!(stringBuffer).append(String::from("taxes"));
        jcall!(stringBuffer).append(jcall!(AbPathData::getInstance()).SEPARATOR.clone());
        let mut abPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> = AbPath::new(jcall!(stringBuffer).toString(), self.TAXATIONFILENAME.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
        let mut transformInfoObjectConfigData: Option<Rc<RefCell<TransformInfoObjectConfigData>>> = TransformInfoObjectConfigData::getInstance();
        let mut data: String = jcall!(CryptFileReader::new(jcall!(transformInfoObjectConfigData).UNCRYPTED_EXTENSION.clone(), jcall!(transformInfoObjectConfigData).ENCRYPTED_EXTENSION.clone())).get(abPath.clone());
        let mut document: Option<Rc<RefCell<Document>>> = DomDocumentHelper::create1(data.clone());
        let mut taxNameNodeList: Option<Rc<RefCell<NodeList>>> = jcall!(document).getElementsByTagName(crate::org::allbinary::business::user::commerce::money::tax::components::TaxData::TaxData_NAME.with(Clone::clone).clone());
        for index in (0..jcall!(taxNameNodeList).getLength()) {
            let mut node: Option<Rc<RefCell<Node>>> = jcall!(taxNameNodeList).item(index.clone());
            let mut nodeList: Option<Rc<RefCell<NodeList>>> = jcall!(node).getChildNodes();
            if nodeList.is_some() {
                let mut classNameNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(crate::org::allbinary::business::DynamicObjectData::DynamicObjectData_NAME.with(Clone::clone).clone(), nodeList.clone());
                if classNameNode.is_some() {
                    let mut className: String = DomNodeHelper::getTextNodeValue(classNameNode.clone());
                    return (jcall!(AbeFactory::getInstance()).getInstance2(abeClientInformation.clone(), className.clone())).clone() /* as TaxModuleInterface */;
                } else {
                    if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).TAX.clone()) {
                        jcall!(self.logUtil).putF(String::from("Class Node Null"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).GET_INSTANCE.clone());
                    }
                }
            } else {
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).TAX.clone()) {
                    jcall!(self.logUtil).putF(String::from("Tax Name Node Node Children"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).GET_INSTANCE.clone());
                }
            }
        }
        return (SimpleStateTaxModule::new()).clone() /* as TaxModuleInterface */;
    }
}

