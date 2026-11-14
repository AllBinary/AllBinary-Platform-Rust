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
use super::*;
use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::business::entry::EntryData::EntryData;
use crate::org::allbinary::data::tables::TableMappingInterface::TableMappingInterface;
use crate::org::allbinary::data::tree::dom::DomNodeInterface::DomNodeInterface;
use crate::org::allbinary::data::tree::dom::ModDomHelper::ModDomHelper;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use crate::java::lang::Exception::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct UserDomNode {
    pub dataMappingInterface: Option<Rc<RefCell<dyn TableMappingInterface>>>,
}

impl DomNodeInterface for UserDomNode {
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}

impl UserDomNode {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn new_struct(mut dataMappingInterface: Option<Rc<RefCell<dyn TableMappingInterface>>>) -> Self {
        let mut this = Self {
            dataMappingInterface: None,
        };
        this.dataMappingInterface = dataMappingInterface.clone();
        this
    }

    pub fn new(mut dataMappingInterface: Option<Rc<RefCell<dyn TableMappingInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(dataMappingInterface))))
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        let mut EMPTY_STRING: String = { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone();
        let mut hashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> = jcall!(self.dataMappingInterface).toHashMap();
        jcall!(hashMap).put(crate::java_into_object({ let java_field = jcall!(EntryData::getInstance()).LASTMODIFIED.clone(); java_field }.clone()), crate::java_into_object(EMPTY_STRING.clone()));
        jcall!(hashMap).put(crate::java_into_object({ let java_field = jcall!(EntryData::getInstance()).ENCRYPTION.clone(); java_field }.clone()), crate::java_into_object(EMPTY_STRING.clone()));
        jcall!(hashMap).put(crate::java_into_object(UserData_PASSWORD.with(Clone::clone).clone()), crate::java_into_object(EMPTY_STRING.clone()));
        return ModDomHelper::createNameValueNodes_Document_String_HashMap(document.clone(), UserData_NAME.with(Clone::clone).clone(), hashMap.clone());
    }
    

    // throws Exception
    pub fn toXmlDoc(&mut self) -> Option<Rc<RefCell<Document>>> {
        return None;
    }
    

    // throws Exception
    pub fn view(&mut self) -> String {
        return None;
    }
}

