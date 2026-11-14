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
use crate::java::lang::Exception::*;
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
        if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).CATEGORY.clone(); java_field }.clone()) {
            jcall!(this.logUtil).putF({ let java_field = jcall!(this.commonStrings).START.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("CategoryLoader(CategoryFactoryInterface categoryFactoryInterface)"));
        }
        this
    }

    pub fn new(mut categoryFactoryInterface: Option<Rc<RefCell<dyn CategoryFactoryInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(categoryFactoryInterface))))
    }
    

    // synchronized
    pub fn getDoc(&mut self, mut categoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>) -> Option<Rc<RefCell<Document>>> {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
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
                let mut cryptFileReader: Option<Rc<RefCell<CryptFileReader>>> = CryptFileReader::new({ let java_field = jcall!(CategoryData::getInstance()).UNCRYPTED_EXTENSION.clone(); java_field }.clone(), { let java_field = jcall!(CategoryData::getInstance()).ENCRYPTED_EXTENSION.clone(); java_field }.clone());
                let mut document: Option<Rc<RefCell<Document>>> = DomDocumentHelper::create1(jcall!(cryptFileReader).get(fileAbPath.clone()));
                return Some(document.clone());
            } else {
                panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Null Category"))));
            }None
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGING.clone(); java_field }.clone()) {
                match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    jcall!(categoryInterface).log();
                })) {
                    Ok(_) => {}
                    Err(e2) => {
            let e2: String = crate::java::lang::Exception::java_panic_message(&e2);

                    jcall!(self.logUtil).put(String::from("Could Not Log Category"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getDoc"), crate::java_into_object(e.clone()));
                    }
                }
                jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).FAILURE.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getDoc"), crate::java_into_object(e.clone()));
            }
            return None;
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    // synchronized
    pub fn get(&mut self, mut categoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>) -> Option<Rc<RefCell<dyn CategoryInterface>>> {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut document: Option<Rc<RefCell<Document>>> = self.getDoc(categoryInterface.clone());
            let mut categoryNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode({ let java_field = jcall!(CategoryData::getInstance()).NAME.clone(); java_field }.clone(), jcall!(document).getChildNodes());
            return Some({ let java_arguments = (categoryInterface.clone(), jcall!(categoryNode).getChildNodes(),); self.addProperties(java_arguments.0, java_arguments.1) });
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGING.clone(); java_field }.clone()) {
                jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).FAILURE.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).GET.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
            return None;
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    // Java overloaded method: Rust needs a different name
    // synchronized
    pub fn get_Document(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<dyn CategoryInterface>>> {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut categoryNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode({ let java_field = jcall!(CategoryData::getInstance()).NAME.clone(); java_field }.clone(), jcall!(document).getChildNodes());
            let mut loadedCategoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>> = jcall!(self.categoryFactoryInterface).getRootInstanceFromNode(categoryNode.clone());
            return Some({ let java_arguments = (loadedCategoryInterface.clone(), jcall!(categoryNode).getChildNodes(),); self.addProperties(java_arguments.0, java_arguments.1) });
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGING.clone(); java_field }.clone()) {
                jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).FAILURE.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).GET.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
            return None;
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    // synchronized
    pub fn getAll(&mut self, mut categoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>) -> Option<Rc<RefCell<dyn CategoryInterface>>> {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut document: Option<Rc<RefCell<Document>>> = self.getDoc(categoryInterface.clone());
            return Some(self.getAll_Document(document.clone()));
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGING.clone(); java_field }.clone()) {
                jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).FAILURE.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).GET.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
            return None;
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    // Java overloaded method: Rust needs a different name
    // synchronized
    pub fn getAll_Document(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<dyn CategoryInterface>>> {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut categoryNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode({ let java_field = jcall!(CategoryData::getInstance()).NAME.clone(); java_field }.clone(), jcall!(document).getChildNodes());
            let mut loadedCategoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>> = jcall!(self.categoryFactoryInterface).getRootInstanceFromNode(categoryNode.clone());
            return Some({ let java_arguments = (loadedCategoryInterface.clone(), jcall!(categoryNode).getChildNodes(),); self.addProperties(java_arguments.0, java_arguments.1) });
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGING.clone(); java_field }.clone()) {
                jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).FAILURE.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).GET.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
            return None;
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    // Java overloaded method: Rust needs a different name
    // synchronized
    pub fn getAll2(&mut self, mut parentCategoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>, mut categoryNodeList: Option<Rc<RefCell<NodeList>>>) -> Option<Rc<RefCell<dyn CategoryInterface>>> {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if categoryNodeList.is_some() {
                for index in (0..{ let java_bound = jcall!(categoryNodeList).getLength(); java_bound }) {
                    let mut categoryNode: Option<Rc<RefCell<Node>>> = jcall!(categoryNodeList).item(index.clone());
                    if categoryNode.is_some() && jcall!(jcall!(categoryNode).getNodeName()).compareTo({ let java_field = jcall!(CategoryData::getInstance()).NAME.clone(); java_field }.clone()) == 0 {
                        let mut categoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>> = ({ let java_receiver = self.categoryFactoryInterface.clone(); let java_arguments = (jcall!(jcall!(parentCategoryInterface).getHierarchy()).getRoot(), parentCategoryInterface.clone(), categoryNode.clone(),); let java_result = jcall!(java_receiver).getInstance_CategoryInterface_CategoryInterface_Node(java_arguments.0, java_arguments.1, java_arguments.2); java_result }).clone();
                        jcall!(parentCategoryInterface).addChild(categoryInterface.clone());
                    }
                }
            }
            return Some(parentCategoryInterface.clone());
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGING.clone(); java_field }.clone()) {
                jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).FAILURE.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).GET.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
            return None;
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    // synchronized
    pub fn addProperties(&mut self, mut loadedCategoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>, mut categoryNodeList: Option<Rc<RefCell<NodeList>>>) -> Option<Rc<RefCell<dyn CategoryInterface>>> {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if categoryNodeList.is_some() {
                for index in (0..{ let java_bound = jcall!(categoryNodeList).getLength(); java_bound }) {
                    let mut categoryNode: Option<Rc<RefCell<Node>>> = jcall!(categoryNodeList).item(index.clone());
                    if categoryNode.is_some() && jcall!(jcall!(categoryNode).getNodeName()).compareTo({ let java_field = jcall!(CategoryData::getInstance()).NAME.clone(); java_field }.clone()) == 0 {
                        let mut categoryPropertiesInterface: Option<Rc<RefCell<dyn CategoryPropertiesInterface>>> = (CategoryProperties::new1_Node(categoryNode.clone())).clone() /* as CategoryPropertiesInterface */;
                        jcall!(loadedCategoryInterface).addChildProperty(categoryPropertiesInterface.clone());
                    }
                }
            }
            return Some(loadedCategoryInterface.clone());
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGING.clone(); java_field }.clone()) {
                jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).FAILURE.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).GET.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
            return None;
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
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
impl crate::org::allbinary::data::tree::category::CategoryModifierTreeInterface::CategoryModifierTreeInterface for CategoryLoader {
    fn insert(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::business::category::CategoryInterface::CategoryInterface>>>, p2: Option<Rc<RefCell<dyn crate::org::allbinary::business::category::CategoryInterface::CategoryInterface>>>) { crate::org::allbinary::data::tree::category::CategoryModifierTree::CategoryModifierTree::insert(crate::org::allbinary::data::tree::category::CategoryModifierTree::CategoryModifierTreeDyn::as_CategoryModifierTree_mut(self), p1, p2) }
    fn delete(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::business::category::CategoryInterface::CategoryInterface>>>, p2: Option<Rc<RefCell<dyn crate::org::allbinary::business::category::CategoryInterface::CategoryInterface>>>) { crate::org::allbinary::data::tree::category::CategoryModifierTree::CategoryModifierTree::delete(crate::org::allbinary::data::tree::category::CategoryModifierTree::CategoryModifierTreeDyn::as_CategoryModifierTree_mut(self), p1, p2) }
}


