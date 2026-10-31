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
use super::*;
use crate::org::allbinary::data::tree::dom::ModDomHelper::ModDomHelper;
use crate::org::allbinary::globals::FREEBLISKET_PATH_GLOBALS::FREEBLISKET_PATH_GLOBALS;
use crate::org::allbinary::globals::URLGLOBALS::URLGLOBALS;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use super::SearchParams::SearchParams;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TableSearchRequest {
    pub xslFile: String,
    pub pageName: String,
    pub contentType: String,
    pub keywords: String,
    pub searchParams: Option<Rc<RefCell<SearchParams>>>,
}


impl TableSearchRequest {
    

    pub fn new_struct(mut keywords: String, mut searchParams: Option<Rc<RefCell<SearchParams>>>, mut xslFile: String, mut contentType: String) -> Self {
        let mut this = Self {
            xslFile: String::new(),
            pageName: String::new(),
            contentType: String::new(),
            keywords: String::new(),
            searchParams: None,
        };
        this.keywords = keywords.clone();
        this.searchParams = searchParams.clone();
        this.xslFile = xslFile.clone();
        this.pageName = this.pageName.clone();
        this.contentType = contentType.clone();
        this
    }

    pub fn new(mut keywords: String, mut searchParams: Option<Rc<RefCell<SearchParams>>>, mut xslFile: String, mut contentType: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(keywords, searchParams, xslFile, contentType))))
    }
    

    pub fn setKeywords(&mut self, mut value: String) {
        self.keywords = value.clone();
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
    

    pub fn getKeywords(&mut self) -> String {
        return self.keywords.clone();
    }
    

    pub fn getParams(&mut self) -> Option<Rc<RefCell<SearchParams>>> {
        return self.searchParams.clone();
    }
    

    // throws Exception
    pub fn getXslFile(&mut self) -> String {
        let mut path: String = format!("{}{}", format!("{}{}", URLGLOBALS::getMainPath(), jcall!(FREEBLISKET_PATH_GLOBALS::getInstance()).XSLPATH), self.xslFile);
        return path.clone();
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

