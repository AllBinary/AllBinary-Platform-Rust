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
use crate::org::allbinary::data::tree::dom::DomNodeInterface::DomNodeInterface;
use crate::org::allbinary::data::tree::dom::ModDomHelper::ModDomHelper;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use super::BasicItemData::BasicItemData;
use super::ItemInterface::ItemInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicItemView {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub basicItemData: Option<Rc<RefCell<BasicItemData>>>,
    pub itemInterface: Option<Rc<RefCell<dyn ItemInterface>>>,
    pub vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}

impl DomNodeInterface for BasicItemView {
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}

impl BasicItemView {
    

    pub fn new_struct(mut itemInterface: Option<Rc<RefCell<dyn ItemInterface>>>, mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            basicItemData: None,
            itemInterface: None,
            vector: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.basicItemData = BasicItemData::getInstance();
        this.itemInterface = itemInterface.clone();
        this.vector = vector.clone();
        this
    }

    pub fn new(mut itemInterface: Option<Rc<RefCell<dyn ItemInterface>>>, mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(itemInterface, vector))))
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).PRODUCTSEARCHLOGGING.clone()) {
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("toXmlNode"));
        }
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = jcall!(self.itemInterface).toHashMap();
        let mut EMPTY_STRING: String = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        jcall!(hashMap).put(jcall!(self.basicItemData).IMAGE.clone(), EMPTY_STRING.clone());
        let mut stringUtil: Option<Rc<RefCell<StringUtil>>> = StringUtil::getInstance();
        let mut keySet: Option<Rc<RefCell<dyn Set>>> = jcall!(hashMap).keySet();
        let mut node: Option<Rc<RefCell<Node>>> = jcall!(document).createElement(jcall!(self.basicItemData).ITEM.clone());
        let mut nameArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(keySet).toArray();
        let mut size2: i32 = (nameArray.len() as i32).clone();
        for index in (0..size2) {
            let mut name: String = (nameArray[(index) as usize]).clone() /* as String */;
            let mut value: String = (jcall!(hashMap).get(name.clone())).clone() /* as String */;
            value = jcall!(stringUtil).getNonNull(value.clone());
            jcall!(node).appendChild(ModDomHelper::createNameValueNodes(document.clone(), name.clone(), value.clone()));
        }
        let mut totalNode: Option<Rc<RefCell<Node>>> = ModDomHelper::createNameValueNodes(document.clone(), jcall!(self.basicItemData).TOTAL.clone(), jcall!(jcall!(self.itemInterface).getTotal()).toString());
        jcall!(node).appendChild(totalNode.clone());
        let mut size: i32 = jcall!(self.vector).size();
        for index in (0..size) {
            let mut domNodeInterface: Option<Rc<RefCell<dyn DomNodeInterface>>> = { let java_object = jcall!(self.vector).get(index.clone()); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn DomNodeInterface>>>>().expect("ClassCastException").clone()) };
            let mut customNode: Option<Rc<RefCell<Node>>> = jcall!(domNodeInterface).toXmlNode(document.clone());
            jcall!(node).appendChild(customNode.clone());
        }
        return node.clone();
    }
    

    pub fn toXmlDoc(&mut self) -> Option<Rc<RefCell<Document>>> {
        return None;
    }
    

    pub fn view(&mut self) -> String {
        return None;
    }
}

