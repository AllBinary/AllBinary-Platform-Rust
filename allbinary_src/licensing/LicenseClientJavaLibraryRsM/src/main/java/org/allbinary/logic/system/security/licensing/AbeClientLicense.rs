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
// package org.allbinary.logic.system.security.licensing
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use super::AbeLicenseInterface::AbeLicenseInterface;
use super::LicenseType::LicenseType;
use super::AbeClientInformationData::AbeClientInformationData;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AbeClientLicense {
    pub hashtable: Option<Rc<RefCell<ABHashtable<crate::JavaObject, crate::JavaObject>>>>,
    pub id: String,
    pub servers: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub special: String,
    pub licenseType: Option<Rc<RefCell<LicenseType>>>,
}

impl AbeLicenseInterface for AbeClientLicense {
    fn hasKey(&mut self) -> bool { Self::hasKey(self) }
    fn getKey(&mut self, mut keyName: String) -> String { Self::getKey(self, keyName) }
    fn getLicenseId(&mut self) -> String { Self::getLicenseId(self) }
    fn getServers(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Self::getServers(self) }
    fn getSpecial(&mut self) -> String { Self::getSpecial(self) }
    fn getLicenseType(&mut self) -> Option<Rc<RefCell<LicenseType>>> { Self::getLicenseType(self) }
    fn isValid(&mut self) -> bool { Self::isValid(self) }
    fn toString(&mut self) -> String { Self::toString(self) }
}

impl AbeClientLicense {
    

    pub fn new_struct(mut hashtable: Option<Rc<RefCell<ABHashtable<crate::JavaObject, crate::JavaObject>>>>) -> Self {
        let mut this = Self {
            hashtable: None,
            id: String::new(),
            servers: None,
            special: String::new(),
            licenseType: None,
        };
        this.hashtable = { let java_value = (jcall!(StdUtil::getInstance()).createHashtable()).clone(); java_value };
        let mut abeClientInformationData: Option<Rc<RefCell<AbeClientInformationData>>> = AbeClientInformationData::getInstance();
        let mut keyValue: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(hashtable).get(crate::java_into_object({ let java_field = jcall!(abeClientInformationData).KEY.clone(); java_field }.clone()));
        if keyValue.is_some() {
            if (keyValue).is_some() /* instanceof String */ {
                jcall!(this.hashtable).put(crate::java_into_object({ let java_field = jcall!(abeClientInformationData).KEY.clone(); java_field }.clone()), keyValue.clone());
            }
        }
        this.id = { let java_value = ({ let java_object = (jcall!(hashtable).get(crate::java_into_object({ let java_field = jcall!(abeClientInformationData).LICENSEID.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        let mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = { let java_object = (jcall!(hashtable).get(crate::java_into_object({ let java_field = jcall!(abeClientInformationData).LICENSESERVERS.clone(); java_field }.clone()))).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>>().expect("ClassCastException").clone()) };
        this.servers = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut size: i32 = jcall!(vector).size();
        for index in (0..size) {
            { let java_receiver = this.servers.clone(); let java_arguments = (crate::java_into_object(jcall!(vector).get(index.clone())),); jcall!(java_receiver).add(java_arguments.0); };
        }
        let mut stringUtil: Option<Rc<RefCell<StringUtil>>> = StringUtil::getInstance();
        { let java_arguments = (jcall!(stringUtil).getNonNull({ let java_object = (jcall!(hashtable).get(crate::java_into_object({ let java_field = jcall!(abeClientInformationData).SPECIAL.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }),); this.setSpecial(java_arguments.0) };
        let mut licenseTypeString: String = jcall!(stringUtil).getNonNull({ let java_object = (jcall!(hashtable).get(crate::java_into_object({ let java_field = jcall!(abeClientInformationData).LICENSE_TYPE.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() });
        this.licenseType = { let java_value = (jcall!(LicenseTypeFactory::LicenseTypeFactory::getInstance()).getInstance1(licenseTypeString.clone())).clone(); java_value };
        this
    }

    pub fn new(mut hashtable: Option<Rc<RefCell<ABHashtable<crate::JavaObject, crate::JavaObject>>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(hashtable))))
    }
    

    pub fn hasRequiredKeys(mut resultHashtable: Option<Rc<RefCell<ABHashtable<crate::JavaObject, crate::JavaObject>>>>) -> bool {
        let mut abeClientInformationData: Option<Rc<RefCell<AbeClientInformationData>>> = AbeClientInformationData::getInstance();
        if jcall!(resultHashtable).containsKey(crate::java_into_object({ let java_field = jcall!(abeClientInformationData).LICENSEID.clone(); java_field }.clone())) && jcall!(resultHashtable).containsKey(crate::java_into_object({ let java_field = jcall!(abeClientInformationData).LICENSESERVERS.clone(); java_field }.clone())) && jcall!(resultHashtable).containsKey(crate::java_into_object({ let java_field = jcall!(abeClientInformationData).KEY.clone(); java_field }.clone())) && jcall!(resultHashtable).containsKey(crate::java_into_object({ let java_field = jcall!(abeClientInformationData).LICENSE_TYPE.clone(); java_field }.clone())) {
            return true;
        } else {
            return false;
        }
    }
    

    pub fn hasKey(&mut self) -> bool {
        return self.isValid();
    }
    

    pub fn getKey(&mut self, mut keyName: String) -> String {
        return { let java_object = (jcall!(self.hashtable).get(crate::java_into_object(keyName.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
    }
    

    pub fn getLicenseId(&mut self) -> String {
        return self.id.clone();
    }
    

    pub fn getServers(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.servers.clone();
    }
    

    pub fn isValid(&mut self) -> bool {
        let mut stringValidationUtil: Option<Rc<RefCell<StringValidationUtil>>> = StringValidationUtil::getInstance();
        if !jcall!(stringValidationUtil).isEmpty({ let java_arguments = ({ let java_field = jcall!(AbeClientInformationData::getInstance()).KEY.clone(); java_field }.clone(),); self.getKey(java_arguments.0) }) {
            return true;
        }
        return false;
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut BREAK: String = String::from("<br/>");
        jcall!(stringBuffer).append(String::from("License Id: "));
        jcall!(stringBuffer).append(self.getLicenseId());
        jcall!(stringBuffer).append(BREAK.clone());
        jcall!(stringBuffer).append(String::from("Is Valid: "));
        jcall!(stringBuffer).appendboolean(self.isValid());
        jcall!(stringBuffer).append(BREAK.clone());
        jcall!(stringBuffer).append(String::from("Keys: "));
        jcall!(stringBuffer).append(jcall!(self.hashtable).toString());
        jcall!(stringBuffer).append(BREAK.clone());
        let mut serverVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.getServers();
        let mut size: i32 = jcall!(serverVector).size();
        for index in (0..size) {
            let mut nextServerString: String = { let java_object = (jcall!(serverVector).get(index.clone())).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
            jcall!(stringBuffer).append(String::from("Server: "));
            jcall!(stringBuffer).append(nextServerString.clone());
            jcall!(stringBuffer).append(BREAK.clone());
        }
        return jcall!(stringBuffer).toString();
    }
    

    pub fn setSpecial(&mut self, mut special: String) {
        self.special = special.clone();
    }
    

    pub fn getSpecial(&mut self) -> String {
        return self.special.clone();
    }
    

    pub fn getLicenseType(&mut self) -> Option<Rc<RefCell<LicenseType>>> {
        return self.licenseType.clone();
    }
}

