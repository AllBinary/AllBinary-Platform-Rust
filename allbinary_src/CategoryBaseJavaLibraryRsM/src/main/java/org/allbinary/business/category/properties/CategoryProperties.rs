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
// package org.allbinary.business.category.properties
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::business::category::CategoryData::CategoryData;
use crate::org::allbinary::business::category::CategoryInterface::CategoryInterface;
use crate::org::allbinary::business::category::CategoryUtil::CategoryUtil;
use crate::org::allbinary::business::category::hierarchy::CategoryHierarchyInterface::CategoryHierarchyInterface;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData;
use crate::org::allbinary::logic::io::path::AbPath::AbPath;
use crate::org::allbinary::logic::io::path::AbPathData::AbPathData;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use crate::java::lang::Exception::*;
use super::CategoryPropertiesInterface::CategoryPropertiesInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CategoryProperties {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub category: String,
}

pub static CategoryProperties_MAXDEPTH: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 100);
impl CategoryPropertiesInterface for CategoryProperties {
    fn setValue(&mut self, mut value: String) { Self::setValue(self, value) }
    fn getValue(&mut self) -> String { Self::getValue(self) }
    fn getWebAppPath(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> { Self::getWebAppPath(self) }
    fn getPath(&mut self, mut categoryHierarchyInterface: Option<Rc<RefCell<dyn CategoryHierarchyInterface>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> { Self::getPath(self, categoryHierarchyInterface) }
    fn getFileName(&mut self) -> String { Self::getFileName(self) }
    fn isRoot(&mut self) -> bool { Self::isRoot(self) }
    fn isRealRoot(&mut self) -> bool { Self::isRealRoot(self) }
}
impl crate::org::allbinary::data::tables::TableMappingInterface::TableMappingInterface for CategoryProperties {
    fn getKey(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { Self::getKey(self) }
    fn toVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Self::toVector(self) }
    fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> { Self::toHashMap(self) }
}
impl crate::org::allbinary::logic::control::validate::ValidationInterface::ValidationInterface for CategoryProperties {
    fn isValid(&mut self) -> Option<bool> { Self::isValid(self) }
    fn toValidationInfoNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toValidationInfoNode(self, document) }
    fn toValidationInfoDoc(&mut self) -> Option<Rc<RefCell<Document>>> { Self::toValidationInfoDoc(self) }
    fn validationInfo(&mut self) -> String { Self::validationInfo(self) }
}

impl CategoryProperties {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn new_struct1(mut name: String) -> Self {
        let mut this = Self {
            logUtil: None,
            category: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.category = name.clone();
        this
    }

