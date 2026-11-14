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
// package org.allbinary.logic.communication.http.file.upload
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::java::anyType::clazz::ClassUtil::ClassUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::apache::commons::fileupload::FileItem::FileItem;
use crate::org::apache::commons::fileupload::FileItemStream::FileItemStream;
use crate::java::lang::Exception::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct HttpFileUploadUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

thread_local! { pub static HttpFileUploadUtil_instance: Option<Rc<RefCell<HttpFileUploadUtil>>> = HttpFileUploadUtil::new(); }

impl HttpFileUploadUtil {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<HttpFileUploadUtil>>> {
        return HttpFileUploadUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn isValid(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> bool {
        if anyType.is_some() && !((anyType).is_some() /* instanceof String */) {
            if (anyType).is_some() /* instanceof FileItem */ {
                return true;
            } else {
                panic!("{:?}", format!("{}: {:?}", "Exception", (format!("{}{}", String::from("Object Not Instance Of FileItem but is: "), ClassUtil::viewAll(anyType.clone(), { let java_field = jcall!(CommonSeps::getInstance()).NEW_LINE.clone(); java_field }.clone())))));
            }
        }
        return false;
    }
    

    // throws Exception
    pub fn log(mut fileItem: Option<Rc<RefCell<FileItemStream>>>) {
        let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut commonSeps: Option<Rc<RefCell<CommonSeps>>> = CommonSeps::getInstance();
        jcall!(stringBuffer).append(String::from("FileItem Log:"));
        jcall!(stringBuffer).append({ let java_field = jcall!(commonSeps).NEW_LINE.clone(); java_field }.clone());
        jcall!(stringBuffer).append(String::from("Content Type:"));
        jcall!(stringBuffer).append(jcall!(fileItem).getContentType());
        jcall!(stringBuffer).append({ let java_field = jcall!(commonSeps).NEW_LINE.clone(); java_field }.clone());
        jcall!(stringBuffer).append(String::from("Field Name:"));
        jcall!(stringBuffer).append(jcall!(fileItem).getFieldName());
        jcall!(stringBuffer).append({ let java_field = jcall!(commonSeps).NEW_LINE.clone(); java_field }.clone());
        jcall!(stringBuffer).append(String::from("Name:"));
        jcall!(stringBuffer).append(jcall!(fileItem).getName());
        jcall!(stringBuffer).append({ let java_field = jcall!(commonSeps).NEW_LINE.clone(); java_field }.clone());
        jcall!(stringBuffer).append(String::from("Size:"));
        jcall!(stringBuffer).append(String::from(" Not Calculated Yet"));
        if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).HTTP.clone(); java_field }.clone()) {
            jcall!(logUtil).putF({ let java_argument = jcall!(stringBuffer).toString(); java_argument }, crate::java_into_object(Self::getInstance()), String::from("log()"));
        }
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn log_FileItem(mut fileItem: Option<Rc<RefCell<FileItem>>>) {
        let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut commonSeps: Option<Rc<RefCell<CommonSeps>>> = CommonSeps::getInstance();
        jcall!(stringBuffer).append(String::from("FileItem Log:"));
        jcall!(stringBuffer).append({ let java_field = jcall!(commonSeps).NEW_LINE.clone(); java_field }.clone());
        jcall!(stringBuffer).append(String::from("Content Type:"));
        jcall!(stringBuffer).append(jcall!(fileItem).getContentType());
        jcall!(stringBuffer).append({ let java_field = jcall!(commonSeps).NEW_LINE.clone(); java_field }.clone());
        jcall!(stringBuffer).append(String::from("Field Name:"));
        jcall!(stringBuffer).append(jcall!(fileItem).getFieldName());
        jcall!(stringBuffer).append({ let java_field = jcall!(commonSeps).NEW_LINE.clone(); java_field }.clone());
        jcall!(stringBuffer).append(String::from("Name:"));
        jcall!(stringBuffer).append(jcall!(fileItem).getName());
        jcall!(stringBuffer).append({ let java_field = jcall!(commonSeps).NEW_LINE.clone(); java_field }.clone());
        jcall!(stringBuffer).append(String::from("Size:"));
        jcall!(stringBuffer).appendlong(((jcall!(fileItem).getSize()) as i64));
        if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).HTTPREQUEST.clone(); java_field }.clone()) {
            jcall!(logUtil).putF({ let java_argument = jcall!(stringBuffer).toString(); java_argument }, crate::java_into_object(Self::getInstance()), String::from("log()"));
        }
    }
}

