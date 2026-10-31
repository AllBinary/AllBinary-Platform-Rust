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
use crate::org::allbinary::business::category::properties::CategoryPropertiesFactory::CategoryPropertiesFactory;
use crate::org::allbinary::business::category::properties::CategoryPropertiesFactoryInterface::CategoryPropertiesFactoryInterface;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::io::path::AbPath::AbPath;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::w3c::dom::Node::Node;
use super::CategoryFactoryInterface::CategoryFactoryInterface;
use super::CategoryInterface::CategoryInterface;
use super::Category::Category;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CategoryAbstractFactory {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
}

// impl CategoryFactoryInterface for CategoryAbstractFactory with the methods of the trait

impl CategoryAbstractFactory {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getRootInstance(&mut self) -> Option<Rc<RefCell<dyn CategoryInterface>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn getRootInstance1(&mut self, mut categoryAbPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>) -> Option<Rc<RefCell<dyn CategoryInterface>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getRootInstanceFromNode(&mut self, mut node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<dyn CategoryInterface>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getInstance(&mut self, mut categoryName: String) -> Option<Rc<RefCell<dyn CategoryInterface>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).CATEGORY.clone()) {
                jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getInstance(String)"));
            }
            let mut categoryPropertiesFactoryInterface: Option<Rc<RefCell<dyn CategoryPropertiesFactoryInterface>>> = (CategoryPropertiesFactory::new1(categoryName.clone())).clone() /* as CategoryPropertiesFactoryInterface */;
            return (Category::new1(categoryPropertiesFactoryInterface.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::category::Category::CategoryDyn>>)).clone() /* as CategoryInterface */;
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
    pub fn getInstance3(&mut self, mut rootCategoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>, mut parentCategoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>, mut categoryName: String) -> Option<Rc<RefCell<dyn CategoryInterface>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).CATEGORY.clone()) {
                jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getInstance(CategoryInterface, CategoryInterface, String)"));
            }
            let mut categoryPropertiesFactoryInterface: Option<Rc<RefCell<dyn CategoryPropertiesFactoryInterface>>> = (CategoryPropertiesFactory::new1(categoryName.clone())).clone() /* as CategoryPropertiesFactoryInterface */;
            return (Category::new3(rootCategoryInterface.clone(), parentCategoryInterface.clone(), categoryPropertiesFactoryInterface.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::category::Category::CategoryDyn>>)).clone() /* as CategoryInterface */;
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
    pub fn getInstance(&mut self, mut node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<dyn CategoryInterface>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).CATEGORY.clone()) {
                jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getInstance(Node)"));
            }
            let mut categoryPropertiesFactoryInterface: Option<Rc<RefCell<dyn CategoryPropertiesFactoryInterface>>> = (CategoryPropertiesFactory::new1(node.clone())).clone() /* as CategoryPropertiesFactoryInterface */;
            return (Category::new1(categoryPropertiesFactoryInterface.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::category::Category::CategoryDyn>>)).clone() /* as CategoryInterface */;
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
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).CATEGORY.clone()) {
                jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getInstance(CategoryInterface, CategoryInterface, Node)"));
            }
            let mut categoryPropertiesFactoryInterface: Option<Rc<RefCell<dyn CategoryPropertiesFactoryInterface>>> = (CategoryPropertiesFactory::new1(node.clone())).clone() /* as CategoryPropertiesFactoryInterface */;
            return (Category::new3(rootCategoryInterface.clone(), parentCategoryInterface.clone(), categoryPropertiesFactoryInterface.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::category::Category::CategoryDyn>>)).clone() /* as CategoryInterface */;
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
    pub fn getInstance3(&mut self, mut rootCategoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>, mut parentCategoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>, mut categoryPropertiesHashMap: Option<Rc<RefCell<HashMap>>>) -> Option<Rc<RefCell<dyn CategoryInterface>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).CATEGORY.clone()) {
                jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getInstance(CategoryInterface, CategoryInterface, HashMap)"));
            }
            let mut categoryPropertiesFactoryInterface: Option<Rc<RefCell<dyn CategoryPropertiesFactoryInterface>>> = (CategoryPropertiesFactory::new1(categoryPropertiesHashMap.clone())).clone() /* as CategoryPropertiesFactoryInterface */;
            return (Category::new3(rootCategoryInterface.clone(), parentCategoryInterface.clone(), categoryPropertiesFactoryInterface.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::category::Category::CategoryDyn>>)).clone() /* as CategoryInterface */;
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).CATEGORYERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getInstance(CategoryInterface, CategoryInterface, HashMap)"), crate::java_into_object(e.clone()));
            }
            return None;
            }
        }
    }
}
pub trait CategoryAbstractFactoryDyn {
    fn as_CategoryAbstractFactory(&self) -> &CategoryAbstractFactory;
    fn as_CategoryAbstractFactory_mut(&mut self) -> &mut CategoryAbstractFactory;
    fn getRootInstance(&mut self) -> Option<Rc<RefCell<dyn CategoryInterface>>> { CategoryAbstractFactory::getRootInstance(self.as_CategoryAbstractFactory_mut()) }
    fn getRootInstance1(&mut self, mut categoryAbPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>) -> Option<Rc<RefCell<dyn CategoryInterface>>> { CategoryAbstractFactory::getRootInstance1(self.as_CategoryAbstractFactory_mut(), categoryAbPath) }
    fn getRootInstanceFromNode(&mut self, mut node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<dyn CategoryInterface>>> { CategoryAbstractFactory::getRootInstanceFromNode(self.as_CategoryAbstractFactory_mut(), node) }
}
impl CategoryAbstractFactoryDyn for CategoryAbstractFactory {
    fn as_CategoryAbstractFactory(&self) -> &CategoryAbstractFactory { self }
    fn as_CategoryAbstractFactory_mut(&mut self) -> &mut CategoryAbstractFactory { self }
}


