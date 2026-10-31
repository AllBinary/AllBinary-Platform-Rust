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
// package org.allbinary.logic.visual.transform.info
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
// use crate::javax::servlet::jsp::PageContext::PageContext;
use crate::org::allbinary::business::context::AbContext::AbContext;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontInterface::StoreFrontInterface;
use crate::org::allbinary::logic::io::path::AbPath::AbPath;
use crate::org::allbinary::logic::visual::transform::info::objectConfig::TransformInfoObjectConfigInterface::TransformInfoObjectConfigInterface;
use crate::org::w3c::dom::Document::Document;
use super::TransformInfoHttpInterface::TransformInfoHttpInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//Hack class should not exist
pub struct TransformInfoBasic {
    pub base: AbContext,
    pub storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>>,
}

// impl TransformInfoHttpInterface for TransformInfoBasic with the methods of the trait

impl TransformInfoBasic {
    

    pub fn new_struct(mut storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>>, mut propertiesHashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Self {
        let mut this = Self {
            base: AbContext::new_struct(propertiesHashMap.clone(), pageContext.clone()),
            storeFrontInterface: None,
        };
        // super(...) is in the struct literal above
        this.storeFrontInterface = storeFrontInterface.clone();
        this
    }

    pub fn new(mut storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>>, mut propertiesHashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(storeFrontInterface, propertiesHashMap, pageContext))))
    }
    

    pub fn getStoreName(&mut self) -> String {
        return jcall!(self.storeFrontInterface).getName();
    }
    

    pub fn log(&mut self) -> String {
        return jcall!(self.base.commonStrings).NOT_IMPLEMENTED.clone();
    }
    

    pub fn r#override(&mut self, mut hashMap: Option<Rc<RefCell<HashMap>>>) {
    }
    

    pub fn getName(&mut self) -> String {
        return None;
    }
    

    pub fn getObjectFile(&mut self) -> String {
        return None;
    }
    

    pub fn getObject(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return None;
    }
    

    pub fn getObjectConfigInterface(&mut self) -> Option<Rc<RefCell<dyn TransformInfoObjectConfigInterface>>> {
        return None;
    }
    

    pub fn setObjectConfigInterface(&mut self, mut transformInfoObjectConfigInterface: Option<Rc<RefCell<dyn TransformInfoObjectConfigInterface>>>) {
    }
    

    pub fn getTemplateFilePath(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> {
        return None;
    }
    

    pub fn getTemplateFile(&mut self) -> String {
        return None;
    }
    

    pub fn getTemplate(&mut self) -> String {
        return None;
    }
    

    pub fn getDataFilePath(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> {
        return None;
    }
    

    pub fn getDataFile(&mut self) -> String {
        return None;
    }
    

    // throws Exception
    pub fn getDataDocument(&mut self) -> Option<Rc<RefCell<Document>>> {
        panic!("{:?}", format!("{}: {:?}", "Exception", (jcall!(self.base.commonStrings).NOT_IMPLEMENTED.clone())));
    }
    

    pub fn setName(&mut self, mut value: String) {
    }
    

    pub fn setStoreName(&mut self, mut value: String) {
    }
    

    pub fn setObjectFile(&mut self, mut value: String) {
    }
    

    pub fn setObject(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) {
    }
    

    pub fn setObjectConfigFile(&mut self, mut value: String) {
    }
    

    pub fn setTemplateFile(&mut self, mut value: String) {
    }
    

    pub fn setTemplate(&mut self, mut value: String) {
    }
    

    pub fn setDataFile(&mut self, mut value: String) {
    }
    

    pub fn setData(&mut self, mut value: String) {
    }
    

    pub fn setChild(&mut self) {
    }
    

    pub fn isChild(&mut self) -> bool {
        return true;
    }
    

    pub fn getImportUriPath(&mut self) -> String {
        return None;
    }
    

    pub fn setImportUriPath(&mut self, mut importUriPath: String) {
    }
    

    // throws Exception
    pub fn getKey(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return None;
    }
    

    // throws Exception
    pub fn toVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return None;
    }
    

    // throws Exception
    pub fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> {
        return None;
    }
}
impl std::ops::Deref for TransformInfoBasic {
    type Target = AbContext;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for TransformInfoBasic {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::business::context::AbContext::AbContextDyn for TransformInfoBasic {
    fn as_AbContext(&self) -> &crate::org::allbinary::business::context::AbContext::AbContext { &self.base }
    fn as_AbContext_mut(&mut self) -> &mut crate::org::allbinary::business::context::AbContext::AbContext { &mut self.base }
}


