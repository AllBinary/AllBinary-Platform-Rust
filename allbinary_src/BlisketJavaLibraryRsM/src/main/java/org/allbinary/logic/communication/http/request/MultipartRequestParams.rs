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
use super::*;
use crate::java::util::HashMap::HashMap;
// use crate::java::util::List::List;
// use crate::javax::servlet::http::HttpServletRequest::HttpServletRequest;
// use crate::javax::servlet::jsp::PageContext::PageContext;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::http::file::upload::AbFileUploadFactory::AbFileUploadFactory;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use crate::org::apache::commons::fileupload::FileItem::FileItem;
use crate::org::apache::commons::fileupload::FileUploadBase::InvalidContentTypeException::InvalidContentTypeException;
use crate::java::lang::Exception::*;
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
    

    pub fn new_struct1_PageContext(mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Self {
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

    pub fn new1_PageContext(mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1_PageContext(pageContext))))
    }
    

    pub fn setSpecial(&mut self) {
        self.special = true;
    }
    

    pub fn processMultipartRequest(&mut self, mut request: Option<Rc<RefCell<HttpServletRequest>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut specialRequest: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> = jcall!(StdUtil::getInstance()).createHashMap();
            let mut multipartRequestList: Option<Rc<RefCell<dyn List>>> = jcall!(AbFileUploadFactory::getInstance()).getFileItemStreamList(request.clone());
            if multipartRequestList.is_some() {
                if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).HTTPREQUEST.clone(); java_field }.clone()) {
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("FileItem List Size: "), jcall!(multipartRequestList).size()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("processMultipartRequest()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                }
                let mut fileItemArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(multipartRequestList).toArray();
                let mut size: i32 = (fileItemArray.len() as i32).clone();
                for index in (0..size) {
                    let mut fileItem: Option<Rc<RefCell<FileItem>>> = (fileItemArray[(index) as usize]).clone() /* as FileItem */;
                    let mut name: String = jcall!(fileItem).getName();
                    if jcall!(StringValidationUtil::getInstance()).isEmpty(name.clone()) {
                        jcall!(specialRequest).put(crate::java_into_object(jcall!(fileItem).getFieldName()), crate::java_into_object(jcall!(fileItem).getString()));
                    } else {
                        jcall!(specialRequest).put(crate::java_into_object(jcall!(fileItem).getFieldName()), crate::java_into_object(fileItem.clone()));
                    }
                }
                if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).HTTPREQUEST.clone(); java_field }.clone()) {
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Special Request Data: "), jcall!(specialRequest).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("processMultipartRequest()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                }
                self.setSpecial();
                self.setMap(specialRequest.clone().map(|reference| reference as Rc<RefCell<dyn crate::java::util::Map::Map>>));
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).HTTPREQUEST.clone(); java_field }.clone()) {
                jcall!(self.logUtil).putF(String::from("Using Normal RequestParams"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("processMultipartRequest()"));
            }
            { let java_arguments = (jcall!(request).getParameterMap(),); self.base.setMap(java_arguments.0) };
            }
        }
    }
    

    // throws Exception
    pub fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> {
        if self.special {
            return (self.getMap()).clone().map(|reference| { assert!({ let java_borrowed = reference.borrow(); let java_any: &dyn std::any::Any = &*java_borrowed; java_any.is::<HashMap>() }, "ClassCastException: not a HashMap"); unsafe { Rc::from_raw(Rc::into_raw(reference) as *const RefCell<HashMap>) } });
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
    fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> { Self::toHashMap(self) }
}


