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
// package org.allbinary.logic.communication.smtp.configuration.user.event
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
// use crate::java::util::Set::Set;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::data::tree::dom::DomNodeInterface::DomNodeInterface;
use crate::org::allbinary::data::tree::dom::DomSearchHelper::DomSearchHelper;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use crate::org::w3c::dom::NodeList::NodeList;
use super::UserEmailEventsConfigurationInterface::UserEmailEventsConfigurationInterface;
use super::UserEmailEventsConfiguration::UserEmailEventsConfiguration;
use super::UserEmailEventConfigurationView::UserEmailEventConfigurationView;
use super::UserEmailEventsConfigurationMapping::UserEmailEventsConfigurationMapping;
use super::UserEmailEventConfigurationInterface::UserEmailEventConfigurationInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct UserEmailEventsConfigurationView {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub userEmailEventsConfigurationInterface: Option<Rc<RefCell<dyn UserEmailEventsConfigurationInterface>>>,
}

impl DomNodeInterface for UserEmailEventsConfigurationView {
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}

impl UserEmailEventsConfigurationView {
    

    pub fn new_struct1(mut node: Option<Rc<RefCell<Node>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            userEmailEventsConfigurationInterface: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.userEmailEventsConfigurationInterface = (UserEmailEventsConfiguration::new()).clone() /* as UserEmailEventsConfigurationInterface */;
        let mut childNodeList: Option<Rc<RefCell<NodeList>>> = jcall!(node).getChildNodes();
        let mut emailEventConfigurationNodeVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = DomSearchHelper::getAllNodesNoThrow(UserEmailEventConfigurationData_NAME.with(Clone::clone).clone(), childNodeList.clone());
        let mut size: i32 = jcall!(emailEventConfigurationNodeVector).size();
        for index in (0..size) {
            let mut userEmailConfigurationNode: Option<Rc<RefCell<Node>>> = (jcall!(emailEventConfigurationNodeVector).get(index.clone())).clone() /* as Node */;
            let mut userEmailEventConfigurationView: Option<Rc<RefCell<UserEmailEventConfigurationView>>> = UserEmailEventConfigurationView::new1(userEmailConfigurationNode.clone());
            { let java_receiver = this.userEmailEventsConfigurationInterface.clone(); let java_arguments = (jcall!(userEmailEventConfigurationView).getUserEmailEventConfigurationInterface(),); jcall!(java_receiver).addUserEmailEventConfiguration(java_arguments.0); };
        }
        this
    }

    pub fn new1(mut node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(node))))
    }
    

    pub fn new_struct1(mut userEmailEventsConfigurationInterface: Option<Rc<RefCell<dyn UserEmailEventsConfigurationInterface>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            userEmailEventsConfigurationInterface: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.userEmailEventsConfigurationInterface = userEmailEventsConfigurationInterface.clone();
        this
    }

    pub fn new1(mut userEmailEventsConfigurationInterface: Option<Rc<RefCell<dyn UserEmailEventsConfigurationInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(userEmailEventsConfigurationInterface))))
    }
    

    pub fn getUserEmailEventsConfigurationInterface(&mut self) -> Option<Rc<RefCell<dyn UserEmailEventsConfigurationInterface>>> {
        return self.userEmailEventsConfigurationInterface.clone();
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        let mut userEmailConfigurationMapping: Option<Rc<RefCell<UserEmailEventsConfigurationMapping>>> = UserEmailEventsConfigurationMapping::new(self.userEmailEventsConfigurationInterface.clone());
        let mut node: Option<Rc<RefCell<Node>>> = jcall!(document).createElement(UserEmailEventsConfigurationData_NAME.with(Clone::clone).clone());
        let mut emailEventHashMap: Option<Rc<RefCell<HashMap>>> = jcall!(self.getUserEmailEventsConfigurationInterface()).getEventConfigurationHashMap();
        let mut set: Option<Rc<RefCell<dyn Set>>> = jcall!(emailEventHashMap).keySet();
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).EMAILLOGGING.clone()) {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Number Of Email Events Specified in file: "), jcall!(set).size()), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).CONSTRUCTOR.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        let mut eventNameArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(set).toArray();
        let mut size: i32 = (eventNameArray.len() as i32).clone();
        for index in (0..size) {
            let mut eventName: String = (eventNameArray[(index) as usize]).clone() /* as String */;
            let mut userEmailEventsConfigurationInterface: Option<Rc<RefCell<dyn UserEmailEventConfigurationInterface>>> = (jcall!(emailEventHashMap).get(eventName.clone())).clone() /* as UserEmailEventConfigurationInterface */;
            let mut userEmailEventConfigurationView: Option<Rc<RefCell<UserEmailEventConfigurationView>>> = UserEmailEventConfigurationView::new1(userEmailEventsConfigurationInterface.clone());
            jcall!(node).appendChild(jcall!(userEmailEventConfigurationView).toXmlNode(document.clone()));
        }
        return node.clone();
    }
}

