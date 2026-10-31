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
// package org.allbinary.logic.control.workflow.request
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use crate::Calendar;
use std::collections::HashMap;
use super::*;
// use crate::java::util::Calendar::Calendar;
// use crate::java::util::HashMap::HashMap;
// use crate::java::util::Set::Set;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
// use crate::javax::servlet::jsp::PageContext::PageContext;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontData::StoreFrontData;
use crate::org::allbinary::business::entry::EntryData::EntryData;
use crate::org::allbinary::data::tree::dom::ModDomHelper::ModDomHelper;
use crate::org::allbinary::data::tree::dom::document::DomDocumentHelper::DomDocumentHelper;
use crate::org::allbinary::data::tree::dom::document::XmlDocumentHelper::XmlDocumentHelper;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::http::request::RequestParams::RequestParams;
use crate::org::allbinary::logic::communication::http::request::session::WeblisketSession::WeblisketSession;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::control::workflow::WorkFlowData::WorkFlowData;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicWorkFlow {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub workFlowDoc: Option<Rc<RefCell<Document>>>,
    pub workFlowName: String,
    pub storeName: String,
}


impl BasicWorkFlow {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            workFlowDoc: None,
            workFlowName: String::new(),
            storeName: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.workFlowName = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        this.workFlowDoc = DomDocumentHelper::create();
        let mut workFlowNode: Option<Rc<RefCell<Node>>> = { let java_receiver = this.workFlowDoc.clone(); let java_arguments = (jcall!(WorkFlowData::getInstance()).WORKFLOW.clone(),); let java_result = jcall!(java_receiver).createElement(java_arguments.0); java_result };
        jcall!(this.workFlowDoc).appendChild(workFlowNode.clone());
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn new_struct2(mut hashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            workFlowDoc: None,
            workFlowName: String::new(),
            storeName: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        let mut weblisketSession: Option<Rc<RefCell<WeblisketSession>>> = WeblisketSession::new2(hashMap.clone(), pageContext.clone());
        this.storeName = jcall!(weblisketSession).getStoreName();
        let mut requestHashMap: Option<Rc<RefCell<HashMap>>> = jcall!(RequestParams::new1(pageContext.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::communication::http::request::RequestParams::RequestParamsDyn>>)).toHashMap();
        this.workFlowName = (jcall!(requestHashMap).get(jcall!(WorkFlowData::getInstance()).NAME.clone())).clone() /* as String */;
        this.workFlowDoc = DomDocumentHelper::create1((jcall!(requestHashMap).get(jcall!(WorkFlowData::getInstance()).DATA.clone())).clone() /* as String */);
        this
    }

