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
// package org.allbinary.business.context.modules.storefront
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::business::user::commerce::inventory::basket::BasketData::BasketData;
use crate::org::allbinary::data::tree::dom::DomNodeInterface::DomNodeInterface;
use crate::org::allbinary::data::tree::dom::ModDomHelper::ModDomHelper;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use crate::java::lang::Exception::*;
use super::StoreFrontInterface::StoreFrontInterface;
use super::StoreFrontData::StoreFrontData;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct StoreFrontView {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>>,
}

impl DomNodeInterface for StoreFrontView {
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}

impl StoreFrontView {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn new_struct(mut storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            storeFrontInterface: None,
        };
        this.logUtil = LogUtil::getInstance();
        if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).VIEW.clone(); java_field }.clone()) {
            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            { let java_receiver = this.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Constructing: StoreFrontView for: "), jcall!(storeFrontInterface).getName()), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(commonStrings).GET_INSTANCE.clone(); java_field }.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        this.storeFrontInterface = storeFrontInterface.clone();
        this
    }

    pub fn new(mut storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(storeFrontInterface))))
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        let mut hashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> = jcall!(self.storeFrontInterface).toHashMap();
        let mut storeFrontData: Option<Rc<RefCell<StoreFrontData>>> = StoreFrontData::getInstance();
        jcall!(hashMap).put(crate::java_into_object({ let java_field = jcall!(storeFrontData).CURRENTHOMEHOSTNAME.clone(); java_field }.clone()), crate::java_into_object(jcall!(self.storeFrontInterface).getCurrentHomeHostName()));
        jcall!(hashMap).put(crate::java_into_object({ let java_field = jcall!(storeFrontData).CURRENTHOMEHOSTNAMEPATH.clone(); java_field }.clone()), crate::java_into_object(jcall!(self.storeFrontInterface).getCurrentHomeHostNamePath()));
        jcall!(hashMap).put(crate::java_into_object({ let java_field = jcall!(storeFrontData).CURRENTHOSTNAME.clone(); java_field }.clone()), crate::java_into_object(jcall!(self.storeFrontInterface).getCurrentHostName()));
        jcall!(hashMap).put(crate::java_into_object({ let java_field = jcall!(storeFrontData).CURRENTHOSTNAMEPATH.clone(); java_field }.clone()), crate::java_into_object(jcall!(self.storeFrontInterface).getCurrentHostNamePath()));
        jcall!(hashMap).put(crate::java_into_object(crate::org::allbinary::business::user::commerce::inventory::basket::BasketData::BasketData_NAME.with(Clone::clone).clone()), crate::java_into_object(jcall!(self.storeFrontInterface).getBasketName()));
        let mut node: Option<Rc<RefCell<Node>>> = ModDomHelper::createNameValueNodes_Document_String_HashMap(document.clone(), { let java_field = jcall!(storeFrontData).NAME.clone(); java_field }.clone(), hashMap.clone());
        return node.clone();
    }
}

