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
// package org.allbinary.business.context.modules.storefront.statistics.advertisements
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
use super::StoreFrontAdvertisementsStatisticsInterface::StoreFrontAdvertisementsStatisticsInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct StoreFrontAdvertisementsStatisticsView {
    pub storeFrontAdvertisementsStatisticsInterface: Option<Rc<RefCell<dyn StoreFrontAdvertisementsStatisticsInterface>>>,
}

impl DomNodeInterface for StoreFrontAdvertisementsStatisticsView {
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}

impl StoreFrontAdvertisementsStatisticsView {
    

    pub fn new_struct(mut storeFrontAdvertisementsStatisticsInterface: Option<Rc<RefCell<dyn StoreFrontAdvertisementsStatisticsInterface>>>) -> Self {
        let mut this = Self {
            storeFrontAdvertisementsStatisticsInterface: None,
        };
        this.storeFrontAdvertisementsStatisticsInterface = storeFrontAdvertisementsStatisticsInterface.clone();
        this
    }

    pub fn new(mut storeFrontAdvertisementsStatisticsInterface: Option<Rc<RefCell<dyn StoreFrontAdvertisementsStatisticsInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(storeFrontAdvertisementsStatisticsInterface))))
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = jcall!(self.storeFrontAdvertisementsStatisticsInterface).toHashMap();
        let mut node: Option<Rc<RefCell<Node>>> = ModDomHelper::createNameValueNodes(document.clone(), jcall!(StoreFrontAdvertisementsStatisticsData::StoreFrontAdvertisementsStatisticsData::getInstance()).NAME.clone(), hashMap.clone());
        return node.clone();
    }
}

