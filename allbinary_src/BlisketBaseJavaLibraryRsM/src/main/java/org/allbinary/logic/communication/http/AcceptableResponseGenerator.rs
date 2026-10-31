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
// package org.allbinary.logic.communication.http
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::servlet::http::HttpServletRequest::HttpServletRequest;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::AcceptableResponseUtil::AcceptableResponseUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AcceptableResponseGenerator {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

thread_local! { pub static AcceptableResponseGenerator_instance: Option<Rc<RefCell<AcceptableResponseGenerator>>> = AcceptableResponseGenerator::new(); }

impl AcceptableResponseGenerator {
    

    pub fn getInstance() -> Option<Rc<RefCell<AcceptableResponseGenerator>>> {
        return AcceptableResponseGenerator_instance.with(Clone::clone).clone();
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
    pub fn get(&mut self, mut httpServletRequest: Option<Rc<RefCell<HttpServletRequest>>>) -> String {
        let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut acceptableResponseUtil: Option<Rc<RefCell<AcceptableResponseUtil>>> = AcceptableResponseUtil::getInstance();
            let mut acceptable: String = jcall!(httpServletRequest).getHeader(String::from("accept"));
            let mut result: String = jcall!(acceptableResponseUtil).getTagName(0);
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).HTTP.clone()) {
                jcall!(self.logUtil).putF(format!("{}{}", String::from("Request Type: "), acceptable), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).GET.clone());
            }
            if true {
                let mut size: i32 = jcall!(acceptableResponseUtil).size();
                for index in (0..size) {
                    if (acceptable.cmp(&jcall!(acceptableResponseUtil).get(index.clone())) as i32) == 0 {
                        result = jcall!(acceptableResponseUtil).getTagName(index.clone());
                    }
                }
            }
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).HTTP.clone()) {
                jcall!(self.logUtil).putF(format!("{}{}", String::from("Response Type: "), result), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).GET.clone());
            }
            return result.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).HTTPERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).GET.clone(), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", e);
            }
        }
    }
}

