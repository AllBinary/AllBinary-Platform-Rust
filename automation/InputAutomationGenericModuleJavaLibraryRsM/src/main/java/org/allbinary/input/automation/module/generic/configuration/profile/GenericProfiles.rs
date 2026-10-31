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
// package org.allbinary.input.automation.module.generic.configuration.profile
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
use crate::org::allbinary::input::automation::module::generic::configuration::profile::actions::GenericProfileActionData::GenericProfileActionData;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use crate::org::w3c::dom::NodeList::NodeList;
use super::GenericProfile::GenericProfile;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GenericProfiles {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub hashMap: Option<Rc<RefCell<HashMap>>>,
    pub profilesDefaultListModelHelper: Option<Rc<RefCell<DefaultListModelHelper>>>,
    pub fileName: String,
}

thread_local! { pub static GenericProfiles_DEFAULT_PROFILES_PATH: String = String::from("./modules/configs/profiles/"); }
thread_local! { pub static GenericProfiles_DEFAULT_PROFILE_ACTIONS_PATH: String = format!("{}{}", GenericProfiles_DEFAULT_PROFILES_PATH.with(Clone::clone), String::from("actions/")); }
thread_local! { pub static GenericProfiles_DEFAULT_FILE: String = format!("{}{}", GenericProfiles_DEFAULT_PROFILES_PATH.with(Clone::clone), String::from("profiles.xml")); }
impl DomNodeInterface for GenericProfiles {
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}
impl DomDocumentMappingInterface for GenericProfiles {
    fn toXmlDoc(&mut self) -> Option<Rc<RefCell<Document>>> { Self::toXmlDoc(self) }
}

impl GenericProfiles {
    

    pub fn new_struct(mut fileName: String) -> Self {
        let mut this = Self {
            logUtil: None,
            hashMap: None,
            profilesDefaultListModelHelper: None,
            fileName: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.fileName = fileName.clone();
        this.profilesDefaultListModelHelper = DefaultListModelHelper::new();
        this.hashMap = jcall!(StdUtil::getInstance()).createHashMap();
        if jcall!(File::new(fileName.clone())).isFile() {
            let mut bytes: Vec<i8> = vec![0; (100000) as usize];
            let mut idFile: Option<Rc<RefCell<FileInputStream>>> = FileInputStream::new(fileName.clone());
            let mut length: i32 = jcall!(idFile).read(bytes.clone());
            let mut data: String = String::from_utf8(bytes.clone().iter().map(|value| *value as u8).collect()).unwrap();
            let mut endIndex: i32 = data.rfind('>').map(|index| index as i32).unwrap_or(-1);
            let mut document: Option<Rc<RefCell<Document>>> = DomDocumentHelper::create1(data[(0) as usize..(endIndex + 1) as usize].to_string());
            let mut nodeList: Option<Rc<RefCell<NodeList>>> = jcall!(document).getElementsByTagName(unsafe { GenericProfilesData_NAME.clone() }.clone());
            { let java_receiver = this.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Number Of Profiles Specified: "), jcall!(nodeList).getLength()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("Contructor"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            if nodeList.is_some() {
                { let java_arguments = (jcall!(nodeList).item(0),); this.initProfiles(java_arguments.0) };
            } else {
                panic!("{:?}", format!("{}: {:?}", "Exception", (format!("{}{}", unsafe { crate::org::allbinary::input::automation::module::generic::configuration::profile::actions::GenericProfileActionData::GenericProfileActionData_NAME.clone() }, String::from(" Name Node Node Children")))));
            }
            { let java_receiver = this.logUtil.clone(); let java_arguments = (format!("{}{}", format!("{}{}", String::from("Loaded: "), jcall!(this.hashMap).size()), String::from(" Configuration Profile Actions")), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("Contructor"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        } else {
            jcall!(this.logUtil).putF(format!("{}{}", String::from("No Generic Profile: "), fileName), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("Contructor"));
        }
        jcall!(this.getDefaultListModelHelper()).initDefaultModelList();
        this
    }

    pub fn new(mut fileName: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(fileName))))
    }
    

    // throws Exception
    pub fn initProfiles(&mut self, mut node: Option<Rc<RefCell<Node>>>) {
        let mut nodeList: Option<Rc<RefCell<NodeList>>> = jcall!(node).getChildNodes();
        for index in (0..jcall!(nodeList).getLength()) {
            let mut profileNameNode: Option<Rc<RefCell<Node>>> = jcall!(nodeList).item(index.clone());
            if jcall!(jcall!(profileNameNode).getNodeName()).compareTo(unsafe { GenericProfileData_NAME.clone() }.clone()) == 0 {
                let mut genericProfile: Option<Rc<RefCell<GenericProfile>>> = GenericProfile::new1(profileNameNode.clone());
                { let java_receiver = self.hashMap.clone(); let java_arguments = (jcall!(genericProfile).getName(), genericProfile.clone(),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1); };
                jcall!(self.getDefaultListModelHelper()).add(jcall!(genericProfile).getName());
            }
        }
    }
    

    pub fn getDefaultListModelHelper(&mut self) -> Option<Rc<RefCell<DefaultListModelHelper>>> {
        return self.profilesDefaultListModelHelper.clone();
    }
    

    // throws Exception
    pub fn get(&mut self, mut name: String) -> Option<Rc<RefCell<GenericProfile>>> {
        let mut genericProfile: Option<Rc<RefCell<GenericProfile>>> = (jcall!(self.hashMap).get(name.clone())).clone() /* as GenericProfile */;
        if genericProfile.is_none() {
            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(self.logUtil).putF(format!("{}{}", format!("{}{}", format!("{}{}", String::from("No Generic Profile Named: "), name), String::from(" availability was: ")), self.hashMap), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).GET.clone());
        }
        return genericProfile.clone();
    }
    

    // throws Exception
    pub fn add(&mut self, mut name: String) {
        jcall!(self.hashMap).put(name.clone(), GenericProfile::new1(name.clone()));
        jcall!(self.getDefaultListModelHelper()).add(name.clone());
        jcall!(self.getDefaultListModelHelper()).initDefaultModelList();
        self.save();
    }
    

    // throws Exception
    pub fn remove(&mut self, mut name: String) {
        jcall!(self.hashMap).remove(name.clone());
        jcall!(self.getDefaultListModelHelper()).remove(name.clone());
        jcall!(self.getDefaultListModelHelper()).initDefaultModelList();
        self.save();
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        let mut node: Option<Rc<RefCell<Node>>> = jcall!(document).createElement(unsafe { GenericProfilesData_NAME.clone() }.clone());
        let mut set: Option<Rc<RefCell<dyn Set>>> = jcall!(self.hashMap).keySet();
        let mut nameArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(set).toArray();
        let mut size: i32 = (nameArray.len() as i32).clone();
        for index in (0..size) {
            let mut genericProfile: Option<Rc<RefCell<GenericProfile>>> = (jcall!(self.hashMap).get((nameArray[(index) as usize]).clone() /* as String */)).clone() /* as GenericProfile */;
            jcall!(node).appendChild(jcall!(genericProfile).toXmlNode(document.clone()));
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
    

    // throws Exception
    pub fn save(&mut self) {
        let mut idFile: Option<Rc<RefCell<FileOutputStream>>> = FileOutputStream::new(self.fileName.clone());
        let mut idOutData: Option<Rc<RefCell<DataOutputStream>>> = DataOutputStream::new(idFile.clone());
        let mut documentString: String = XmlDocumentHelper::toString(self.toXmlDoc());
        jcall!(idOutData).writeBytes(documentString.clone());
    }
}

