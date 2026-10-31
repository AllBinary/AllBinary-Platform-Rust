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
// package admin.taghelpers
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
// use crate::java::util::Map::Map;
// use crate::java::util::Set::Set;
// use crate::javax::servlet::http::HttpServletRequest::HttpServletRequest;
// use crate::javax::servlet::jsp::PageContext::PageContext;
use crate::org::allbinary::business::category::CategoryComponent::CategoryComponent;
use crate::org::allbinary::business::category::CategoryData::CategoryData;
use crate::org::allbinary::business::category::CategoryInterface::CategoryInterface;
use crate::org::allbinary::business::category::store::StoreCategoryFactory::StoreCategoryFactory;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontData::StoreFrontData;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontFactory::StoreFrontFactory;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontInterface::StoreFrontInterface;
use crate::org::allbinary::data::tree::category::CategoryLoaderFactory::CategoryLoaderFactory;
use crate::org::allbinary::data::tree::category::CategoryLoaderInterface::CategoryLoaderInterface;
use crate::org::allbinary::data::tree::dom::DomNodeHelper::DomNodeHelper;
use crate::org::allbinary::data::tree::dom::DomSearchHelper::DomSearchHelper;
use crate::org::allbinary::data::tree::dom::document::DomDocumentHelper::DomDocumentHelper;
use crate::org::allbinary::data::tree::dom::document::XmlDocumentHelper::XmlDocumentHelper;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoBasic::TransformInfoBasic;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use super::ModifyTable::ModifyTable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CategoryRequestHelper {
    pub base: ModifyTable,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub pageContext: Option<Rc<RefCell<PageContext>>>,
    pub hashMap: Option<Rc<RefCell<HashMap>>>,
    pub request: Option<Rc<RefCell<HttpServletRequest>>>,
    pub categoryLoaderInterface: Option<Rc<RefCell<dyn CategoryLoaderInterface>>>,
    pub categoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>,
    pub childCategoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>,
    pub transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>,
}

thread_local! { pub static CategoryRequestHelper_categoryRequest: String = jcall!(StringMaker::new()).append(String::from("<")).append(jcall!(CategoryData::getInstance()).REQUEST.clone()).append(String::from(">")).toString(); }

impl CategoryRequestHelper {
    

