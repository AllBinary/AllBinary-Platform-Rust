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
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
// use crate::javax::servlet::http::HttpServletRequest::HttpServletRequest;
use crate::org::allbinary::business::category::properties::root::store::RootStoreCategoryProperties::RootStoreCategoryProperties;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontFactory::StoreFrontFactory;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontInterface::StoreFrontInterface;
use crate::org::allbinary::globals::FREEBLISKET_PATH_GLOBALS::FREEBLISKET_PATH_GLOBALS;
use crate::org::allbinary::globals::URLGLOBALS::URLGLOBALS;
use crate::org::allbinary::logic::io::path::AbPath::AbPath;
use crate::org::allbinary::logic::io::path::AbPathData::AbPathData;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpStoreInterface::TransformInfoHttpStoreInterface;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use crate::org::w3c::dom::Node::Node;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RootStoreThemeCategoryProperties {
    pub base: RootStoreCategoryProperties,
}


impl RootStoreThemeCategoryProperties {
    

    pub fn new_struct1(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Self {
        let mut this = Self {
            base: RootStoreCategoryProperties::new_struct1(transformInfoInterface.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new1(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(transformInfoInterface))))
    }
    

    pub fn new_struct2(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut abPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>) -> Self {
        let mut this = Self {
            base: RootStoreCategoryProperties::new_struct2(transformInfoInterface.clone(), abPath.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new2(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut abPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2(transformInfoInterface, abPath))))
    }
    

    pub fn new_struct2(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut node: Option<Rc<RefCell<Node>>>) -> Self {
        let mut this = Self {
            base: RootStoreCategoryProperties::new_struct2(transformInfoInterface.clone(), node.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new2(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2(transformInfoInterface, node))))
    }
    

    pub fn new_struct2(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut categoryPropertiesHashMap: Option<Rc<RefCell<HashMap>>>) -> Self {
        let mut this = Self {
            base: RootStoreCategoryProperties::new_struct2(transformInfoInterface.clone(), categoryPropertiesHashMap.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new2(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut categoryPropertiesHashMap: Option<Rc<RefCell<HashMap>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2(transformInfoInterface, categoryPropertiesHashMap))))
    }
    

    // throws Exception
    pub fn initPath(&mut self) {
        let mut transformInfoHttpStoreInterface: Option<Rc<RefCell<dyn TransformInfoHttpStoreInterface>>> = (self.transformInfoInterface).clone() /* as TransformInfoHttpStoreInterface */;
        let mut storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>> = StoreFrontFactory::getInstance(jcall!(transformInfoHttpStoreInterface).getStoreName());
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(jcall!(AbPathData::getInstance()).SEPARATOR.clone());
        jcall!(stringBuffer).append(jcall!(FREEBLISKET_PATH_GLOBALS::getInstance()).XSLPATH.clone());
        jcall!(stringBuffer).append(jcall!(storeFrontInterface).getName());
        jcall!(stringBuffer).append(jcall!(AbPathData::getInstance()).SEPARATOR.clone());
        jcall!(stringBuffer).append(jcall!(FREEBLISKET_PATH_GLOBALS::getInstance()).THEMEPATH.clone());
        let mut abPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> = AbPath::new(jcall!(stringBuffer).toString(), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
        let mut httpServletRequest: Option<Rc<RefCell<HttpServletRequest>>> = (jcall!(jcall!(transformInfoHttpStoreInterface).getPageContext()).getRequest()).clone() /* as HttpServletRequest */;
        self.webAppAbPath = AbPath::new(format!("{}{}", jcall!(httpServletRequest).getContextPath(), jcall!(abPath).toString()), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
        { let java_arguments = (AbPath::new(format!("{}{}", URLGLOBALS::getMainPath(), jcall!(abPath).toString()), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>),); self.setRootFilePath(java_arguments.0) };
    }
}
impl std::ops::Deref for RootStoreThemeCategoryProperties {
    type Target = RootStoreCategoryProperties;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for RootStoreThemeCategoryProperties {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::business::category::properties::root::store::RootStoreCategoryProperties::RootStoreCategoryPropertiesDyn for RootStoreThemeCategoryProperties {
    fn as_RootStoreCategoryProperties(&self) -> &crate::org::allbinary::business::category::properties::root::store::RootStoreCategoryProperties::RootStoreCategoryProperties { &self.base }
    fn as_RootStoreCategoryProperties_mut(&mut self) -> &mut crate::org::allbinary::business::category::properties::root::store::RootStoreCategoryProperties::RootStoreCategoryProperties { &mut self.base }
    fn initPath(&mut self) { Self::initPath(self) }
}
impl crate::org::allbinary::business::category::properties::root::RootCategoryPropertiesInterface::RootCategoryPropertiesInterface for RootStoreThemeCategoryProperties {
    fn setRootFilePath(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>) { crate::org::allbinary::business::category::properties::root::store::RootStoreCategoryProperties::RootStoreCategoryProperties::setRootFilePath(crate::org::allbinary::business::category::properties::root::store::RootStoreCategoryProperties::RootStoreCategoryPropertiesDyn::as_RootStoreCategoryProperties_mut(self), p1) }
    fn getRootFilePath(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> { crate::org::allbinary::business::category::properties::root::store::RootStoreCategoryProperties::RootStoreCategoryProperties::getRootFilePath(crate::org::allbinary::business::category::properties::root::store::RootStoreCategoryProperties::RootStoreCategoryPropertiesDyn::as_RootStoreCategoryProperties_mut(self)) }
}


