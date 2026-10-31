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
// package org.allbinary.business.category.properties.root.store
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
// use crate::javax::servlet::http::HttpServletRequest::HttpServletRequest;
use crate::org::allbinary::business::category::CategoryData::CategoryData;
use crate::org::allbinary::business::category::CategoryUtil::CategoryUtil;
use crate::org::allbinary::business::category::hierarchy::CategoryHierarchyInterface::CategoryHierarchyInterface;
use crate::org::allbinary::business::category::properties::CategoryPropertiesInterface::CategoryPropertiesInterface;
use crate::org::allbinary::business::category::properties::root::RootCategoryPropertiesInterface::RootCategoryPropertiesInterface;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontFactory::StoreFrontFactory;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontInterface::StoreFrontInterface;
use crate::org::allbinary::globals::URLGLOBALS::URLGLOBALS;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::io::path::AbPath::AbPath;
use crate::org::allbinary::logic::io::path::AbPathData::AbPathData;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpInterface::TransformInfoHttpInterface;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RootStoreCategoryProperties {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub pathData: Option<Rc<RefCell<AbPathData>>>,
    pub fileAbPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>,
    pub abPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>,
    pub category: String,
    pub isRealRoot: bool,
    pub webAppAbPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>,
    pub transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>,
}

impl RootCategoryPropertiesInterface for RootStoreCategoryProperties {
    fn setRootFilePath(&mut self, mut value: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>) { Self::setRootFilePath(self, value) }
    fn getRootFilePath(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> { Self::getRootFilePath(self) }
}
impl CategoryPropertiesInterface for RootStoreCategoryProperties {
    fn setValue(&mut self, mut value: String) { Self::setValue(self, value) }
    fn getValue(&mut self) -> String { Self::getValue(self) }
    fn getWebAppPath(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> { Self::getWebAppPath(self) }
    fn getPath(&mut self, mut categoryHierarchyInterface: Option<Rc<RefCell<dyn CategoryHierarchyInterface>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> { Self::getPath(self, categoryHierarchyInterface) }
    fn getFileName(&mut self) -> String { Self::getFileName(self) }
    fn isRoot(&mut self) -> bool { Self::isRoot(self) }
    fn isRealRoot(&mut self) -> bool { Self::isRealRoot(self) }
}
impl crate::org::allbinary::data::tables::TableMappingInterface::TableMappingInterface for RootStoreCategoryProperties {
    fn getKey(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { Self::getKey(self) }
    fn toVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Self::toVector(self) }
    fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> { Self::toHashMap(self) }
}
impl crate::org::allbinary::logic::control::validate::ValidationInterface::ValidationInterface for RootStoreCategoryProperties {
    fn isValid(&mut self) -> Option<bool> { Self::isValid(self) }
    fn toValidationInfoNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toValidationInfoNode(self, document) }
    fn toValidationInfoDoc(&mut self) -> Option<Rc<RefCell<Document>>> { Self::toValidationInfoDoc(self) }
    fn validationInfo(&mut self) -> String { Self::validationInfo(self) }
}

impl RootStoreCategoryProperties {
    

    pub fn new_struct1(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            pathData: None,
            fileAbPath: None,
            abPath: None,
            category: String::new(),
            isRealRoot: false,
            webAppAbPath: None,
            transformInfoInterface: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.pathData = AbPathData::getInstance();
        this.transformInfoInterface = transformInfoInterface.clone();
        this.abPath = AbPath::createAbPath();
        this.category = jcall!(CategoryData::getInstance()).ROOTCATEGORY.clone();
        this.isRealRoot = true;
        this.initPath();
        this.log();
        this
    }

