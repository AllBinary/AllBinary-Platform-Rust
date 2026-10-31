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
// package org.allbinary.business.time.modified
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
use super::TimeLastModified::TimeLastModified;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TimeLastModifiedView {
    pub timeLastModified: Option<Rc<RefCell<TimeLastModified>>>,
}

impl DomNodeInterface for TimeLastModifiedView {
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}

impl TimeLastModifiedView {
    

    pub fn new_struct1(mut timeLastModified: Option<Rc<RefCell<TimeLastModified>>>) -> Self {
        let mut this = Self {
            timeLastModified: None,
        };
        this.setTimeLastModified(timeLastModified.clone());
        this
    }

    pub fn new1(mut timeLastModified: Option<Rc<RefCell<TimeLastModified>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(timeLastModified))))
    }
    

    pub fn new_struct1(mut node: Option<Rc<RefCell<Node>>>) -> Self {
        let mut this = Self {
            timeLastModified: None,
        };
        { let java_arguments = (TimeLastModified::new1(DomNodeHelper::getTextNodeValue(node.clone())),); this.setTimeLastModified(java_arguments.0) };
        this
    }

    pub fn new1(mut node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(node))))
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        return ModDomHelper::createTextNode(document.clone(), jcall!(EntryData::getInstance()).TIMECREATED.clone(), jcall!(self.getTimeLastModified()).toString());
    }
    

    pub fn getTimeLastModified(&mut self) -> Option<Rc<RefCell<TimeLastModified>>> {
        return self.timeLastModified.clone();
    }
    

    pub fn setTimeLastModified(&mut self, mut timeLastModified: Option<Rc<RefCell<TimeLastModified>>>) {
        self.timeLastModified = timeLastModified.clone();
    }
}

