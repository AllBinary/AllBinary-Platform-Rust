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
use crate::Enumeration;
use super::*;
// use crate::java::util::Enumeration::Enumeration;
use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
// use crate::javax::servlet::http::HttpServletRequest::HttpServletRequest;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use super::AbeHttpRequestInfoData::AbeHttpRequestInfoData;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AbeHttpRequestInfo {
    pub NAME: String,
    pub abeHttpRequestInfoData: Option<Rc<RefCell<AbeHttpRequestInfoData>>>,
    pub commonSeps: Option<Rc<RefCell<CommonSeps>>>,
    pub httpUserAgent: String,
    pub remoteAddress: String,
    pub remoteHost: String,
    pub remoteHostByAddr: String,
    pub remotePort: String,
    pub requestedFilePath: String,
}


impl AbeHttpRequestInfo {
    

    pub fn new_struct1(mut hashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) -> Self {
        let mut this = Self {
            NAME: String::new(),
            abeHttpRequestInfoData: None,
            commonSeps: None,
            httpUserAgent: String::new(),
            remoteAddress: String::new(),
            remoteHost: String::new(),
            remoteHostByAddr: String::new(),
            remotePort: String::new(),
            requestedFilePath: String::new(),
        };
        this.NAME = String::from("AbeHttpRequestInfo");
        this.abeHttpRequestInfoData = AbeHttpRequestInfoData::getInstance();
        this.commonSeps = CommonSeps::getInstance();
        let mut stringUtil: Option<Rc<RefCell<StringUtil>>> = StringUtil::getInstance();
        this.httpUserAgent = { let java_value = (jcall!(stringUtil).getNonNull({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(this.abeHttpRequestInfoData).HTTP_USER_AGENT.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() })).clone(); java_value };
        this.remoteAddress = { let java_value = (jcall!(stringUtil).getNonNull({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(this.abeHttpRequestInfoData).REMOTE_ADDRESS.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() })).clone(); java_value };
        this.remoteHost = { let java_value = (jcall!(stringUtil).getNonNull({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(this.abeHttpRequestInfoData).REMOTE_HOST.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() })).clone(); java_value };
        this.remoteHostByAddr = { let java_value = (jcall!(stringUtil).getNonNull({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(this.abeHttpRequestInfoData).REMOTE_HOST_BY_ADDRESS.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() })).clone(); java_value };
        this.remotePort = { let java_value = (jcall!(stringUtil).getNonNull({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(this.abeHttpRequestInfoData).REMOTE_PORT.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() })).clone(); java_value };
        this.requestedFilePath = { let java_value = (jcall!(stringUtil).getNonNull({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(this.abeHttpRequestInfoData).REQUEST_FILE_PATH.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() })).clone(); java_value };
        this
    }

