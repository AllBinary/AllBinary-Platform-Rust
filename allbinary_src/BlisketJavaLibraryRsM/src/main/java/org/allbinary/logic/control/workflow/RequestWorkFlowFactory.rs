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
// package org.allbinary.logic.control.workflow
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
// use crate::javax::servlet::ServletConfig::ServletConfig;
// use crate::javax::servlet::ServletContext::ServletContext;
// use crate::javax::servlet::ServletRequest::ServletRequest;
// use crate::javax::servlet::ServletResponse::ServletResponse;
// use crate::javax::servlet::http::HttpServletRequest::HttpServletRequest;
// use crate::javax::servlet::jsp::PageContext::PageContext;
use crate::org::allbinary::data::tables::workflow::WorkFlowEntityFactory::WorkFlowEntityFactory;
use crate::org::allbinary::logic::communication::http::request::session::WeblisketSession::WeblisketSession;
use crate::org::allbinary::logic::io::path::AbPathData::AbPathData;
use crate::org::allbinary::logic::system::security::licensing::LicensingException::LicensingException;
use super::WorkFlowInterface::WorkFlowInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RequestWorkFlowFactory {
}


impl RequestWorkFlowFactory {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception, LicensingException
    pub fn getInstance4(mut request: Option<Rc<RefCell<ServletRequest>>>, mut response: Option<Rc<RefCell<ServletResponse>>>, mut servletConfig: Option<Rc<RefCell<ServletConfig>>>, mut servletContext: Option<Rc<RefCell<ServletContext>>>) -> Option<Rc<RefCell<dyn WorkFlowInterface>>> {
        let mut httpRequest: Option<Rc<RefCell<HttpServletRequest>>> = (request).clone() /* as HttpServletRequest */;
        let mut requestCommand: String = jcall!(httpRequest).getPathInfo();
        let mut index: i32 = requestCommand.find(&jcall!(AbPathData::getInstance()).SEPARATOR.clone()).map(|index| index as i32).unwrap_or(-1);
        let mut storeName: String = requestCommand[(0) as usize..(index) as usize].to_string();
        let mut requestName: String = requestCommand[(index) as usize..((requestCommand.chars().count() as i32)) as usize].to_string();
        return jcall!(jcall!(WorkFlowEntityFactory::getInstance()).create2()).get(requestName.clone(), storeName.clone());
    }
    

    // throws Exception, LicensingException
    // Java overloaded method: Rust needs a different name
    pub fn getInstance(mut hashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Option<Rc<RefCell<dyn WorkFlowInterface>>> {
        let mut workFlowName: String = (jcall!(jcall!(pageContext).getRequest()).getParameter(jcall!(WorkFlowData::WorkFlowData::getInstance()).NAME.clone())).clone() /* as String */;
        let mut weblisketSession: Option<Rc<RefCell<WeblisketSession>>> = WeblisketSession::new2(hashMap.clone(), pageContext.clone());
        let mut storeName: String = jcall!(weblisketSession).getStoreName();
        return jcall!(jcall!(WorkFlowEntityFactory::getInstance()).create2()).get(workFlowName.clone(), storeName.clone());
    }
}

