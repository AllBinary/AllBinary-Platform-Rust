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
// package admin.tags
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use crate::Calendar;
use super::*;
// use crate::java::util::Calendar::Calendar;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
// use crate::javax::servlet::http::HttpServletRequest::HttpServletRequest;
// use crate::javax::servlet::jsp::JspTagException::JspTagException;
// use crate::javax::servlet::jsp::tagext::TagSupport::TagSupport;
use crate::org::allbinary::business::init::InstallerInfo::InstallerInfo;
use crate::org::allbinary::business::user::role::BasicUserRole::BasicUserRole;
use crate::org::allbinary::business::user::role::BasicUserRoleFactory::BasicUserRoleFactory;
use crate::org::allbinary::logic::communication::http::request::AbResponseHandler::AbResponseHandler;
use crate::org::allbinary::logic::communication::http::request::session::BasicWeblisketSession::BasicWeblisketSession;
use crate::org::allbinary::logic::communication::http::request::session::WeblisketSessionData::WeblisketSessionData;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::java::bool::BooleanFactory::BooleanFactory;
use crate::org::allbinary::logic::java::bool::BooleanUtil::BooleanUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::system::security::licensing::LicensingException::LicensingException;
use crate::tags::CustomTagSupport::CustomTagSupport;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct FileAuthenticationTag {
    pub base: CustomTagSupport,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub attemptsInteger: Option<i32>,
    pub weblisketSession: Option<Rc<RefCell<BasicWeblisketSession>>>,
    pub request: Option<Rc<RefCell<HttpServletRequest>>>,
    pub command: String,
    pub userName: String,
    pub password: String,
    pub newPassword: String,
    pub authenticated: String,
    pub sessionUserName: String,
    pub role: Option<Rc<RefCell<BasicUserRole>>>,
    pub timeout: String,
    pub roles: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}


