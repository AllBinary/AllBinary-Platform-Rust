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
// package org.allbinary.business.time.created
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::business::entry::EntryData::EntryData;
use crate::org::allbinary::data::tree::dom::DomNodeHelper::DomNodeHelper;
use crate::org::allbinary::data::tree::dom::DomNodeInterface::DomNodeInterface;
use crate::org::allbinary::data::tree::dom::ModDomHelper::ModDomHelper;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use super::TimeCreated::TimeCreated;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TimeCreatedView {
    pub timeCreated: Option<Rc<RefCell<TimeCreated>>>,
}

impl DomNodeInterface for TimeCreatedView {
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}

impl TimeCreatedView {
    

    pub fn new_struct1(mut timeCreated: Option<Rc<RefCell<TimeCreated>>>) -> Self {
        let mut this = Self {
            timeCreated: None,
        };
        this.setTimeCreated(timeCreated.clone());
        this
    }

    pub fn new1(mut timeCreated: Option<Rc<RefCell<TimeCreated>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(timeCreated))))
    }
    

    pub fn new_struct1(mut node: Option<Rc<RefCell<Node>>>) -> Self {
        let mut this = Self {
            timeCreated: None,
        };
        { let java_arguments = (TimeCreated::new1(DomNodeHelper::getTextNodeValue(node.clone())),); this.setTimeCreated(java_arguments.0) };
        this
    }

    pub fn new1(mut node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(node))))
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        return ModDomHelper::createTextNode(document.clone(), jcall!(EntryData::getInstance()).TIMECREATED.clone(), jcall!(self.getTimeCreated()).toString());
    }
    

    pub fn getTimeCreated(&mut self) -> Option<Rc<RefCell<TimeCreated>>> {
        return self.timeCreated.clone();
    }
    

    pub fn setTimeCreated(&mut self, mut timeCreated: Option<Rc<RefCell<TimeCreated>>>) {
        self.timeCreated = timeCreated.clone();
    }
}

