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
// package org.allbinary.business.category.store
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::business::category::Category::Category;
use crate::org::allbinary::business::category::CategoryAbstractFactory::CategoryAbstractFactory;
use crate::org::allbinary::business::category::CategoryFactoryInterface::CategoryFactoryInterface;
use crate::org::allbinary::business::category::CategoryInterface::CategoryInterface;
use crate::org::allbinary::business::category::CategoryUtil::CategoryUtil;
use crate::org::allbinary::business::category::properties::CategoryPropertiesFactoryInterface::CategoryPropertiesFactoryInterface;
use crate::org::allbinary::business::category::properties::root::store::RootStoreCategoryPropertiesFactory::RootStoreCategoryPropertiesFactory;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::io::path::AbPath::AbPath;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use crate::org::w3c::dom::Node::Node;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct StoreCategoryFactory {
    pub base: CategoryAbstractFactory,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>,
}

// impl CategoryFactoryInterface for StoreCategoryFactory with the methods of the trait

impl StoreCategoryFactory {
    

    pub fn new_struct(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Self {
        let mut this = Self {
            base: CategoryAbstractFactory::new_struct(),
            logUtil: None,
            transformInfoInterface: None,
        };
        this.logUtil = LogUtil::getInstance();
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
            let mut categoryPropertiesFactoryInterface: Option<Rc<RefCell<dyn CategoryPropertiesFactoryInterface>>> = (RootStoreCategoryPropertiesFactory::new1(self.transformInfoInterface.clone())).clone() /* as CategoryPropertiesFactoryInterface */;
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
    pub fn getRootInstance1(&mut self, mut categoryPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>) -> Option<Rc<RefCell<dyn CategoryInterface>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut level: i32 = CategoryUtil::getPathLevel(categoryPath.clone());
            let mut categoryPropertiesFactoryInterface: Option<Rc<RefCell<dyn CategoryPropertiesFactoryInterface>>> = (RootStoreCategoryPropertiesFactory::new2(self.transformInfoInterface.clone(), categoryPath.clone())).clone() /* as CategoryPropertiesFactoryInterface */;
            return (Category::new2(categoryPropertiesFactoryInterface.clone(), level.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::category::Category::CategoryDyn>>)).clone() /* as CategoryInterface */;
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).ENTITYFACTORYERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.base.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getInstance(String, int)"), crate::java_into_object(e.clone()));
            }
            return None;
            }
        }
    }
    

    pub fn getRootInstanceFromNode(&mut self, mut node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<dyn CategoryInterface>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut categoryPropertiesFactoryInterface: Option<Rc<RefCell<dyn CategoryPropertiesFactoryInterface>>> = (RootStoreCategoryPropertiesFactory::new2(self.transformInfoInterface.clone(), node.clone())).clone() /* as CategoryPropertiesFactoryInterface */;
            let mut categoryPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> = AbPath::new(CategoryUtil::getNameFromNode(node.clone()), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
            let mut level: i32 = CategoryUtil::getPathLevel(categoryPath.clone());
            return (Category::new2(categoryPropertiesFactoryInterface.clone(), level.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::category::Category::CategoryDyn>>)).clone() /* as CategoryInterface */;
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
}
impl std::ops::Deref for StoreCategoryFactory {
    type Target = CategoryAbstractFactory;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for StoreCategoryFactory {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::business::category::CategoryAbstractFactory::CategoryAbstractFactoryDyn for StoreCategoryFactory {
    fn as_CategoryAbstractFactory(&self) -> &crate::org::allbinary::business::category::CategoryAbstractFactory::CategoryAbstractFactory { &self.base }
    fn as_CategoryAbstractFactory_mut(&mut self) -> &mut crate::org::allbinary::business::category::CategoryAbstractFactory::CategoryAbstractFactory { &mut self.base }
    fn getRootInstance(&mut self) -> Option<Rc<RefCell<dyn CategoryInterface>>> { Self::getRootInstance(self) }
    fn getRootInstance1(&mut self, mut categoryPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>) -> Option<Rc<RefCell<dyn CategoryInterface>>> { Self::getRootInstance1(self, categoryPath) }
    fn getRootInstanceFromNode(&mut self, mut node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<dyn CategoryInterface>>> { Self::getRootInstanceFromNode(self, node) }
}