impl FileAuthenticationTag {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: CustomTagSupport::new_struct(),
            logUtil: None,
            attemptsInteger: None,
            weblisketSession: None,
            request: None,
            command: String::new(),
            userName: String::new(),
            password: String::new(),
            newPassword: String::new(),
            authenticated: String::new(),
            sessionUserName: String::new(),
            role: None,
            timeout: String::new(),
            roles: None,
        };
        this.logUtil = LogUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn setCommand(&mut self, mut command: String) {
        self.command = command.clone();
    }
    

    pub fn setUserName(&mut self, mut userName: String) {
        self.userName = userName.clone();
    }
    

    pub fn setPassword(&mut self, mut value: String) {
        self.password = value.clone();
    }
    

    pub fn setNewPassword(&mut self, mut value: String) {
        self.newPassword = value.clone();
    }
    

    pub fn setRoles(&mut self, mut values: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        self.roles = values.clone();
    }
    

    pub fn makeSessionValid(&mut self, mut storedRole: Option<Rc<RefCell<BasicUserRole>>>, mut userName: String) {
        jcall!(self.weblisketSession).setAuthenticated();
        jcall!(self.weblisketSession).setRole(storedRole.clone());
        jcall!(self.weblisketSession).setUserName(userName.clone());
        jcall!(self.request).removeAttribute(crate::org::allbinary::logic::communication::http::request::session::WeblisketSessionData::WeblisketSessionData_REMOVABLEUSERNAME.with(Clone::clone).clone());
        jcall!(self.request).removeAttribute(crate::org::allbinary::logic::communication::http::request::session::WeblisketSessionData::WeblisketSessionData_REMOVABLEPASSWORD.with(Clone::clone).clone());
    }
    

    pub fn getFormData(&mut self) {
        self.authenticated = jcall!(self.weblisketSession).getAuthentication();
        self.sessionUserName = jcall!(self.weblisketSession).getUserName();
        self.role = jcall!(self.weblisketSession).getRole();
        self.attemptsInteger = jcall!(self.weblisketSession).getAttempts();
        self.timeout = jcall!(self.weblisketSession).getTimeout();
    }
    

    pub fn nextAttempt(&mut self) {
        if self.attemptsInteger.is_some() && ((self.attemptsInteger).unwrap() as i32) > 0 {
            { let java_receiver = self.weblisketSession.clone(); let java_arguments = (Some((((self.attemptsInteger).unwrap() as i32) + 1) as i32),); jcall!(java_receiver).setAttempts(java_arguments.0); };
        } else {
            jcall!(self.weblisketSession).setAttempts(Some((1) as i32));
        }
    }
    

    // throws LicensingException
    pub fn changePassword(&mut self) -> bool {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut userName: String = jcall!(self.request).getParameter(crate::org::allbinary::logic::communication::http::request::session::WeblisketSessionData::WeblisketSessionData_REMOVABLEUSERNAME.with(Clone::clone).clone());
            let mut password: String = jcall!(self.request).getParameter(crate::org::allbinary::logic::communication::http::request::session::WeblisketSessionData::WeblisketSessionData_REMOVABLEPASSWORD.with(Clone::clone).clone());
            let mut newPassword: String = jcall!(self.request).getParameter(crate::org::allbinary::logic::communication::http::request::session::WeblisketSessionData::WeblisketSessionData_REMOVABLENEWPASSWORD.with(Clone::clone).clone());
            let mut newReenteredPassword: String = jcall!(self.request).getParameter(crate::org::allbinary::logic::communication::http::request::session::WeblisketSessionData::WeblisketSessionData_REMOVABLEREENTERNEWPASSWORD.with(Clone::clone).clone());
            if (newPassword.cmp(&newReenteredPassword.clone()) as i32) != 0 {
                return false;
            }
            let mut installerInfo: Option<Rc<RefCell<InstallerInfo>>> = InstallerInfo::new();
            if jcall!(installerInfo).isValid(userName.clone(), password.clone()) {
                jcall!(installerInfo).setPassword(newPassword.clone());
                jcall!(installerInfo).write();
                jcall!(self.weblisketSession).setUserName(userName.clone());
                jcall!(self.weblisketSession).setPassword(newPassword.clone());
                return true;
            }
            return false;
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            panic!("{:?}", e);
            }
        }
    }
    

    pub fn invalidateSession(&mut self) -> i32 {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.weblisketSession).clear();
            return ((Some((unsafe { crate::javax::servlet::jsp::tagext::TagSupport::TagSupport_EVAL_BODY_INCLUDE.clone() }.clone()) as i32)).unwrap() as i32);
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).JSPTAGERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("invalidateSession()"), crate::java_into_object(e.clone()));
            }
            return unsafe { crate::javax::servlet::jsp::tagext::TagSupport::TagSupport_SKIP_BODY.clone() }.clone();
            }
        }
    }
    

    // throws LicensingException
    pub fn isSessionOld(&mut self) -> bool {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if self.role.is_some() {
                let mut timeCreated: i64 = jcall!(self.weblisketSession).getCreationTime();
                let mut sessionTimout: i64 = jcall!(self.role).getSessionTimeout();
                let mut calendar: Option<Rc<RefCell<Calendar>>> = Calendar::getInstance();
                let mut timeFirst: i64 = jcall!(calendar).getTimeInMillis() - sessionTimout;
                if timeFirst > timeCreated {
                    if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).AUTHENTICATION.clone()) {
                        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                        jcall!(stringBuffer).append(String::from("Session Is Old - Timeout: "));
                        jcall!(stringBuffer).appendlong(sessionTimout.clone());
                        jcall!(stringBuffer).append(String::from(" Eval: "));
                        jcall!(stringBuffer).appendlong(timeFirst.clone());
                        jcall!(stringBuffer).append(String::from(" > "));
                        jcall!(stringBuffer).appendlong(timeCreated.clone());
                        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("isSessionOld()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                    }
                    return true.unwrap();
                }
            } else {
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).AUTHENTICATION.clone()) {
                    jcall!(self.logUtil).putF(String::from("Role Is Not In Session"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("isSessionOld()"));
                }
            }
            return self.isRarelyUsedSession().unwrap();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).JSPTAGERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("isSessionOld()"), crate::java_into_object(e.clone()));
            }
            return false;
            }
        }
    }
    

    pub fn isRarelyUsedSession(&mut self) -> Option<bool> {
        if self.role.is_some() {
            let mut timeCreated: i64 = jcall!(self.weblisketSession).getCreationTime();
            let mut lastAccess: i64 = jcall!(self.weblisketSession).getLastAccessedTime();
            let mut timePassed: i64 = jcall!(self.role).getSessionInactivityTimeout();
            let mut calendar: Option<Rc<RefCell<Calendar>>> = Calendar::getInstance();
            let mut timeFirst: i64 = jcall!(calendar).getTimeInMillis() - lastAccess;
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).AUTHENTICATION.clone()) {
                let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                jcall!(stringBuffer).append(String::from("Session Is Rarely Used - Timeout: "));
                jcall!(stringBuffer).appendlong(lastAccess.clone());
                jcall!(stringBuffer).append(String::from(" Eval: "));
                jcall!(stringBuffer).appendlong(timeFirst.clone());
                jcall!(stringBuffer).append(String::from(" > "));
                jcall!(stringBuffer).appendlong(timePassed.clone());
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("isRarelyUsedSession()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
        } else {
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).AUTHENTICATION.clone()) {
                jcall!(self.logUtil).putF(String::from("Role Is Not In Session"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("isRarelyUsedSession()"));
            }
        }
        return false;
    }
    

    // throws LicensingException
    pub fn isRoleValid(&mut self) -> bool {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            { let java_arguments = (jcall!(BasicUserRoleFactory::getInstance()).INSTALLER.clone(), self.userName.clone(),); self.makeSessionValid(java_arguments.0, java_arguments.1) };
            return true.unwrap();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).JSPTAGERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("isRoleValid()"), crate::java_into_object(e.clone()));
            }
            return false;
            }
        }
    }
    

    // throws LicensingException
    pub fn validRole(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(String::from("Trying New login<p>"));
            return jcall!(stringBuffer).toString();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut error: String = String::from("Failed to set valid role");
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).JSPTAGERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("validRole()"), crate::java_into_object(e.clone()));
            }
            return error.clone();
            }
        }
    }
    

    // throws LicensingException
    pub fn invalidRole(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.weblisketSession).setAuthenticated1(false);
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(String::from("Sorry your username and password is invalid on this page."));
            jcall!(stringBuffer).append(String::from("Trying New login<p>"));
            return jcall!(stringBuffer).toString();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut error: String = String::from("Failed to set role invalid");
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).JSPTAGERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("invalidRole()"), crate::java_into_object(e.clone()));
            }
            return error.clone();
            }
        }
    }
    

    // throws LicensingException
    pub fn isAuthenticationSessionValid(&mut self) -> bool {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if jcall!(BooleanUtil::getInstance()).getFromString(self.authenticated.clone()) {
                let mut size: i32 = jcall!(self.roles).size();
                for index in (0..size) {
                    let mut mustBeOfRole: Option<Rc<RefCell<BasicUserRole>>> = { let java_object = (jcall!(self.roles).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<BasicUserRole>>>>().expect("ClassCastException").clone()) };
                    if true && jcall!(self.role).equals(mustBeOfRole.clone()) {
                        return true.unwrap();
                    }
                }
                return false.unwrap();
            }
            return false.unwrap();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).JSPTAGERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("isAuthenticationSessionValid()"), crate::java_into_object(e.clone()));
            }
            return false;
            }
        }
    }
    

    // throws LicensingException
    pub fn processIfNewLogin(&mut self) -> bool {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).TAGHELPERERROR.clone()) {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", format!("{}{}", format!("{}{}", String::from("Starting for User: "), self.userName), String::from(" Attempt: ")), jcall!(self.weblisketSession).getAttempts()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("processIfNewLogin()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            if true && (self.userName.cmp(&jcall!(StringUtil::getInstance()).EMPTY_STRING.clone()) as i32) != 0 && true && (self.password.cmp(&jcall!(StringUtil::getInstance()).EMPTY_STRING.clone()) as i32) != 0 {
                let mut login: String = String::new();
                let mut installerInfo: Option<Rc<RefCell<InstallerInfo>>> = InstallerInfo::new();
                if jcall!(installerInfo).isValid(self.userName.clone(), self.password.clone()) {
                    login = jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).globals).GLOBALS2).LOGINSUCCESS.clone();
                } else {
                    login = jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).globals).GLOBALS2).LOGINFAILED.clone();
                }
                if (login.cmp(&jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).globals).GLOBALS2).LOGINSUCCESS.clone()) as i32) == 0 {
                    jcall!(self.weblisketSession).setAttempts(Some((0) as i32));
                    return true.unwrap();
                }
                self.nextAttempt();
                return false.unwrap();
            }
            self.nextAttempt();
            return false.unwrap();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).JSPTAGERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("processIfNewLogin()"), crate::java_into_object(e.clone()));
            }
            return false;
            }
        }
    }
    

    // throws LicensingException
    pub fn processInvalidation(&mut self) -> bool {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if jcall!(BooleanUtil::getInstance()).getFromString(self.timeout.clone()) {
                self.invalidateSession();
                return true.unwrap();
            }
            {
                { let java_receiver = self.weblisketSession.clone(); let java_arguments = (jcall!(BooleanFactory::getInstance()).TRUE_STRING.clone(),); jcall!(java_receiver).setTimeout(java_arguments.0); };
                return false.unwrap();
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).JSPTAGERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("isSetToInvalidate()"), crate::java_into_object(e.clone()));
            }
            return false;
            }
        }
    }
    

    // throws JspTagException
    pub fn doStartTag(&mut self) -> i32 {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut stringUtil: Option<Rc<RefCell<StringUtil>>> = StringUtil::getInstance();
            self.request = (jcall!(self.base.pageContext).getRequest()).clone() /* as HttpServletRequest */;
            self.weblisketSession = BasicWeblisketSession::new(self.base.pageContext.clone());
            self.getFormData();
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).JSPTAG.clone()) {
                jcall!(self.logUtil).putF(String::from("Started"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.tagStrings).DO_START_TAG.clone());
            }
            if true {
                if (self.command.cmp(&jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).globals).GLOBALS2).CHANGEPASSWORD.clone()) as i32) == 0 {
                    if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).JSPTAG.clone()) {
                        jcall!(self.logUtil).putF(String::from("Change Password"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.tagStrings).DO_START_TAG.clone());
                    }
                    if !self.changePassword() {
                        jcall!(jcall!(self.base.pageContext).getOut()).print(String::from("Password Change Attempt Failed.<p>"));
                        return EVAL_BODY_INCLUDE.clone();
                    } else {
                        jcall!(jcall!(self.base.pageContext).getOut()).print(String::from("Password Changed Successfully.<p>"));
                        return SKIP_BODY.clone();
                    }
                } else if (self.command.cmp(&crate::org::allbinary::logic::communication::http::request::session::WeblisketSessionData::WeblisketSessionData_INVALIDATESESSION.with(Clone::clone).clone()) as i32) == 0 {
                    if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).JSPTAG.clone()) {
                        jcall!(self.logUtil).putF(String::from("INVALIDATESESSION"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.tagStrings).DO_START_TAG.clone());
                    }
                    return self.invalidateSession();
                } else {
                    if self.isSessionOld() {
                        if self.processInvalidation() {
                            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).JSPTAG.clone()) {
                                jcall!(self.logUtil).putF(String::from("Processing Invalidation"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.tagStrings).DO_START_TAG.clone());
                            }
                            jcall!(jcall!(self.base.pageContext).getOut()).print(String::from("Please login again.<p>"));
                            if (self.command.cmp(&jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).globals).GLOBALS2).PROCESSBODYIFAUTHENTICATED.clone()) as i32) == 0 {
                                return unsafe { crate::javax::servlet::jsp::tagext::TagSupport::TagSupport_SKIP_BODY.clone() }.clone();
                            } else {
                                return unsafe { crate::javax::servlet::jsp::tagext::TagSupport::TagSupport_EVAL_BODY_INCLUDE.clone() }.clone();
                            }
                        } else {
                            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).JSPTAG.clone()) {
                                jcall!(self.logUtil).putF(String::from("Preparing session for invalidation"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.tagStrings).DO_START_TAG.clone());
                            }
                            if (self.command.cmp(&jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).globals).GLOBALS2).PROCESSBODYIFAUTHENTICATED.clone()) as i32) == 0 {
                                return unsafe { crate::javax::servlet::jsp::tagext::TagSupport::TagSupport_SKIP_BODY.clone() }.clone();
                            } else {
                                return unsafe { crate::javax::servlet::jsp::tagext::TagSupport::TagSupport_EVAL_BODY_INCLUDE.clone() }.clone();
                            }
                        }
                    }
                    if (false || (self.userName.cmp(&jcall!(stringUtil).EMPTY_STRING.clone()) as i32) == 0) && (false || (self.password.cmp(&jcall!(stringUtil).EMPTY_STRING.clone()) as i32) == 0) && self.roles.is_some() && self.isAuthenticationSessionValid() {
                        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).JSPTAG.clone()) {
                            jcall!(self.logUtil).putF(String::from("Authenticated session is valid"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.tagStrings).DO_START_TAG.clone());
                        }
                        if (self.command.cmp(&jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).globals).GLOBALS2).PROCESSBODYIFAUTHENTICATED.clone()) as i32) == 0 {
                            return unsafe { crate::javax::servlet::jsp::tagext::TagSupport::TagSupport_EVAL_BODY_INCLUDE.clone() }.clone();
                        } else {
                            return unsafe { crate::javax::servlet::jsp::tagext::TagSupport::TagSupport_SKIP_BODY.clone() }.clone();
                        }
                    }
                    if true && true && self.processIfNewLogin() {
                        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).JSPTAG.clone()) {
                            jcall!(self.logUtil).putF(String::from("Processing new login."), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.tagStrings).DO_START_TAG.clone());
                        }
                        if self.isRoleValid() {
                            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).JSPTAG.clone()) {
                                jcall!(self.logUtil).putF(String::from("Role is valid - Processing new login"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.tagStrings).DO_START_TAG.clone());
                            }
                            jcall!(jcall!(self.base.pageContext).getOut()).print(self.validRole());
                            if (self.command.cmp(&jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).globals).GLOBALS2).PROCESSBODYIFAUTHENTICATED.clone()) as i32) == 0 {
                                return EVAL_BODY_INCLUDE.clone();
                            } else {
                                return SKIP_BODY.clone();
                            }
                        } else {
                            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).JSPTAG.clone()) {
                                jcall!(self.logUtil).putF(String::from("Role is not valid - Processing new login"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.tagStrings).DO_START_TAG.clone());
                            }
                            jcall!(jcall!(self.base.pageContext).getOut()).print(self.invalidRole());
                            if (self.command.cmp(&jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).globals).GLOBALS2).PROCESSBODYIFAUTHENTICATED.clone()) as i32) == 0 {
                                return SKIP_BODY.clone();
                            } else {
                                return EVAL_BODY_INCLUDE.clone();
                            }
                        }
                    } else {
                        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).JSPTAG.clone()) {
                            jcall!(self.logUtil).putF(String::from("Invalid Login"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.tagStrings).DO_START_TAG.clone());
                        }
                        if true && (self.userName.cmp(&jcall!(StringUtil::getInstance()).EMPTY_STRING.clone()) as i32) != 0 && true && (self.password.cmp(&jcall!(StringUtil::getInstance()).EMPTY_STRING.clone()) as i32) != 0 {
                            jcall!(jcall!(self.base.pageContext).getOut()).print(String::from("Sorry your username and/or password is invalid.<p>"));
                        }
                    }
                    if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).JSPTAG.clone()) {
                        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                        jcall!(stringBuffer).append(String::from("Major authentication error - userName: "));
                        jcall!(stringBuffer).append(self.userName.clone());
                        jcall!(stringBuffer).append(String::from(" Password: "));
                        jcall!(stringBuffer).append(self.password.clone());
                        jcall!(stringBuffer).append(String::from(" Command: "));
                        jcall!(stringBuffer).append(self.command.clone());
                        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.tagStrings).DO_START_TAG.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                    }
                    if (self.command.cmp(&jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).globals).GLOBALS2).PROCESSBODYIFAUTHENTICATED.clone()) as i32) == 0 {
                        return unsafe { crate::javax::servlet::jsp::tagext::TagSupport::TagSupport_SKIP_BODY.clone() }.clone();
                    } else {
                        return unsafe { crate::javax::servlet::jsp::tagext::TagSupport::TagSupport_EVAL_BODY_INCLUDE.clone() }.clone();
                    }
                }
            }
            return SKIP_BODY.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            AbResponseHandler::sendJspTagLicensingRedirect(self.pageContext.clone(), e.clone());
            return SKIP_BODY.clone();
            }
        }
    }
}
impl std::ops::Deref for FileAuthenticationTag {
    type Target = CustomTagSupport;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for FileAuthenticationTag {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::tags::CustomTagSupport::CustomTagSupportDyn for FileAuthenticationTag {
    fn as_CustomTagSupport(&self) -> &crate::tags::CustomTagSupport::CustomTagSupport { &self.base }
    fn as_CustomTagSupport_mut(&mut self) -> &mut crate::tags::CustomTagSupport::CustomTagSupport { &mut self.base }
}


