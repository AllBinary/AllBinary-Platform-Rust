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
// package org.allbinary.business.user
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
// use crate::javax::servlet::http::HttpServletRequest::HttpServletRequest;
use crate::org::allbinary::business::user::modules::User::User;
use crate::org::allbinary::business::user::role::UserRoleData::UserRoleData;
use crate::org::allbinary::logic::communication::http::request::RequestParams::RequestParams;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpInterface::TransformInfoHttpInterface;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use super::UserInterface::UserInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NewUserFactory {
}


impl NewUserFactory {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn getInstance() -> Option<Rc<RefCell<dyn UserInterface>>> {
        return User::new().map(|reference| reference as Rc<RefCell<dyn UserInterface>>);
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn getInstance1(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<dyn UserInterface>>> {
        let mut httpTransformInfoInterface: Option<Rc<RefCell<dyn TransformInfoHttpInterface>>> = panic!("ClassCastException: a Rust dyn TransformInfoInterface cannot be cast to dyn TransformInfoHttpInterface");
        let mut httpServletRequest: Option<Rc<RefCell<HttpServletRequest>>> = (jcall!(jcall!(httpTransformInfoInterface).getPageContext()).getRequest()).clone() /* as HttpServletRequest */;
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = jcall!(RequestParams::new1(httpServletRequest.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::communication::http::request::RequestParams::RequestParamsDyn>>)).toHashMap();
        let mut role: String = (jcall!(jcall!(httpTransformInfoInterface).getPropertiesHashMap()).get(crate::org::allbinary::business::user::role::UserRoleData::UserRoleData_NAME.with(Clone::clone).clone())).clone() /* as String */;
        jcall!(hashMap).put(crate::org::allbinary::business::user::role::UserRoleData::UserRoleData_NAME.with(Clone::clone).clone(), role.clone());
        return CreateUserFactory::CreateUserFactory::getInstance(hashMap.clone());
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn getInstance2(mut httpServletRequest: Option<Rc<RefCell<HttpServletRequest>>>, mut propertiesHashMap: Option<Rc<RefCell<HashMap>>>) -> Option<Rc<RefCell<dyn UserInterface>>> {
        if propertiesHashMap.is_some() {
            let mut requestHashMap: Option<Rc<RefCell<HashMap>>> = jcall!(RequestParams::new1(httpServletRequest.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::communication::http::request::RequestParams::RequestParamsDyn>>)).toHashMap();
            let mut roleString: String = (jcall!(propertiesHashMap).get(crate::org::allbinary::business::user::role::UserRoleData::UserRoleData_NAME.with(Clone::clone).clone())).clone() /* as String */;
            jcall!(requestHashMap).put(crate::org::allbinary::business::user::role::UserRoleData::UserRoleData_NAME.with(Clone::clone).clone(), roleString.clone());
            return CreateUserFactory::CreateUserFactory::getInstance(requestHashMap.clone());
        } else {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Null HashMap"))));
        }
    }
}