    pub fn new1(mut name: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(name))))
    }
    

    pub fn new_struct1_Node(mut node: Option<Rc<RefCell<Node>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            category: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.category = { let java_value = (CategoryUtil::getNameFromNode(node.clone())).clone(); java_value };
        this
    }

    pub fn new1_Node(mut node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1_Node(node))))
    }
    

    pub fn new_struct1_HashMap(mut categoryPropertiesHashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            category: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.category = { let java_value = ({ let java_object = (jcall!(categoryPropertiesHashMap).get(crate::java_into_object({ let java_field = jcall!(CategoryData::getInstance()).NAME.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }.clone()).clone(); java_value };
        this
    }

    pub fn new1_HashMap(mut categoryPropertiesHashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1_HashMap(categoryPropertiesHashMap))))
    }
    

    pub fn isRealRoot(&mut self) -> bool {
        return false;
    }
    

    pub fn isRoot(&mut self) -> bool {
        return false;
    }
    

    pub fn getKey(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return crate::java_into_object(crate::java_into_object((self.getValue()).clone()));
    }
    

    pub fn getValue(&mut self) -> String {
        return self.category.clone();
    }
    

    // throws Exception
    pub fn getPath(&mut self, mut categoryHierarchyInterface: Option<Rc<RefCell<dyn CategoryHierarchyInterface>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> {
        let mut abPathData: Option<Rc<RefCell<AbPathData>>> = AbPathData::getInstance();
        let mut pathStringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut nextParentCategoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>> = jcall!(categoryHierarchyInterface).getParent();
        if !match &(jcall!(nextParentCategoryInterface).getProperties()) { Some(reference) => std::ptr::addr_eq(&*self as *const Self, reference.as_ptr()), None => false } {
            let mut depthIndex: i32 = 0;
            while nextParentCategoryInterface.is_some() {
                if jcall!(jcall!(nextParentCategoryInterface).getProperties()).isRealRoot() {
                    break;
                }
                if jcall!(jcall!(nextParentCategoryInterface).getProperties()).isRoot() {
                    jcall!(list).addAt(0, crate::java_into_object(jcall!(jcall!(nextParentCategoryInterface).getPath()).toString()));
                } else {
                    jcall!(pathStringBuffer).delete(0, jcall!(pathStringBuffer).length());
                    jcall!(list).addAt(0, crate::java_into_object(jcall!(pathStringBuffer).append({ let java_field = jcall!(abPathData).SEPARATOR.clone(); java_field }.clone()).append(jcall!(jcall!(nextParentCategoryInterface).getProperties()).getValue()).toString()));
                }
                if match (&nextParentCategoryInterface, &jcall!(jcall!(nextParentCategoryInterface).getHierarchy()).getParent()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                    break;
                }
                nextParentCategoryInterface = { let java_value = (jcall!(jcall!(nextParentCategoryInterface).getHierarchy()).getParent()).clone(); java_value };
                depthIndex += 1;
                if depthIndex > CategoryProperties_MAXDEPTH.with(Clone::clone) {
                    panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Probably Major Error"))));
                }
            }
        }
        jcall!(pathStringBuffer).delete(0, jcall!(pathStringBuffer).length());
        jcall!(pathStringBuffer).append({ let java_field = jcall!(abPathData).SEPARATOR.clone(); java_field }.clone()).append(self.getValue());
        let mut size: i32 = jcall!(list).size();
        for index in (0..size) {
            jcall!(pathStringBuffer).append({ let java_object = (jcall!(list).get(index.clone())).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() });
        }
        jcall!(pathStringBuffer).append({ let java_field = jcall!(abPathData).SEPARATOR.clone(); java_field }.clone());
        if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).CATEGORY.clone(); java_field }.clone()) {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("path = "), jcall!(pathStringBuffer).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getPath"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        return AbPath::new(jcall!(pathStringBuffer).toString(), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
    }
    

    pub fn getFileName(&mut self) -> String {
        return format!("{}{}", format!("{}{}", self.getValue(), { let java_field = jcall!(AbPathData::getInstance()).EXTENSION_SEP.clone(); java_field }), { let java_field = jcall!(CategoryData::getInstance()).UNCRYPTED_EXTENSION.clone(); java_field });
    }
    

    // throws Exception
    pub fn getWebAppPath(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> {
        panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Not Root"))));
    }
    

    pub fn setValue(&mut self, mut value: String) {
        self.category = value.clone();
    }
    

    pub fn isValid(&mut self) -> Option<bool> {
        let mut returnBoolean: Option<bool> = false;
        if jcall!(StringValidationUtil::getInstance()).isValidRequired(self.category.clone(), 1, crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXSTRING.with(Clone::clone).clone()) {
            returnBoolean = { let java_value = (true).clone(); java_value }.clone();
        }
        return returnBoolean.clone();
    }
    

    pub fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> {
        let mut categoryHashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> = jcall!(StdUtil::getInstance()).createHashMap();
        jcall!(categoryHashMap).put(crate::java_into_object({ let java_field = jcall!(CategoryData::getInstance()).NAME.clone(); java_field }.clone()), crate::java_into_object(self.category.clone()));
        return categoryHashMap.clone();
    }
    

    pub fn toVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut categoryVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        jcall!(categoryVector).add(crate::java_into_object(self.category.clone()));
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
}

