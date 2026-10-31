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
// package org.allbinary.input.automation.module.configuration
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::io::File::File;
// use crate::java::io::FileInputStream::FileInputStream;
// use crate::java::util::HashMap::HashMap;
// use crate::java::util::List::List;
use crate::org::allbinary::data::tree::dom::document::DomDocumentHelper::DomDocumentHelper;
use crate::org::allbinary::input::automation::module::InputAutomationModuleData::InputAutomationModuleData;
use crate::org::allbinary::input::automation::module::NewInputAutomationModulesData::NewInputAutomationModulesData;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use crate::org::w3c::dom::NodeList::NodeList;
use super::InputAutomationModuleConfiguration::InputAutomationModuleConfiguration;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InputAutomationModuleConfigurations {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub hashMap: Option<Rc<RefCell<HashMap>>>,
}


impl InputAutomationModuleConfigurations {
    

    pub fn new_struct2(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut file: Option<Rc<RefCell<File>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            hashMap: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.setHashMap(HashMap::new());
        let mut bytes: Vec<i8> = vec![0; (100000) as usize];
        let mut idFile: Option<Rc<RefCell<FileInputStream>>> = FileInputStream::new(file.clone());
        let mut length: i32 = jcall!(idFile).read(bytes.clone());
        let mut data: String = String::from_utf8(bytes.clone().iter().map(|value| *value as u8).collect()).unwrap();
        let mut endIndex: i32 = data.rfind('>').map(|index| index as i32).unwrap_or(-1);
        let mut document: Option<Rc<RefCell<Document>>> = DomDocumentHelper::create1(data[(0) as usize..(endIndex + 1) as usize].to_string());
        let mut nodeList: Option<Rc<RefCell<NodeList>>> = jcall!(document).getElementsByTagName(unsafe { crate::org::allbinary::input::automation::module::NewInputAutomationModulesData::NewInputAutomationModulesData_NAME.clone() }.clone());
        if jcall!(nodeList).getLength() > 0 {
            let mut nameNodeList: Option<Rc<RefCell<NodeList>>> = jcall!(document).getElementsByTagName(unsafe { crate::org::allbinary::input::automation::module::InputAutomationModuleData::InputAutomationModuleData_NAME.clone() }.clone());
            { let java_receiver = this.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Number Of Module(s) Specified: "), jcall!(nameNodeList).getLength()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("Contructor"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            for index in (0..jcall!(nameNodeList).getLength()) {
                let mut node: Option<Rc<RefCell<Node>>> = jcall!(nameNodeList).item(index.clone());
                this.add(InputAutomationModuleConfiguration::new2(abeClientInformation.clone(), node.clone()));
            }
        }
        this
    }

    pub fn new2(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut file: Option<Rc<RefCell<File>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2(abeClientInformation, file))))
    }
    

    pub fn new_struct2(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut document: Option<Rc<RefCell<Document>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            hashMap: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.setHashMap(HashMap::new());
        let mut nameNodeList: Option<Rc<RefCell<NodeList>>> = jcall!(document).getElementsByTagName(unsafe { crate::org::allbinary::input::automation::module::InputAutomationModuleData::InputAutomationModuleData_NAME.clone() }.clone());
        { let java_receiver = this.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Number Of Module(s) Specified: "), jcall!(nameNodeList).getLength()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("Contructor"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        for index in (0..jcall!(nameNodeList).getLength()) {
            let mut node: Option<Rc<RefCell<Node>>> = jcall!(nameNodeList).item(index.clone());
            this.add(InputAutomationModuleConfiguration::new2(abeClientInformation.clone(), node.clone()));
        }
        this
    }

    pub fn new2(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2(abeClientInformation, document))))
    }
    

    pub fn new_struct1(mut inputAutomationModuleConfigurationList: Option<Rc<RefCell<dyn List<Option<Rc<RefCell<InputAutomationModuleConfiguration>>>>>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            hashMap: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.setHashMap(HashMap::new());
        let mut size: i32 = jcall!(inputAutomationModuleConfigurationList).size();
        let mut inputAutomationModuleConfiguration: Option<Rc<RefCell<InputAutomationModuleConfiguration>>> = None;
        for index in (0..size) {
            inputAutomationModuleConfiguration = jcall!(inputAutomationModuleConfigurationList).get(index.clone());
            this.add(inputAutomationModuleConfiguration.clone());
        }
        this
    }

    pub fn new1(mut inputAutomationModuleConfigurationList: Option<Rc<RefCell<dyn List<Option<Rc<RefCell<InputAutomationModuleConfiguration>>>>>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(inputAutomationModuleConfigurationList))))
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            hashMap: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.setHashMap(HashMap::new());
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn add(&mut self, mut inputAutomationModuleConfiguration: Option<Rc<RefCell<InputAutomationModuleConfiguration>>>) {
        jcall!(self.getHashMap()).put(jcall!(inputAutomationModuleConfiguration).getClassName(), inputAutomationModuleConfiguration.clone());
    }
    

    pub fn remove(&mut self, mut inputAutomationModuleConfiguration: Option<Rc<RefCell<InputAutomationModuleConfiguration>>>) {
        jcall!(self.getHashMap()).remove(jcall!(inputAutomationModuleConfiguration).getClassName());
    }
    

    pub fn getHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> {
        return self.hashMap.clone();
    }
    

    pub fn setHashMap(&mut self, mut hashMap: Option<Rc<RefCell<HashMap>>>) {
        self.hashMap = hashMap.clone();
    }
}

