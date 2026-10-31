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
// package org.allbinary.logic.communication.java.vm.stats
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::data::tree::dom::DomNodeInterface::DomNodeInterface;
use crate::org::allbinary::data::tree::dom::ModDomHelper::ModDomHelper;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use super::JvmStatisticsMapping::JvmStatisticsMapping;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct JvmStatisticsView {
    pub base: JvmStatisticsMapping,
}

impl DomNodeInterface for JvmStatisticsView {
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}

impl JvmStatisticsView {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: JvmStatisticsMapping::new_struct(),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = self.toHashMap();
        let mut node: Option<Rc<RefCell<Node>>> = ModDomHelper::createNameValueNodes(document.clone(), JvmStatisticsData_NAME.with(Clone::clone).clone(), hashMap.clone());
        return node.clone();
    }
}
impl std::ops::Deref for JvmStatisticsView {
    type Target = JvmStatisticsMapping;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for JvmStatisticsView {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::communication::java::vm::stats::JvmStatistics::JvmStatisticsDyn for JvmStatisticsView {
    fn as_JvmStatistics(&self) -> &crate::org::allbinary::logic::communication::java::vm::stats::JvmStatistics::JvmStatistics { &self.base.base }
    fn as_JvmStatistics_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::java::vm::stats::JvmStatistics::JvmStatistics { &mut self.base.base }
}
impl crate::org::allbinary::logic::communication::java::vm::stats::JvmStatisticsMapping::JvmStatisticsMappingDyn for JvmStatisticsView {
    fn as_JvmStatisticsMapping(&self) -> &crate::org::allbinary::logic::communication::java::vm::stats::JvmStatisticsMapping::JvmStatisticsMapping { &self.base }
    fn as_JvmStatisticsMapping_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::java::vm::stats::JvmStatisticsMapping::JvmStatisticsMapping { &mut self.base }
}


