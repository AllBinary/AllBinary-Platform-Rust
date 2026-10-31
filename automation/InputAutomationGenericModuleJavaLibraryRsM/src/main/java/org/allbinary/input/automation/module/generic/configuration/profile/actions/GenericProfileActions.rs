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
// package org.allbinary.input.automation.module.generic.configuration.profile.actions
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::io::DataOutputStream::DataOutputStream;
// use crate::java::io::File::File;
// use crate::java::io::FileInputStream::FileInputStream;
// use crate::java::io::FileOutputStream::FileOutputStream;
// use crate::java::util::HashMap::HashMap;
// use crate::java::util::Set::Set;
use crate::org::allbinary::data::tree::dom::DomNodeInterface::DomNodeInterface;
use crate::org::allbinary::data::tree::dom::document::DomDocumentHelper::DomDocumentHelper;
use crate::org::allbinary::data::tree::dom::document::XmlDocumentHelper::XmlDocumentHelper;
use crate::org::allbinary::data::tree::dom::document::mapping::DomDocumentMappingInterface::DomDocumentMappingInterface;
use crate::org::allbinary::input::automation::module::DefaultListModelHelper::DefaultListModelHelper;
use crate::org::allbinary::input::automation::module::generic::configuration::profile::GenericProfiles::GenericProfiles;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::io::file::CommonDataFileStrings::CommonDataFileStrings;
use crate::org::allbinary::logic::io::path::AbPath::AbPath;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use crate::org::w3c::dom::NodeList::NodeList;
use super::GenericProfileActionsJPanel::GenericProfileActionsJPanel;
use super::GenericProfileAction::GenericProfileAction;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GenericProfileActions {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonFileStrings: Option<Rc<RefCell<CommonDataFileStrings>>>,
    pub name: String,
    pub genericProfileActionsJPanel: Option<Rc<RefCell<GenericProfileActionsJPanel>>>,
    pub actionsDefaultListModelHelper: Option<Rc<RefCell<DefaultListModelHelper>>>,
    pub hashMap: Option<Rc<RefCell<HashMap>>>,
}

thread_local! { pub static GenericProfileActions_DEFAULT_PROFILE_ACTIONS_PATH: String = format!("{}{}", crate::org::allbinary::input::automation::module::generic::configuration::profile::GenericProfiles::GenericProfiles_DEFAULT_PROFILES_PATH.with(Clone::clone), String::from("actions/")); }
impl DomNodeInterface for GenericProfileActions {
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}
impl DomDocumentMappingInterface for GenericProfileActions {
    fn toXmlDoc(&mut self) -> Option<Rc<RefCell<Document>>> { Self::toXmlDoc(self) }
}

impl GenericProfileActions {
    

