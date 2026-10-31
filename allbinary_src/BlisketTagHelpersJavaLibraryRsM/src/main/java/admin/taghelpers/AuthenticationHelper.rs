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
use crate::Calendar;
use std::collections::HashMap;
use super::*;
// use crate::java::util::Calendar::Calendar;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
// use crate::javax::servlet::http::HttpServletRequest::HttpServletRequest;
// use crate::javax::servlet::jsp::PageContext::PageContext;
// use crate::javax::servlet::jsp::tagext::TagSupport::TagSupport;
use crate::org::allbinary::business::user::UserInterface::UserInterface;
use crate::org::allbinary::business::user::role::BasicUserRole::BasicUserRole;
use crate::org::allbinary::business::user::role::UserRole::UserRole;
use crate::org::allbinary::business::user::role::UserRoleFactory::UserRoleFactory;
use crate::org::allbinary::data::tables::user::UserEntityFactory::UserEntityFactory;
use crate::org::allbinary::data::tables::user::UserEntityInterface::UserEntityInterface;
use crate::org::allbinary::globals::GLOBALS2::GLOBALS2;
use crate::org::allbinary::logic::communication::http::request::session::WeblisketSession::WeblisketSession;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::java::bool::BooleanFactory::BooleanFactory;
use crate::org::allbinary::logic::java::bool::BooleanUtil::BooleanUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use super::TagHelper::TagHelper;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AuthenticationHelper {
    pub base: TagHelper,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub weblisketSession: Option<Rc<RefCell<WeblisketSession>>>,
    pub authenticated: String,
    pub sessionUserName: String,
    pub role: Option<Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>>,
    pub timeout: String,
    pub attemptsInteger: Option<i32>,
}


impl AuthenticationHelper {
    

