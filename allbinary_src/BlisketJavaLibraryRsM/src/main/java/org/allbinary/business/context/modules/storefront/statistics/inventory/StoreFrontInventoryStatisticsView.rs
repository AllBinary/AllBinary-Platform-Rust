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
// package org.allbinary.business.context.modules.storefront.statistics.inventory
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
use super::StoreFrontInventoryStatisticsInterface::StoreFrontInventoryStatisticsInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct StoreFrontInventoryStatisticsView {
    pub storeFrontInventoryStatisticsInterface: Option<Rc<RefCell<dyn StoreFrontInventoryStatisticsInterface>>>,
}

impl DomNodeInterface for StoreFrontInventoryStatisticsView {
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}

impl StoreFrontInventoryStatisticsView {
    

    pub fn new_struct(mut storeFrontInventoryStatisticsInterface: Option<Rc<RefCell<dyn StoreFrontInventoryStatisticsInterface>>>) -> Self {
        let mut this = Self {
            storeFrontInventoryStatisticsInterface: None,
        };
        this.storeFrontInventoryStatisticsInterface = storeFrontInventoryStatisticsInterface.clone();
        this
    }

    pub fn new(mut storeFrontInventoryStatisticsInterface: Option<Rc<RefCell<dyn StoreFrontInventoryStatisticsInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(storeFrontInventoryStatisticsInterface))))
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = jcall!(self.storeFrontInventoryStatisticsInterface).toHashMap();
        let mut node: Option<Rc<RefCell<Node>>> = ModDomHelper::createNameValueNodes(document.clone(), jcall!(StoreFrontInventoryStatisticsData::StoreFrontInventoryStatisticsData::getInstance()).NAME.clone(), hashMap.clone());
        return node.clone();
    }
}

