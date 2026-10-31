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
// package admin.taghelpers
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::business::user::role::BasicUserRole::BasicUserRole;
use crate::org::allbinary::business::user::role::UserRole::UserRole;
use crate::org::allbinary::globals::URLGLOBALS::URLGLOBALS;
use crate::org::allbinary::logic::communication::http::file::upload::HttpFilePermissions::HttpFilePermissions;
use crate::org::allbinary::logic::communication::http::request::HttpRequestUtil::HttpRequestUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::io::path::AbPath::AbPath;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use super::AuthenticationHelper::AuthenticationHelper;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AuthenticationHelperUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

thread_local! { pub static AuthenticationHelperUtil_instance: Option<Rc<RefCell<AuthenticationHelperUtil>>> = AuthenticationHelperUtil::new(); }

impl AuthenticationHelperUtil {

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
    

    pub fn getInstance() -> Option<Rc<RefCell<AuthenticationHelperUtil>>> {
        return AuthenticationHelperUtil_instance.with(Clone::clone).clone();
    }
    

    // throws Exception
    pub fn isAuthorized(&mut self, mut authenticationHelper: Option<Rc<RefCell<AuthenticationHelper>>>, mut filePath: String) -> bool {
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEWERROR.clone()) {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", jcall!(CommonLabels::getInstance()).START, filePath), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("isAuthorized()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        let mut endIndex: i32 = jcall!(HttpRequestUtil::getInstance()).getLastSeparatorIndex(filePath.clone());
        let mut fullPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> = AbPath::new(format!("{}{}", URLGLOBALS::getWebappPath(), filePath[(0) as usize..(endIndex) as usize].to_string()), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
        let mut downloadFilePermissions: Option<Rc<RefCell<HttpFilePermissions>>> = HttpFilePermissions::new(fullPath.clone());
        let mut userRoles: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(downloadFilePermissions).getUserRoles();
        let mut userRole: Option<Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>> = jcall!(authenticationHelper).getRole();
        let mut basicUserRole: Option<Rc<RefCell<BasicUserRole>>> = jcall!(userRole).getBasicUserRole();
        if jcall!(userRoles).contains(crate::java_into_object(basicUserRole.clone())) {
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                jcall!(self.logUtil).putF(String::from("Authorized"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("isAuthorized()"));
            }
            return true;
        } else {
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Not Authorized: "), jcall!(basicUserRole).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("isAuthorized()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            return false;
        }
    }
}

