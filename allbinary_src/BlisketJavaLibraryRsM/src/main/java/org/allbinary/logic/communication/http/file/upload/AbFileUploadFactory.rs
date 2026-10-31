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
// use crate::java::util::ArrayList::ArrayList;
// use crate::java::util::List::List;
// use crate::javax::servlet::http::HttpServletRequest::HttpServletRequest;
use crate::org::allbinary::logic::communication::http::AbFileItemFactory::AbFileItemFactory;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::apache::commons::fileupload::FileItemIterator::FileItemIterator;
use crate::org::apache::commons::fileupload::FileItemStream::FileItemStream;
use crate::org::apache::commons::fileupload::FileItemUtil::FileItemUtil;
use super::AbFileUpload::AbFileUpload;
use super::AbFileItem::AbFileItem;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AbFileUploadFactory {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

thread_local! { pub static AbFileUploadFactory_instance: Option<Rc<RefCell<AbFileUploadFactory>>> = AbFileUploadFactory::new(); }

impl AbFileUploadFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<AbFileUploadFactory>>> {
        return AbFileUploadFactory_instance.with(Clone::clone).clone();
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
    pub fn getFileItemStreamList(&mut self, mut httpServletRequest: Option<Rc<RefCell<HttpServletRequest>>>) -> Option<Rc<RefCell<dyn List>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut fileUpload: Option<Rc<RefCell<AbFileUpload>>> = AbFileUpload::new(AbFileItemFactory::new());
            let mut iterator: Option<Rc<RefCell<FileItemIterator>>> = jcall!(fileUpload).getItemIterator(httpServletRequest.clone());
            let mut items: Option<Rc<RefCell<dyn List>>> = ArrayList::new();
            while jcall!(iterator).hasNext() {
                let mut fileItemStream: Option<Rc<RefCell<FileItemStream>>> = jcall!(iterator).next();
                HttpFileUploadUtil::HttpFileUploadUtil::log(fileItemStream.clone());
                let mut fileItem: Option<Rc<RefCell<AbFileItem>>> = AbFileItem::new(jcall!(fileItemStream).getName(), jcall!(fileItemStream).getFieldName(), jcall!(FileItemUtil::getInstance()).getBytes(fileItemStream.clone()));
                jcall!(items).add(fileItem.clone());
            }
            return items.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).HTTPERROR.clone()) {
                let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
                jcall!(self.logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("get()"), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", e);
            }
        }
    }
    

    // throws Exception
    pub fn getFileItemList(&mut self, mut httpServletRequest: Option<Rc<RefCell<HttpServletRequest>>>) -> Option<Rc<RefCell<dyn List>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut abFileUpload: Option<Rc<RefCell<AbFileUpload>>> = AbFileUpload::new(AbFileItemFactory::new());
            return jcall!(abFileUpload).parseRequest(httpServletRequest.clone());
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).HTTPERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("get()"), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", e);
            }
        }
    }
}