    pub fn new_struct2(mut hashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Self {
        let mut this = Self::new_struct2(hashMap.clone(), (jcall!(pageContext).getRequest()).clone() /* as HttpServletRequest */);
        // this(...) is in the struct literal above
        this
    }

    pub fn new2(mut hashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2(hashMap, pageContext))))
    }
    

    pub fn new_struct2(mut hashMap: Option<Rc<RefCell<HashMap>>>, mut httpServletRequest: Option<Rc<RefCell<HttpServletRequest>>>) -> Self {
        let mut this = Self {
            base: TagHelper::new_struct(),
            logUtil: None,
            weblisketSession: None,
            authenticated: String::new(),
            sessionUserName: String::new(),
            role: None,
            timeout: String::new(),
            attemptsInteger: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.weblisketSession = WeblisketSession::new2(hashMap.clone(), httpServletRequest.clone());
        this.getFormData();
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).AUTHENTICATION.clone()) {
            { let java_receiver = this.logUtil.clone(); let java_arguments = (this.outputSessionInfo(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        this
    }

    pub fn new2(mut hashMap: Option<Rc<RefCell<HashMap>>>, mut httpServletRequest: Option<Rc<RefCell<HttpServletRequest>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2(hashMap, httpServletRequest))))
    }
    

    // throws Exception
    pub fn getFormData(&mut self) {
        self.authenticated = jcall!(self.weblisketSession).getAuthentication();
        self.sessionUserName = jcall!(self.weblisketSession).getUserName();
        self.role = jcall!(self.weblisketSession).getRole();
        self.attemptsInteger = jcall!(self.weblisketSession).getAttempts();
        self.timeout = jcall!(self.weblisketSession).getTimeout();
    }
    

    // throws Exception
    pub fn invalidateSession(&mut self) -> Option<i32> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.weblisketSession).clear();
            return Some((unsafe { crate::javax::servlet::jsp::tagext::TagSupport::TagSupport_EVAL_BODY_INCLUDE.clone() }.clone()) as i32);
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).AUTHENTICATIONERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("invalidateSession()"), crate::java_into_object(e.clone()));
            }
            return None;
            }
        }
    }
    

    pub fn isSessionOld(&mut self) -> Option<bool> {
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
                return true;
            }
        } else {
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).AUTHENTICATION.clone()) {
                jcall!(self.logUtil).putF(String::from("Role Is Not In Session"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("isSessionOld()"));
            }
        }
        return self.isRarelyUsedSession();
    }
    

    pub fn isRarelyUsedSession(&mut self) -> Option<bool> {
        if self.role.is_some() {
            let mut timeCreated: i64 = jcall!(self.weblisketSession).getCreationTime();
            let mut lastAccess: i64 = jcall!(self.weblisketSession).getLastAccessedTime();
            let mut inactivityAllowed: i64 = jcall!(self.role).getSessionInactivityTimeout();
            let mut calendar: Option<Rc<RefCell<Calendar>>> = Calendar::getInstance();
            let mut timeFirst: i64 = jcall!(calendar).getTimeInMillis() - lastAccess;
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).AUTHENTICATION.clone()) {
                let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                jcall!(stringBuffer).append(String::from("Session Is Rarely Used - Timeout: "));
                jcall!(stringBuffer).appendlong(lastAccess.clone());
                jcall!(stringBuffer).append(String::from(" Eval: "));
                jcall!(stringBuffer).appendlong(timeFirst.clone());
                jcall!(stringBuffer).append(String::from(" > "));
                jcall!(stringBuffer).appendlong(inactivityAllowed.clone());
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("isRarelyUsedSession()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
        } else {
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).AUTHENTICATION.clone()) {
                jcall!(self.logUtil).putF(String::from("Role Is Not In Session"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("isRarelyUsedSession()"));
            }
        }
        return false;
    }
    

    pub fn validRole(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(String::from("Trying New login<p/>"));
            return jcall!(stringBuffer).toString();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut error: String = String::from("Failed to set valid role");
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).AUTHENTICATIONERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("validRole()"), crate::java_into_object(e.clone()));
            }
            return error.clone();
            }
        }
    }
    

    pub fn invalidRole(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.weblisketSession).setAuthenticated1(false);
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(String::from("Sorry your username and password is invalid on this page. "));
            jcall!(stringBuffer).append(String::from("Trying New login<p/>"));
            return jcall!(stringBuffer).toString();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut error: String = String::from("Failed to set role invalid");
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).AUTHENTICATIONERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("invalidRole()"), crate::java_into_object(e.clone()));
            }
            return error.clone();
            }
        }
    }
    

    // throws Exception
    pub fn isAuthenticated(&mut self) -> bool {
        if jcall!(BooleanUtil::getInstance()).getFromString(self.authenticated.clone()) {
            return true;
        } else {
            return false;
        }
    }
    

    pub fn isAuthenticationSessionValid(&mut self, mut roles: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Option<bool> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if self.role.is_some() {
                if self.isAuthenticated() {
                    if true {
                        let mut basicUserRole: Option<Rc<RefCell<BasicUserRole>>> = jcall!(self.role).getBasicUserRole();
                        let mut basicUserRoleArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(roles).toArray();
                        let mut size: i32 = (basicUserRoleArray.len() as i32).clone();
                        for index in (0..size) {
                            let mut mustBeOfRole: Option<Rc<RefCell<BasicUserRole>>> = (basicUserRoleArray[(index) as usize]).clone() /* as BasicUserRole */;
                            if jcall!(basicUserRole).equals(mustBeOfRole.clone()) {
                                if jcall!(basicUserRole).equals(jcall!(UserRoleFactory::getInstance()).CUSTOMER.clone()) {
                                    let mut userEntityInterface: Option<Rc<RefCell<dyn UserEntityInterface>>> = UserEntityFactory::getInstance().map(|reference| reference as Rc<RefCell<dyn UserEntityInterface>>);
                                    let mut userInterface: Option<Rc<RefCell<dyn UserInterface>>> = jcall!(userEntityInterface).getUser(self.sessionUserName.clone());
                                    if !jcall!(userInterface).isSessionValid().unwrap() {
                                        return false;
                                    }
                                }
                                return true;
                            }
                        }
                    }
                    return false;
                }
            } else {
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).AUTHENTICATION.clone()) {
                    jcall!(self.logUtil).putF(String::from("Role Is Not In Session"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("isAuthenticationSessionValid()"));
                }
            }
            return false;
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).AUTHENTICATIONERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("isAuthenticatedSessionValid()"), crate::java_into_object(e.clone()));
            }
            return false;
            }
        }
    }
    

    pub fn nextAttempt(&mut self) {
        if self.attemptsInteger.is_some() && ((self.attemptsInteger).unwrap() as i32) > 0 {
            { let java_receiver = self.weblisketSession.clone(); let java_arguments = (Some((((self.attemptsInteger).unwrap() as i32) + 1) as i32),); jcall!(java_receiver).setAttempts(java_arguments.0); };
        } else {
            jcall!(self.weblisketSession).setAttempts(Some((1) as i32));
        }
    }
    

    pub fn processIfNewLogin(&mut self, mut userName: String, mut password: String) -> Option<bool> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).AUTHENTICATIONERROR.clone()) {
                jcall!(self.logUtil).putF(format!("{}{}", String::from("Starting: "), userName), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("processIfNewLogin()"));
            }
            if true && (userName.cmp(&jcall!(StringUtil::getInstance()).EMPTY_STRING.clone()) as i32) != 0 && true && (password.cmp(&jcall!(StringUtil::getInstance()).EMPTY_STRING.clone()) as i32) != 0 {
                let mut login: String = String::new();
                let mut userEntityInterface: Option<Rc<RefCell<dyn UserEntityInterface>>> = UserEntityFactory::getInstance().map(|reference| reference as Rc<RefCell<dyn UserEntityInterface>>);
                login = jcall!(userEntityInterface).login(userName.clone(), password.clone());
                if (login.cmp(&crate::org::allbinary::globals::GLOBALS2::GLOBALS2_LOGINSUCCESS.with(Clone::clone).clone()) as i32) == 0 {
                    let mut userInterface: Option<Rc<RefCell<dyn UserInterface>>> = jcall!(userEntityInterface).getUser(userName.clone());
                    if jcall!(userInterface).isSessionValid().unwrap() {
                        jcall!(self.weblisketSession).setAttempts(Some((0) as i32));
                        return true;
                    }
                }
                self.nextAttempt();
                return false;
            }
            self.nextAttempt();
            return false;
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).AUTHENTICATIONERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("processIfNewLogin()"), crate::java_into_object(e.clone()));
            }
            return false;
            }
        }
    }
    

    pub fn processInvalidation(&mut self) -> Option<bool> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if jcall!(BooleanUtil::getInstance()).getFromString(self.timeout.clone()) {
                self.invalidateSession();
                return true;
            }
            {
                { let java_receiver = self.weblisketSession.clone(); let java_arguments = (jcall!(BooleanFactory::getInstance()).TRUE_STRING.clone(),); jcall!(java_receiver).setTimeout(java_arguments.0); };
                return false;
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).AUTHENTICATIONERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("processInvalidation()"), crate::java_into_object(e.clone()));
            }
            return false;
            }
        }
    }
    

    pub fn outputSessionInfo(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut calendar: Option<Rc<RefCell<Calendar>>> = Calendar::getInstance();
            let mut timeCreated: i64 = jcall!(self.weblisketSession).getCreationTime();
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            let mut lastAccess: i64 = jcall!(self.weblisketSession).getLastAccessedTime();
            jcall!(stringBuffer).append(format!("{}{}", format!("{}{}", String::from("Time Since Last Access: "), (jcall!(calendar).getTimeInMillis() - lastAccess)), String::from("<br/>\n")));
            if self.role.is_some() {
                jcall!(stringBuffer).append(String::from("Auth: "));
                jcall!(stringBuffer).append(self.authenticated.clone());
                jcall!(stringBuffer).append(String::from(" UserName: "));
                jcall!(stringBuffer).append(self.sessionUserName.clone());
                jcall!(stringBuffer).append(String::from(" Role: "));
                jcall!(stringBuffer).append(jcall!(self.role).toString());
                jcall!(stringBuffer).append(String::from("<br/>\n"));
                jcall!(stringBuffer).append(String::from("Time Left: "));
                jcall!(stringBuffer).appendlong((((jcall!(self.role).getSessionTimeout() - (jcall!(calendar).getTimeInMillis() - timeCreated))) as i64).clone());
                jcall!(stringBuffer).append(String::from("<br/>\n"));
                jcall!(stringBuffer).append(String::from("Inactivity Time Allowed: "));
                jcall!(stringBuffer).appendlong(jcall!(self.role).getSessionTimeout());
                jcall!(stringBuffer).append(String::from("<br/>\n"));
                jcall!(stringBuffer).append(String::from("Inactivity Time Allowed: "));
                jcall!(stringBuffer).appendlong(jcall!(self.role).getSessionInactivityTimeout());
                jcall!(stringBuffer).append(String::from("<br/>\n"));
            } else {
                jcall!(stringBuffer).append(String::from("Role Is Not Set<br/>\n"));
            }
            return jcall!(stringBuffer).toString();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            return String::from("Error");
            }
        }
    }
    

    pub fn getRole(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>> {
        return self.role.clone();
    }
}
impl std::ops::Deref for AuthenticationHelper {
    type Target = TagHelper;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AuthenticationHelper {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::admin::taghelpers::TagHelper::TagHelperDyn for AuthenticationHelper {
    fn as_TagHelper(&self) -> &crate::admin::taghelpers::TagHelper::TagHelper { &self.base }
    fn as_TagHelper_mut(&mut self) -> &mut crate::admin::taghelpers::TagHelper::TagHelper { &mut self.base }
}
impl crate::admin::taghelpers::TagHelperInterface::TagHelperInterface for AuthenticationHelper {
}


