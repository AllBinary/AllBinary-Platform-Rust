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
// package org.allbinary.business.user.modules.configuration
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::business::entry::EntryData::EntryData;
use crate::org::allbinary::business::time::created::TimeCreatedView::TimeCreatedView;
use crate::org::allbinary::business::time::modified::TimeLastModifiedView::TimeLastModifiedView;
use crate::org::allbinary::data::tree::dom::DomNodeInterface::DomNodeInterface;
use crate::org::allbinary::data::tree::dom::DomSearchHelper::DomSearchHelper;
use crate::org::allbinary::logic::communication::smtp::configuration::user::UserEmailConfigurationData::UserEmailConfigurationData;
use crate::org::allbinary::logic::communication::smtp::configuration::user::UserEmailConfigurationInterface::UserEmailConfigurationInterface;
use crate::org::allbinary::logic::communication::smtp::configuration::user::UserEmailConfigurationView::UserEmailConfigurationView;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use crate::org::w3c::dom::NodeList::NodeList;
use super::UserConfigurationInterface::UserConfigurationInterface;
use super::UserConfiguration::UserConfiguration;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct UserConfigurationView {
    pub userConfigurationInterface: Option<Rc<RefCell<dyn UserConfigurationInterface>>>,
}

impl DomNodeInterface for UserConfigurationView {
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}

impl UserConfigurationView {
    

    pub fn new_struct1(mut node: Option<Rc<RefCell<Node>>>) -> Self {
        let mut this = Self {
            userConfigurationInterface: None,
        };
        this.userConfigurationInterface = (UserConfiguration::new()).clone() /* as UserConfigurationInterface */;
        let mut childNodeList: Option<Rc<RefCell<NodeList>>> = jcall!(node).getChildNodes();
        let mut emailConfigurationNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(crate::org::allbinary::logic::communication::smtp::configuration::user::UserEmailConfigurationData::UserEmailConfigurationData_NAME.with(Clone::clone).clone(), childNodeList.clone());
        let mut userEmailConfigurationView: Option<Rc<RefCell<UserEmailConfigurationView>>> = UserEmailConfigurationView::new1(emailConfigurationNode.clone());
        let mut userEmailConfigurationInterface: Option<Rc<RefCell<dyn UserEmailConfigurationInterface>>> = (jcall!(userEmailConfigurationView).getEmailConfigurationInterface()).clone() /* as UserEmailConfigurationInterface */;
        jcall!(this.userConfigurationInterface).setUserEmailConfigurationInterface(userEmailConfigurationInterface.clone());
        let mut timeCreatedNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(jcall!(EntryData::getInstance()).TIMECREATED.clone(), childNodeList.clone());
        let mut timeCreatedView: Option<Rc<RefCell<TimeCreatedView>>> = TimeCreatedView::new1(timeCreatedNode.clone());
        { let java_receiver = this.userConfigurationInterface.clone(); let java_arguments = (jcall!(timeCreatedView).getTimeCreated(),); jcall!(java_receiver).setTimeCreated(java_arguments.0); };
        let mut timeLastModifiedNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(jcall!(EntryData::getInstance()).LASTMODIFIED.clone(), childNodeList.clone());
        let mut timeLastModifiedView: Option<Rc<RefCell<TimeLastModifiedView>>> = TimeLastModifiedView::new1(timeLastModifiedNode.clone());
        { let java_receiver = this.userConfigurationInterface.clone(); let java_arguments = (jcall!(timeLastModifiedView).getTimeLastModified(),); jcall!(java_receiver).setTimeLastModified(java_arguments.0); };
        this
    }

    pub fn new1(mut node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(node))))
    }
    

    pub fn new_struct1(mut userConfigurationInterface: Option<Rc<RefCell<dyn UserConfigurationInterface>>>) -> Self {
        let mut this = Self {
            userConfigurationInterface: None,
        };
        this.userConfigurationInterface = userConfigurationInterface.clone();
        this
    }

    pub fn new1(mut userConfigurationInterface: Option<Rc<RefCell<dyn UserConfigurationInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(userConfigurationInterface))))
    }
    

    pub fn getUserConfigurationInterface(&mut self) -> Option<Rc<RefCell<dyn UserConfigurationInterface>>> {
        return self.userConfigurationInterface.clone();
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        let mut node: Option<Rc<RefCell<Node>>> = jcall!(document).createElement(UserConfigurationData_NAME.with(Clone::clone).clone());
        let mut userEmailConfigurationView: Option<Rc<RefCell<UserEmailConfigurationView>>> = UserEmailConfigurationView::new1(jcall!(self.userConfigurationInterface).getUserEmailConfigurationInterface());
        jcall!(node).appendChild(jcall!(userEmailConfigurationView).toXmlNode(document.clone()));
        let mut timeCreatedView: Option<Rc<RefCell<TimeCreatedView>>> = TimeCreatedView::new1(jcall!(self.userConfigurationInterface).getTimeCreated());
        jcall!(node).appendChild(jcall!(timeCreatedView).toXmlNode(document.clone()));
        let mut timeLastModifiedView: Option<Rc<RefCell<TimeLastModifiedView>>> = TimeLastModifiedView::new1(jcall!(self.userConfigurationInterface).getTimeLastModified());
        jcall!(node).appendChild(jcall!(timeLastModifiedView).toXmlNode(document.clone()));
        return node.clone();
    }
}