    pub fn new_struct(mut hashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Self {
        let mut this = Self {
            base: ModifyTable::new_struct(),
            logUtil: None,
            pageContext: None,
            hashMap: None,
            request: None,
            categoryLoaderInterface: None,
            categoryInterface: None,
            childCategoryInterface: None,
            transformInfoInterface: None,
        };
        this.logUtil = LogUtil::getInstance();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            this.request = (jcall!(pageContext).getRequest()).clone() /* as HttpServletRequest */;
            this.pageContext = pageContext.clone();
            this.hashMap = hashMap.clone();
            this.getXmlData();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).TAGHELPERERROR.clone()) {
                jcall!(this.logUtil).put(jcall!(this.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("CategoryRequestHelper()"), crate::java_into_object(e.clone()));
            }
            }
        }
        this
    }

    pub fn new(mut hashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(hashMap, pageContext))))
    }
    

    pub fn getXmlData(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut map: Option<Rc<RefCell<dyn Map>>> = jcall!(self.request).getParameterMap();
            let mut categoryData: Option<Rc<RefCell<CategoryData>>> = CategoryData::getInstance();
            let mut keys: Option<Rc<RefCell<dyn Set>>> = jcall!(map).keySet();
            let mut keyArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(keys).toArray();
            let mut size: i32 = (keyArray.len() as i32).clone();
            if size > 0 {
                let mut xmlRequest: String = (keyArray[(0) as usize]).clone() /* as String */;
                let mut index: i32 = 1;
                while index < size && !xmlRequest.starts_with(&CategoryRequestHelper_categoryRequest.with(Clone::clone).clone()) {
                    xmlRequest = (keyArray[(index) as usize]).clone() /* as String */;
                    if xmlRequest.starts_with(&CategoryRequestHelper_categoryRequest.with(Clone::clone).clone()) {
                        break;
                    }
                }
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).TAGHELPER.clone()) {
                    jcall!(self.logUtil).putF(xmlRequest.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getXmlData()"));
                }
                let mut document: Option<Rc<RefCell<Document>>> = DomDocumentHelper::create1(xmlRequest.clone());
                let mut requestNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(jcall!(categoryData).REQUEST.clone(), jcall!(document).getChildNodes());
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).TAGHELPER.clone()) {
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (XmlDocumentHelper::toString(document.clone()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getXmlData()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                }
                self.setCategoryLoader(requestNode.clone());
                let mut parentCategoryNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(jcall!(categoryData).PARENT.clone(), jcall!(requestNode).getChildNodes());
                let mut categoryNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(jcall!(categoryData).NAME.clone(), jcall!(parentCategoryNode).getChildNodes());
                self.categoryInterface = (jcall!(StoreCategoryFactory::new(self.transformInfoInterface.clone())).getRootInstanceFromNode(categoryNode.clone())).clone() /* as CategoryInterface */;
                self.categoryInterface = jcall!(self.categoryLoaderInterface).get(self.categoryInterface.clone());
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).TAGHELPER.clone()) {
                    jcall!(self.categoryInterface).log();
                    jcall!(self.logUtil).putF(String::from("Loaded Parent Category"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getXmlData()"));
                }
                let mut childCategoryNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNodeNoThrow(jcall!(categoryData).NAME.clone(), jcall!(requestNode).getChildNodes());
                if childCategoryNode.is_some() {
                    if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).TAGHELPER.clone()) {
                        jcall!(self.logUtil).putF(String::from("Loading Child Category"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getXmlData()"));
                    }
                    self.childCategoryInterface = (jcall!(StoreCategoryFactory::new(self.transformInfoInterface.clone())).getInstance(childCategoryNode.clone())).clone() /* as CategoryInterface */;
                    if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).TAGHELPER.clone()) {
                        jcall!(self.childCategoryInterface).log();
                        jcall!(self.logUtil).putF(String::from("Loaded Child Category"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getXmlData()"));
                    }
                } else {
                    self.childCategoryInterface = self.categoryInterface.clone();
                }
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).TAGHELPERERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.base.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getXmlData()"), crate::java_into_object(e.clone()));
            }
            }
        }
    }
    

    // throws Exception
    pub fn setCategoryLoader(&mut self, mut requestNode: Option<Rc<RefCell<Node>>>) {
        let mut storeNameNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(jcall!(StoreFrontData::getInstance()).NAME.clone(), jcall!(requestNode).getChildNodes());
        if storeNameNode.is_some() {
            let mut storeName: String = DomNodeHelper::getTextNodeValue(storeNameNode.clone());
            if true {
                let mut storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>> = StoreFrontFactory::getInstance(storeName.clone());
                self.transformInfoInterface = (TransformInfoBasic::new(storeFrontInterface.clone(), self.hashMap.clone(), self.pageContext.clone())).clone() /* as TransformInfoInterface */;
                if self.transformInfoInterface.is_none() {
                    panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("TransformInfo null"))));
                }
                let mut storeCategoryFactory: Option<Rc<RefCell<StoreCategoryFactory>>> = StoreCategoryFactory::new(self.transformInfoInterface.clone());
                self.categoryLoaderInterface = CategoryLoaderFactory::getInstance(storeCategoryFactory.clone());
            } else {
                panic!("{:?}", format!("{}: {:?}", "Exception", (format!("{}{}", String::from("Store Name Error: "), storeName))));
            }
        }
    }
    

    pub fn insert(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut success: String = String::from("Successfully Added the following to the Category table");
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGS.clone()) {
                jcall!(self.logUtil).putF(String::from("inserting"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("insert()"));
            }
            jcall!(self.categoryLoaderInterface).insert(self.categoryInterface.clone(), self.childCategoryInterface.clone());
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGS.clone()) {
                jcall!(self.logUtil).putF(success.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("insert()"));
            }
            return success.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut error: String = String::from("Failed to add item to Category");
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGSERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("insert()"), crate::java_into_object(e.clone()));
            }
            return error.clone();
            }
        }
    }
    

    pub fn delete(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut success: String = format!("{}{}", format!("{}{}", format!("{}{}", String::from("Successfully Removed "), jcall!(CategoryData::getInstance()).NAME), String::from("=")), jcall!(self.childCategoryInterface).getPath());
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGS.clone()) {
                jcall!(self.logUtil).putF(String::from("Deleting"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("delete()"));
            }
            jcall!(self.categoryLoaderInterface).delete(self.categoryInterface.clone(), self.childCategoryInterface.clone());
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGS.clone()) {
                jcall!(self.logUtil).putF(success.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("delete()"));
            }
            return success.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let mut error: String = format!("{}{}", String::from("Failed to remove category: "), jcall!(self.childCategoryInterface).getPath());
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGSERROR.clone()) {
                    jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("delete()"), crate::java_into_object(e.clone()));
                }
                return error.clone();
            })) {
                Ok(java_return) => return java_return,
                Err(e2) => {
            let e2: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e2 as Box<dyn std::any::Any>)));

                let mut error: String = String::from("Failed to remove category and show the path of the failed category");
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGSERROR.clone()) {
                    jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("delete()"), crate::java_into_object(e2.clone()));
                }
                return error.clone();
                }
            }
            }
        }
    }
    

    pub fn viewCategory(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut document: Option<Rc<RefCell<Document>>> = jcall!(CategoryComponent::new(self.childCategoryInterface.clone())).toXmlDoc();
            let mut xmlString: String = XmlDocumentHelper::toString(document.clone());
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGS.clone()) {
                jcall!(self.logUtil).putF(xmlString.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("viewCategory()"));
            }
            return xmlString.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut error: String = String::from("Failed to get Category");
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGSERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("viewCategory()"), crate::java_into_object(e.clone()));
            }
            return error.clone();
            }
        }
    }
    

    pub fn viewCategories(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut success: String = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGS.clone()) {
                jcall!(self.logUtil).putF(success.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("viewCategories()"));
            }
            return success.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut error: String = String::from("Failed to view Categories table");
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGSERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("viewCategories()"), crate::java_into_object(e.clone()));
            }
            return error.clone();
            }
        }
    }
    

    pub fn update(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut success: String = String::from("New Item Successfully added");
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGS.clone()) {
                jcall!(self.logUtil).putF(success.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("update()"));
            }
            return success.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut error: String = String::from("Failed to add Item");
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGSERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("update()"), crate::java_into_object(e.clone()));
            }
            return error.clone();
            }
        }
    }
}
impl std::ops::Deref for CategoryRequestHelper {
    type Target = ModifyTable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for CategoryRequestHelper {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::admin::taghelpers::ModifyTable::ModifyTableDyn for CategoryRequestHelper {
    fn as_ModifyTable(&self) -> &crate::admin::taghelpers::ModifyTable::ModifyTable { &self.base }
    fn as_ModifyTable_mut(&mut self) -> &mut crate::admin::taghelpers::ModifyTable::ModifyTable { &mut self.base }
}
impl crate::admin::taghelpers::ModifyTableInterface::ModifyTableInterface for CategoryRequestHelper {
    fn insert(&mut self) -> String { Self::insert(self) }
    fn delete(&mut self) -> String { Self::delete(self) }
    fn update(&mut self) -> String { Self::update(self) }
}


