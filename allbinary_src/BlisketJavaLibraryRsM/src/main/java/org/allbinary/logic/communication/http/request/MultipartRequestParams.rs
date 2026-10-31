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
// use crate::java::util::List::List;
// use crate::javax::servlet::http::HttpServletRequest::HttpServletRequest;
// use crate::javax::servlet::jsp::PageContext::PageContext;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::http::file::upload::AbFileUploadFactory::AbFileUploadFactory;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use crate::org::apache::commons::fileupload::FileItem::FileItem;
use crate::org::apache::commons::fileupload::FileUploadBase::InvalidContentTypeException::InvalidContentTypeException;
use super::RequestParams::RequestParams;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MultipartRequestParams {
    pub base: RequestParams,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub special: bool,
}


impl MultipartRequestParams {
    

    pub fn new_struct1(mut request: Option<Rc<RefCell<HttpServletRequest>>>) -> Self {
        let mut this = Self {
            base: RequestParams::new_struct(),
            logUtil: None,
            special: false,
        };
        this.logUtil = LogUtil::getInstance();
        this.special = false;
        // super(...) is in the struct literal above
        this.processMultipartRequest(request.clone());
        this
    }

    pub fn new1(mut request: Option<Rc<RefCell<HttpServletRequest>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(request))))
    }
    

    pub fn new_struct1(mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Self {
        let mut this = Self {
            base: RequestParams::new_struct(),
            logUtil: None,
            special: false,
        };
        this.logUtil = LogUtil::getInstance();
        this.special = false;
        // super(...) is in the struct literal above
        { let java_arguments = ((jcall!(pageContext).getRequest()).clone() /* as HttpServletRequest */,); this.processMultipartRequest(java_arguments.0) };
        this
    }

    pub fn new1(mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(pageContext))))
    }
    

    pub fn setSpecial(&mut self) {
        self.special = true;
    }
    

    pub fn processMultipartRequest(&mut self, mut request: Option<Rc<RefCell<HttpServletRequest>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut specialRequest: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
            let mut multipartRequestList: Option<Rc<RefCell<dyn List>>> = jcall!(AbFileUploadFactory::getInstance()).getFileItemStreamList(request.clone());
            if multipartRequestList.is_some() {
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).HTTPREQUEST.clone()) {
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("FileItem List Size: "), jcall!(multipartRequestList).size()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("processMultipartRequest()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                }
                let mut fileItemArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(multipartRequestList).toArray();
                let mut size: i32 = (fileItemArray.len() as i32).clone();
                for index in (0..size) {
                    let mut fileItem: Option<Rc<RefCell<FileItem>>> = (fileItemArray[(index) as usize]).clone() /* as FileItem */;
                    let mut name: String = jcall!(fileItem).getName();
                    if jcall!(StringValidationUtil::getInstance()).isEmpty(name.clone()) {
                        jcall!(specialRequest).put(jcall!(fileItem).getFieldName(), jcall!(fileItem).getString());
                    } else {
                        jcall!(specialRequest).put(jcall!(fileItem).getFieldName(), fileItem.clone());
                    }
                }
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).HTTPREQUEST.clone()) {
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Special Request Data: "), jcall!(specialRequest).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("processMultipartRequest()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                }
                self.setSpecial();
                self.setMap(specialRequest.clone());
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).HTTPREQUEST.clone()) {
                jcall!(self.logUtil).putF(String::from("Using Normal RequestParams"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("processMultipartRequest()"));
            }
            self.base.setMap(jcall!(request).getParameterMap());
            }
        }
    }
    

    // throws Exception
    pub fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> {
        if self.special {
            return (self.getMap()).clone() /* as HashMap */;
        } else {
            return self.base.toHashMap();
        }
    }
}
impl std::ops::Deref for MultipartRequestParams {
    type Target = RequestParams;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for MultipartRequestParams {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::communication::http::request::RequestParams::RequestParamsDyn for MultipartRequestParams {
    fn as_RequestParams(&self) -> &crate::org::allbinary::logic::communication::http::request::RequestParams::RequestParams { &self.base }
    fn as_RequestParams_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::http::request::RequestParams::RequestParams { &mut self.base }
    fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> { Self::toHashMap(self) }
}


