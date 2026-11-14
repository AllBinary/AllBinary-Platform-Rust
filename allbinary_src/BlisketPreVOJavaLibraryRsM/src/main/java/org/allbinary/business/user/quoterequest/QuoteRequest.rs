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
// package org.allbinary.business.user.quoterequest
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::java::util::Calendar::Calendar;
use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
// use crate::javax::servlet::http::HttpServletRequest::HttpServletRequest;
use crate::org::allbinary::business::entry::EntryData::EntryData;
use crate::org::allbinary::business::quoterequest::QuoteRequestData::QuoteRequestData;
use crate::org::allbinary::business::user::UserData::UserData;
use crate::org::allbinary::data::generator::QuoteRequestIdGenerator::QuoteRequestIdGenerator;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::java::lang::Exception::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct QuoteRequest {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub id: String,
    pub userName: String,
    pub projectInfo: String,
    pub userComments: String,
    pub budget: String,
    pub timeFrame: String,
    pub comments: String,
}


impl QuoteRequest {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn new_struct2(mut userName: String, mut request: Option<Rc<RefCell<HttpServletRequest>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            id: String::new(),
            userName: String::new(),
            projectInfo: String::new(),
            userComments: String::new(),
            budget: String::new(),
            timeFrame: String::new(),
            comments: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        let mut quoteRequestData: Option<Rc<RefCell<QuoteRequestData>>> = QuoteRequestData::getInstance();
        this.id = { let java_value = (jcall!(QuoteRequestIdGenerator::new()).getNext()).clone(); java_value };
        this.setUserName(userName.clone());
        { let java_arguments = ((jcall!(request).getParameter({ let java_field = jcall!(quoteRequestData).PROJECT_INFO.clone(); java_field }.clone())).clone() /* as String */,); this.setProjectInfo(java_arguments.0) };
        { let java_arguments = ((jcall!(request).getParameter({ let java_field = jcall!(quoteRequestData).CUSTOMER_COMMENTS.clone(); java_field }.clone())).clone() /* as String */,); this.setUserComments(java_arguments.0) };
        { let java_arguments = ((jcall!(request).getParameter({ let java_field = jcall!(quoteRequestData).BUDGET.clone(); java_field }.clone())).clone() /* as String */,); this.setBudget(java_arguments.0) };
        { let java_arguments = ((jcall!(request).getParameter({ let java_field = jcall!(quoteRequestData).TIMEFRAME.clone(); java_field }.clone())).clone() /* as String */,); this.setTimeFrame(java_arguments.0) };
        { let java_arguments = ((jcall!(request).getParameter({ let java_field = jcall!(quoteRequestData).COMMENTS.clone(); java_field }.clone())).clone() /* as String */,); this.setComments(java_arguments.0) };
        if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).TAGHELPER.clone(); java_field }.clone()) {
            { let java_receiver = this.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Created with: "), this.toHashMap()), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(this.commonStrings).CONSTRUCTOR.clone(); java_field }.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        this
    }

    pub fn new2(mut userName: String, mut request: Option<Rc<RefCell<HttpServletRequest>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2(userName, request))))
    }
    

    pub fn new_struct1(mut hashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            id: String::new(),
            userName: String::new(),
            projectInfo: String::new(),
            userComments: String::new(),
            budget: String::new(),
            timeFrame: String::new(),
            comments: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        let mut quoteRequestData: Option<Rc<RefCell<QuoteRequestData>>> = QuoteRequestData::getInstance();
        this.id = { let java_value = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(quoteRequestData).ID.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        { let java_arguments = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object(crate::org::allbinary::business::user::UserData::UserData_USERNAME.with(Clone::clone).clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() },); this.setUserName(java_arguments.0) };
        { let java_arguments = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(quoteRequestData).PROJECT_INFO.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() },); this.setProjectInfo(java_arguments.0) };
        { let java_arguments = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(quoteRequestData).CUSTOMER_COMMENTS.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() },); this.setUserComments(java_arguments.0) };
        { let java_arguments = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(quoteRequestData).BUDGET.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() },); this.setBudget(java_arguments.0) };
        { let java_arguments = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(quoteRequestData).TIMEFRAME.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() },); this.setTimeFrame(java_arguments.0) };
        { let java_arguments = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(quoteRequestData).COMMENTS.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() },); this.setComments(java_arguments.0) };
        if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).TAGHELPER.clone(); java_field }.clone()) {
            { let java_receiver = this.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Created with: "), this.toHashMap()), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(this.commonStrings).CONSTRUCTOR.clone(); java_field }.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        this
    }

    pub fn new1(mut hashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(hashMap))))
    }
    

    pub fn isValid(&mut self) -> Option<bool> {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut valid: Option<bool> = true;
            if false || (self.userName.chars().count() as i32) < 4 || (self.userName.chars().count() as i32) > 250 {
                valid = { let java_value = (false).clone(); java_value }.clone();
            }
            if false || (self.projectInfo.chars().count() as i32) < 0 || (self.projectInfo.chars().count() as i32) > crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXBLOB.with(Clone::clone) {
                valid = { let java_value = (false).clone(); java_value }.clone();
            }
            if false || (self.userComments.chars().count() as i32) < 0 || (self.userComments.chars().count() as i32) > crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXBLOB.with(Clone::clone) {
                valid = { let java_value = (false).clone(); java_value }.clone();
            }
            if false || (self.budget.chars().count() as i32) < 0 || (self.budget.chars().count() as i32) > crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXBLOB.with(Clone::clone) {
                valid = { let java_value = (false).clone(); java_value }.clone();
            }
            if false || (self.timeFrame.chars().count() as i32) < 0 || (self.timeFrame.chars().count() as i32) > crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXBLOB.with(Clone::clone) {
                valid = { let java_value = (false).clone(); java_value }.clone();
            }
            if false || (self.comments.chars().count() as i32) < 0 || (self.comments.chars().count() as i32) > crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXBLOB.with(Clone::clone) {
                valid = { let java_value = (false).clone(); java_value }.clone();
            }
            return Some(valid.clone());
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLTAGSERROR.clone(); java_field }.clone()) {
                jcall!(self.logUtil).put(String::from("Failed to validate form"), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).IS_VALID.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
            return false;
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    pub fn validationInfo(&mut self) -> String {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            if false || (self.userName.chars().count() as i32) < 4 || (self.userName.chars().count() as i32) > 250 {
                jcall!(stringBuffer).append(String::from("Please enter a User Name with more than 4 characters.<br>"));
            }
            if false || (self.projectInfo.chars().count() as i32) < 0 || (self.projectInfo.chars().count() as i32) > crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXBLOB.with(Clone::clone) {
                jcall!(stringBuffer).append(format!("{}{}", format!("{}{}", String::from("Please enter Project Info with less than "), crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXBLOB.with(Clone::clone)), String::from(" characters.<br>")));
            }
            if false || (self.userComments.chars().count() as i32) < 0 || (self.userComments.chars().count() as i32) > crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXBLOB.with(Clone::clone) {
                jcall!(stringBuffer).append(format!("{}{}", format!("{}{}", String::from("Please enter User Comments with less than "), crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXBLOB.with(Clone::clone)), String::from(" characters.<br>")));
            }
            if false || (self.budget.chars().count() as i32) < 0 || (self.budget.chars().count() as i32) > crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXBLOB.with(Clone::clone) {
                jcall!(stringBuffer).append(format!("{}{}", format!("{}{}", String::from("Please enter Budget with less than "), crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXBLOB.with(Clone::clone)), String::from(" characters.<br>")));
            }
            if false || (self.timeFrame.chars().count() as i32) < 0 || (self.timeFrame.chars().count() as i32) > crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXBLOB.with(Clone::clone) {
                jcall!(stringBuffer).append(format!("{}{}", format!("{}{}", String::from("Please enter time frame with less than "), crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXBLOB.with(Clone::clone)), String::from(" characters.<br>")));
            }
            if false || (self.comments.chars().count() as i32) < 0 || (self.comments.chars().count() as i32) > crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXBLOB.with(Clone::clone) {
                jcall!(stringBuffer).append(format!("{}{}", format!("{}{}", String::from("Please enter comments with less than "), crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXBLOB.with(Clone::clone)), String::from(" characters.<br>")));
            }
            return Some(jcall!(stringBuffer).toString());
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLTAGSERROR.clone(); java_field }.clone()) {
                jcall!(self.logUtil).put(String::from("Failed to generate validation error info"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("validationInfo()"), crate::java_into_object(e.clone()));
            }
            return String::from("Error Validating Form");
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    pub fn toVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut values: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        jcall!(values).add(crate::java_into_object(self.id.clone()));
        jcall!(values).add(crate::java_into_object(self.userName.clone()));
        jcall!(values).add(crate::java_into_object(self.projectInfo.clone()));
        jcall!(values).add(crate::java_into_object(self.userComments.clone()));
        jcall!(values).add(crate::java_into_object(self.budget.clone()));
        jcall!(values).add(crate::java_into_object(self.timeFrame.clone()));
        jcall!(values).add(crate::java_into_object(self.comments.clone()));
        let mut calendar: Option<Rc<RefCell<Calendar>>> = Calendar::getInstance();
        let mut time: String = (Some((jcall!(calendar).getTimeInMillis()) as i64)).expect("NullPointerException").to_string().clone();
        jcall!(values).add(crate::java_into_object(time.clone()));
        jcall!(values).add(crate::java_into_object(time.clone()));
        return values.clone();
    }
    

    pub fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> {
        let mut quoteRequestData: Option<Rc<RefCell<QuoteRequestData>>> = QuoteRequestData::getInstance();
        let mut values: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> = jcall!(StdUtil::getInstance()).createHashMap();
        jcall!(values).put(crate::java_into_object(crate::org::allbinary::business::user::UserData::UserData_USERNAME.with(Clone::clone).clone()), crate::java_into_object(self.userName.clone()));
        jcall!(values).put(crate::java_into_object({ let java_field = jcall!(quoteRequestData).PROJECT_INFO.clone(); java_field }.clone()), crate::java_into_object(self.projectInfo.clone()));
        jcall!(values).put(crate::java_into_object({ let java_field = jcall!(quoteRequestData).CUSTOMER_COMMENTS.clone(); java_field }.clone()), crate::java_into_object(self.userComments.clone()));
        jcall!(values).put(crate::java_into_object({ let java_field = jcall!(quoteRequestData).BUDGET.clone(); java_field }.clone()), crate::java_into_object(self.budget.clone()));
        jcall!(values).put(crate::java_into_object({ let java_field = jcall!(quoteRequestData).TIMEFRAME.clone(); java_field }.clone()), crate::java_into_object(self.timeFrame.clone()));
        jcall!(values).put(crate::java_into_object({ let java_field = jcall!(quoteRequestData).COMMENTS.clone(); java_field }.clone()), crate::java_into_object(self.comments.clone()));
        let mut calendar: Option<Rc<RefCell<Calendar>>> = Calendar::getInstance();
        let mut time: String = (Some((jcall!(calendar).getTimeInMillis()) as i64)).expect("NullPointerException").to_string().clone();
        jcall!(values).put(crate::java_into_object({ let java_field = jcall!(EntryData::getInstance()).LASTMODIFIED.clone(); java_field }.clone()), crate::java_into_object(time.clone()));
        return values.clone();
    }
    

    pub fn setUserName(&mut self, mut value: String) {
        self.userName = value.clone();
    }
    

    pub fn setProjectInfo(&mut self, mut value: String) {
        self.projectInfo = value.clone();
    }
    

    pub fn setUserComments(&mut self, mut value: String) {
        self.userComments = value.clone();
    }
    

    pub fn setBudget(&mut self, mut value: String) {
        self.budget = value.clone();
    }
    

    pub fn setTimeFrame(&mut self, mut value: String) {
        self.timeFrame = value.clone();
    }
    

    pub fn setComments(&mut self, mut value: String) {
        self.comments = value.clone();
        let mut stringUtil: Option<Rc<RefCell<StringUtil>>> = StringUtil::getInstance();
        if false {
            self.comments = { let java_value = ({ let java_field = jcall!(stringUtil).EMPTY_STRING.clone(); java_field }).clone(); java_value }.clone();
        }
    }
    

    pub fn getUserName(&mut self) -> String {
        return self.userName.clone();
    }
    

    pub fn getProjectInfo(&mut self) -> String {
        return self.projectInfo.clone();
    }
    

    pub fn getUserComments(&mut self) -> String {
        return self.userComments.clone();
    }
    

    pub fn getBudget(&mut self) -> String {
        return self.budget.clone();
    }
    

    pub fn getTimeFrame(&mut self) -> String {
        return self.timeFrame.clone();
    }
    

    pub fn getComments(&mut self) -> String {
        return self.comments.clone();
    }
}

