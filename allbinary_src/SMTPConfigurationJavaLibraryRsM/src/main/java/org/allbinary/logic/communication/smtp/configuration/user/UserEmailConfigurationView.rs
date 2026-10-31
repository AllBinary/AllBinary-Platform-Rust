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
// package org.allbinary.logic.communication.smtp.configuration.user
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::data::tree::dom::DomNodeInterface::DomNodeInterface;
use crate::org::allbinary::data::tree::dom::DomSearchHelper::DomSearchHelper;
use crate::org::allbinary::data::tree::dom::ModDomHelper::ModDomHelper;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::smtp::configuration::user::event::UserEmailEventsConfigurationData::UserEmailEventsConfigurationData;
use crate::org::allbinary::logic::communication::smtp::configuration::user::event::UserEmailEventsConfigurationInterface::UserEmailEventsConfigurationInterface;
use crate::org::allbinary::logic::communication::smtp::configuration::user::event::UserEmailEventsConfigurationView::UserEmailEventsConfigurationView;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use crate::org::w3c::dom::NodeList::NodeList;
use super::UserEmailConfigurationInterface::UserEmailConfigurationInterface;
use super::UserEmailConfiguration::UserEmailConfiguration;
use super::UserEmailConfigurationMapping::UserEmailConfigurationMapping;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct UserEmailConfigurationView {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub userEmailConfigurationInterface: Option<Rc<RefCell<dyn UserEmailConfigurationInterface>>>,
}

impl DomNodeInterface for UserEmailConfigurationView {
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}

impl UserEmailConfigurationView {
    

    pub fn new_struct1(mut node: Option<Rc<RefCell<Node>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            userEmailConfigurationInterface: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).EMAILLOGGING.clone()) {
            jcall!(this.logUtil).putF(jcall!(this.commonStrings).CONSTRUCTOR.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone());
        }
        this.userEmailConfigurationInterface = (UserEmailConfiguration::new()).clone() /* as UserEmailConfigurationInterface */;
        let mut childNodeList: Option<Rc<RefCell<NodeList>>> = jcall!(node).getChildNodes();
        let mut userEmailEventsConfigurationNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNodeNoThrow(crate::org::allbinary::logic::communication::smtp::configuration::user::event::UserEmailEventsConfigurationData::UserEmailEventsConfigurationData_NAME.with(Clone::clone).clone(), childNodeList.clone());
        let mut userEmailEventsConfigurationView: Option<Rc<RefCell<UserEmailEventsConfigurationView>>> = UserEmailEventsConfigurationView::new1(userEmailEventsConfigurationNode.clone());
        { let java_receiver = this.userEmailConfigurationInterface.clone(); let java_arguments = (jcall!(userEmailEventsConfigurationView).getUserEmailEventsConfigurationInterface(),); jcall!(java_receiver).setUserEmailEventsConfigurationInterface(java_arguments.0); };
        this
    }

    pub fn new1(mut node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(node))))
    }
    

    pub fn new_struct1(mut userEmailConfigurationInterface: Option<Rc<RefCell<dyn UserEmailConfigurationInterface>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            userEmailConfigurationInterface: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.userEmailConfigurationInterface = userEmailConfigurationInterface.clone();
        this
    }

    pub fn new1(mut userEmailConfigurationInterface: Option<Rc<RefCell<dyn UserEmailConfigurationInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(userEmailConfigurationInterface))))
    }
    

    pub fn getEmailConfigurationInterface(&mut self) -> Option<Rc<RefCell<dyn UserEmailConfigurationInterface>>> {
        return self.userEmailConfigurationInterface.clone();
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        let mut emailConfigurationMapping: Option<Rc<RefCell<UserEmailConfigurationMapping>>> = UserEmailConfigurationMapping::new(self.userEmailConfigurationInterface.clone());
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = jcall!(emailConfigurationMapping).toHashMap();
        let mut node: Option<Rc<RefCell<Node>>> = ModDomHelper::createNameValueNodes(document.clone(), UserEmailConfigurationData_NAME.with(Clone::clone).clone(), hashMap.clone());
        let mut userEmailConfigurationInterface: Option<Rc<RefCell<dyn UserEmailConfigurationInterface>>> = self.getEmailConfigurationInterface();
        let mut userEmailEventsConfigurationInterface: Option<Rc<RefCell<dyn UserEmailEventsConfigurationInterface>>> = jcall!(userEmailConfigurationInterface).getUserEmailEventsConfigurationInterface();
        let mut userEmailEventsConfigurationView: Option<Rc<RefCell<UserEmailEventsConfigurationView>>> = UserEmailEventsConfigurationView::new1(userEmailEventsConfigurationInterface.clone());
        jcall!(node).appendChild(jcall!(userEmailEventsConfigurationView).toXmlNode(document.clone()));
        return node.clone();
    }
}

