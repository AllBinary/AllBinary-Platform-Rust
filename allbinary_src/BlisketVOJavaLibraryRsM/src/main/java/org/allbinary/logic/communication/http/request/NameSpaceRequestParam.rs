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
// package org.allbinary.logic.communication.http.request
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::string::tokens::Tokenizer::Tokenizer;
use crate::org::allbinary::logic::visual::dhtml::html::name::HtmlNameMathData::HtmlNameMathData;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NameSpaceRequestParam {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub nameSpaceVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub nameSpacePropertiesHashMap: Option<Rc<RefCell<HashMap>>>,
    pub value: String,
}


impl NameSpaceRequestParam {
    

    pub fn new_struct(mut nameSpace: String, mut value: String) -> Self {
        let mut this = Self {
            logUtil: None,
            nameSpaceVector: None,
            nameSpacePropertiesHashMap: None,
            value: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.nameSpaceVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.nameSpacePropertiesHashMap = jcall!(StdUtil::getInstance()).createHashMap();
        this.value = value.clone();
        let mut beginIndex: i32 = nameSpace.find(&NameSpaceRequestParamData_NAME.with(Clone::clone).clone()).map(|index| index as i32).unwrap_or(-1);
        if beginIndex < 0 {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Not a NameSpaceRequest"))));
        }
        let mut packages: String = nameSpace[(beginIndex + (NameSpaceRequestParamData_NAME.with(Clone::clone).chars().count() as i32) + (NameSpaceRequestParamData_SEP.with(Clone::clone).chars().count() as i32)) as usize..].to_string();
        let mut sepTokenizer: Option<Rc<RefCell<Tokenizer>>> = Tokenizer::new(NameSpaceRequestParamData_SEP.with(Clone::clone).clone());
        let mut nameSpaceWithPropertiesVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(sepTokenizer).getTokensFromString(packages.clone(), BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayListD::BasicArrayListDDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>));
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).HTTPREQUEST.clone()) {
            { let java_receiver = this.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("\nSep Tokens: "), jcall!(nameSpaceWithPropertiesVector).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("NameSpaceRequestParam()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        let mut nameSpaceAndPropertiesTokenizer: Option<Rc<RefCell<Tokenizer>>> = Tokenizer::new(NameSpaceRequestParamData_PROPERTIES.with(Clone::clone).clone());
        let mut packageIndex: i32 = 0;
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut index: i32 = 0;
        let mut size: i32 = jcall!(nameSpaceWithPropertiesVector).size();
        while index < size {
            let mut nameSpaceWithProperties: String = (jcall!(nameSpaceWithPropertiesVector).get({ let value = index; index += 1; value })).clone() /* as String */;
            jcall!(list).clear();
            let mut nameSpaceAndPropertiesVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(nameSpaceAndPropertiesTokenizer).getTokensFromString(nameSpaceWithProperties.clone(), list.clone());
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).HTTPREQUEST.clone()) {
                { let java_receiver = this.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("\nPackageName and PackageName Properties Tokens: "), jcall!(nameSpaceAndPropertiesVector).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("NameSpaceRequestParam()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            if jcall!(nameSpaceAndPropertiesVector).size() > 0 {
                let mut packageName: String = (jcall!(nameSpaceAndPropertiesVector).get(0)).clone() /* as String */;
                if false {
                    panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Package Name Is Null"))));
                }
                jcall!(this.nameSpaceVector).add(crate::java_into_object(packageName.clone()));
                if jcall!(nameSpaceAndPropertiesVector).size() > 1 {
                    let mut properties: String = (jcall!(nameSpaceAndPropertiesVector).get(1)).clone() /* as String */;
                    let mut packagePropertiesHashMap: Option<Rc<RefCell<HashMap>>> = this.generatePackagePropertiesHashMap(properties.clone());
                    { let java_receiver = this.nameSpacePropertiesHashMap.clone(); let java_arguments = ((packageIndex.clone()).to_string(), packagePropertiesHashMap.clone(),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1); };
                }
                packageIndex += 1;
            }
        }
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).HTTPREQUEST.clone()) {
            { let java_receiver = this.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("NameSpace: "), jcall!(this.nameSpaceVector).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("NameSpaceRequestParam()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        this
    }

    pub fn new(mut nameSpace: String, mut value: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(nameSpace, value))))
    }
    

    pub fn getPackages(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).HTTPREQUEST.clone()) {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("NameSpace: "), jcall!(self.nameSpaceVector).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getPackages()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        return self.nameSpaceVector.clone();
    }
    

    pub fn getPackageProperties(&mut self, mut packageIndex: i32) -> Option<Rc<RefCell<HashMap>>> {
        let mut packagePropertiesHashMap: Option<Rc<RefCell<HashMap>>> = ({ let java_receiver = self.nameSpacePropertiesHashMap.clone(); let java_arguments = ((packageIndex.clone()).to_string(),); let java_result = jcall!(java_receiver).get(java_arguments.0); java_result }).clone() /* as HashMap */;
        if packagePropertiesHashMap.is_none() {
            packagePropertiesHashMap = jcall!(StdUtil::getInstance()).createHashMap();
        }
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).HTTPREQUEST.clone()) {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Package Properties: "), jcall!(packagePropertiesHashMap).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getPackagesProperties()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        return packagePropertiesHashMap.clone();
    }
    

    pub fn getValue(&mut self) -> String {
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).HTTPREQUEST.clone()) {
            jcall!(self.logUtil).putF(format!("{}{}", String::from("\nValue: "), self.value), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getValue()"));
        }
        return self.value.clone();
    }
    

    // throws Exception
    pub fn generatePackagePropertiesHashMap(&mut self, mut properties: String) -> Option<Rc<RefCell<HashMap>>> {
        let mut propertiesTokenizer: Option<Rc<RefCell<Tokenizer>>> = Tokenizer::new(NameSpaceRequestParamData_PROPERTIESSEPARATOR.with(Clone::clone).clone());
        let mut packagePropertiesHashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        let mut propertyVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(propertiesTokenizer).getTokensFromString(properties.clone(), BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayListD::BasicArrayListDDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>));
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut propertyTokenizer: Option<Rc<RefCell<Tokenizer>>> = Tokenizer::new(jcall!(HtmlNameMathData::getInstance()).EQUALS.clone());
        let mut index: i32 = 0;
        let mut size: i32 = jcall!(propertyVector).size();
        while index < size {
            let mut property: String = (jcall!(propertyVector).get(index.clone())).clone() /* as String */;
            jcall!(list).clear();
            let mut propertyNameValueVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(propertyTokenizer).getTokensFromString(property.clone(), list.clone());
            let mut index2: i32 = 0;
            while index2 < jcall!(propertyNameValueVector).size() {
                let mut propertyName: String = (jcall!(propertyNameValueVector).get({ let value = index2; index2 += 1; value })).clone() /* as String */;
                if jcall!(propertyNameValueVector).size() > 1 {
                    let mut propertyValue: String = (jcall!(propertyNameValueVector).get({ let value = index2; index2 += 1; value })).clone() /* as String */;
                    if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).HTTPREQUEST.clone()) {
                        jcall!(self.logUtil).putF(format!("{}{}", format!("{}{}", format!("{}{}", String::from("\nProperty Name: "), propertyName), String::from("\nProperty Value: ")), propertyValue), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("generatePackagePropertiesHashMap()"));
                    }
                    jcall!(packagePropertiesHashMap).put(propertyName.clone(), propertyValue.clone());
                } else {
                    jcall!(packagePropertiesHashMap).put(propertyName.clone(), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone());
                }
            }
        }
        return packagePropertiesHashMap.clone();
    }
}

