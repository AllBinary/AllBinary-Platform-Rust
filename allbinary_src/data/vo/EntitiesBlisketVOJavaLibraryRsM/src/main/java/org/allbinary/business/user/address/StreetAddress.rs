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
// package org.allbinary.business.user.address
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
// use crate::java::util::Set::Set;
// use crate::javax::servlet::http::HttpServletRequest::HttpServletRequest;
use crate::org::allbinary::data::tree::dom::DomNodeHelper::DomNodeHelper;
use crate::org::allbinary::data::tree::dom::DomSearchHelper::DomSearchHelper;
use crate::org::allbinary::data::tree::dom::ModDomHelper::ModDomHelper;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct StreetAddress {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub id: String,
    pub name: String,
    pub street: String,
    pub city: String,
    pub state: String,
    pub code: String,
    pub country: String,
    pub isDefault: bool,
}


impl StreetAddress {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            id: String::new(),
            name: String::new(),
            street: String::new(),
            city: String::new(),
            state: String::new(),
            code: String::new(),
            country: String::new(),
            isDefault: false,
        };
        this.logUtil = LogUtil::getInstance();
        this.isDefault = false;
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn new_struct1(mut node: Option<Rc<RefCell<Node>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            id: String::new(),
            name: String::new(),
            street: String::new(),
            city: String::new(),
            state: String::new(),
            code: String::new(),
            country: String::new(),
            isDefault: false,
        };
        this.logUtil = LogUtil::getInstance();
        this.isDefault = false;
        let mut nameNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(StreetAddressData_NAME.with(Clone::clone).clone(), jcall!(node).getChildNodes());
        this.name = DomNodeHelper::getTextNodeValue(nameNode.clone());
        let mut streetNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(StreetAddressData_STREET.with(Clone::clone).clone(), jcall!(node).getChildNodes());
        this.street = DomNodeHelper::getTextNodeValue(streetNode.clone());
        let mut cityNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(StreetAddressData_CITY.with(Clone::clone).clone(), jcall!(node).getChildNodes());
        this.city = DomNodeHelper::getTextNodeValue(cityNode.clone());
        let mut stateNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(StreetAddressData_STATE.with(Clone::clone).clone(), jcall!(node).getChildNodes());
        this.state = DomNodeHelper::getTextNodeValue(stateNode.clone());
        let mut codeNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(StreetAddressData_CODE.with(Clone::clone).clone(), jcall!(node).getChildNodes());
        this.code = DomNodeHelper::getTextNodeValue(codeNode.clone());
        let mut countryNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(StreetAddressData_COUNTRY.with(Clone::clone).clone(), jcall!(node).getChildNodes());
        this.country = DomNodeHelper::getTextNodeValue(countryNode.clone());
        this.log();
        this
    }

    pub fn new1(mut node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(node))))
    }
    

    pub fn new_struct1(mut streetAddress: Option<Rc<RefCell<StreetAddress>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            id: String::new(),
            name: String::new(),
            street: String::new(),
            city: String::new(),
            state: String::new(),
            code: String::new(),
            country: String::new(),
            isDefault: false,
        };
        this.logUtil = LogUtil::getInstance();
        this.isDefault = false;
        this.id = jcall!(streetAddress).getId();
        this.name = jcall!(streetAddress).getName();
        this.street = jcall!(streetAddress).getStreet();
        this.city = jcall!(streetAddress).getCity();
        this.state = jcall!(streetAddress).getState();
        this.code = jcall!(streetAddress).getCode();
        this.country = jcall!(streetAddress).getCountry();
        this.isDefault = jcall!(streetAddress).isDefault();
        this.log();
        this
    }

    pub fn new1(mut streetAddress: Option<Rc<RefCell<StreetAddress>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(streetAddress))))
    }
    

    pub fn new_struct1(mut request: Option<Rc<RefCell<HttpServletRequest>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            id: String::new(),
            name: String::new(),
            street: String::new(),
            city: String::new(),
            state: String::new(),
            code: String::new(),
            country: String::new(),
            isDefault: false,
        };
        this.logUtil = LogUtil::getInstance();
        this.isDefault = false;
        { let java_arguments = (jcall!(request).getParameter(StreetAddressData_ID.with(Clone::clone).clone()),); this.setId(java_arguments.0) };
        { let java_arguments = (jcall!(request).getParameter(StreetAddressData_NAME.with(Clone::clone).clone()),); this.setName(java_arguments.0) };
        { let java_arguments = (jcall!(request).getParameter(StreetAddressData_STREET.with(Clone::clone).clone()),); this.setStreet(java_arguments.0) };
        { let java_arguments = (jcall!(request).getParameter(StreetAddressData_CITY.with(Clone::clone).clone()),); this.setCity(java_arguments.0) };
        { let java_arguments = (jcall!(request).getParameter(StreetAddressData_STATE.with(Clone::clone).clone()),); this.setState(java_arguments.0) };
        { let java_arguments = (jcall!(request).getParameter(StreetAddressData_CODE.with(Clone::clone).clone()),); this.setCode(java_arguments.0) };
        { let java_arguments = (jcall!(request).getParameter(StreetAddressData_COUNTRY.with(Clone::clone).clone()),); this.setCountry(java_arguments.0) };
        this.log();
        this
    }

    pub fn new1(mut request: Option<Rc<RefCell<HttpServletRequest>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(request))))
    }
    

    pub fn new_struct6(mut name: String, mut street: String, mut city: String, mut state: String, mut code: String, mut country: String) -> Self {
        let mut this = Self {
            logUtil: None,
            id: String::new(),
            name: String::new(),
            street: String::new(),
            city: String::new(),
            state: String::new(),
            code: String::new(),
            country: String::new(),
            isDefault: false,
        };
        this.logUtil = LogUtil::getInstance();
        this.isDefault = false;
        this.name = name.clone();
        this.street = street.clone();
        this.city = city.clone();
        this.state = state.clone();
        this.code = code.clone();
        this.country = country.clone();
        this.log();
        this
    }

    pub fn new6(mut name: String, mut street: String, mut city: String, mut state: String, mut code: String, mut country: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct6(name, street, city, state, code, country))))
    }
    

    pub fn new_struct1(mut address: Option<Rc<RefCell<HashMap>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            id: String::new(),
            name: String::new(),
            street: String::new(),
            city: String::new(),
            state: String::new(),
            code: String::new(),
            country: String::new(),
            isDefault: false,
        };
        this.logUtil = LogUtil::getInstance();
        this.isDefault = false;
        this.id = (jcall!(address).get(StreetAddressData_ID.with(Clone::clone).clone())).clone() /* as String */;
        this.name = (jcall!(address).get(StreetAddressData_NAME.with(Clone::clone).clone())).clone() /* as String */;
        this.street = (jcall!(address).get(StreetAddressData_STREET.with(Clone::clone).clone())).clone() /* as String */;
        this.city = (jcall!(address).get(StreetAddressData_CITY.with(Clone::clone).clone())).clone() /* as String */;
        this.state = (jcall!(address).get(StreetAddressData_STATE.with(Clone::clone).clone())).clone() /* as String */;
        this.code = (jcall!(address).get(StreetAddressData_CODE.with(Clone::clone).clone())).clone() /* as String */;
        this.country = (jcall!(address).get(StreetAddressData_COUNTRY.with(Clone::clone).clone())).clone() /* as String */;
        let mut def: String = (jcall!(address).get(StreetAddressData_DEFAULT.with(Clone::clone).clone())).clone() /* as String */;
        if true && (def.cmp(&StreetAddressData_DEFAULT.with(Clone::clone).clone()) as i32) == 0 {
            this.isDefault = true;
        } else {
            this.isDefault = false;
        }
        this.log();
        this
    }

    pub fn new1(mut address: Option<Rc<RefCell<HashMap>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(address))))
    }
    

    pub fn log(&mut self) {
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Created Address: \n"), self.toHashMap()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("log"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
    }
    

    pub fn setId(&mut self, mut id: String) {
        self.id = id.clone();
    }
    

    pub fn setName(&mut self, mut name: String) {
        self.name = name.clone();
    }
    

    pub fn setStreet(&mut self, mut value: String) {
        self.street = value.clone();
    }
    

    pub fn setCity(&mut self, mut value: String) {
        self.city = value.clone();
    }
    

    pub fn setState(&mut self, mut value: String) {
        self.state = value.clone();
    }
    

    pub fn setCode(&mut self, mut value: String) {
        self.code = value.clone();
    }
    

    pub fn setCountry(&mut self, mut value: String) {
        self.country = value.clone();
    }
    

    pub fn getName(&mut self) -> String {
        return self.name.clone();
    }
    

    pub fn getStreet(&mut self) -> String {
        return self.street.clone();
    }
    

    pub fn getCity(&mut self) -> String {
        return self.city.clone();
    }
    

    pub fn getState(&mut self) -> String {
        return self.state.clone();
    }
    

    pub fn getCode(&mut self) -> String {
        return self.code.clone();
    }
    

    pub fn getCountry(&mut self) -> String {
        return self.country.clone();
    }
    

    pub fn getId(&mut self) -> String {
        return self.id.clone();
    }
    

    pub fn isDefault(&mut self) -> bool {
        return self.isDefault.clone();
    }
    

    pub fn isEmpty(&mut self) -> Option<bool> {
        let mut EMPTY_STRING: String = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        if false && jcall!(self.getName()).compareTo(EMPTY_STRING.clone()) == 0 && self.getStreet().is_none() && jcall!(self.getStreet()).compareTo(EMPTY_STRING.clone()) == 0 && self.getCity().is_none() && jcall!(self.getCity()).compareTo(EMPTY_STRING.clone()) == 0 && self.getState().is_none() && jcall!(self.getState()).compareTo(EMPTY_STRING.clone()) == 0 && self.getCode().is_none() && jcall!(self.getCode()).compareTo(EMPTY_STRING.clone()) == 0 && self.getCountry().is_none() && jcall!(self.getCountry()).compareTo(EMPTY_STRING.clone()) == 0 {
            return true;
        }
        return false;
    }
    

    pub fn isValid(&mut self) -> Option<bool> {
        let mut EMPTY_STRING: String = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        if false || jcall!(self.getName()).compareTo(EMPTY_STRING.clone()) == 0 || self.getStreet().is_none() || jcall!(self.getStreet()).compareTo(EMPTY_STRING.clone()) == 0 || self.getCity().is_none() || jcall!(self.getCity()).compareTo(EMPTY_STRING.clone()) == 0 || self.getState().is_none() || jcall!(self.getState()).compareTo(EMPTY_STRING.clone()) == 0 || self.getCode().is_none() || jcall!(self.getCode()).compareTo(EMPTY_STRING.clone()) == 0 || self.getCountry().is_none() || jcall!(self.getCountry()).compareTo(EMPTY_STRING.clone()) == 0 {
            return false;
        }
        return true;
    }
    

    pub fn validationInfo(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(String::from("Address Failed To Validate - All fields must contain data.<br></br>"));
        let mut stringValidationUtil: Option<Rc<RefCell<StringValidationUtil>>> = StringValidationUtil::getInstance();
        if !jcall!(stringValidationUtil).isValidRequired(self.getName(), StreetAddressData_MIN.with(Clone::clone).clone(), StreetAddressData_MAX.with(Clone::clone).clone()) {
            jcall!(stringBuffer).append(String::from("Please enter a valid name"));
            jcall!(stringBuffer).append(String::from("<br></br>"));
        }
        if !jcall!(stringValidationUtil).isValidRequired(self.getStreet(), StreetAddressData_MIN.with(Clone::clone).clone(), StreetAddressData_MAX.with(Clone::clone).clone()) {
            jcall!(stringBuffer).append(String::from("Please enter a valid street"));
            jcall!(stringBuffer).append(String::from("<br></br>"));
        }
        if !jcall!(stringValidationUtil).isValidRequired(self.getCity(), StreetAddressData_MIN.with(Clone::clone).clone(), StreetAddressData_MAX.with(Clone::clone).clone()) {
            jcall!(stringBuffer).append(String::from("Please enter a valid city"));
            jcall!(stringBuffer).append(String::from("<br></br>"));
        }
        if !jcall!(stringValidationUtil).isValidRequired(self.getState(), StreetAddressData_MIN.with(Clone::clone).clone(), StreetAddressData_MAX.with(Clone::clone).clone()) {
            jcall!(stringBuffer).append(String::from("Please enter a valid state"));
            jcall!(stringBuffer).append(String::from("<br></br>"));
        }
        if !jcall!(stringValidationUtil).isValidRequired(self.getCode(), StreetAddressData_MIN.with(Clone::clone).clone(), StreetAddressData_MAX.with(Clone::clone).clone()) {
            jcall!(stringBuffer).append(String::from("Please enter a valid zipcode"));
            jcall!(stringBuffer).append(String::from("<br></br>"));
        }
        if !jcall!(stringValidationUtil).isValidRequired(self.getCountry(), StreetAddressData_MIN.with(Clone::clone).clone(), StreetAddressData_MAX.with(Clone::clone).clone()) {
            jcall!(stringBuffer).append(String::from("Please enter a valid country"));
            jcall!(stringBuffer).append(String::from("<br></br>"));
        }
        return jcall!(stringBuffer).toString();
    }
    

    pub fn toValidationInfoDoc(&mut self) -> Option<Rc<RefCell<Document>>> {
        return None;
    }
    

    pub fn toValidationInfoNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut node: Option<Rc<RefCell<Node>>> = jcall!(document).createElement(StreetAddressData_FORM.with(Clone::clone).clone());
            jcall!(node).appendChild(self.toXmlNode(document.clone()));
            return node.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGINGERROR.clone()) {
                let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
                jcall!(self.logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("toValidationInfoNode"), crate::java_into_object(e.clone()));
            }
            return None;
            }
        }
    }
    

    pub fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> {
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        jcall!(hashMap).put(StreetAddressData_ID.with(Clone::clone).clone(), self.id.clone());
        jcall!(hashMap).put(StreetAddressData_NAME.with(Clone::clone).clone(), self.name.clone());
        jcall!(hashMap).put(StreetAddressData_STREET.with(Clone::clone).clone(), self.street.clone());
        jcall!(hashMap).put(StreetAddressData_CITY.with(Clone::clone).clone(), self.city.clone());
        jcall!(hashMap).put(StreetAddressData_STATE.with(Clone::clone).clone(), self.state.clone());
        jcall!(hashMap).put(StreetAddressData_CODE.with(Clone::clone).clone(), self.code.clone());
        jcall!(hashMap).put(StreetAddressData_COUNTRY.with(Clone::clone).clone(), self.country.clone());
        return hashMap.clone();
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        let mut stringUtil: Option<Rc<RefCell<StringUtil>>> = StringUtil::getInstance();
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = self.toHashMap();
        let mut keySet: Option<Rc<RefCell<dyn Set>>> = jcall!(hashMap).keySet();
        let mut keyArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(keySet).toArray();
        let mut size: i32 = (keyArray.len() as i32).clone();
        let mut node: Option<Rc<RefCell<Node>>> = jcall!(document).createElement(StreetAddressData_ADDRESS.with(Clone::clone).clone());
        for i in (0..size) {
            let mut name: String = (keyArray[(i) as usize]).clone() /* as String */;
            let mut value: String = jcall!(stringUtil).getNonNull((jcall!(hashMap).get(name.clone())).clone() /* as String */);
            jcall!(node).appendChild(ModDomHelper::createNameValueNodes(document.clone(), name.clone(), value.clone()));
        }
        return node.clone();
    }
}

