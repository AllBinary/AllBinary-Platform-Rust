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
// package org.allbinary.business.category
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::business::category::hierarchy::CategoryHierarchy::CategoryHierarchy;
use crate::org::allbinary::business::category::hierarchy::CategoryHierarchyInterface::CategoryHierarchyInterface;
use crate::org::allbinary::business::category::properties::CategoryPropertiesFactoryInterface::CategoryPropertiesFactoryInterface;
use crate::org::allbinary::business::category::properties::CategoryPropertiesInterface::CategoryPropertiesInterface;
use crate::org::allbinary::business::category::properties::root::RootCategoryPropertiesInterface::RootCategoryPropertiesInterface;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::io::path::AbPath::AbPath;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use super::CategoryInterface::CategoryInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct Category {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub categoryHierarchyInterface: Option<Rc<RefCell<dyn CategoryHierarchyInterface>>>,
    pub categoryPropertiesInterface: Option<Rc<RefCell<dyn CategoryPropertiesInterface>>>,
    pub childCategoryVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub typeVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub PROPERTIES: Option<i32>,
    pub CATEGORY: Option<i32>,
}

thread_local! { pub static Category_NOT_ROOT: String = String::from("This is not the root so it has no file path."); }
impl CategoryInterface for Category {
    fn getChildNodes(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Self::getChildNodes(self) }
    fn addChildProperty(&mut self, mut categoryPropertiesInterface: Option<Rc<RefCell<dyn CategoryPropertiesInterface>>>) -> bool { Self::addChildProperty(self, categoryPropertiesInterface) }
    fn addChild(&mut self, mut categoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>) -> bool { Self::addChild(self, categoryInterface) }
    fn removeChild(&mut self, mut categoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>) -> bool { Self::removeChild(self, categoryInterface) }
    fn isLeaf(&mut self) -> bool { Self::isLeaf(self) }
    fn getProperties(&mut self) -> Option<Rc<RefCell<dyn CategoryPropertiesInterface>>> { Self::getProperties(self) }
    fn setProperties(&mut self, mut categoryPropertiesInterface: Option<Rc<RefCell<dyn CategoryPropertiesInterface>>>) { Self::setProperties(self, categoryPropertiesInterface) }
    fn getHierarchy(&mut self) -> Option<Rc<RefCell<dyn CategoryHierarchyInterface>>> { Self::getHierarchy(self) }
    fn setHierarchy(&mut self, mut categoryHierarchyInterface: Option<Rc<RefCell<dyn CategoryHierarchyInterface>>>) { Self::setHierarchy(self, categoryHierarchyInterface) }
    fn getPath(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> { Self::getPath(self) }
    fn getFilePath(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> { Self::getFilePath(self) }
    fn getWebAppPath(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> { Self::getWebAppPath(self) }
    fn getRootFilePath(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> { Self::getRootFilePath(self) }
    fn log(&mut self) { Self::log(self) }
}
impl crate::org::allbinary::data::tables::TableMappingInterface::TableMappingInterface for Category {
    fn getKey(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { Self::getKey(self) }
    fn toVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Self::toVector(self) }
    fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> { Self::toHashMap(self) }
}
impl crate::org::allbinary::logic::control::validate::ValidationInterface::ValidationInterface for Category {
    fn isValid(&mut self) -> Option<bool> { Self::isValid(self) }
    fn toValidationInfoNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toValidationInfoNode(self, document) }
    fn toValidationInfoDoc(&mut self) -> Option<Rc<RefCell<Document>>> { Self::toValidationInfoDoc(self) }
    fn validationInfo(&mut self) -> String { Self::validationInfo(self) }
}

impl Category {
    

    pub fn new_struct1(mut categoryPropertiesFactoryInterface: Option<Rc<RefCell<dyn CategoryPropertiesFactoryInterface>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            categoryHierarchyInterface: None,
            categoryPropertiesInterface: None,
            childCategoryVector: None,
            typeVector: None,
            PROPERTIES: None,
            CATEGORY: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.childCategoryVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.typeVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.PROPERTIES = Some((1) as i32);
        this.CATEGORY = Some((0) as i32);
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).CATEGORY.clone()) {
            jcall!(this.logUtil).putF(jcall!(this.commonStrings).CONSTRUCTOR.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("Category(CategoryPropertiesFactoryInterface)"));
        }
        this.categoryPropertiesInterface = jcall!(categoryPropertiesFactoryInterface).getInstance();
        this.log();
        this
    }

    pub fn new1(mut categoryPropertiesFactoryInterface: Option<Rc<RefCell<dyn CategoryPropertiesFactoryInterface>>>) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct1(categoryPropertiesFactoryInterface))));
            this.categoryHierarchyInterface = (CategoryHierarchy::new2((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()), (|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()))).clone() /* as CategoryHierarchyInterface */;
        this
    }
    

    pub fn new_struct2(mut categoryPropertiesFactoryInterface: Option<Rc<RefCell<dyn CategoryPropertiesFactoryInterface>>>, mut level: i32) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            categoryHierarchyInterface: None,
            categoryPropertiesInterface: None,
            childCategoryVector: None,
            typeVector: None,
            PROPERTIES: None,
            CATEGORY: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.childCategoryVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.typeVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.PROPERTIES = Some((1) as i32);
        this.CATEGORY = Some((0) as i32);
        this.categoryPropertiesInterface = jcall!(categoryPropertiesFactoryInterface).getInstance();
        this.log();
        this
    }

    pub fn new2(mut categoryPropertiesFactoryInterface: Option<Rc<RefCell<dyn CategoryPropertiesFactoryInterface>>>, mut level: i32) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct2(categoryPropertiesFactoryInterface, level))));
            this.categoryHierarchyInterface = (CategoryHierarchy::new3((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()), (|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()), level.clone())).clone() /* as CategoryHierarchyInterface */;
        this
    }
    

    pub fn new_struct3(mut rootCategoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>, mut parentCategoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>, mut categoryPropertiesFactoryInterface: Option<Rc<RefCell<dyn CategoryPropertiesFactoryInterface>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            categoryHierarchyInterface: None,
            categoryPropertiesInterface: None,
            childCategoryVector: None,
            typeVector: None,
            PROPERTIES: None,
            CATEGORY: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.childCategoryVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.typeVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.PROPERTIES = Some((1) as i32);
        this.CATEGORY = Some((0) as i32);
        this.categoryPropertiesInterface = (jcall!(categoryPropertiesFactoryInterface).getInstance()).clone() /* as CategoryPropertiesInterface */;
        this.categoryHierarchyInterface = (CategoryHierarchy::new2(rootCategoryInterface.clone(), parentCategoryInterface.clone())).clone() /* as CategoryHierarchyInterface */;
        this.log();
        this
    }

    pub fn new3(mut rootCategoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>, mut parentCategoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>, mut categoryPropertiesFactoryInterface: Option<Rc<RefCell<dyn CategoryPropertiesFactoryInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct3(rootCategoryInterface, parentCategoryInterface, categoryPropertiesFactoryInterface))))
    }
    

    // throws Exception
    pub fn getKey(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return jcall!(self.categoryPropertiesInterface).getKey();
    }
    

    // throws Exception
    pub fn getRootFilePath(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> {
        let mut rootAbPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> = AbPath::new(Category_NOT_ROOT.with(Clone::clone).clone(), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
        let mut categoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>> = jcall!(self.categoryHierarchyInterface).getRoot();
        if jcall!(jcall!(categoryInterface).getProperties()).isRoot() {
            let mut rootCategoryPropertiesInterface: Option<Rc<RefCell<dyn RootCategoryPropertiesInterface>>> = (jcall!(categoryInterface).getProperties()).clone() /* as RootCategoryPropertiesInterface */;
            rootAbPath = jcall!(rootCategoryPropertiesInterface).getRootFilePath();
        }
        return rootAbPath.clone();
    }
    

    // throws Exception
    pub fn getFilePath(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> {
        return AbPath::new(format!("{}{}", jcall!(self.getRootFilePath()).toString(), jcall!(jcall!(self.getProperties()).getPath(self.getHierarchy())).toString()), jcall!(self.getProperties()).getFileName()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
    }
    

    // throws Exception
    pub fn getWebAppPath(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> {
        let mut categoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>> = jcall!(self.categoryHierarchyInterface).getRoot();
        return jcall!(jcall!(categoryInterface).getProperties()).getWebAppPath();
    }
    

    // throws Exception
    pub fn getPath(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> {
        return jcall!(self.categoryPropertiesInterface).getPath(self.categoryHierarchyInterface.clone());
    }
    

    // synchronized
    pub fn addChildProperty(&mut self, mut categoryPropertiesInterface: Option<Rc<RefCell<dyn CategoryPropertiesInterface>>>) -> bool {
        jcall!(self.typeVector).add(crate::java_into_object((self.PROPERTIES.clone()).expect("NullPointerException")));
        return jcall!(self.childCategoryVector).add(crate::java_into_object(categoryPropertiesInterface.clone()));
    }
    

    // synchronized
    pub fn addChild(&mut self, mut categoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>) -> bool {
        self.removeDuplicateChild(categoryInterface.clone());
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).CATEGORY.clone()) {
            jcall!(self.logUtil).putF(String::from("adding"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("addChild"));
        }
        let mut childCategoryHierarchyInterface: Option<Rc<RefCell<dyn CategoryHierarchyInterface>>> = jcall!(categoryInterface).getHierarchy();
        jcall!(childCategoryHierarchyInterface).setParent(None /* Java this: a &mut self method has no Rc of itself */);
        jcall!(childCategoryHierarchyInterface).setRoot(jcall!(self.getHierarchy()).getRoot());
        jcall!(categoryInterface).setHierarchy(childCategoryHierarchyInterface.clone());
        jcall!(self.typeVector).add(crate::java_into_object((self.CATEGORY.clone()).expect("NullPointerException")));
        return jcall!(self.childCategoryVector).add(crate::java_into_object(categoryInterface.clone()));
    }
    

    pub fn getChildNodes(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.childCategoryVector.clone();
    }
    

    // synchronized
    pub fn removeChild(&mut self, mut categoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>) -> bool {
        let mut removalVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut bool_return: bool = false;
        let mut size: i32 = jcall!(self.childCategoryVector).size();
        for index in (0..size) {
            if match (&self.CATEGORY, &jcall!(self.typeVector).get(index.clone())) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(self.childCategoryVector).get(index.clone());
                let mut currentCategoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>> = (anyType).clone() /* as CategoryInterface */;
                if jcall!(jcall!(jcall!(currentCategoryInterface).getProperties()).getValue()).compareTo(jcall!(jcall!(categoryInterface).getProperties()).getValue()) == 0 {
                    jcall!(removalVector).add(crate::java_into_object(currentCategoryInterface.clone()));
                    bool_return = true;
                }
            }
        }
        self.removal(removalVector.clone());
        return bool_return.clone();
    }
    

    pub fn removal(&mut self, mut removalVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        let mut removalSize: i32 = jcall!(removalVector).size();
        for index in (0..removalSize) {
            let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(removalVector).get(index.clone());
            let mut objectIndex: i32 = jcall!(self.childCategoryVector).indexOf(crate::java_into_object(anyType.clone()));
            jcall!(self.typeVector).remove(crate::java_into_object(objectIndex.clone()));
            jcall!(self.childCategoryVector).remove(crate::java_into_object(objectIndex.clone()));
        }
    }
    

    // synchronized
    pub fn removeDuplicateChild(&mut self, mut categoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>) -> bool {
        let mut removalVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut bool_return: bool = false;
        let mut size: i32 = jcall!(self.childCategoryVector).size();
        for index in (0..size) {
            let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(self.childCategoryVector).get(index.clone());
            if match (&self.PROPERTIES, &jcall!(self.typeVector).get(index.clone())) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                let mut categoryPropertiesInterface: Option<Rc<RefCell<dyn CategoryPropertiesInterface>>> = (anyType).clone() /* as CategoryPropertiesInterface */;
                if jcall!(jcall!(jcall!(categoryInterface).getProperties()).getValue()).compareTo(jcall!(categoryPropertiesInterface).getValue()) == 0 {
                    jcall!(removalVector).add(crate::java_into_object(categoryPropertiesInterface.clone()));
                    bool_return = true;
                    break;
                }
            } else if match (&self.CATEGORY, &jcall!(self.typeVector).get(index.clone())) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                let mut existingCategoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>> = (anyType).clone() /* as CategoryInterface */;
                let mut categoryPropertiesInterface: Option<Rc<RefCell<dyn CategoryPropertiesInterface>>> = jcall!(existingCategoryInterface).getProperties();
                if jcall!(jcall!(jcall!(categoryInterface).getProperties()).getValue()).compareTo(jcall!(categoryPropertiesInterface).getValue()) == 0 {
                    jcall!(removalVector).add(crate::java_into_object(existingCategoryInterface.clone()));
                    bool_return = true;
                    break;
                }
            }
        }
        self.removal(removalVector.clone());
        return bool_return.clone();
    }
    

    // throws Exception
    pub fn isLeaf(&mut self) -> bool {
        if self.childCategoryVector.is_none() {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Category Error"))));
        }
        if jcall!(self.childCategoryVector).size() == 0 {
            return true;
        } else {
            return false;
        }
    }
    

    pub fn getProperties(&mut self) -> Option<Rc<RefCell<dyn CategoryPropertiesInterface>>> {
        return self.categoryPropertiesInterface.clone();
    }
    

    pub fn setProperties(&mut self, mut categoryPropertiesInterface: Option<Rc<RefCell<dyn CategoryPropertiesInterface>>>) {
        self.categoryPropertiesInterface = categoryPropertiesInterface.clone();
    }
    

    pub fn getHierarchy(&mut self) -> Option<Rc<RefCell<dyn CategoryHierarchyInterface>>> {
        return self.categoryHierarchyInterface.clone();
    }
    

    pub fn setHierarchy(&mut self, mut categoryHierarchyInterface: Option<Rc<RefCell<dyn CategoryHierarchyInterface>>>) {
        self.categoryHierarchyInterface = categoryHierarchyInterface.clone();
    }
    

    // throws Exception
    // synchronized
    pub fn isValid(&mut self) -> Option<bool> {
        if !jcall!(self.categoryPropertiesInterface).isValid().unwrap() {
            return false;
        }
        let mut size: i32 = jcall!(self.childCategoryVector).size();
        for index in (0..size) {
            let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(self.childCategoryVector).get(index.clone());
            if match (&self.PROPERTIES, &jcall!(self.typeVector).get(index.clone())) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                let mut categoryPropertiesInterface: Option<Rc<RefCell<dyn CategoryPropertiesInterface>>> = (anyType).clone() /* as CategoryPropertiesInterface */;
                if !jcall!(categoryPropertiesInterface).isValid().unwrap() {
                    return false;
                }
            } else if match (&self.CATEGORY, &jcall!(self.typeVector).get(index.clone())) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                let mut categoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>> = (anyType).clone() /* as CategoryInterface */;
                if !jcall!(categoryInterface).isValid().unwrap() {
                    return false;
                }
            }
        }
        return true;
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
    

    // throws Exception
    pub fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> {
        let mut categoryHashMap: Option<Rc<RefCell<HashMap>>> = jcall!(self.categoryPropertiesInterface).toHashMap();
        return categoryHashMap.clone();
    }
    

    // throws Exception
    pub fn toVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut categoryVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(self.categoryPropertiesInterface).toVector();
        jcall!(categoryVector).add(crate::java_into_object(self.childCategoryVector.clone()));
        return categoryVector.clone();
    }
    

    // throws Exception
    pub fn log(&mut self) {
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).CATEGORY.clone()) {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(String::from("Category Name: "));
            jcall!(stringBuffer).append(jcall!(self.categoryPropertiesInterface).getValue());
            jcall!(stringBuffer).append(String::from("\nPath = "));
            jcall!(stringBuffer).append(jcall!(self.getPath()).toString());
            jcall!(stringBuffer).append(String::from("\nFile Path: "));
            jcall!(stringBuffer).append(jcall!(self.getFilePath()).toString());
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("log()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
    }
}
pub trait CategoryDyn {
    fn as_Category(&self) -> &Category;
    fn as_Category_mut(&mut self) -> &mut Category;
    fn getKey(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { Category::getKey(self.as_Category_mut()) }
    fn getRootFilePath(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> { Category::getRootFilePath(self.as_Category_mut()) }
    fn getFilePath(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> { Category::getFilePath(self.as_Category_mut()) }
    fn getWebAppPath(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> { Category::getWebAppPath(self.as_Category_mut()) }
    fn getPath(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> { Category::getPath(self.as_Category_mut()) }
    fn addChildProperty(&mut self, mut categoryPropertiesInterface: Option<Rc<RefCell<dyn CategoryPropertiesInterface>>>) -> bool { Category::addChildProperty(self.as_Category_mut(), categoryPropertiesInterface) }
    fn addChild(&mut self, mut categoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>) -> bool { Category::addChild(self.as_Category_mut(), categoryInterface) }
    fn getChildNodes(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Category::getChildNodes(self.as_Category_mut()) }
    fn removeChild(&mut self, mut categoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>) -> bool { Category::removeChild(self.as_Category_mut(), categoryInterface) }
    fn removal(&mut self, mut removalVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { Category::removal(self.as_Category_mut(), removalVector) }
    fn removeDuplicateChild(&mut self, mut categoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>) -> bool { Category::removeDuplicateChild(self.as_Category_mut(), categoryInterface) }
    fn isLeaf(&mut self) -> bool { Category::isLeaf(self.as_Category_mut()) }
    fn getProperties(&mut self) -> Option<Rc<RefCell<dyn CategoryPropertiesInterface>>> { Category::getProperties(self.as_Category_mut()) }
    fn setProperties(&mut self, mut categoryPropertiesInterface: Option<Rc<RefCell<dyn CategoryPropertiesInterface>>>) { Category::setProperties(self.as_Category_mut(), categoryPropertiesInterface) }
    fn getHierarchy(&mut self) -> Option<Rc<RefCell<dyn CategoryHierarchyInterface>>> { Category::getHierarchy(self.as_Category_mut()) }
    fn setHierarchy(&mut self, mut categoryHierarchyInterface: Option<Rc<RefCell<dyn CategoryHierarchyInterface>>>) { Category::setHierarchy(self.as_Category_mut(), categoryHierarchyInterface) }
    fn isValid(&mut self) -> Option<bool> { Category::isValid(self.as_Category_mut()) }
    fn toValidationInfoDoc(&mut self) -> Option<Rc<RefCell<Document>>> { Category::toValidationInfoDoc(self.as_Category_mut()) }
    fn toValidationInfoNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Category::toValidationInfoNode(self.as_Category_mut(), document) }
    fn validationInfo(&mut self) -> String { Category::validationInfo(self.as_Category_mut()) }
    fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> { Category::toHashMap(self.as_Category_mut()) }
    fn toVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Category::toVector(self.as_Category_mut()) }
    fn log(&mut self) { Category::log(self.as_Category_mut()) }
}
impl CategoryDyn for Category {
    fn as_Category(&self) -> &Category { self }
    fn as_Category_mut(&mut self) -> &mut Category { self }
}


