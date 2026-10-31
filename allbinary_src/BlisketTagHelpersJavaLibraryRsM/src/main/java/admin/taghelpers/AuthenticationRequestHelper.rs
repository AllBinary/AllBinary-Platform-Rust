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
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
// use crate::javax::servlet::http::HttpServletRequest::HttpServletRequest;
// use crate::javax::servlet::jsp::PageContext::PageContext;
use crate::org::allbinary::business::user::UserData::UserData;
use crate::org::allbinary::business::user::UserInterface::UserInterface;
use crate::org::allbinary::business::user::password::Password::Password;
use crate::org::allbinary::business::user::role::BasicUserRole::BasicUserRole;
use crate::org::allbinary::data::tables::user::UserEntityFactory::UserEntityFactory;
use crate::org::allbinary::data::tables::user::UserEntityInterface::UserEntityInterface;
use crate::org::allbinary::globals::GLOBALS2::GLOBALS2;
use crate::org::allbinary::logic::communication::http::request::session::WeblisketSession::WeblisketSession;
use crate::org::allbinary::logic::communication::http::request::session::WeblisketSessionData::WeblisketSessionData;
use crate::org::allbinary::logic::communication::http::request::session::WeblisketSessionInterface::WeblisketSessionInterface;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::smtp::email::NewPasswordEmail::NewPasswordEmail;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::logic::system::security::licensing::ServiceClientInformationInterfaceFactory::ServiceClientInformationInterfaceFactory;
use super::TagHelper::TagHelper;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AuthenticationRequestHelper {
    pub base: TagHelper,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>,
    pub weblisketSession: Option<Rc<RefCell<WeblisketSession>>>,
    pub request: Option<Rc<RefCell<HttpServletRequest>>>,
}


impl AuthenticationRequestHelper {
    

