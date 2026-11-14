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
use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
// use crate::javax::servlet::jsp::PageContext::PageContext;
use crate::org::allbinary::globals::GLOBALS2::GLOBALS2;
use crate::org::allbinary::logic::communication::http::request::RequestMapInterface::RequestMapInterface;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::tags::HelperTag::HelperTag;
use crate::java::lang::Exception::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MultipartRequestParamForwardHelper {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub pageContext: Option<Rc<RefCell<PageContext>>>,
    pub requestMapInterface: Option<Rc<RefCell<dyn RequestMapInterface>>>,
    pub page: String,
    pub command: String,
    pub paramVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}


impl MultipartRequestParamForwardHelper {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn new_struct(mut parentHelperTag: Option<Rc<RefCell<dyn crate::tags::HelperTag::HelperTagDyn>>>, mut page: String, mut command: String, mut paramVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            pageContext: None,
            requestMapInterface: None,
            page: String::new(),
            command: String::new(),
            paramVector: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.pageContext = pageContext.clone();
        this.page = page.clone();
        this.command = command.clone();
        this.paramVector = paramVector.clone();
        this.requestMapInterface = { let java_value = (jcall!(InventoryItemViewParentTagHelper::InventoryItemViewParentTagHelper::getInstance()).getRequestMapInterface(parentHelperTag.clone())).clone(); java_value };
        this
    }

    pub fn new(mut parentHelperTag: Option<Rc<RefCell<dyn crate::tags::HelperTag::HelperTagDyn>>>, mut page: String, mut command: String, mut paramVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(parentHelperTag, page, command, paramVector, pageContext))))
    }
    

    // throws Exception
    pub fn forward(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if self.shouldForward() {
                let mut params: String = self.getParams();
                let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                jcall!(stringBuffer).append(self.page.clone());
                jcall!(stringBuffer).append(params.clone());
                { let java_receiver = self.pageContext.clone(); let java_arguments = (jcall!(stringBuffer).toString(),); jcall!(java_receiver).forward(java_arguments.0); };
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLTAGSERROR.clone(); java_field }.clone()) {
                jcall!(self.logUtil).put({ let java_field = jcall!(commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("forward()"), crate::java_into_object(e.clone()));
            }
            }
        }
    }
    

    pub fn getParams(&mut self) -> String {
        let mut commonSeps: Option<Rc<RefCell<CommonSeps>>> = CommonSeps::getInstance();
        let mut AMP: String = { let java_field = jcall!(commonSeps).AMPERSAND.clone(); java_field }.clone();
        let mut QUESTION: String = { let java_field = jcall!(commonSeps).QUESTION.clone(); java_field }.clone();
        let mut EQUALS: String = { let java_field = jcall!(commonSeps).EQUALS.clone(); java_field }.clone();
        let mut hashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> = jcall!(self.requestMapInterface).getRequestHashMap();
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut size: i32 = jcall!(self.paramVector).size();
        for index in (0..size) {
            let mut key: String = { let java_object = (jcall!(self.paramVector).get(index.clone())).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
            let mut value: String = { let java_object = (jcall!(hashMap).get(crate::java_into_object(key.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
            if index != 0 {
                jcall!(stringBuffer).append(AMP.clone());
            } else {
                jcall!(stringBuffer).append(QUESTION.clone());
            }
            jcall!(stringBuffer).append(key.clone());
            jcall!(stringBuffer).append(EQUALS.clone());
            jcall!(stringBuffer).append(value.clone());
        }
        return jcall!(stringBuffer).toString();
    }
    

    pub fn shouldForward(&mut self) -> bool {
        let mut hashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> = jcall!(self.requestMapInterface).getRequestHashMap();
        let mut requestCommand: String = { let java_object = (jcall!(hashMap).get(crate::java_into_object(crate::org::allbinary::globals::GLOBALS2::GLOBALS2_ADMINCOMMAND.with(Clone::clone).clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
        if jcall!(StringValidationUtil::getInstance()).isEmpty(self.command.clone()) || (!jcall!(StringValidationUtil::getInstance()).isEmpty(requestCommand.clone()) && (self.command.cmp(&requestCommand.clone()) as i32) == 0) {
            return true;
        }
        return false;
    }
}

