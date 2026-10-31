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
// package org.allbinary.business.user.commerce.shipping
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::business::DynamicObjectData::DynamicObjectData;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontInterface::StoreFrontInterface;
use crate::org::allbinary::business::user::commerce::shipping::modules::BasicWeightShippingModuleView::BasicWeightShippingModuleView;
use crate::org::allbinary::business::user::commerce::shipping::modules::NoShippingModuleView::NoShippingModuleView;
use crate::org::allbinary::business::user::commerce::shipping::modules::ShippingInterface::ShippingInterface;
use crate::org::allbinary::data::tree::dom::DomNodeHelper::DomNodeHelper;
use crate::org::allbinary::data::tree::dom::DomSearchHelper::DomSearchHelper;
use crate::org::allbinary::data::tree::dom::document::DomDocumentHelper::DomDocumentHelper;
use crate::org::allbinary::globals::FREEBLISKET_PATH_GLOBALS::FREEBLISKET_PATH_GLOBALS;
use crate::org::allbinary::globals::URLGLOBALS::URLGLOBALS;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::control::crypt::file::CryptFileReader::CryptFileReader;
use crate::org::allbinary::logic::io::path::AbPath::AbPath;
use crate::org::allbinary::logic::io::path::AbPathData::AbPathData;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::system::loader::AbeFactory::AbeFactory;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::logic::visual::transform::info::objectConfig::TransformInfoObjectConfigData::TransformInfoObjectConfigData;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use crate::org::w3c::dom::NodeList::NodeList;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ShippingMethodsFactory {
    pub defaultShippingMethodInterface: Option<Rc<RefCell<dyn ShippingInterface>>>,
    pub document: Option<Rc<RefCell<Document>>>,
    pub shippingVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}

thread_local! { pub static ShippingMethodsFactory_SHIPPINGMETHODSFILEPATHSTRING: String = String::new(); }
thread_local! { pub static ShippingMethodsFactory_SHIPPINGMETHODSFILENAME: String = String::from("shippingMethods.xml"); }

impl ShippingMethodsFactory {
    

    // Java static initializer: call this once at startup
    pub fn static_init() {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut sep: String = jcall!(AbPathData::getInstance()).SEPARATOR.clone();
        jcall!(stringBuffer).append(sep.clone());
        jcall!(stringBuffer).append(String::from("generic"));
        jcall!(stringBuffer).append(sep.clone());
        jcall!(stringBuffer).append(String::from("shipping"));
        jcall!(stringBuffer).append(sep.clone());
        ShippingMethodsFactory_SHIPPINGMETHODSFILEPATHSTRING.with(Clone::clone) = jcall!(stringBuffer).toString();
    }
    

    pub fn new_struct(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>>) -> Self {
        let mut this = Self {
            defaultShippingMethodInterface: None,
            document: None,
            shippingVector: None,
        };
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut sep: String = jcall!(AbPathData::getInstance()).SEPARATOR.clone();
        jcall!(stringBuffer).append(URLGLOBALS::getMainPath());
        jcall!(stringBuffer).append(jcall!(FREEBLISKET_PATH_GLOBALS::getInstance()).XSLPATH.clone());
        jcall!(stringBuffer).append(jcall!(storeFrontInterface).getName());
        jcall!(stringBuffer).append(sep.clone());
        jcall!(stringBuffer).append(ShippingMethodsFactory_SHIPPINGMETHODSFILEPATHSTRING.with(Clone::clone).clone());
        let mut abPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> = (AbPath::new(jcall!(stringBuffer).toString(), ShippingMethodsFactory_SHIPPINGMETHODSFILENAME.with(Clone::clone).clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>)).clone() /* as AbPath */;
        let mut data: String = jcall!(CryptFileReader::new(jcall!(TransformInfoObjectConfigData::getInstance()).UNCRYPTED_EXTENSION.clone(), jcall!(TransformInfoObjectConfigData::getInstance()).ENCRYPTED_EXTENSION.clone())).get(abPath.clone());
        this.document = DomDocumentHelper::create1(data.clone());
        this.shippingVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut nodeList: Option<Rc<RefCell<NodeList>>> = jcall!(this.document).getElementsByTagName(ShippingMethodsData_NAME.with(Clone::clone).clone());
        for index in (0..jcall!(nodeList).getLength()) {
            let mut node: Option<Rc<RefCell<Node>>> = jcall!(nodeList).item(index.clone());
            let mut shippingMethodNodeChildren: Option<Rc<RefCell<NodeList>>> = jcall!(node).getChildNodes();
            let mut shippingMethodNameNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(ShippingMethodData_NAME.with(Clone::clone).clone(), shippingMethodNodeChildren.clone());
            let mut classNameNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(crate::org::allbinary::business::DynamicObjectData::DynamicObjectData_NAME.with(Clone::clone).clone(), jcall!(shippingMethodNameNode).getChildNodes());
            let mut shippingMethodClassName: String = DomNodeHelper::getTextNodeValue(classNameNode.clone());
            let mut shippingMethodInterface: Option<Rc<RefCell<dyn ShippingInterface>>> = (jcall!(AbeFactory::getInstance()).getInstance2(abeClientInformation.clone(), shippingMethodClassName.clone())).clone() /* as ShippingInterface */;
            jcall!(this.shippingVector).add(crate::java_into_object(shippingMethodInterface.clone()));
            let mut defaultShippingMethodNameNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(ShippingMethodData_DEFAULT.with(Clone::clone).clone(), jcall!(shippingMethodNameNode).getChildNodes());
            let mut defaultShippingMethodNameNodeValue: String = DomNodeHelper::getTextNodeValue(defaultShippingMethodNameNode.clone());
            if true && (defaultShippingMethodNameNodeValue.cmp(&ShippingMethodData_DEFAULT.with(Clone::clone).clone()) as i32) == 0 {
                this.defaultShippingMethodInterface = shippingMethodInterface.clone();
            }
        }
        if jcall!(this.shippingVector).size() < 1 {
            this.defaultShippingMethodInterface = BasicWeightShippingModuleView::new().map(|reference| reference as Rc<RefCell<dyn ShippingInterface>>);
            jcall!(this.shippingVector).add(crate::java_into_object(this.defaultShippingMethodInterface.clone()));
            jcall!(this.shippingVector).add(crate::java_into_object(NoShippingModuleView::new()));
        }
        this
    }

    pub fn new(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(abeClientInformation, storeFrontInterface))))
    }
    

    // throws Exception
    pub fn getInstance(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.shippingVector.clone();
    }
    

    pub fn getDefaultInstance(&mut self) -> Option<Rc<RefCell<dyn ShippingInterface>>> {
        return self.defaultShippingMethodInterface.clone();
    }
}

