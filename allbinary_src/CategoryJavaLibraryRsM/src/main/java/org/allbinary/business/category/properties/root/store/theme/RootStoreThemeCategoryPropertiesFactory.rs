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
// package org.allbinary.business.category.properties.root.store.theme
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::business::category::properties::CategoryPropertiesFactoryInterface::CategoryPropertiesFactoryInterface;
use crate::org::allbinary::business::category::properties::CategoryPropertiesInterface::CategoryPropertiesInterface;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::io::path::AbPath::AbPath;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::w3c::dom::Node::Node;
use crate::java::lang::Exception::*;
use super::RootStoreThemeCategoryProperties::RootStoreThemeCategoryProperties;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RootStoreThemeCategoryPropertiesFactory {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub categoryPropertiesInterface: Option<Rc<RefCell<dyn CategoryPropertiesInterface>>>,
}

impl CategoryPropertiesFactoryInterface for RootStoreThemeCategoryPropertiesFactory {
    fn getInstance(&mut self) -> Option<Rc<RefCell<dyn CategoryPropertiesInterface>>> { Self::getInstance(self) }
}

impl RootStoreThemeCategoryPropertiesFactory {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn new_struct1(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            categoryPropertiesInterface: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.categoryPropertiesInterface = (RootStoreThemeCategoryProperties::new1(transformInfoInterface.clone())).clone() /* as CategoryPropertiesInterface */;
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
        this.categoryPropertiesInterface = (RootStoreThemeCategoryProperties::new2(transformInfoInterface.clone(), abPath.clone())).clone() /* as CategoryPropertiesInterface */;
        this
    }

    pub fn new2(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut abPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2(transformInfoInterface, abPath))))
    }
    

    pub fn new_struct2_TransformInfoInterface_Node(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut node: Option<Rc<RefCell<Node>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            categoryPropertiesInterface: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.categoryPropertiesInterface = (RootStoreThemeCategoryProperties::new2_TransformInfoInterface_Node(transformInfoInterface.clone(), node.clone())).clone() /* as CategoryPropertiesInterface */;
        this
    }

    pub fn new2_TransformInfoInterface_Node(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2_TransformInfoInterface_Node(transformInfoInterface, node))))
    }
    

    pub fn new_struct2_TransformInfoInterface_HashMap(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut categoryPropertiesHashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            categoryPropertiesInterface: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.categoryPropertiesInterface = (RootStoreThemeCategoryProperties::new2_TransformInfoInterface_HashMap(transformInfoInterface.clone(), categoryPropertiesHashMap.clone())).clone() /* as CategoryPropertiesInterface */;
        this
    }

    pub fn new2_TransformInfoInterface_HashMap(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut categoryPropertiesHashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2_TransformInfoInterface_HashMap(transformInfoInterface, categoryPropertiesHashMap))))
    }
    

    pub fn getInstance(&mut self) -> Option<Rc<RefCell<dyn CategoryPropertiesInterface>>> {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            return Some((self.categoryPropertiesInterface).clone());
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).ENTITYFACTORYERROR.clone(); java_field }.clone()) {
                jcall!(self.logUtil).put({ let java_field = jcall!(commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(commonStrings).GET_INSTANCE.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
            return None;
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
}

