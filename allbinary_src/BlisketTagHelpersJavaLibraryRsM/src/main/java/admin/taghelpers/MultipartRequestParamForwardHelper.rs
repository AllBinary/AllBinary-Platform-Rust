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
// use crate::javax::servlet::jsp::PageContext::PageContext;
use crate::org::allbinary::globals::GLOBALS2::GLOBALS2;
use crate::org::allbinary::logic::communication::http::request::RequestMapInterface::RequestMapInterface;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::tags::HelperTag::HelperTag;

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
        this.requestMapInterface = jcall!(InventoryItemViewParentTagHelper::InventoryItemViewParentTagHelper::getInstance()).getRequestMapInterface(parentHelperTag.clone());
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
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGSERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("forward()"), crate::java_into_object(e.clone()));
            }
            }
        }
    }
    

    pub fn getParams(&mut self) -> String {
        let mut commonSeps: Option<Rc<RefCell<CommonSeps>>> = CommonSeps::getInstance();
        let mut AMP: String = jcall!(commonSeps).AMPERSAND.clone();
        let mut QUESTION: String = jcall!(commonSeps).QUESTION.clone();
        let mut EQUALS: String = jcall!(commonSeps).EQUALS.clone();
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = jcall!(self.requestMapInterface).getRequestHashMap();
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut size: i32 = jcall!(self.paramVector).size();
        for index in (0..size) {
            let mut key: String = (jcall!(self.paramVector).get(index.clone())).clone() /* as String */;
            let mut value: String = (jcall!(hashMap).get(key.clone())).clone() /* as String */;
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
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = jcall!(self.requestMapInterface).getRequestHashMap();
        let mut requestCommand: String = (jcall!(hashMap).get(crate::org::allbinary::globals::GLOBALS2::GLOBALS2_ADMINCOMMAND.with(Clone::clone).clone())).clone() /* as String */;
        if jcall!(StringValidationUtil::getInstance()).isEmpty(self.command.clone()) || (!jcall!(StringValidationUtil::getInstance()).isEmpty(requestCommand.clone()) && (self.command.cmp(&requestCommand.clone()) as i32) == 0) {
            return true;
        }
        return false;
    }
}