    pub fn new_struct2(mut hashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Self {
        let mut this = Self {
            base: TagHelper::new_struct(),
            logUtil: None,
            abeClientInformation: None,
            weblisketSession: None,
            request: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.abeClientInformation = ServiceClientInformationInterfaceFactory::getInstance();
        this.weblisketSession = WeblisketSession::new2(hashMap.clone(), pageContext.clone());
        this.request = (jcall!(pageContext).getRequest()).clone() /* as HttpServletRequest */;
        this
    }

    pub fn new2(mut hashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2(hashMap, pageContext))))
    }
    

    pub fn new_struct2(mut hashMap: Option<Rc<RefCell<HashMap>>>, mut httpServletRequest: Option<Rc<RefCell<HttpServletRequest>>>) -> Self {
        let mut this = Self {
            base: TagHelper::new_struct(),
            logUtil: None,
            abeClientInformation: None,
            weblisketSession: None,
            request: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.abeClientInformation = ServiceClientInformationInterfaceFactory::getInstance();
        this.weblisketSession = WeblisketSession::new2(hashMap.clone(), httpServletRequest.clone());
        this.request = httpServletRequest.clone();
        this
    }

    pub fn new2(mut hashMap: Option<Rc<RefCell<HashMap>>>, mut httpServletRequest: Option<Rc<RefCell<HttpServletRequest>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2(hashMap, httpServletRequest))))
    }
    

    // throws Exception
    pub fn generateNewPassword(&mut self) -> String {
        if self.weblisketSession.is_some() && jcall!(self.weblisketSession).getId().is_some() {
            let mut startIndex: i32 = jcall!(jcall!(self.weblisketSession).getId()).length();
            if startIndex >= 8 {
                return jcall!(jcall!(self.weblisketSession).getId()).substring(startIndex - 8);
            } else {
                panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Error Generating New Password"))));
            }
        } else {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("No Session Available For Generating New Password"))));
        }
    }
    

    pub fn newPassword(&mut self) -> Option<bool> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut userName: String = jcall!(self.request).getParameter(crate::org::allbinary::logic::communication::http::request::session::WeblisketSessionData::WeblisketSessionData_REMOVABLEUSERNAME.with(Clone::clone).clone());
            let mut email: String = jcall!(self.request).getParameter(crate::org::allbinary::business::user::UserData::UserData_MAINEMAIL.with(Clone::clone).clone());
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).TAGHELPER.clone()) {
                jcall!(self.logUtil).putF(format!("{}{}", String::from("Generating New Password For: "), userName), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("newPassword()"));
            }
            let mut userEntityInterface: Option<Rc<RefCell<dyn UserEntityInterface>>> = UserEntityFactory::getInstance().map(|reference| reference as Rc<RefCell<dyn UserEntityInterface>>);
            let mut userInterface: Option<Rc<RefCell<dyn UserInterface>>> = jcall!(userEntityInterface).getUser(userName.clone());
            if jcall!(jcall!(userInterface).getMainEmail()).compareTo(email.clone()) != 0 {
                if jcall!(userInterface).getSecondaryEmail().is_some() && jcall!(jcall!(userInterface).getSecondaryEmail()).compareTo(email.clone()) != 0 {
                    if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).TAGHELPER.clone()) {
                        jcall!(self.logUtil).putF(String::from("Requested Email Is Not In User Profile"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("newPassword()"));
                    }
                    return false;
                }
            }
            let mut newPassword: String = self.generateNewPassword();
            jcall!(userInterface).setPassword(newPassword.clone());
            let mut newPasswordHashMap: Option<Rc<RefCell<HashMap>>> = jcall!(userInterface).toPasswordHashMap();
            jcall!(UserEntityFactory::getInstance()).update(userName.clone(), newPasswordHashMap.clone());
            jcall!(NewPasswordEmail::new(self.abeClientInformation.clone(), userInterface.clone(), newPassword.clone())).process();
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).TAGHELPER.clone()) {
                jcall!(self.logUtil).putF(format!("{}{}", String::from("Generated New Password For: "), userName), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("newPassword()"));
            }
            return true;
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).TAGHELPERERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("newPassword()"), crate::java_into_object(e.clone()));
            }
            return false;
            }
        }
    }
    

    pub fn changePassword(&mut self) -> Option<bool> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut userName: String = jcall!(self.request).getParameter(crate::org::allbinary::logic::communication::http::request::session::WeblisketSessionData::WeblisketSessionData_REMOVABLEUSERNAME.with(Clone::clone).clone());
            let mut passwordString: String = jcall!(self.request).getParameter(crate::org::allbinary::logic::communication::http::request::session::WeblisketSessionData::WeblisketSessionData_REMOVABLEPASSWORD.with(Clone::clone).clone());
            let mut newPassword: String = jcall!(self.request).getParameter(crate::org::allbinary::logic::communication::http::request::session::WeblisketSessionData::WeblisketSessionData_REMOVABLENEWPASSWORD.with(Clone::clone).clone());
            let mut newReenteredPassword: String = jcall!(self.request).getParameter(crate::org::allbinary::logic::communication::http::request::session::WeblisketSessionData::WeblisketSessionData_REMOVABLEREENTERNEWPASSWORD.with(Clone::clone).clone());
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).TAGHELPER.clone()) {
                jcall!(self.logUtil).putF(format!("{}{}", String::from("Changing Password For User: "), userName), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("changePassword()"));
            }
            let mut password: Option<Rc<RefCell<Password>>> = Password::new(newPassword.clone());
            if !jcall!(password).isValid().unwrap() {
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).TAGHELPER.clone()) {
                    jcall!(self.logUtil).putF(String::from("New Password Is Not Valid"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("changePassword()"));
                }
                return false;
            }
            if (newPassword.cmp(&newReenteredPassword.clone()) as i32) != 0 {
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).TAGHELPER.clone()) {
                    jcall!(self.logUtil).putF(String::from("New Password Fields Do Not Match"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("changePassword()"));
                }
                return false;
            }
            let mut sessionPassword: String = jcall!(self.weblisketSession).getPassword();
            let mut userEntityInterface: Option<Rc<RefCell<dyn UserEntityInterface>>> = UserEntityFactory::getInstance().map(|reference| reference as Rc<RefCell<dyn UserEntityInterface>>);
            let mut userInterface: Option<Rc<RefCell<dyn UserInterface>>> = jcall!(userEntityInterface).getUser(userName.clone());
            let mut login: String = jcall!(userEntityInterface).login(userName.clone(), passwordString.clone());
            if (login.cmp(&crate::org::allbinary::globals::GLOBALS2::GLOBALS2_LOGINSUCCESS.with(Clone::clone).clone()) as i32) == 0 {
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).TAGHELPER.clone()) {
                    jcall!(self.logUtil).putF(String::from("Authentication Successful"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("changePassword()"));
                }
                let mut newPasswordHashMap: Option<Rc<RefCell<HashMap>>> = jcall!(password).toHashMap(None);
                jcall!(UserEntityFactory::getInstance()).update(userName.clone(), newPasswordHashMap.clone());
                jcall!(NewPasswordEmail::new(self.abeClientInformation.clone(), userInterface.clone(), newPassword.clone())).process();
                return true;
            }
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).TAGHELPER.clone()) {
                jcall!(self.logUtil).putF(String::from("Authentication Failed"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("changePassword()"));
            }
            return false;
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).TAGHELPERERROR.clone()) {
                jcall!(self.logUtil).put(String::from("Authentication Failed"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("changePassword()"), crate::java_into_object(e.clone()));
            }
            return false;
            }
        }
    }
    

    pub fn isRoleValid(&mut self, mut userName: String, mut password: String, mut roles: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Option<bool> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut userEntityInterface: Option<Rc<RefCell<dyn UserEntityInterface>>> = UserEntityFactory::getInstance().map(|reference| reference as Rc<RefCell<dyn UserEntityInterface>>);
            let mut userInterface: Option<Rc<RefCell<dyn UserInterface>>> = jcall!(userEntityInterface).getUser(userName.clone());
            if jcall!(userInterface).getRole().is_none() {
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).TAGHELPER.clone()) {
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", format!("{}{}", format!("{}{}", String::from("Role is null: "), jcall!(userInterface).getRole()), String::from(" Valid Roles: ")), jcall!(roles).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("isRoleValid()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                }
                return false;
            }
            let mut basicUserRoleArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(roles).toArray();
            let mut size: i32 = (basicUserRoleArray.len() as i32).clone();
            for index in (0..size) {
                let mut nextRole: Option<Rc<RefCell<BasicUserRole>>> = (basicUserRoleArray[(index) as usize]).clone() /* as BasicUserRole */;
                if jcall!(jcall!(jcall!(userInterface).getRole()).getBasicUserRole()).equals(nextRole.clone()) {
                    jcall!(userInterface).validateSession((self.weblisketSession).clone() /* as WeblisketSessionInterface */);
                    jcall!(self.request).removeAttribute(crate::org::allbinary::logic::communication::http::request::session::WeblisketSessionData::WeblisketSessionData_REMOVABLEUSERNAME.with(Clone::clone).clone());
                    jcall!(self.request).removeAttribute(crate::org::allbinary::logic::communication::http::request::session::WeblisketSessionData::WeblisketSessionData_REMOVABLEPASSWORD.with(Clone::clone).clone());
                    return true;
                }
            }
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).TAGHELPER.clone()) {
                let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                jcall!(stringBuffer).append(String::from("Role is not valid: "));
                jcall!(stringBuffer).append(jcall!(jcall!(userInterface).getRole()).toString());
                jcall!(stringBuffer).append(String::from(" Valid Roles: "));
                jcall!(stringBuffer).append(jcall!(roles).toString());
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("isRoleValid()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            return false;
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).TAGHELPERERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("isRoleValid()"), crate::java_into_object(e.clone()));
            }
            return false;
            }
        }
    }
}
impl std::ops::Deref for AuthenticationRequestHelper {
    type Target = TagHelper;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AuthenticationRequestHelper {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::admin::taghelpers::TagHelper::TagHelperDyn for AuthenticationRequestHelper {
    fn as_TagHelper(&self) -> &crate::admin::taghelpers::TagHelper::TagHelper { &self.base }
    fn as_TagHelper_mut(&mut self) -> &mut crate::admin::taghelpers::TagHelper::TagHelper { &mut self.base }
}
impl crate::admin::taghelpers::TagHelperInterface::TagHelperInterface for AuthenticationRequestHelper {
}