    pub fn new1(mut hashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(hashMap))))
    }
    

    pub fn new_struct1_HttpServletRequest(mut httpServletRequest: Option<Rc<RefCell<HttpServletRequest>>>) -> Self {
        let mut this = Self {
            NAME: String::new(),
            abeHttpRequestInfoData: None,
            commonSeps: None,
            httpUserAgent: String::new(),
            remoteAddress: String::new(),
            remoteHost: String::new(),
            remoteHostByAddr: String::new(),
            remotePort: String::new(),
            requestedFilePath: String::new(),
        };
        this.NAME = String::from("AbeHttpRequestInfo");
        this.abeHttpRequestInfoData = AbeHttpRequestInfoData::getInstance();
        this.commonSeps = CommonSeps::getInstance();
        let mut stringUtil: Option<Rc<RefCell<StringUtil>>> = StringUtil::getInstance();
        this.httpUserAgent = { let java_value = ({ let java_field = jcall!(stringUtil).EMPTY_STRING.clone(); java_field }).clone(); java_value }.clone();
        let mut enumuration: Option<Rc<RefCell<dyn Enumeration>>> = jcall!(httpServletRequest).getHeaderNames();
        while jcall!(enumuration).hasMoreElements() {
            let mut key: String = { let java_object = (jcall!(enumuration).nextElement()).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
            let mut value: String = jcall!(httpServletRequest).getHeader(key.clone());
            if key.find(&String::from("user")).map(|index| index as i32).unwrap_or(-1) >= 0 {
                this.httpUserAgent = format!("{}{}", format!("{}{}", format!("{}{}", String::from("key: "), key), String::from(" value: ")), value);
            }
        }
        this.remoteHostByAddr = { let java_value = ({ let java_field = jcall!(stringUtil).EMPTY_STRING.clone(); java_field }).clone(); java_value }.clone();
        this.remoteAddress = { let java_value = (jcall!(httpServletRequest).getRemoteAddr()).clone(); java_value };
        this.remoteHost = { let java_value = (jcall!(httpServletRequest).getRemoteHost()).clone(); java_value };
        this.requestedFilePath = { let java_value = (jcall!(httpServletRequest).getServletPath()).clone(); java_value };
        this.remotePort = { let java_value = ((jcall!(httpServletRequest).getRemotePort()).to_string()).clone(); java_value };
        this
    }

    pub fn new1_HttpServletRequest(mut httpServletRequest: Option<Rc<RefCell<HttpServletRequest>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1_HttpServletRequest(httpServletRequest))))
    }
    

    pub fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> {
        let mut hashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> = jcall!(StdUtil::getInstance()).createHashMap();
        jcall!(hashMap).put(crate::java_into_object({ let java_field = jcall!(self.abeHttpRequestInfoData).HTTP_USER_AGENT.clone(); java_field }.clone()), crate::java_into_object(self.httpUserAgent.clone()));
        jcall!(hashMap).put(crate::java_into_object({ let java_field = jcall!(self.abeHttpRequestInfoData).REMOTE_ADDRESS.clone(); java_field }.clone()), crate::java_into_object(self.remoteAddress.clone()));
        jcall!(hashMap).put(crate::java_into_object({ let java_field = jcall!(self.abeHttpRequestInfoData).REMOTE_HOST.clone(); java_field }.clone()), crate::java_into_object(self.remoteHost.clone()));
        jcall!(hashMap).put(crate::java_into_object({ let java_field = jcall!(self.abeHttpRequestInfoData).REMOTE_HOST_BY_ADDRESS.clone(); java_field }.clone()), crate::java_into_object(self.remoteHostByAddr.clone()));
        jcall!(hashMap).put(crate::java_into_object({ let java_field = jcall!(self.abeHttpRequestInfoData).REMOTE_PORT.clone(); java_field }.clone()), crate::java_into_object(self.remotePort.clone()));
        jcall!(hashMap).put(crate::java_into_object({ let java_field = jcall!(self.abeHttpRequestInfoData).REQUEST_FILE_PATH.clone(); java_field }.clone()), crate::java_into_object(self.requestedFilePath.clone()));
        return hashMap.clone();
    }
    

    pub fn toVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        jcall!(vector).add(crate::java_into_object(self.httpUserAgent.clone()));
        jcall!(vector).add(crate::java_into_object(self.remoteAddress.clone()));
        jcall!(vector).add(crate::java_into_object(self.remoteHost.clone()));
        jcall!(vector).add(crate::java_into_object(self.remoteHostByAddr.clone()));
        jcall!(vector).add(crate::java_into_object(self.remotePort.clone()));
        jcall!(vector).add(crate::java_into_object(self.requestedFilePath.clone()));
        return vector.clone();
    }
    

    pub fn toString(&mut self) -> String {
        return jcall!(StringMaker::new()).append(self.NAME.clone()).append({ let java_field = jcall!(self.commonSeps).COLON_SEP.clone(); java_field }.clone()).append({ let java_field = jcall!(self.abeHttpRequestInfoData).HTTP_USER_AGENT.clone(); java_field }.clone()).append({ let java_field = jcall!(self.commonSeps).COLON_SEP.clone(); java_field }.clone()).append(self.httpUserAgent.clone()).append({ let java_field = jcall!(self.commonSeps).SPACE.clone(); java_field }.clone()).append({ let java_field = jcall!(self.abeHttpRequestInfoData).REMOTE_ADDRESS.clone(); java_field }.clone()).append({ let java_field = jcall!(self.commonSeps).COLON_SEP.clone(); java_field }.clone()).append(self.remoteAddress.clone()).append({ let java_field = jcall!(self.commonSeps).SPACE.clone(); java_field }.clone()).append({ let java_field = jcall!(self.abeHttpRequestInfoData).REMOTE_HOST.clone(); java_field }.clone()).append({ let java_field = jcall!(self.commonSeps).COLON_SEP.clone(); java_field }.clone()).append(self.remoteHost.clone()).append({ let java_field = jcall!(self.commonSeps).SPACE.clone(); java_field }.clone()).append({ let java_field = jcall!(self.abeHttpRequestInfoData).REMOTE_HOST_BY_ADDRESS.clone(); java_field }.clone()).append({ let java_field = jcall!(self.commonSeps).COLON_SEP.clone(); java_field }.clone()).append(self.remoteHostByAddr.clone()).append({ let java_field = jcall!(self.commonSeps).SPACE.clone(); java_field }.clone()).append({ let java_field = jcall!(self.abeHttpRequestInfoData).REMOTE_PORT.clone(); java_field }.clone()).append({ let java_field = jcall!(self.commonSeps).COLON_SEP.clone(); java_field }.clone()).append(self.remotePort.clone()).append({ let java_field = jcall!(self.commonSeps).SPACE.clone(); java_field }.clone()).append({ let java_field = jcall!(self.abeHttpRequestInfoData).REQUEST_FILE_PATH.clone(); java_field }.clone()).append({ let java_field = jcall!(self.commonSeps).COLON_SEP.clone(); java_field }.clone()).append(self.requestedFilePath.clone()).toString();
    }
}

