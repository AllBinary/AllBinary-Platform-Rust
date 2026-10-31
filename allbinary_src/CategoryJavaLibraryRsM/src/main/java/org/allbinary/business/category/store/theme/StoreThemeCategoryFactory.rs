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
// package org.allbinary.business.category.store.theme
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::business::category::CategoryFactoryInterface::CategoryFactoryInterface;
use crate::org::allbinary::business::category::CategoryInterface::CategoryInterface;
use crate::org::allbinary::business::category::CategoryUtil::CategoryUtil;
use crate::org::allbinary::business::category::properties::CategoryPropertiesFactory::CategoryPropertiesFactory;
use crate::org::allbinary::business::category::properties::CategoryPropertiesFactoryInterface::CategoryPropertiesFactoryInterface;
use crate::org::allbinary::business::category::properties::root::store::theme::RootStoreThemeCategoryPropertiesFactory::RootStoreThemeCategoryPropertiesFactory;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::io::path::AbPath::AbPath;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::w3c::dom::Node::Node;
use super::StoreThemeCategory::StoreThemeCategory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct StoreThemeCategoryFactory {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>,
}

// impl CategoryFactoryInterface for StoreThemeCategoryFactory with the methods of the trait

impl StoreThemeCategoryFactory {
    

    pub fn new_struct(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            transformInfoInterface: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).CATEGORY.clone()) {
            jcall!(this.logUtil).putF(jcall!(this.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("StoreCategoryFactory(TransformInfoInterface transformInfoInterface)"));
        }
        this.transformInfoInterface = transformInfoInterface.clone();
        this
    }

    pub fn new(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(transformInfoInterface))))
    }
    

    pub fn getRootInstance(&mut self) -> Option<Rc<RefCell<dyn CategoryInterface>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut categoryPropertiesFactoryInterface: Option<Rc<RefCell<dyn CategoryPropertiesFactoryInterface>>> = (RootStoreThemeCategoryPropertiesFactory::new1(self.transformInfoInterface.clone())).clone() /* as CategoryPropertiesFactoryInterface */;
            return (StoreThemeCategory::new3(self.transformInfoInterface.clone(), categoryPropertiesFactoryInterface.clone(), 0)).clone() /* as CategoryInterface */;
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).ENTITYFACTORYERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getRootInstance()"), crate::java_into_object(e.clone()));
            }
            return None;
            }
        }
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn getRootInstance1(&mut self, mut categoryAbPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>) -> Option<Rc<RefCell<dyn CategoryInterface>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut level: i32 = CategoryUtil::getPathLevel(categoryAbPath.clone());
            let mut categoryPropertiesFactoryInterface: Option<Rc<RefCell<dyn CategoryPropertiesFactoryInterface>>> = (RootStoreThemeCategoryPropertiesFactory::new2(self.transformInfoInterface.clone(), categoryAbPath.clone())).clone() /* as CategoryPropertiesFactoryInterface */;
            return (StoreThemeCategory::new3(self.transformInfoInterface.clone(), categoryPropertiesFactoryInterface.clone(), level.clone())).clone() /* as CategoryInterface */;
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).ENTITYFACTORYERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getRootInstance(String)"), crate::java_into_object(e.clone()));
            }
            return None;
            }
        }
    }
    

    pub fn getRootInstanceFromNode(&mut self, mut node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<dyn CategoryInterface>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut categoryPropertiesFactoryInterface: Option<Rc<RefCell<dyn CategoryPropertiesFactoryInterface>>> = (RootStoreThemeCategoryPropertiesFactory::new2(self.transformInfoInterface.clone(), node.clone())).clone() /* as CategoryPropertiesFactoryInterface */;
            let mut categoryPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> = AbPath::new(CategoryUtil::getNameFromNode(node.clone()), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
            let mut level: i32 = CategoryUtil::getPathLevel(categoryPath.clone());
            return (StoreThemeCategory::new4(self.transformInfoInterface.clone(), node.clone(), categoryPropertiesFactoryInterface.clone(), level.clone())).clone() /* as CategoryInterface */;
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).ENTITYFACTORYERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getRootInstance(node)"), crate::java_into_object(e.clone()));
            }
            return None;
            }
        }
    }
    

    pub fn getInstance(&mut self, mut categoryPath: String, mut level: i32) -> Option<Rc<RefCell<dyn CategoryInterface>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut categoryPropertiesFactoryInterface: Option<Rc<RefCell<dyn CategoryPropertiesFactoryInterface>>> = (CategoryPropertiesFactory::new1(categoryPath.clone())).clone() /* as CategoryPropertiesFactoryInterface */;
            return (StoreThemeCategory::new3(self.transformInfoInterface.clone(), categoryPropertiesFactoryInterface.clone(), level.clone())).clone() /* as CategoryInterface */;
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).ENTITYFACTORYERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getInstance(String, int)"), crate::java_into_object(e.clone()));
            }
            return None;
            }
        }
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn getInstance3(&mut self, mut rootCategoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>, mut parentCategoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>, mut categoryPath: String) -> Option<Rc<RefCell<dyn CategoryInterface>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut categoryPropertiesFactoryInterface: Option<Rc<RefCell<dyn CategoryPropertiesFactoryInterface>>> = (CategoryPropertiesFactory::new1(categoryPath.clone())).clone() /* as CategoryPropertiesFactoryInterface */;
            return (StoreThemeCategory::new4(self.transformInfoInterface.clone(), rootCategoryInterface.clone(), parentCategoryInterface.clone(), categoryPropertiesFactoryInterface.clone())).clone() /* as CategoryInterface */;
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).ENTITYFACTORYERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getInstance(CategoryInterface, CategoryInterface , String, int)"), crate::java_into_object(e.clone()));
            }
            return None;
            }
        }
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn getInstance(&mut self, mut node: Option<Rc<RefCell<Node>>>, mut level: i32) -> Option<Rc<RefCell<dyn CategoryInterface>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut categoryPropertiesFactoryInterface: Option<Rc<RefCell<dyn CategoryPropertiesFactoryInterface>>> = (CategoryPropertiesFactory::new1(node.clone())).clone() /* as CategoryPropertiesFactoryInterface */;
            return (StoreThemeCategory::new3(self.transformInfoInterface.clone(), categoryPropertiesFactoryInterface.clone(), level.clone())).clone() /* as CategoryInterface */;
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).ENTITYFACTORYERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getInstance(Node, int)"), crate::java_into_object(e.clone()));
            }
            return None;
            }
        }
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn getInstance3(&mut self, mut rootCategoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>, mut parentCategoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>, mut node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<dyn CategoryInterface>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut categoryPropertiesFactoryInterface: Option<Rc<RefCell<dyn CategoryPropertiesFactoryInterface>>> = (CategoryPropertiesFactory::new1(node.clone())).clone() /* as CategoryPropertiesFactoryInterface */;
            return (StoreThemeCategory::new4(self.transformInfoInterface.clone(), rootCategoryInterface.clone(), parentCategoryInterface.clone(), categoryPropertiesFactoryInterface.clone())).clone() /* as CategoryInterface */;
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).ENTITYFACTORYERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getInstance(CategoryInterface, CategoryInterface, Node, int)"), crate::java_into_object(e.clone()));
            }
            return None;
            }
        }
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn getInstance(&mut self, mut categoryPropertiesHashMap: Option<Rc<RefCell<HashMap>>>, mut level: i32) -> Option<Rc<RefCell<dyn CategoryInterface>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut categoryPropertiesFactoryInterface: Option<Rc<RefCell<dyn CategoryPropertiesFactoryInterface>>> = (CategoryPropertiesFactory::new1(categoryPropertiesHashMap.clone())).clone() /* as CategoryPropertiesFactoryInterface */;
            return (StoreThemeCategory::new3(self.transformInfoInterface.clone(), categoryPropertiesFactoryInterface.clone(), level.clone())).clone() /* as CategoryInterface */;
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).ENTITYFACTORYERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getInstance(HashMap)"), crate::java_into_object(e.clone()));
            }
            return None;
            }
        }
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn getInstance3(&mut self, mut rootCategoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>, mut parentCategoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>, mut categoryPropertiesHashMap: Option<Rc<RefCell<HashMap>>>) -> Option<Rc<RefCell<dyn CategoryInterface>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut categoryPropertiesFactoryInterface: Option<Rc<RefCell<dyn CategoryPropertiesFactoryInterface>>> = (CategoryPropertiesFactory::new1(categoryPropertiesHashMap.clone())).clone() /* as CategoryPropertiesFactoryInterface */;
            return (StoreThemeCategory::new4(self.transformInfoInterface.clone(), rootCategoryInterface.clone(), parentCategoryInterface.clone(), categoryPropertiesFactoryInterface.clone())).clone() /* as CategoryInterface */;
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).ENTITYFACTORYERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getInstance(CategoryInterface, CategoryInterface, HashMap)"), crate::java_into_object(e.clone()));
            }
            return None;
            }
        }
    }
}

