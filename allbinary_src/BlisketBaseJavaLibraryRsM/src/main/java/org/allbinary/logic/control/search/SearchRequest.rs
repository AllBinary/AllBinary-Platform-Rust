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
// package org.allbinary.logic.control.search
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
// use crate::javax::servlet::jsp::PageContext::PageContext;
use crate::org::allbinary::business::context::AbContext::AbContext;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontInterface::StoreFrontInterface;
use crate::org::allbinary::data::tree::dom::ModDomHelper::ModDomHelper;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use super::SearchParams::SearchParams;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SearchRequest {
    pub base: AbContext,
    pub xslFile: String,
    pub pageName: String,
    pub contentType: String,
    pub searchParams: Option<Rc<RefCell<SearchParams>>>,
    pub storeFront: Option<Rc<RefCell<dyn StoreFrontInterface>>>,
}


impl SearchRequest {
    

    pub fn new_struct(mut storeFront: Option<Rc<RefCell<dyn StoreFrontInterface>>>, mut searchParams: Option<Rc<RefCell<SearchParams>>>, mut xslFile: String, mut contentType: String, mut propertiesHashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Self {
        let mut this = Self {
            base: AbContext::new_struct(propertiesHashMap.clone(), pageContext.clone()),
            xslFile: String::new(),
            pageName: String::new(),
            contentType: String::new(),
            searchParams: None,
            storeFront: None,
        };
        // super(...) is in the struct literal above
        this.storeFront = storeFront.clone();
        this.searchParams = searchParams.clone();
        this.xslFile = xslFile.clone();
        this.pageName = this.pageName.clone();
        this.contentType = contentType.clone();
        this
    }

    pub fn new(mut storeFront: Option<Rc<RefCell<dyn StoreFrontInterface>>>, mut searchParams: Option<Rc<RefCell<SearchParams>>>, mut xslFile: String, mut contentType: String, mut propertiesHashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(storeFront, searchParams, xslFile, contentType, propertiesHashMap, pageContext))))
    }
    

    pub fn setStoreFront(&mut self, mut storeFront: Option<Rc<RefCell<dyn StoreFrontInterface>>>) {
        self.storeFront = storeFront.clone();
    }
    

    pub fn setParams(&mut self, mut searchParams: Option<Rc<RefCell<SearchParams>>>) {
        self.searchParams = searchParams.clone();
    }
    

    pub fn setXslFile(&mut self, mut value: String) {
        self.xslFile = value.clone();
    }
    

    pub fn setFileBaseName(&mut self, mut value: String) {
        self.pageName = value.clone();
    }
    

    pub fn setContentType(&mut self, mut value: String) {
        self.contentType = value.clone();
    }
    

    pub fn getStoreFront(&mut self) -> Option<Rc<RefCell<dyn StoreFrontInterface>>> {
        return self.storeFront.clone();
    }
    

    pub fn getParams(&mut self) -> Option<Rc<RefCell<SearchParams>>> {
        return self.searchParams.clone();
    }
    

    pub fn getXslFile(&mut self) -> String {
        return self.xslFile.clone();
    }
    

    pub fn getFileBaseName(&mut self) -> String {
        return self.pageName.clone();
    }
    

    pub fn getContentType(&mut self) -> String {
        return self.contentType.clone();
    }
    

    // throws Exception
    pub fn getFileBaseNameNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        return ModDomHelper::createNameValueNodes(document.clone(), SearchData_FILENAME.with(Clone::clone).clone(), self.getFileBaseName());
    }
}
impl std::ops::Deref for SearchRequest {
    type Target = AbContext;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for SearchRequest {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::business::context::AbContext::AbContextDyn for SearchRequest {
    fn as_AbContext(&self) -> &crate::org::allbinary::business::context::AbContext::AbContext { &self.base }
    fn as_AbContext_mut(&mut self) -> &mut crate::org::allbinary::business::context::AbContext::AbContext { &mut self.base }
}


