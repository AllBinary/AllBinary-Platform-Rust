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
// package org.allbinary.logic.visual.transform.data
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::servlet::http::HttpServletRequest::HttpServletRequest;
// use crate::javax::servlet::jsp::PageContext::PageContext;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontViewFactory::StoreFrontViewFactory;
use crate::org::allbinary::logic::communication::http::request::RequestParams::RequestParams;
use crate::org::allbinary::logic::communication::http::request::session::WeblisketSession::WeblisketSession;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::control::search::SearchRequest::SearchRequest;
use super::TransformHttpRequestDocument::TransformHttpRequestDocument;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TransformStoreDocument {
    pub base: TransformHttpRequestDocument,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}


impl TransformStoreDocument {
    

    pub fn new_struct2(mut pageContext: Option<Rc<RefCell<PageContext>>>, mut weblisketSession: Option<Rc<RefCell<WeblisketSession>>>) -> Self {
        let mut this = Self {
            base: TransformHttpRequestDocument::new_struct2(pageContext.clone(), weblisketSession.clone()),
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut storeName: String = jcall!(weblisketSession).getStoreName();
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                jcall!(this.logUtil).putF(format!("{}{}", String::from("Using Session StoreName: "), storeName), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone());
            }
            jcall!(this.getBaseNode()).appendChild(jcall!(StoreFrontViewFactory::getInstance(storeName.clone())).toXmlNode(this.getDoc()));
            jcall!(this.getBaseNode()).appendChild(jcall!(RequestParams::new1((jcall!(pageContext).getRequest()).clone() /* as HttpServletRequest */).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::communication::http::request::RequestParams::RequestParamsDyn>>)).toXmlNode(this.getDoc()));
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEWERROR.clone()) {
                jcall!(this.logUtil).put(jcall!(this.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone(), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", e);
            }
        }
        this
    }

    pub fn new2(mut pageContext: Option<Rc<RefCell<PageContext>>>, mut weblisketSession: Option<Rc<RefCell<WeblisketSession>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2(pageContext, weblisketSession))))
    }
    

    pub fn new_struct1(mut searchRequest: Option<Rc<RefCell<SearchRequest>>>) -> Self {
        let mut this = Self {
            base: TransformHttpRequestDocument::new_struct1(searchRequest.clone()),
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(this.getBaseNode()).appendChild(jcall!(StoreFrontViewFactory::getInstance(jcall!(jcall!(searchRequest).getStoreFront()).getName())).toXmlNode(this.getDoc()));
            jcall!(this.getBaseNode()).appendChild(jcall!(jcall!(searchRequest).getParams()).getParamsNode(this.getDoc()));
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEWERROR.clone()) {
                jcall!(this.logUtil).put(jcall!(this.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone(), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", e);
            }
        }
        this
    }

    pub fn new1(mut searchRequest: Option<Rc<RefCell<SearchRequest>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(searchRequest))))
    }
}
impl std::ops::Deref for TransformStoreDocument {
    type Target = TransformHttpRequestDocument;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for TransformStoreDocument {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::visual::transform::data::TransformHttpRequestDocument::TransformHttpRequestDocumentDyn for TransformStoreDocument {
    fn as_TransformHttpRequestDocument(&self) -> &crate::org::allbinary::logic::visual::transform::data::TransformHttpRequestDocument::TransformHttpRequestDocument { &self.base }
    fn as_TransformHttpRequestDocument_mut(&mut self) -> &mut crate::org::allbinary::logic::visual::transform::data::TransformHttpRequestDocument::TransformHttpRequestDocument { &mut self.base }
}


