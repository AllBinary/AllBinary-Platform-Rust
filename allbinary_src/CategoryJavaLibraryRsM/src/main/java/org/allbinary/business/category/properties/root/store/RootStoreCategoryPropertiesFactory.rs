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
use crate::org::allbinary::business::category::properties::CategoryPropertiesFactoryInterface::CategoryPropertiesFactoryInterface;
use crate::org::allbinary::business::category::properties::CategoryPropertiesInterface::CategoryPropertiesInterface;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::io::path::AbPath::AbPath;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::w3c::dom::Node::Node;
use super::RootStoreCategoryProperties::RootStoreCategoryProperties;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RootStoreCategoryPropertiesFactory {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub categoryPropertiesInterface: Option<Rc<RefCell<dyn CategoryPropertiesInterface>>>,
}

impl CategoryPropertiesFactoryInterface for RootStoreCategoryPropertiesFactory {
    fn getInstance(&mut self) -> Option<Rc<RefCell<dyn CategoryPropertiesInterface>>> { Self::getInstance(self) }
}

impl RootStoreCategoryPropertiesFactory {
    

    pub fn new_struct1(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            categoryPropertiesInterface: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.categoryPropertiesInterface = (RootStoreCategoryProperties::new1(transformInfoInterface.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::category::properties::root::store::RootStoreCategoryProperties::RootStoreCategoryPropertiesDyn>>)).clone() /* as CategoryPropertiesInterface */;
        this
    }

    pub fn new1(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(transformInfoInterface))))
    }
    

    pub fn new_struct2(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut abPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            categoryPropertiesInterface: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.categoryPropertiesInterface = (RootStoreCategoryProperties::new2(transformInfoInterface.clone(), abPath.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::category::properties::root::store::RootStoreCategoryProperties::RootStoreCategoryPropertiesDyn>>)).clone() /* as CategoryPropertiesInterface */;
        this
    }

    pub fn new2(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut abPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2(transformInfoInterface, abPath))))
    }
    

    pub fn new_struct2(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut node: Option<Rc<RefCell<Node>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            categoryPropertiesInterface: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.categoryPropertiesInterface = (RootStoreCategoryProperties::new2(transformInfoInterface.clone(), node.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::category::properties::root::store::RootStoreCategoryProperties::RootStoreCategoryPropertiesDyn>>)).clone() /* as CategoryPropertiesInterface */;
        this
    }

    pub fn new2(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2(transformInfoInterface, node))))
    }
    

    pub fn new_struct2(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut categoryPropertiesHashMap: Option<Rc<RefCell<HashMap>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            categoryPropertiesInterface: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.categoryPropertiesInterface = (RootStoreCategoryProperties::new2(transformInfoInterface.clone(), categoryPropertiesHashMap.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::category::properties::root::store::RootStoreCategoryProperties::RootStoreCategoryPropertiesDyn>>)).clone() /* as CategoryPropertiesInterface */;
        this
    }

    pub fn new2(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut categoryPropertiesHashMap: Option<Rc<RefCell<HashMap>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2(transformInfoInterface, categoryPropertiesHashMap))))
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

