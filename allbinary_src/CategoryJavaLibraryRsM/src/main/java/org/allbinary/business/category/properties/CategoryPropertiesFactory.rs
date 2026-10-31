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
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::w3c::dom::Node::Node;
use super::CategoryPropertiesFactoryInterface::CategoryPropertiesFactoryInterface;
use super::CategoryPropertiesInterface::CategoryPropertiesInterface;
use super::CategoryProperties::CategoryProperties;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CategoryPropertiesFactory {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub categoryPropertiesInterface: Option<Rc<RefCell<dyn CategoryPropertiesInterface>>>,
}

impl CategoryPropertiesFactoryInterface for CategoryPropertiesFactory {
    fn getInstance(&mut self) -> Option<Rc<RefCell<dyn CategoryPropertiesInterface>>> { Self::getInstance(self) }
}

impl CategoryPropertiesFactory {
    

    pub fn new_struct1(mut name: String) -> Self {
        let mut this = Self {
            logUtil: None,
            categoryPropertiesInterface: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.categoryPropertiesInterface = (CategoryProperties::new1(name.clone())).clone() /* as CategoryPropertiesInterface */;
        this
    }

    pub fn new1(mut name: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(name))))
    }
    

    pub fn new_struct1(mut node: Option<Rc<RefCell<Node>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            categoryPropertiesInterface: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.categoryPropertiesInterface = (CategoryProperties::new1(node.clone())).clone() /* as CategoryPropertiesInterface */;
        this
    }

    pub fn new1(mut node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(node))))
    }
    

    pub fn new_struct1(mut categoryPropertiesHashMap: Option<Rc<RefCell<HashMap>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            categoryPropertiesInterface: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.categoryPropertiesInterface = (CategoryProperties::new1(categoryPropertiesHashMap.clone())).clone() /* as CategoryPropertiesInterface */;
        this
    }

    pub fn new1(mut categoryPropertiesHashMap: Option<Rc<RefCell<HashMap>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(categoryPropertiesHashMap))))
    }
    

    pub fn getInstance(&mut self) -> Option<Rc<RefCell<dyn CategoryPropertiesInterface>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            return (self.categoryPropertiesInterface).clone() /* as CategoryPropertiesInterface */;
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).ENTITYFACTORYERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).GET_INSTANCE.clone(), crate::java_into_object(e.clone()));
            }
            return None;
            }
        }
    }
}

