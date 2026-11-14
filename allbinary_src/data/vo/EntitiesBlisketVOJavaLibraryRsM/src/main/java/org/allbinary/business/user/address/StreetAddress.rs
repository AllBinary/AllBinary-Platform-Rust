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
use super::*;
use crate::java::util::HashMap::HashMap;
use crate::java::util::Set::Set;
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
use crate::java::lang::Exception::*;

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

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

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
        this.name = { let java_value = (DomNodeHelper::getTextNodeValue(nameNode.clone())).clone(); java_value };
        let mut streetNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(StreetAddressData_STREET.with(Clone::clone).clone(), jcall!(node).getChildNodes());
        this.street = { let java_value = (DomNodeHelper::getTextNodeValue(streetNode.clone())).clone(); java_value };
        let mut cityNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(StreetAddressData_CITY.with(Clone::clone).clone(), jcall!(node).getChildNodes());
        this.city = { let java_value = (DomNodeHelper::getTextNodeValue(cityNode.clone())).clone(); java_value };
        let mut stateNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(StreetAddressData_STATE.with(Clone::clone).clone(), jcall!(node).getChildNodes());
        this.state = { let java_value = (DomNodeHelper::getTextNodeValue(stateNode.clone())).clone(); java_value };
        let mut codeNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(StreetAddressData_CODE.with(Clone::clone).clone(), jcall!(node).getChildNodes());
        this.code = { let java_value = (DomNodeHelper::getTextNodeValue(codeNode.clone())).clone(); java_value };
        let mut countryNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(StreetAddressData_COUNTRY.with(Clone::clone).clone(), jcall!(node).getChildNodes());
        this.country = { let java_value = (DomNodeHelper::getTextNodeValue(countryNode.clone())).clone(); java_value };
        this.log();
        this
    }

    pub fn new1(mut node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(node))))
    }
    

    pub fn new_struct1_StreetAddress(mut streetAddress: Option<Rc<RefCell<StreetAddress>>>) -> Self {
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
        this.id = { let java_value = (jcall!(streetAddress).getId()).clone(); java_value };
        this.name = { let java_value = (jcall!(streetAddress).getName()).clone(); java_value };
        this.street = { let java_value = (jcall!(streetAddress).getStreet()).clone(); java_value };
        this.city = { let java_value = (jcall!(streetAddress).getCity()).clone(); java_value };
        this.state = { let java_value = (jcall!(streetAddress).getState()).clone(); java_value };
        this.code = { let java_value = (jcall!(streetAddress).getCode()).clone(); java_value };
        this.country = { let java_value = (jcall!(streetAddress).getCountry()).clone(); java_value };
        this.isDefault = { let java_value = (jcall!(streetAddress).isDefault()).clone(); java_value };
        this.log();
        this
    }

    pub fn new1_StreetAddress(mut streetAddress: Option<Rc<RefCell<StreetAddress>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1_StreetAddress(streetAddress))))
    }
    

    pub fn new_struct1_HttpServletRequest(mut request: Option<Rc<RefCell<HttpServletRequest>>>) -> Self {
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

    pub fn new1_HttpServletRequest(mut request: Option<Rc<RefCell<HttpServletRequest>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1_HttpServletRequest(request))))
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
    

    pub fn new_struct1_HashMap(mut address: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) -> Self {
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
        this.id = { let java_value = ({ let java_object = (jcall!(address).get(crate::java_into_object(StreetAddressData_ID.with(Clone::clone).clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        this.name = { let java_value = ({ let java_object = (jcall!(address).get(crate::java_into_object(StreetAddressData_NAME.with(Clone::clone).clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        this.street = { let java_value = ({ let java_object = (jcall!(address).get(crate::java_into_object(StreetAddressData_STREET.with(Clone::clone).clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        this.city = { let java_value = ({ let java_object = (jcall!(address).get(crate::java_into_object(StreetAddressData_CITY.with(Clone::clone).clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        this.state = { let java_value = ({ let java_object = (jcall!(address).get(crate::java_into_object(StreetAddressData_STATE.with(Clone::clone).clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        this.code = { let java_value = ({ let java_object = (jcall!(address).get(crate::java_into_object(StreetAddressData_CODE.with(Clone::clone).clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        this.country = { let java_value = ({ let java_object = (jcall!(address).get(crate::java_into_object(StreetAddressData_COUNTRY.with(Clone::clone).clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        let mut def: String = { let java_object = (jcall!(address).get(crate::java_into_object(StreetAddressData_DEFAULT.with(Clone::clone).clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
        if true && (def.cmp(&StreetAddressData_DEFAULT.with(Clone::clone).clone()) as i32) == 0 {
            this.isDefault = true;
        } else {
            this.isDefault = false;
        }
        this.log();
        this
    }

    pub fn new1_HashMap(mut address: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1_HashMap(address))))
    }
    

    pub fn log(&mut self) {
        if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGING.clone(); java_field }.clone()) {
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
        let mut EMPTY_STRING: String = { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone();
        if false && { let java_value = (self.getName().cmp(&EMPTY_STRING.clone()) as i32); java_value } == 0 && false && { let java_value = (self.getStreet().cmp(&EMPTY_STRING.clone()) as i32); java_value } == 0 && false && { let java_value = (self.getCity().cmp(&EMPTY_STRING.clone()) as i32); java_value } == 0 && false && { let java_value = (self.getState().cmp(&EMPTY_STRING.clone()) as i32); java_value } == 0 && false && { let java_value = (self.getCode().cmp(&EMPTY_STRING.clone()) as i32); java_value } == 0 && false && { let java_value = (self.getCountry().cmp(&EMPTY_STRING.clone()) as i32); java_value } == 0 {
            return true;
        }
        return false;
    }
    

    pub fn isValid(&mut self) -> Option<bool> {
        let mut EMPTY_STRING: String = { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone();
        if false || { let java_value = (self.getName().cmp(&EMPTY_STRING.clone()) as i32); java_value } == 0 || false || { let java_value = (self.getStreet().cmp(&EMPTY_STRING.clone()) as i32); java_value } == 0 || false || { let java_value = (self.getCity().cmp(&EMPTY_STRING.clone()) as i32); java_value } == 0 || false || { let java_value = (self.getState().cmp(&EMPTY_STRING.clone()) as i32); java_value } == 0 || false || { let java_value = (self.getCode().cmp(&EMPTY_STRING.clone()) as i32); java_value } == 0 || false || { let java_value = (self.getCountry().cmp(&EMPTY_STRING.clone()) as i32); java_value } == 0 {
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
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut node: Option<Rc<RefCell<Node>>> = jcall!(document).createElement(StreetAddressData_FORM.with(Clone::clone).clone());
            jcall!(node).appendChild(self.toXmlNode(document.clone()));
            return Some(node.clone());
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGINGERROR.clone(); java_field }.clone()) {
                let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
                jcall!(self.logUtil).put({ let java_field = jcall!(commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("toValidationInfoNode"), crate::java_into_object(e.clone()));
            }
            return None;
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    pub fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> {
        let mut hashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> = jcall!(StdUtil::getInstance()).createHashMap();
        jcall!(hashMap).put(crate::java_into_object(StreetAddressData_ID.with(Clone::clone).clone()), crate::java_into_object(self.id.clone()));
        jcall!(hashMap).put(crate::java_into_object(StreetAddressData_NAME.with(Clone::clone).clone()), crate::java_into_object(self.name.clone()));
        jcall!(hashMap).put(crate::java_into_object(StreetAddressData_STREET.with(Clone::clone).clone()), crate::java_into_object(self.street.clone()));
        jcall!(hashMap).put(crate::java_into_object(StreetAddressData_CITY.with(Clone::clone).clone()), crate::java_into_object(self.city.clone()));
        jcall!(hashMap).put(crate::java_into_object(StreetAddressData_STATE.with(Clone::clone).clone()), crate::java_into_object(self.state.clone()));
        jcall!(hashMap).put(crate::java_into_object(StreetAddressData_CODE.with(Clone::clone).clone()), crate::java_into_object(self.code.clone()));
        jcall!(hashMap).put(crate::java_into_object(StreetAddressData_COUNTRY.with(Clone::clone).clone()), crate::java_into_object(self.country.clone()));
        return hashMap.clone();
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        let mut stringUtil: Option<Rc<RefCell<StringUtil>>> = StringUtil::getInstance();
        let mut hashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> = self.toHashMap();
        let mut keySet: Option<Rc<RefCell<dyn Set>>> = jcall!(hashMap).keySet();
        let mut keyArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(keySet).toArray();
        let mut size: i32 = (keyArray.len() as i32).clone();
        let mut node: Option<Rc<RefCell<Node>>> = jcall!(document).createElement(StreetAddressData_ADDRESS.with(Clone::clone).clone());
        for i in (0..size) {
            let mut name: String = { let java_object = (keyArray[(i) as usize]).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
            let mut value: String = jcall!(stringUtil).getNonNull({ let java_object = (jcall!(hashMap).get(crate::java_into_object(name.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() });
            jcall!(node).appendChild(ModDomHelper::createNameValueNodes(document.clone(), name.clone(), value.clone()));
        }
        return node.clone();
    }
}