    pub fn new1(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(transformInfoInterface))))
    }
    

    pub fn new_struct2(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut categoryAbPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            pathData: None,
            fileAbPath: None,
            abPath: None,
            category: String::new(),
            isRealRoot: false,
            webAppAbPath: None,
            transformInfoInterface: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.pathData = AbPathData::getInstance();
        this.transformInfoInterface = transformInfoInterface.clone();
        this.abPath = categoryAbPath.clone();
        this.category = { let java_receiver = this.pathData.clone(); let java_arguments = (jcall!(categoryAbPath).toString(),); let java_result = jcall!(java_receiver).getNameFromPath(java_arguments.0); java_result };
        if jcall!(StringValidationUtil::getInstance()).isEmpty(this.category.clone()) {
            this.isRealRoot = true;
            this.category = jcall!(CategoryData::getInstance()).ROOTCATEGORY.clone();
        }
        this.initPath();
        this.log();
        this
    }

    pub fn new2(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut categoryAbPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2(transformInfoInterface, categoryAbPath))))
    }
    

    pub fn new_struct2(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut node: Option<Rc<RefCell<Node>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            pathData: None,
            fileAbPath: None,
            abPath: None,
            category: String::new(),
            isRealRoot: false,
            webAppAbPath: None,
            transformInfoInterface: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.pathData = AbPathData::getInstance();
        this.transformInfoInterface = transformInfoInterface.clone();
        let mut categoryPath: String = CategoryUtil::getNameFromNode(node.clone());
        this.abPath = AbPath::new(categoryPath.clone(), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
        this.category = jcall!(this.pathData).getNameFromPath(categoryPath.clone());
        if jcall!(StringValidationUtil::getInstance()).isEmpty(this.category.clone()) {
            this.isRealRoot = true;
            this.category = jcall!(CategoryData::getInstance()).ROOTCATEGORY.clone();
        }
        this.initPath();
        this.log();
        this
    }

    pub fn new2(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2(transformInfoInterface, node))))
    }
    

    pub fn new_struct2(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut categoryPropertiesHashMap: Option<Rc<RefCell<HashMap>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            pathData: None,
            fileAbPath: None,
            abPath: None,
            category: String::new(),
            isRealRoot: false,
            webAppAbPath: None,
            transformInfoInterface: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.pathData = AbPathData::getInstance();
        this.transformInfoInterface = transformInfoInterface.clone();
        let mut categoryPath: String = (jcall!(categoryPropertiesHashMap).get(jcall!(CategoryData::getInstance()).NAME.clone())).clone() /* as String */.clone();
        this.abPath = AbPath::new(categoryPath.clone(), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
        this.category = jcall!(this.pathData).getNameFromPath(categoryPath.clone());
        if jcall!(StringValidationUtil::getInstance()).isEmpty(this.category.clone()) {
            this.isRealRoot = true;
            this.category = jcall!(CategoryData::getInstance()).ROOTCATEGORY.clone();
        }
        this.initPath();
        this.log();
        this
    }

    pub fn new2(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut categoryPropertiesHashMap: Option<Rc<RefCell<HashMap>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2(transformInfoInterface, categoryPropertiesHashMap))))
    }
    

    // throws Exception
    pub fn initPath(&mut self) {
        let mut transformInfoHttpStoreInterface: Option<Rc<RefCell<dyn TransformInfoHttpInterface>>> = (self.transformInfoInterface).clone() /* as TransformInfoHttpInterface */;
        let mut storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>> = StoreFrontFactory::getInstance(jcall!(transformInfoHttpStoreInterface).getStoreName());
        let mut postPath: String = jcall!(storeFrontInterface).getCurrentHostNamePath() + jcall!(storeFrontInterface).getCategoryPath();
        let mut httpServletRequest: Option<Rc<RefCell<HttpServletRequest>>> = (jcall!(jcall!(transformInfoHttpStoreInterface).getPageContext()).getRequest()).clone() /* as HttpServletRequest */;
        self.webAppAbPath = AbPath::new(format!("{}{}", jcall!(httpServletRequest).getContextPath(), postPath), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
        { let java_arguments = (AbPath::new(format!("{}{}", URLGLOBALS::getMainPath(), postPath), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>),); self.setRootFilePath(java_arguments.0) };
    }
    

    pub fn isRealRoot(&mut self) -> bool {
        return self.isRealRoot.clone();
    }
    

    pub fn isRoot(&mut self) -> bool {
        return true;
    }
    

    pub fn getKey(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return (self.getValue()).clone() /* as Any */;
    }
    

    pub fn setPath(&mut self, mut path: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>) {
        self.abPath = path.clone();
    }
    

    pub fn setRootFilePath(&mut self, mut value: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>) {
        self.fileAbPath = value.clone();
    }
    

    pub fn getRootFilePath(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> {
        return self.fileAbPath.clone();
    }
    

    pub fn getValue(&mut self) -> String {
        return self.category.clone();
    }
    

    // throws Exception
    pub fn getWebAppPath(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> {
        return self.webAppAbPath.clone();
    }
    

    pub fn getPath(&mut self, mut categoryHierarchyInterface: Option<Rc<RefCell<dyn CategoryHierarchyInterface>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> {
        return self.abPath.clone();
    }
    

    pub fn getFileName(&mut self) -> String {
        return format!("{}{}", format!("{}{}", self.getValue(), jcall!(AbPathData::getInstance()).EXTENSION_SEP), jcall!(CategoryData::getInstance()).UNCRYPTED_EXTENSION);
    }
    

    pub fn setValue(&mut self, mut value: String) {
        self.category = value.clone();
    }
    

    pub fn isValid(&mut self) -> Option<bool> {
        return true;
    }
    

    pub fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> {
        let mut categoryHashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        jcall!(categoryHashMap).put(jcall!(CategoryData::getInstance()).NAME.clone(), self.getValue());
        return categoryHashMap.clone();
    }
    

    pub fn toVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut categoryVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        jcall!(categoryVector).add(crate::java_into_object(self.getValue()));
        return categoryVector.clone();
    }
    

    // throws Exception
    pub fn toValidationInfoDoc(&mut self) -> Option<Rc<RefCell<Document>>> {
        return None;
    }
    

    // throws Exception
    pub fn toValidationInfoNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        return None;
    }
    

    // throws Exception
    pub fn validationInfo(&mut self) -> String {
        return None;
    }
    

    pub fn log(&mut self) {
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).CATEGORY.clone()) {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("filePath = ")).append(jcall!(self.fileAbPath).toString()).append(String::from("\npath = ")).append(jcall!(self.abPath).toString()).append(String::from("\ncategory = ")).append(self.category.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("log()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
    }
}
pub trait RootStoreCategoryPropertiesDyn: crate::org::allbinary::business::category::properties::root::RootCategoryPropertiesInterface::RootCategoryPropertiesInterface {
    fn as_RootStoreCategoryProperties(&self) -> &RootStoreCategoryProperties;
    fn as_RootStoreCategoryProperties_mut(&mut self) -> &mut RootStoreCategoryProperties;
    fn initPath(&mut self) { RootStoreCategoryProperties::initPath(self.as_RootStoreCategoryProperties_mut()) }
    fn isRealRoot(&mut self) -> bool { RootStoreCategoryProperties::isRealRoot(self.as_RootStoreCategoryProperties_mut()) }
    fn isRoot(&mut self) -> bool { RootStoreCategoryProperties::isRoot(self.as_RootStoreCategoryProperties_mut()) }
    fn getKey(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { RootStoreCategoryProperties::getKey(self.as_RootStoreCategoryProperties_mut()) }
    fn setPath(&mut self, mut path: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>) { RootStoreCategoryProperties::setPath(self.as_RootStoreCategoryProperties_mut(), path) }
    fn getValue(&mut self) -> String { RootStoreCategoryProperties::getValue(self.as_RootStoreCategoryProperties_mut()) }
    fn getWebAppPath(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> { RootStoreCategoryProperties::getWebAppPath(self.as_RootStoreCategoryProperties_mut()) }
    fn getPath(&mut self, mut categoryHierarchyInterface: Option<Rc<RefCell<dyn CategoryHierarchyInterface>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> { RootStoreCategoryProperties::getPath(self.as_RootStoreCategoryProperties_mut(), categoryHierarchyInterface) }
    fn getFileName(&mut self) -> String { RootStoreCategoryProperties::getFileName(self.as_RootStoreCategoryProperties_mut()) }
    fn setValue(&mut self, mut value: String) { RootStoreCategoryProperties::setValue(self.as_RootStoreCategoryProperties_mut(), value) }
    fn isValid(&mut self) -> Option<bool> { RootStoreCategoryProperties::isValid(self.as_RootStoreCategoryProperties_mut()) }
    fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> { RootStoreCategoryProperties::toHashMap(self.as_RootStoreCategoryProperties_mut()) }
    fn toVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { RootStoreCategoryProperties::toVector(self.as_RootStoreCategoryProperties_mut()) }
    fn toValidationInfoDoc(&mut self) -> Option<Rc<RefCell<Document>>> { RootStoreCategoryProperties::toValidationInfoDoc(self.as_RootStoreCategoryProperties_mut()) }
    fn toValidationInfoNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { RootStoreCategoryProperties::toValidationInfoNode(self.as_RootStoreCategoryProperties_mut(), document) }
    fn validationInfo(&mut self) -> String { RootStoreCategoryProperties::validationInfo(self.as_RootStoreCategoryProperties_mut()) }
    fn log(&mut self) { RootStoreCategoryProperties::log(self.as_RootStoreCategoryProperties_mut()) }
}
impl RootStoreCategoryPropertiesDyn for RootStoreCategoryProperties {
    fn as_RootStoreCategoryProperties(&self) -> &RootStoreCategoryProperties { self }
    fn as_RootStoreCategoryProperties_mut(&mut self) -> &mut RootStoreCategoryProperties { self }
}