    pub fn new_struct2(mut genericProfileActionsJPanel: Option<Rc<RefCell<GenericProfileActionsJPanel>>>, mut name: String) -> Self {
        let mut this = Self {
            logUtil: None,
            commonFileStrings: None,
            name: String::new(),
            genericProfileActionsJPanel: None,
            actionsDefaultListModelHelper: None,
            hashMap: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonFileStrings = CommonDataFileStrings::getInstance();
        this.setName(name.clone());
        this.init(genericProfileActionsJPanel.clone());
        this.load();
        this
    }

    pub fn new2(mut genericProfileActionsJPanel: Option<Rc<RefCell<GenericProfileActionsJPanel>>>, mut name: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2(genericProfileActionsJPanel, name))))
    }
    

    pub fn new_struct3(mut genericProfileActionsJPanel: Option<Rc<RefCell<GenericProfileActionsJPanel>>>, mut abPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>, mut name: String) -> Self {
        let mut this = Self {
            logUtil: None,
            commonFileStrings: None,
            name: String::new(),
            genericProfileActionsJPanel: None,
            actionsDefaultListModelHelper: None,
            hashMap: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonFileStrings = CommonDataFileStrings::getInstance();
        this.setName(name.clone());
        this.init(genericProfileActionsJPanel.clone());
        this.load();
        this
    }

    pub fn new3(mut genericProfileActionsJPanel: Option<Rc<RefCell<GenericProfileActionsJPanel>>>, mut abPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>, mut name: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct3(genericProfileActionsJPanel, abPath, name))))
    }
    

    pub fn new_struct3(mut genericProfileActionsJPanel: Option<Rc<RefCell<GenericProfileActionsJPanel>>>, mut fileInputStream: Option<Rc<RefCell<FileInputStream>>>, mut name: String) -> Self {
        let mut this = Self {
            logUtil: None,
            commonFileStrings: None,
            name: String::new(),
            genericProfileActionsJPanel: None,
            actionsDefaultListModelHelper: None,
            hashMap: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonFileStrings = CommonDataFileStrings::getInstance();
        this.setName(name.clone());
        this.init(genericProfileActionsJPanel.clone());
        this.fileInit(fileInputStream.clone());
        this
    }

    pub fn new3(mut genericProfileActionsJPanel: Option<Rc<RefCell<GenericProfileActionsJPanel>>>, mut fileInputStream: Option<Rc<RefCell<FileInputStream>>>, mut name: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct3(genericProfileActionsJPanel, fileInputStream, name))))
    }
    

    // throws Exception
    pub fn init(&mut self, mut genericProfileActionsJPanel: Option<Rc<RefCell<GenericProfileActionsJPanel>>>) {
        self.setGenericProfileActionsJPanel(genericProfileActionsJPanel.clone());
        self.actionsDefaultListModelHelper = DefaultListModelHelper::new();
        self.setHashMap(HashMap::new());
    }
    

    // throws Exception
    pub fn save(&mut self) {
        let mut idFile: Option<Rc<RefCell<FileOutputStream>>> = FileOutputStream::new(format!("{}{}", format!("{}{}", GenericProfileActions_DEFAULT_PROFILE_ACTIONS_PATH.with(Clone::clone), self.getName()), jcall!(self.commonFileStrings)._XML));
        let mut idOutData: Option<Rc<RefCell<DataOutputStream>>> = DataOutputStream::new(idFile.clone());
        jcall!(idOutData).writeBytes(XmlDocumentHelper::toString(self.toXmlDoc()));
    }
    

    pub fn getFile(&mut self, mut name: String) -> Option<Rc<RefCell<File>>> {
        let mut fileName: String = format!("{}{}", format!("{}{}", GenericProfileActions_DEFAULT_PROFILE_ACTIONS_PATH.with(Clone::clone), name), jcall!(self.commonFileStrings)._XML);
        return File::new(fileName.clone());
    }
    

    // throws Exception
    pub fn load(&mut self) {
        let mut file: Option<Rc<RefCell<File>>> = { let java_arguments = (self.getName(),); self.getFile(java_arguments.0) };
        if jcall!(file).isFile() {
            let mut idFile: Option<Rc<RefCell<FileInputStream>>> = FileInputStream::new(file.clone());
            self.fileInit(idFile.clone());
        } else {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("No Profile: "), jcall!(file).getAbsolutePath()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("Contructor"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
    }
    

    // throws Exception
    pub fn fileInit(&mut self, mut fileInputStream: Option<Rc<RefCell<FileInputStream>>>) {
        let mut bytes: Vec<i8> = vec![0; (100000) as usize];
        let mut length: i32 = jcall!(fileInputStream).read(bytes.clone());
        let mut data: String = String::from_utf8(bytes.clone().iter().map(|value| *value as u8).collect()).unwrap();
        let mut endIndex: i32 = data.rfind('>').map(|index| index as i32).unwrap_or(-1);
        let mut document: Option<Rc<RefCell<Document>>> = DomDocumentHelper::create1(data[(0) as usize..(endIndex + 1) as usize].to_string());
        let mut nameNodeList: Option<Rc<RefCell<NodeList>>> = jcall!(document).getElementsByTagName(unsafe { GenericProfileActionsData_NAME.clone() }.clone());
        { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Number Of Profiles Specified: "), jcall!(nameNodeList).getLength()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("Contructor"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        for index in (0..jcall!(nameNodeList).getLength()) {
            let mut node: Option<Rc<RefCell<Node>>> = jcall!(nameNodeList).item(index.clone());
            let mut nodeList: Option<Rc<RefCell<NodeList>>> = jcall!(node).getChildNodes();
            if nodeList.is_some() {
                self.initActions(nodeList.clone());
            } else {
                panic!("{:?}", format!("{}: {:?}", "Exception", (format!("{}{}", unsafe { GenericProfileActionData_NAME.clone() }, String::from(" Name Node Node Children")))));
            }
        }
        { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", format!("{}{}", String::from("Loaded: "), jcall!(self.getHashMap()).size()), String::from(" Configuration Profile Action(s)")), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("Contructor"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        jcall!(self.getDefaultListModelHelper()).initDefaultModelList();
    }
    

    // throws Exception
    pub fn initActions(&mut self, mut nodeList: Option<Rc<RefCell<NodeList>>>) {
        for index in (0..jcall!(nodeList).getLength()) {
            let mut actionNode: Option<Rc<RefCell<Node>>> = jcall!(nodeList).item(index.clone());
            if actionNode.is_some() {
                let mut genericConfigurationProfileAction: Option<Rc<RefCell<GenericProfileAction>>> = GenericProfileAction::new2(jcall!(self.getGenericProfileActionsJPanel()).getGenericProfileActionJPanel(), actionNode.clone());
                jcall!(self.getHashMap()).put(jcall!(genericConfigurationProfileAction).getName(), genericConfigurationProfileAction.clone());
                jcall!(self.getDefaultListModelHelper()).add(jcall!(genericConfigurationProfileAction).getName());
            } else {
                panic!("{:?}", format!("{}: {:?}", "Exception", (format!("{}{}", unsafe { GenericProfileActionData_NAME.clone() }, String::from(" Node Null")))));
            }
        }
    }
    

    pub fn getDefaultListModelHelper(&mut self) -> Option<Rc<RefCell<DefaultListModelHelper>>> {
        return self.actionsDefaultListModelHelper.clone();
    }
    

    pub fn getAction(&mut self, mut string: String) -> Option<Rc<RefCell<GenericProfileAction>>> {
        return (jcall!(self.getHashMap()).get(string.clone())).clone() /* as GenericProfileAction */;
    }
    

    // throws Exception
    pub fn add(&mut self, mut name: String) {
        let mut genericProfileAction: Option<Rc<RefCell<GenericProfileAction>>> = GenericProfileAction::new1(name.clone());
        jcall!(self.getHashMap()).put(jcall!(genericProfileAction).getName(), genericProfileAction.clone());
        jcall!(self.getDefaultListModelHelper()).add(jcall!(genericProfileAction).getName());
        jcall!(self.getDefaultListModelHelper()).initDefaultModelList();
        self.save();
    }
    

    // throws Exception
    pub fn remove(&mut self, mut name: String) {
        jcall!(self.getHashMap()).remove(name.clone());
        jcall!(self.getDefaultListModelHelper()).remove(name.clone());
        jcall!(self.getDefaultListModelHelper()).initDefaultModelList();
        self.save();
    }
    

    pub fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> {
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("HashMap: "), jcall!(hashMap).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("toHashMap()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        return hashMap.clone();
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        let mut node: Option<Rc<RefCell<Node>>> = jcall!(document).createElement(unsafe { GenericProfileActionsData_NAME.clone() }.clone());
        let mut set: Option<Rc<RefCell<dyn Set>>> = jcall!(self.getHashMap()).keySet();
        let mut actionNameArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(set).toArray();
        let mut size: i32 = (actionNameArray.len() as i32).clone();
        for index in (0..size) {
            let mut nextActionName: String = (actionNameArray[(index) as usize]).clone() /* as String */;
            let mut nextGenericProfileAction: Option<Rc<RefCell<GenericProfileAction>>> = (self.getAction(nextActionName.clone())).clone() /* as GenericProfileAction */;
            jcall!(node).appendChild(jcall!(nextGenericProfileAction).toXmlNode(document.clone()));
        }
        return node.clone();
    }
    

    // throws Exception
    pub fn toXmlDoc(&mut self) -> Option<Rc<RefCell<Document>>> {
        let mut document: Option<Rc<RefCell<Document>>> = DomDocumentHelper::create();
        let mut node: Option<Rc<RefCell<Node>>> = self.toXmlNode(document.clone());
        jcall!(document).appendChild(node.clone());
        return document.clone();
    }
    

    pub fn getGenericProfileActionsJPanel(&mut self) -> Option<Rc<RefCell<GenericProfileActionsJPanel>>> {
        return self.genericProfileActionsJPanel.clone();
    }
    

    pub fn setGenericProfileActionsJPanel(&mut self, mut genericProfileActionsJPanel: Option<Rc<RefCell<GenericProfileActionsJPanel>>>) {
        self.genericProfileActionsJPanel = genericProfileActionsJPanel.clone();
    }
    

    pub fn getHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> {
        return self.hashMap.clone();
    }
    

    pub fn setHashMap(&mut self, mut hashMap: Option<Rc<RefCell<HashMap>>>) {
        self.hashMap = hashMap.clone();
    }
    

    pub fn getName(&mut self) -> String {
        return self.name.clone();
    }
    

    pub fn setName(&mut self, mut name: String) {
        self.name = name.clone();
    }
}

