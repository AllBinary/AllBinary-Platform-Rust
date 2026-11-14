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
use crate::java::util::HashMap::HashMap;
use crate::java::util::Set::Set;
// use crate::javax::servlet::http::HttpServletRequest::HttpServletRequest;
use crate::org::allbinary::data::tree::dom::ModDomHelper::ModDomHelper;
use crate::org::allbinary::data::tree::dom::ToDomHelper::ToDomHelper;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use crate::java::lang::Exception::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SearchParams {
    pub columnsAndSearchValues: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>,
    pub order: String,
    pub sortBy: String,
    pub listLength: String,
    pub startPage: String,
    pub endPage: String,
}


impl SearchParams {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn new_struct(mut request: Option<Rc<RefCell<HttpServletRequest>>>) -> Self {
        let mut this = Self {
            columnsAndSearchValues: None,
            order: String::new(),
            sortBy: String::new(),
            listLength: String::new(),
            startPage: String::new(),
            endPage: String::new(),
        };
        this.columnsAndSearchValues = { let java_value = (jcall!(StdUtil::getInstance()).createHashMap()).clone(); java_value };
        { let java_arguments = (jcall!(request).getParameter(SearchData_LENGTH.with(Clone::clone).clone()),); this.setLength(java_arguments.0) };
        { let java_arguments = (jcall!(request).getParameter(SearchData_ORDER.with(Clone::clone).clone()),); this.setOrder(java_arguments.0) };
        let mut page: String = jcall!(request).getParameter(SearchData_PAGE.with(Clone::clone).clone());
        this.setStartPage(page.clone());
        this.setEndPage(page.clone());
        { let java_arguments = (jcall!(request).getParameter(SearchData_SORTBY.with(Clone::clone).clone()),); this.setSortBy(java_arguments.0) };
        let mut index: i32 = 0;
        let mut columnName: String = jcall!(request).getParameter(format!("{}{}", SearchData_COLUMNNAME.with(Clone::clone), String::from("[0]")));
        let mut columnValue: String = jcall!(request).getParameter(format!("{}{}", SearchData_COLUMNVALUE.with(Clone::clone), String::from("[0]")));
        while true {
            this.add(columnName.clone(), columnValue.clone());
            index += 1;
            columnName = { let java_value = (jcall!(request).getParameter(format!("{}{}", format!("{}{}", format!("{}{}", SearchData_COLUMNNAME.with(Clone::clone), String::from("[")), index), String::from("]")))).clone(); java_value };
            columnValue = { let java_value = (jcall!(request).getParameter(format!("{}{}", format!("{}{}", format!("{}{}", SearchData_COLUMNVALUE.with(Clone::clone), String::from("[")), index), String::from("]")))).clone(); java_value };
        }
        this
    }

    pub fn new(mut request: Option<Rc<RefCell<HttpServletRequest>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(request))))
    }
    

    pub fn add(&mut self, mut column: String, mut value: String) {
        jcall!(self.columnsAndSearchValues).put(crate::java_into_object(column.clone()), crate::java_into_object(value.clone()));
    }
    

    pub fn setOrder(&mut self, mut value: String) {
        self.order = value.clone();
    }
    

    pub fn setSortBy(&mut self, mut value: String) {
        self.sortBy = value.clone();
    }
    

    pub fn setLength(&mut self, mut value: String) {
        self.listLength = value.clone();
    }
    

    pub fn setStartPage(&mut self, mut value: String) {
        self.startPage = value.clone();
    }
    

    pub fn setEndPage(&mut self, mut value: String) {
        self.endPage = value.clone();
    }
    

    pub fn get(&mut self) -> Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> {
        return self.columnsAndSearchValues.clone();
    }
    

    pub fn getOrder(&mut self) -> String {
        return self.order.clone();
    }
    

    pub fn getSortBy(&mut self) -> String {
        return self.sortBy.clone();
    }
    

    pub fn getLength(&mut self) -> String {
        return self.listLength.clone();
    }
    

    pub fn getLengthInt(&mut self) -> Option<i32> {
        if true {
            return Some((self.listLength.clone()).trim().parse::<i32>().expect("NumberFormatException"));
        } else {
            return Some((0) as i32);
        }
    }
    

    pub fn getStartPage(&mut self) -> String {
        return self.startPage.clone();
    }
    

    pub fn getEndPage(&mut self) -> String {
        return self.endPage.clone();
    }
    

    pub fn getStartPageInt(&mut self) -> Option<i32> {
        if true {
            return Some((self.startPage.clone()).trim().parse::<i32>().expect("NumberFormatException"));
        } else {
            return Some((0) as i32);
        }
    }
    

    pub fn getEndPageInt(&mut self) -> Option<i32> {
        if true {
            return Some((self.endPage.clone()).trim().parse::<i32>().expect("NumberFormatException"));
        } else {
            return Some((0) as i32);
        }
    }
    

    // throws Exception
    pub fn getParamsNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        let mut paramsNode: Option<Rc<RefCell<Node>>> = ModDomHelper::createNameValueNodes_Document_String_HashMap(document.clone(), SearchData_PARAMS.with(Clone::clone).clone(), SearchData_PARAMS.with(Clone::clone).clone());
        jcall!(paramsNode).appendChild(self.getFieldsNode(document.clone()));
        jcall!(paramsNode).appendChild(self.getOrderNode(document.clone()));
        jcall!(paramsNode).appendChild(self.getSortByNode(document.clone()));
        jcall!(paramsNode).appendChild(self.getLengthNode(document.clone()));
        return paramsNode.clone();
    }
    

    // throws Exception
    pub fn getFieldsNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        let mut fieldsNode: Option<Rc<RefCell<Node>>> = ModDomHelper::createNameValueNodes_Document_String_HashMap(document.clone(), SearchData_FIELDS.with(Clone::clone).clone(), (Some((jcall!(self.columnsAndSearchValues).size()) as i32)).expect("NullPointerException").to_string());
        let mut set: Option<Rc<RefCell<dyn Set>>> = jcall!(self.columnsAndSearchValues).keySet();
        let mut searchValueArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(set).toArray();
        let mut size: i32 = (searchValueArray.len() as i32).clone();
        for index in (0..size) {
            let mut key: String = { let java_object = (searchValueArray[(index) as usize]).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
            let mut searchValue: String = { let java_object = (jcall!(self.columnsAndSearchValues).get(crate::java_into_object(key.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
            jcall!(fieldsNode).appendChild(ModDomHelper::createNameValueNodes4_Document_String_String_BasicArrayList(document.clone(), SearchData_FIELD.with(Clone::clone).clone(), key.clone(), ToDomHelper::convertNull(searchValue.clone())));
        }
        return fieldsNode.clone();
    }
    

    // throws Exception
    pub fn getOrderNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        return ModDomHelper::createNameValueNodes_Document_String_HashMap(document.clone(), SearchData_ORDER.with(Clone::clone).clone(), { let java_argument = ToDomHelper::convertNull(self.getOrder()); java_argument });
    }
    

    // throws Exception
    pub fn getSortByNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        return ModDomHelper::createNameValueNodes_Document_String_HashMap(document.clone(), SearchData_SORTBY.with(Clone::clone).clone(), { let java_argument = ToDomHelper::convertNull(self.getSortBy()); java_argument });
    }
    

    // throws Exception
    pub fn getLengthNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        return ModDomHelper::createNameValueNodes_Document_String_HashMap(document.clone(), SearchData_LENGTH.with(Clone::clone).clone(), { let java_argument = ToDomHelper::convertNull(self.getLength()); java_argument });
    }
}