    pub fn new2(mut hashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2(hashMap, pageContext))))
    }
    

    pub fn new_struct1(mut hashMap: Option<Rc<RefCell<HashMap>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            workFlowDoc: None,
            workFlowName: String::new(),
            storeName: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.workFlowName = (jcall!(hashMap).get(jcall!(WorkFlowData::getInstance()).NAME.clone())).clone() /* as String */;
        this.storeName = (jcall!(hashMap).get(jcall!(StoreFrontData::getInstance()).NAME.clone())).clone() /* as String */;
        this.workFlowDoc = DomDocumentHelper::create1((jcall!(hashMap).get(jcall!(WorkFlowData::getInstance()).DATA.clone())).clone() /* as String */);
        this
    }

    pub fn new1(mut hashMap: Option<Rc<RefCell<HashMap>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(hashMap))))
    }
    

    pub fn getName(&mut self) -> String {
        return self.workFlowName.clone();
    }
    

    pub fn getStoreName(&mut self) -> String {
        return self.storeName.clone();
    }
    

    pub fn getKey(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return (self.getName()).clone() /* as Any */;
    }
    

    // throws Exception
    pub fn toVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut values: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        jcall!(values).add(crate::java_into_object(self.workFlowName.clone()));
        jcall!(values).add(crate::java_into_object(self.storeName.clone()));
        jcall!(values).add(crate::java_into_object(XmlDocumentHelper::toString(self.workFlowDoc.clone())));
        let mut calendar: Option<Rc<RefCell<Calendar>>> = Calendar::getInstance();
        let mut time: String = jcall!(Some((jcall!(calendar).getTimeInMillis()) as i64)).toString().clone();
        jcall!(values).add(crate::java_into_object(time.clone()));
        jcall!(values).add(crate::java_into_object(time.clone()));
        return values.clone();
    }
    

    // throws Exception
    pub fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> {
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        jcall!(hashMap).put(jcall!(WorkFlowData::getInstance()).NAME.clone(), self.workFlowName.clone());
        jcall!(hashMap).put(jcall!(StoreFrontData::getInstance()).NAME.clone(), self.storeName.clone());
        jcall!(hashMap).put(jcall!(WorkFlowData::getInstance()).DATA.clone(), XmlDocumentHelper::toString(self.workFlowDoc.clone()));
        let mut calendar: Option<Rc<RefCell<Calendar>>> = Calendar::getInstance();
        let mut time: String = jcall!(Some((jcall!(calendar).getTimeInMillis()) as i64)).toString().clone();
        jcall!(hashMap).put(jcall!(EntryData::getInstance()).LASTMODIFIED.clone(), time.clone());
        return hashMap.clone();
    }
    

    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut hashMap: Option<Rc<RefCell<HashMap>>> = self.toHashMap();
            let mut keySet: Option<Rc<RefCell<dyn Set>>> = jcall!(hashMap).keySet();
            let mut node: Option<Rc<RefCell<Node>>> = jcall!(document).createElement(jcall!(WorkFlowData::getInstance()).WORKFLOW.clone());
            let mut nameArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(keySet).toArray();
            let mut size: i32 = (nameArray.len() as i32).clone();
            for index in (0..size) {
                let mut name: String = (nameArray[(index) as usize]).clone() /* as String */;
                let mut value: String = (jcall!(hashMap).get(name.clone())).clone() /* as String */;
                jcall!(node).appendChild(ModDomHelper::createNameValueNodes(document.clone(), name.clone(), value.clone()));
            }
            return node.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGSERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("toXmlNode()"), crate::java_into_object(e.clone()));
            }
            return None;
            }
        }
    }
    

    pub fn toXmlDoc(&mut self) -> Option<Rc<RefCell<Document>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            return self.workFlowDoc.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGSERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("toXmlDoc()"), crate::java_into_object(e.clone()));
            }
            return None;
            }
        }
    }
    

    pub fn isValid(&mut self) -> Option<bool> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut returnBoolean: Option<bool> = true;
            if false {
                returnBoolean = false;
            }
            return returnBoolean.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGSERROR.clone()) {
                jcall!(self.logUtil).put(String::from("Failed to validate"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).IS_VALID.clone(), crate::java_into_object(e.clone()));
            }
            return false;
            }
        }
    }
    

    pub fn validationInfo(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            if false {
                jcall!(stringBuffer).append(String::from("No WorkFlow name specified."));
            }
            return jcall!(stringBuffer).toString();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEWERROR.clone()) {
                jcall!(self.logUtil).put(String::from("Failed to generate validation error info"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("validationInfo()"), crate::java_into_object(e.clone()));
            }
            return String::from("Error Getting Validation Info");
            }
        }
    }
    

    pub fn toValidationInfoDoc(&mut self) -> Option<Rc<RefCell<Document>>> {
        return None;
    }
    

    pub fn toValidationInfoNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        return None;
    }
}
pub trait BasicWorkFlowDyn {
    fn as_BasicWorkFlow(&self) -> &BasicWorkFlow;
    fn as_BasicWorkFlow_mut(&mut self) -> &mut BasicWorkFlow;
    fn getName(&mut self) -> String { BasicWorkFlow::getName(self.as_BasicWorkFlow_mut()) }
    fn getStoreName(&mut self) -> String { BasicWorkFlow::getStoreName(self.as_BasicWorkFlow_mut()) }
    fn getKey(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { BasicWorkFlow::getKey(self.as_BasicWorkFlow_mut()) }
    fn toVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { BasicWorkFlow::toVector(self.as_BasicWorkFlow_mut()) }
    fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> { BasicWorkFlow::toHashMap(self.as_BasicWorkFlow_mut()) }
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { BasicWorkFlow::toXmlNode(self.as_BasicWorkFlow_mut(), document) }
    fn toXmlDoc(&mut self) -> Option<Rc<RefCell<Document>>> { BasicWorkFlow::toXmlDoc(self.as_BasicWorkFlow_mut()) }
    fn isValid(&mut self) -> Option<bool> { BasicWorkFlow::isValid(self.as_BasicWorkFlow_mut()) }
    fn validationInfo(&mut self) -> String { BasicWorkFlow::validationInfo(self.as_BasicWorkFlow_mut()) }
    fn toValidationInfoDoc(&mut self) -> Option<Rc<RefCell<Document>>> { BasicWorkFlow::toValidationInfoDoc(self.as_BasicWorkFlow_mut()) }
    fn toValidationInfoNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { BasicWorkFlow::toValidationInfoNode(self.as_BasicWorkFlow_mut(), document) }
}
impl BasicWorkFlowDyn for BasicWorkFlow {
    fn as_BasicWorkFlow(&self) -> &BasicWorkFlow { self }
    fn as_BasicWorkFlow_mut(&mut self) -> &mut BasicWorkFlow { self }
}


