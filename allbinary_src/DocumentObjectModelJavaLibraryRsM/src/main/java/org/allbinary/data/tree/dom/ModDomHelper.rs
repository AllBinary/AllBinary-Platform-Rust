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
// package org.allbinary.data.tree.dom
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::java::util::HashMap::HashMap;
use crate::java::util::Set::Set;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::logic::java::anyType::clazz::ClassUtil::ClassUtil;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use crate::java::lang::Exception::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ModDomHelper {
}


impl ModDomHelper {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn createTextNode(mut document: Option<Rc<RefCell<Document>>>, mut name: String, mut text: String) -> Option<Rc<RefCell<Node>>> {
        let mut newValueNode: Option<Rc<RefCell<Node>>> = jcall!(document).createElement(name.clone());
        let mut newValueTextNode: Option<Rc<RefCell<Node>>> = jcall!(document).createTextNode(text.clone());
        jcall!(newValueNode).appendChild(newValueTextNode.clone());
        return newValueNode.clone();
    }
    

    // throws Exception
    pub fn createValueNode(mut document: Option<Rc<RefCell<Document>>>, mut text: String) -> Option<Rc<RefCell<Node>>> {
        return ModDomHelper::createTextNode(document.clone(), DomData_VALUE.with(Clone::clone).clone(), text.clone());
    }
    

    // throws Exception
    pub fn createNameNode(mut document: Option<Rc<RefCell<Document>>>, mut text: String) -> Option<Rc<RefCell<Node>>> {
        return ModDomHelper::createTextNode(document.clone(), DomData_NAME.with(Clone::clone).clone(), text.clone());
    }
    

    // throws Exception
    pub fn createIndexNode(mut document: Option<Rc<RefCell<Document>>>, mut index: i32) -> Option<Rc<RefCell<Node>>> {
        return ModDomHelper::createTextNode(document.clone(), DomData_INDEX.with(Clone::clone).clone(), (Some((index.clone()) as i32)).expect("NullPointerException").to_string());
    }
    

