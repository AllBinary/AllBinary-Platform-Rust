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
// package org.allbinary.logic.communication.http.request
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
// use crate::java::util::Map::Map;
// use crate::java::util::Set::Set;
// use crate::javax::servlet::http::HttpServletRequest::HttpServletRequest;
// use crate::javax::servlet::jsp::PageContext::PageContext;
use crate::org::allbinary::data::tree::dom::ModDomHelper::ModDomHelper;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//Note: if request is destroyed or messed up this will not work. Use fileRequestParams
pub struct RequestParams {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub map: Option<Rc<RefCell<dyn Map>>>,
    pub KEY: String,
    pub VALUE: String,
}


impl RequestParams {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            map: None,
            KEY: String::new(),
            VALUE: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.KEY = String::from("Key: ");
        this.VALUE = String::from(" Value: ");
        jcall!(this.logUtil).putF(String::from("New RequestParams Size: 0"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone());
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn new_struct1(mut request: Option<Rc<RefCell<HttpServletRequest>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            map: None,
            KEY: String::new(),
            VALUE: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.KEY = String::from("Key: ");
        this.VALUE = String::from(" Value: ");
        this.map = jcall!(request).getParameterMap();
        { let java_receiver = this.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("RequestParams Size: "), jcall!(jcall!(this.getMap()).keySet()).size()), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        this
    }

    pub fn new1(mut request: Option<Rc<RefCell<HttpServletRequest>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(request))))
    }
    

    pub fn new_struct1(mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            map: None,
            KEY: String::new(),
            VALUE: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.KEY = String::from("Key: ");
        this.VALUE = String::from(" Value: ");
        this.map = jcall!(jcall!(pageContext).getRequest()).getParameterMap();
        { let java_receiver = this.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Request Params Size: "), jcall!(jcall!(this.getMap()).keySet()).size()), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        this
    }

    pub fn new1(mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(pageContext))))
    }
    

    pub fn setMap(&mut self, mut map: Option<Rc<RefCell<dyn Map>>>) {
        self.map = map.clone();
    }
    

    pub fn getMap(&mut self) -> Option<Rc<RefCell<dyn Map>>> {
        return self.map.clone();
    }
    

    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut node: Option<Rc<RefCell<Node>>> = jcall!(document).createElement(RequestData_REQUEST.with(Clone::clone).clone());
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            let mut keys: Option<Rc<RefCell<dyn Set>>> = jcall!(self.map).keySet();
            let mut keyArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(keys).toArray();
            let mut size: i32 = (keyArray.len() as i32).clone();
            for i in (0..size) {
                let mut key: String = (keyArray[(i) as usize]).clone() /* as String */;
                let mut values: Vec<String> = (jcall!(self.map).get(key.clone())).clone() /* as  */;
                jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
                jcall!(stringBuffer).append(self.KEY.clone());
                jcall!(stringBuffer).append(key.clone());
                jcall!(stringBuffer).append(self.VALUE.clone());
                jcall!(stringBuffer).append(values[(0) as usize].clone());
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("toXmlNode(document)"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                jcall!(node).appendChild(ModDomHelper::createNameValueNodes4(document.clone(), RequestData_PARAMETER.with(Clone::clone).clone(), key.clone().clone(), values[(0) as usize].clone().clone()));
            }
            return node.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("toXmlNode(document)"), crate::java_into_object(e.clone()));
            return None;
            }
        }
    }
    

    // throws Exception
    pub fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> {
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        let mut keys: Option<Rc<RefCell<dyn Set>>> = jcall!(self.map).keySet();
        let mut keyArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(keys).toArray();
        let mut size: i32 = (keyArray.len() as i32).clone();
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        for i in (0..size) {
            let mut key: String = (keyArray[(i) as usize]).clone() /* as String */;
            let mut values: Vec<String> = (jcall!(self.map).get(key.clone())).clone() /* as  */;
            jcall!(hashMap).put(key.clone().clone(), values[(0) as usize].clone().clone());
        }
        jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
        jcall!(stringBuffer).append(String::from("RequestParams: "));
        jcall!(stringBuffer).append(jcall!(self.map).toString());
        jcall!(stringBuffer).append(String::from("\ntoHashMap: "));
        jcall!(stringBuffer).append(jcall!(hashMap).toString());
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("toHashMap()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        return hashMap.clone();
    }
}
pub trait RequestParamsDyn {
    fn as_RequestParams(&self) -> &RequestParams;
    fn as_RequestParams_mut(&mut self) -> &mut RequestParams;
    fn setMap(&mut self, mut map: Option<Rc<RefCell<dyn Map>>>) { RequestParams::setMap(self.as_RequestParams_mut(), map) }
    fn getMap(&mut self) -> Option<Rc<RefCell<dyn Map>>> { RequestParams::getMap(self.as_RequestParams_mut()) }
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { RequestParams::toXmlNode(self.as_RequestParams_mut(), document) }
    fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> { RequestParams::toHashMap(self.as_RequestParams_mut()) }
}
impl RequestParamsDyn for RequestParams {
    fn as_RequestParams(&self) -> &RequestParams { self }
    fn as_RequestParams_mut(&mut self) -> &mut RequestParams { self }
}


