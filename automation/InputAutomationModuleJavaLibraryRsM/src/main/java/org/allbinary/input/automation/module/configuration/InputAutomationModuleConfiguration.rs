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
use super::*;
// use crate::javax::xml::bind::annotation::XmlAccessType::XmlAccessType;
// use crate::javax::xml::bind::annotation::XmlAccessorType::XmlAccessorType;
// use crate::javax::xml::bind::annotation::XmlElement::XmlElement;
// use crate::javax::xml::bind::annotation::XmlRootElement::XmlRootElement;
// use crate::javax::xml::bind::annotation::XmlType::XmlType;
use crate::org::allbinary::business::DynamicObjectData::DynamicObjectData;
use crate::org::allbinary::data::tree::dom::DomNodeHelper::DomNodeHelper;
use crate::org::allbinary::data::tree::dom::DomSearchHelper::DomSearchHelper;
use crate::org::allbinary::data::tree::dom::ModDomHelper::ModDomHelper;
use crate::org::allbinary::input::automation::InputAutomationData::InputAutomationData;
use crate::org::allbinary::input::automation::module::InputAutomationModuleData::InputAutomationModuleData;
use crate::org::allbinary::input::automation::module::InputAutomationModuleFactoryInterface::InputAutomationModuleFactoryInterface;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::system::loader::AbeFactory::AbeFactory;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use crate::org::w3c::dom::NodeList::NodeList;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InputAutomationModuleConfiguration {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub className: String,
    pub name: String,
    pub inputAutomationModuleInterface: Option<Rc<RefCell<dyn InputAutomationModuleFactoryInterface>>>,
}


impl InputAutomationModuleConfiguration {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            className: String::new(),
            name: String::new(),
            inputAutomationModuleInterface: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn new_struct2(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut node: Option<Rc<RefCell<Node>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            className: String::new(),
            name: String::new(),
            inputAutomationModuleInterface: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.init2(abeClientInformation.clone(), node.clone());
        this
    }

    pub fn new2(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2(abeClientInformation, node))))
    }
    

    pub fn new_struct3(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut name: String, mut className: String) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            className: String::new(),
            name: String::new(),
            inputAutomationModuleInterface: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.setName(name.clone());
        this.setClassName(className.clone());
        this.init(abeClientInformation.clone());
        this
    }

    pub fn new3(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut name: String, mut className: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct3(abeClientInformation, name, className))))
    }
    

    pub fn new_struct1(mut inputAutomationModuleInterface: Option<Rc<RefCell<dyn InputAutomationModuleFactoryInterface>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            className: String::new(),
            name: String::new(),
            inputAutomationModuleInterface: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.setInputAutomationModuleInterface(inputAutomationModuleInterface.clone());
        { let java_arguments = (std::any::type_name_of_val(&this.inputAutomationModuleInterface).to_string(),); this.setClassName(java_arguments.0) };
        this
    }

    pub fn new1(mut inputAutomationModuleInterface: Option<Rc<RefCell<dyn InputAutomationModuleFactoryInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(inputAutomationModuleInterface))))
    }
    

    // throws Exception
    pub fn init2(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut node: Option<Rc<RefCell<Node>>>) {
        let mut nodeList: Option<Rc<RefCell<NodeList>>> = jcall!(node).getChildNodes();
        if nodeList.is_some() {
            let mut classNameNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(crate::org::allbinary::business::DynamicObjectData::DynamicObjectData_NAME.with(Clone::clone).clone(), nodeList.clone());
            if classNameNode.is_some() {
                { let java_arguments = (DomNodeHelper::getTextNodeValue(classNameNode.clone()),); self.setClassName(java_arguments.0) };
                self.init(abeClientInformation.clone());
            } else {
                jcall!(self.logUtil).putF(String::from("Class Node Null"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).INIT.clone());
            }
        } else {
            jcall!(self.logUtil).putF(format!("{}{}", unsafe { crate::org::allbinary::input::automation::InputAutomationData::InputAutomationData_NAME.clone() }, String::from(" Node Has No Children")), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("Contructor"));
        }
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn init(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Name: "), self.getName()), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).INIT.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            jcall!(self.logUtil).putF(format!("{}{}", String::from("ClassName: "), self.className), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).INIT.clone());
            { let java_arguments = ((jcall!(jcall!(AbeFactory::getInstance()).getInstance()).getInstance(abeClientInformation.clone(), self.getClassName())).clone() /* as InputAutomationModuleFactoryInterface */,); self.setInputAutomationModuleInterface(java_arguments.0) };
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).INIT.clone(), crate::java_into_object(e.clone()));
            panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
            }
        }
    }
    

    // throws Exception
    pub fn toDomNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        let mut node: Option<Rc<RefCell<Node>>> = jcall!(document).createElement(unsafe { crate::org::allbinary::input::automation::module::InputAutomationModuleData::InputAutomationModuleData_NAME.clone() }.clone());
        let mut classNameNode: Option<Rc<RefCell<Node>>> = ModDomHelper::createTextNode(document.clone(), crate::org::allbinary::business::DynamicObjectData::DynamicObjectData_NAME.with(Clone::clone).clone(), self.getClassName());
        jcall!(node).appendChild(classNameNode.clone());
        return node.clone();
    }
    

    pub fn getName(&mut self) -> String {
        return self.name.clone();
    }
    

    pub fn setName(&mut self, mut name: String) {
        jcall!(self.logUtil).putF(format!("{}{}", String::from("Name: "), name), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setName"));
        self.name = name.clone();
    }
    

    pub fn getClassName(&mut self) -> String {
        return self.className.clone();
    }
    

    pub fn setClassName(&mut self, mut className: String) {
        jcall!(self.logUtil).putF(format!("{}{}", String::from("ClassName : "), className), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setClassName"));
        self.className = className.clone();
    }
    

    pub fn getInputAutomationModuleInterface(&mut self) -> Option<Rc<RefCell<dyn InputAutomationModuleFactoryInterface>>> {
        return self.inputAutomationModuleInterface.clone();
    }
    

    pub fn setInputAutomationModuleInterface(&mut self, mut inputAutomationModuleInterface: Option<Rc<RefCell<dyn InputAutomationModuleFactoryInterface>>>) {
        jcall!(self.logUtil).putF(format!("{}{}", String::from("InputAutomationModuleFactoryInterface : "), inputAutomationModuleInterface), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setInputAutomationModuleInterface"));
        self.inputAutomationModuleInterface = inputAutomationModuleInterface.clone();
        { let java_arguments = (jcall!(self.inputAutomationModuleInterface).getName(),); self.setName(java_arguments.0) };
    }
}

