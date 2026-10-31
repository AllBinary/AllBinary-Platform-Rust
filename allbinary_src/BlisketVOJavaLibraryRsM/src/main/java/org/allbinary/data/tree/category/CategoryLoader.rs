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
// package org.allbinary.data.tree.category
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::business::category::CategoryData::CategoryData;
use crate::org::allbinary::business::category::CategoryFactoryInterface::CategoryFactoryInterface;
use crate::org::allbinary::business::category::CategoryInterface::CategoryInterface;
use crate::org::allbinary::business::category::properties::CategoryProperties::CategoryProperties;
use crate::org::allbinary::business::category::properties::CategoryPropertiesInterface::CategoryPropertiesInterface;
use crate::org::allbinary::data::tree::dom::DomSearchHelper::DomSearchHelper;
use crate::org::allbinary::data::tree::dom::document::DomDocumentHelper::DomDocumentHelper;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::control::crypt::file::CryptFileReader::CryptFileReader;
use crate::org::allbinary::logic::io::file::AbFile::AbFile;
use crate::org::allbinary::logic::io::path::AbPath::AbPath;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use crate::org::w3c::dom::NodeList::NodeList;
use super::CategoryModifierTree::CategoryModifierTree;
use super::CategoryLoaderInterface::CategoryLoaderInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CategoryLoader {
    pub base: CategoryModifierTree,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

// impl CategoryLoaderInterface for CategoryLoader with the methods of the trait

impl CategoryLoader {
    

    pub fn new_struct(mut categoryFactoryInterface: Option<Rc<RefCell<dyn CategoryFactoryInterface>>>) -> Self {
        let mut this = Self {
            base: CategoryModifierTree::new_struct(categoryFactoryInterface.clone()),
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).CATEGORY.clone()) {
            jcall!(this.logUtil).putF(jcall!(this.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("CategoryLoader(CategoryFactoryInterface categoryFactoryInterface)"));
        }
        this
    }

    pub fn new(mut categoryFactoryInterface: Option<Rc<RefCell<dyn CategoryFactoryInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(categoryFactoryInterface))))
    }
    

    // synchronized
    pub fn getDoc(&mut self, mut categoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>) -> Option<Rc<RefCell<Document>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if jcall!(jcall!(categoryInterface).getProperties()).getPath(jcall!(categoryInterface).getHierarchy()).is_some() {
                if jcall!(jcall!(categoryInterface).getHierarchy()).getLevel() == 1 {
                    let mut rootAbFile: Option<Rc<RefCell<AbFile>>> = AbFile::createAbFileFromAbPath(jcall!(categoryInterface).getRootFilePath());
                    if !jcall!(rootAbFile).isDirectory() {
                        jcall!(rootAbFile).mkdirs();
                        if !jcall!(rootAbFile).isDirectory() {
                            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Could Not Create Directory"))));
                        }
                        self.save(categoryInterface.clone());
                    }
                }
                let mut fileAbPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> = jcall!(categoryInterface).getFilePath();
                let mut cryptFileReader: Option<Rc<RefCell<CryptFileReader>>> = CryptFileReader::new(jcall!(CategoryData::getInstance()).UNCRYPTED_EXTENSION.clone(), jcall!(CategoryData::getInstance()).ENCRYPTED_EXTENSION.clone());
                let mut document: Option<Rc<RefCell<Document>>> = DomDocumentHelper::create1(jcall!(cryptFileReader).get(fileAbPath.clone()));
                return document.clone();
            } else {
                panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Null Category"))));
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    jcall!(categoryInterface).log();
                })) {
                    Ok(_) => {}
                    Err(e2) => {
            let e2: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e2 as Box<dyn std::any::Any>)));

                    jcall!(self.logUtil).put(String::from("Could Not Log Category"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getDoc"), crate::java_into_object(e.clone()));
                    }
                }
                jcall!(self.logUtil).put(jcall!(self.commonStrings).FAILURE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getDoc"), crate::java_into_object(e.clone()));
            }
            return None;
            }
        }
    }
    

    // synchronized
    pub fn get(&mut self, mut categoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>) -> Option<Rc<RefCell<dyn CategoryInterface>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut document: Option<Rc<RefCell<Document>>> = self.getDoc(categoryInterface.clone());
            let mut categoryNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(jcall!(CategoryData::getInstance()).NAME.clone(), jcall!(document).getChildNodes());
            return { let java_arguments = (categoryInterface.clone(), jcall!(categoryNode).getChildNodes(),); self.addProperties(java_arguments.0, java_arguments.1) };
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).FAILURE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).GET.clone(), crate::java_into_object(e.clone()));
            }
            return None;
            }
        }
    }
    

    // Java overloaded method: Rust needs a different name
    // synchronized
    pub fn get(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<dyn CategoryInterface>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut categoryNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(jcall!(CategoryData::getInstance()).NAME.clone(), jcall!(document).getChildNodes());
            let mut loadedCategoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>> = jcall!(self.categoryFactoryInterface).getRootInstanceFromNode(categoryNode.clone());
            return { let java_arguments = (loadedCategoryInterface.clone(), jcall!(categoryNode).getChildNodes(),); self.addProperties(java_arguments.0, java_arguments.1) };
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).FAILURE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).GET.clone(), crate::java_into_object(e.clone()));
            }
            return None;
            }
        }
    }
    

    // synchronized
    pub fn getAll(&mut self, mut categoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>) -> Option<Rc<RefCell<dyn CategoryInterface>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut document: Option<Rc<RefCell<Document>>> = self.getDoc(categoryInterface.clone());
            return self.getAll(document.clone());
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).FAILURE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).GET.clone(), crate::java_into_object(e.clone()));
            }
            return None;
            }
        }
    }
    

    // Java overloaded method: Rust needs a different name
    // synchronized
    pub fn getAll(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<dyn CategoryInterface>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut categoryNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(jcall!(CategoryData::getInstance()).NAME.clone(), jcall!(document).getChildNodes());
            let mut loadedCategoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>> = jcall!(self.categoryFactoryInterface).getRootInstanceFromNode(categoryNode.clone());
            return { let java_arguments = (loadedCategoryInterface.clone(), jcall!(categoryNode).getChildNodes(),); self.addProperties(java_arguments.0, java_arguments.1) };
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).FAILURE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).GET.clone(), crate::java_into_object(e.clone()));
            }
            return None;
            }
        }
    }
    

    // Java overloaded method: Rust needs a different name
    // synchronized
    pub fn getAll2(&mut self, mut parentCategoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>, mut categoryNodeList: Option<Rc<RefCell<NodeList>>>) -> Option<Rc<RefCell<dyn CategoryInterface>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if categoryNodeList.is_some() {
                for index in (0..jcall!(categoryNodeList).getLength()) {
                    let mut categoryNode: Option<Rc<RefCell<Node>>> = jcall!(categoryNodeList).item(index.clone());
                    if categoryNode.is_some() && jcall!(jcall!(categoryNode).getNodeName()).compareTo(jcall!(CategoryData::getInstance()).NAME.clone()) == 0 {
                        let mut categoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>> = ({ let java_receiver = self.categoryFactoryInterface.clone(); let java_arguments = (jcall!(jcall!(parentCategoryInterface).getHierarchy()).getRoot(), parentCategoryInterface.clone(), categoryNode.clone(),); let java_result = jcall!(java_receiver).getInstance(java_arguments.0, java_arguments.1, java_arguments.2); java_result }).clone() /* as CategoryInterface */;
                        jcall!(parentCategoryInterface).addChild(categoryInterface.clone());
                    }
                }
            }
            return parentCategoryInterface.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).FAILURE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).GET.clone(), crate::java_into_object(e.clone()));
            }
            return None;
            }
        }
    }
    

    // synchronized
    pub fn addProperties(&mut self, mut loadedCategoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>, mut categoryNodeList: Option<Rc<RefCell<NodeList>>>) -> Option<Rc<RefCell<dyn CategoryInterface>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if categoryNodeList.is_some() {
                for index in (0..jcall!(categoryNodeList).getLength()) {
                    let mut categoryNode: Option<Rc<RefCell<Node>>> = jcall!(categoryNodeList).item(index.clone());
                    if categoryNode.is_some() && jcall!(jcall!(categoryNode).getNodeName()).compareTo(jcall!(CategoryData::getInstance()).NAME.clone()) == 0 {
                        let mut categoryPropertiesInterface: Option<Rc<RefCell<dyn CategoryPropertiesInterface>>> = (CategoryProperties::new1(categoryNode.clone())).clone() /* as CategoryPropertiesInterface */;
                        jcall!(loadedCategoryInterface).addChildProperty(categoryPropertiesInterface.clone());
                    }
                }
            }
            return loadedCategoryInterface.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).FAILURE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).GET.clone(), crate::java_into_object(e.clone()));
            }
            return None;
            }
        }
    }
}
impl std::ops::Deref for CategoryLoader {
    type Target = CategoryModifierTree;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for CategoryLoader {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::data::tree::category::CategoryModifierTree::CategoryModifierTreeDyn for CategoryLoader {
    fn as_CategoryModifierTree(&self) -> &crate::org::allbinary::data::tree::category::CategoryModifierTree::CategoryModifierTree { &self.base }
    fn as_CategoryModifierTree_mut(&mut self) -> &mut crate::org::allbinary::data::tree::category::CategoryModifierTree::CategoryModifierTree { &mut self.base }
}
impl crate::org::allbinary::data::tree::category::CategoryPrivateTree::CategoryPrivateTreeDyn for CategoryLoader {
    fn as_CategoryPrivateTree(&self) -> &crate::org::allbinary::data::tree::category::CategoryPrivateTree::CategoryPrivateTree { &self.base.base }
    fn as_CategoryPrivateTree_mut(&mut self) -> &mut crate::org::allbinary::data::tree::category::CategoryPrivateTree::CategoryPrivateTree { &mut self.base.base }
}