    // throws Exception
    pub fn createNameValueNodes4(mut document: Option<Rc<RefCell<Document>>>, mut elementName: String, mut nameText: String, mut valueText: String) -> Option<Rc<RefCell<Node>>> {
        let mut newNode: Option<Rc<RefCell<Node>>> = jcall!(document).createElement(elementName.clone());
        jcall!(newNode).appendChild(ModDomHelper::createNameNode(document.clone(), nameText.clone()));
        jcall!(newNode).appendChild(ModDomHelper::createValueNode(document.clone(), valueText.clone()));
        return newNode.clone();
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn createNameValueNodes(mut document: Option<Rc<RefCell<Document>>>, mut elementNameAndNameText: String, mut valueText: String) -> Option<Rc<RefCell<Node>>> {
        let mut newNode: Option<Rc<RefCell<Node>>> = ModDomHelper::createNameValueNodes4(document.clone(), elementNameAndNameText.clone(), elementNameAndNameText.clone(), valueText.clone());
        return newNode.clone();
    }
    

    // throws Exception
    pub fn createNameValueIndexNodes(mut document: Option<Rc<RefCell<Document>>>, mut elementNameAndNameText: String, mut index: i32, mut valueText: String) -> Option<Rc<RefCell<Node>>> {
        let mut newNode: Option<Rc<RefCell<Node>>> = ModDomHelper::createNameValueNodes(document.clone(), elementNameAndNameText.clone(), valueText.clone());
        jcall!(newNode).appendChild(ModDomHelper::createIndexNode(document.clone(), index.clone()));
        return newNode.clone();
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn createNameValueNodes_Document_String_HashMap(mut document: Option<Rc<RefCell<Document>>>, mut rootNode: String, mut hashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) -> Option<Rc<RefCell<Node>>> {
        let mut node: Option<Rc<RefCell<Node>>> = jcall!(document).createElement(rootNode.clone());
        return ModDomHelper::createNameValueNodes_Document_Node_HashMap(document.clone(), node.clone(), hashMap.clone());
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn createNameValueNodes_Document_Node_HashMap(mut document: Option<Rc<RefCell<Document>>>, mut node: Option<Rc<RefCell<Node>>>, mut hashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) -> Option<Rc<RefCell<Node>>> {
        let mut stringUtil: Option<Rc<RefCell<StringUtil>>> = StringUtil::getInstance();
        let mut keySet: Option<Rc<RefCell<dyn Set>>> = jcall!(hashMap).keySet();
        let mut keyArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(keySet).toArray();
        let mut size: i32 = (keyArray.len() as i32).clone();
        for i in (0..size) {
            let mut objectKey: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = keyArray[(i) as usize].clone();
            let mut name: String = { let java_object = (objectKey).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
            let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = (jcall!(hashMap).get(objectKey.clone())).clone() /* as Any */;
            if anyType.is_none() {
                let mut value: String = { let java_field = jcall!(stringUtil).EMPTY_STRING.clone(); java_field }.clone();
                jcall!(node).appendChild(ModDomHelper::createNameValueNodes(document.clone(), name.clone(), value.clone()));
            } else if (anyType).is_some() /* instanceof String */ {
                let mut value: String = jcall!(stringUtil).getNonNull({ let java_object = (anyType).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() });
                jcall!(node).appendChild(ModDomHelper::createNameValueNodes(document.clone(), name.clone(), value.clone()));
            } else if (anyType).is_some() /* instanceof HashMap */ {
                jcall!(node).appendChild(ModDomHelper::createNameValueNodes_Document_String_HashMap(document.clone(), name.clone(), hashMap.clone()));
            } else {
                panic!("{:?}", format!("{}: {:?}", "Exception", (format!("{}{}", String::from("HashMap value is the wrong instance and is: "), ClassUtil::viewAll(anyType.clone(), { let java_field = jcall!(CommonSeps::getInstance()).NEW_LINE.clone(); java_field }.clone())))));
            }
        }
        return node.clone();
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn createNameValueNodes4_Document_String_String_BasicArrayList(mut document: Option<Rc<RefCell<Document>>>, mut rootNodeName: String, mut duplicateNodeName: String, mut valueVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Option<Rc<RefCell<Node>>> {
        let mut stringUtil: Option<Rc<RefCell<StringUtil>>> = StringUtil::getInstance();
        let mut node: Option<Rc<RefCell<Node>>> = jcall!(document).createElement(rootNodeName.clone());
        let mut size: i32 = jcall!(valueVector).size();
        for i in (0..size) {
            let mut value: String = jcall!(stringUtil).getNonNull({ let java_object = (jcall!(valueVector).get(i.clone())).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() });
            jcall!(node).appendChild(ModDomHelper::createNameValueNodes(document.clone(), duplicateNodeName.clone(), value.clone()));
        }
        return node.clone();
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn createNameValueIndexNodes_Document_String_String_BasicArrayList(mut document: Option<Rc<RefCell<Document>>>, mut rootNodeName: String, mut duplicateNodeName: String, mut valueVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Option<Rc<RefCell<Node>>> {
        let mut stringUtil: Option<Rc<RefCell<StringUtil>>> = StringUtil::getInstance();
        let mut node: Option<Rc<RefCell<Node>>> = jcall!(document).createElement(rootNodeName.clone());
        let mut size: i32 = jcall!(valueVector).size();
        for index in (0..size) {
            let mut value: String = jcall!(stringUtil).getNonNull({ let java_object = (jcall!(valueVector).get(index.clone())).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() });
            jcall!(node).appendChild(ModDomHelper::createNameValueIndexNodes(document.clone(), duplicateNodeName.clone(), index.clone(), value.clone()));
        }
        return node.clone();
    }
    

    // throws Exception
    pub fn createNodeWithValueNodes(mut document: Option<Rc<RefCell<Document>>>, mut rootNode: String, mut hashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) -> Option<Rc<RefCell<Node>>> {
        let mut node: Option<Rc<RefCell<Node>>> = jcall!(document).createElement(rootNode.clone());
        return ModDomHelper::createNodeWithValueNodes_Document_Node_HashMap(document.clone(), node.clone(), hashMap.clone());
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn createNodeWithValueNodes_Document_Node_HashMap(mut document: Option<Rc<RefCell<Document>>>, mut node: Option<Rc<RefCell<Node>>>, mut hashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) -> Option<Rc<RefCell<Node>>> {
        let mut stringUtil: Option<Rc<RefCell<StringUtil>>> = StringUtil::getInstance();
        let mut keySet: Option<Rc<RefCell<dyn Set>>> = jcall!(hashMap).keySet();
        let mut keyArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(keySet).toArray();
        let mut size: i32 = (keyArray.len() as i32).clone();
        for i in (0..size) {
            let mut name: String = { let java_object = (keyArray[(i) as usize]).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
            let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = (jcall!(hashMap).get(crate::java_into_object(name.clone()))).clone() /* as Any */;
            let mut value: String = jcall!(stringUtil).getNonNull({ let java_object = (anyType).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() });
            jcall!(node).appendChild(ModDomHelper::createTextNode(document.clone(), name.clone(), value.clone()));
        }
        return node.clone();
    }
}

