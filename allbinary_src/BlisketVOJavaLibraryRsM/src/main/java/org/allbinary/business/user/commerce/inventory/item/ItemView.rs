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
// package org.allbinary.business.user.commerce.inventory.item
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
// use crate::java::util::Set::Set;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::business::user::address::StreetAddressData::StreetAddressData;
use crate::org::allbinary::data::tree::dom::ModDomHelper::ModDomHelper;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use super::OrderItemInterface::OrderItemInterface;
use super::BasicItemView::BasicItemView;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ItemView {
    pub itemInterface: Option<Rc<RefCell<dyn OrderItemInterface>>>,
    pub vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}


impl ItemView {
    

    pub fn new_struct(mut itemInterface: Option<Rc<RefCell<dyn OrderItemInterface>>>, mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Self {
        let mut this = Self {
            itemInterface: None,
            vector: None,
        };
        this.itemInterface = itemInterface.clone();
        this.vector = vector.clone();
        this
    }

    pub fn new(mut itemInterface: Option<Rc<RefCell<dyn OrderItemInterface>>>, mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(itemInterface, vector))))
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = jcall!(self.itemInterface).toHashMap();
        let mut keySet: Option<Rc<RefCell<dyn Set>>> = jcall!(hashMap).keySet();
        let mut node: Option<Rc<RefCell<Node>>> = jcall!(document).createElement(ItemData_ITEM.with(Clone::clone).clone());
        let mut basicItemView: Option<Rc<RefCell<BasicItemView>>> = BasicItemView::new(self.itemInterface.clone(), self.vector.clone());
        jcall!(node).appendChild(jcall!(basicItemView).toXmlNode(document.clone()));
        let mut stringUtil: Option<Rc<RefCell<StringUtil>>> = StringUtil::getInstance();
        let mut nameArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(keySet).toArray();
        let mut size: i32 = (nameArray.len() as i32).clone();
        for index in (0..size) {
            let mut name: String = (nameArray[(index) as usize]).clone() /* as String */;
            let mut value: String = (jcall!(hashMap).get(name.clone())).clone() /* as String */;
            value = jcall!(stringUtil).getNonNull(value.clone());
            jcall!(node).appendChild(ModDomHelper::createNameValueNodes(document.clone(), name.clone(), value.clone()));
        }
        let mut shippingAddressNode: Option<Rc<RefCell<Node>>> = jcall!(document).createElement(crate::org::allbinary::business::user::address::StreetAddressData::StreetAddressData_NAME.with(Clone::clone).clone());
        jcall!(shippingAddressNode).appendChild(jcall!(jcall!(self.itemInterface).getShippingAddress()).toXmlNode(document.clone()));
        return node.clone();
    }
}

