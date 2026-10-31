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
// package org.allbinary.dom
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::io::ByteArrayOutputStream::ByteArrayOutputStream;
// use crate::java::io::File::File;
// use crate::javax::xml::parsers::DocumentBuilder::DocumentBuilder;
// use crate::javax::xml::parsers::DocumentBuilderFactory::DocumentBuilderFactory;
// use crate::javax::xml::transform::Transformer::Transformer;
// use crate::javax::xml::transform::TransformerFactory::TransformerFactory;
// use crate::javax::xml::transform::dom::DOMSource::DOMSource;
// use crate::javax::xml::transform::stream::StreamResult::StreamResult;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use crate::org::w3c::dom::NodeList::NodeList;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct DomHelper {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

thread_local! { pub static DomHelper_instance: Option<Rc<RefCell<DomHelper>>> = DomHelper::new(); }

impl DomHelper {
    

    pub fn getInstance() -> Option<Rc<RefCell<DomHelper>>> {
        return DomHelper_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn createDocument(&mut self) -> Option<Rc<RefCell<Document>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut factory: Option<Rc<RefCell<DocumentBuilderFactory>>> = DocumentBuilderFactory::newInstance();
            let mut builder: Option<Rc<RefCell<DocumentBuilder>>> = jcall!(factory).newDocumentBuilder();
            let mut document: Option<Rc<RefCell<Document>>> = jcall!(builder).newDocument();
            return document.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            return None;
            }
        }
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn createDocument1(&mut self, mut xmlFile: Option<Rc<RefCell<File>>>) -> Option<Rc<RefCell<Document>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut factory: Option<Rc<RefCell<DocumentBuilderFactory>>> = DocumentBuilderFactory::newInstance();
            let mut builder: Option<Rc<RefCell<DocumentBuilder>>> = jcall!(factory).newDocumentBuilder();
            let mut document: Option<Rc<RefCell<Document>>> = jcall!(builder).parse(xmlFile.clone());
            return document.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            panic!("{:?}", e);
            }
        }
    }
    

    // throws Exception
    pub fn toString(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut domSource: Option<Rc<RefCell<DOMSource>>> = DOMSource::new(document.clone());
            let mut byteArrayOutputStream: Vec<u8> = Vec::new();
            let mut streamResult: Option<Rc<RefCell<StreamResult>>> = StreamResult::new(byteArrayOutputStream.clone());
            let mut copyTransformerFactory: Option<Rc<RefCell<TransformerFactory>>> = TransformerFactory::newInstance();
            let mut copyTransformer: Option<Rc<RefCell<Transformer>>> = jcall!(copyTransformerFactory).newTransformer();
            jcall!(copyTransformer).transform(domSource.clone(), streamResult.clone());
            return String::from_utf8_lossy(&byteArrayOutputStream).into_owned();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            panic!("{:?}", e);
            }
        }
    }
    

    // throws Exception
    pub fn save(&mut self, mut file: Option<Rc<RefCell<File>>>, mut document: Option<Rc<RefCell<Document>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut copyTransformerFactory: Option<Rc<RefCell<TransformerFactory>>> = TransformerFactory::newInstance();
            let mut copyTransformer: Option<Rc<RefCell<Transformer>>> = jcall!(copyTransformerFactory).newTransformer();
            let mut domSource: Option<Rc<RefCell<DOMSource>>> = DOMSource::new(document.clone());
            jcall!(file).createNewFile();
            let mut streamResult: Option<Rc<RefCell<StreamResult>>> = StreamResult::new(file.clone());
            jcall!(copyTransformer).transform(domSource.clone(), streamResult.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            panic!("{:?}", e);
            }
        }
    }
    

    // throws Exception
    pub fn getChildNodeList(&mut self, mut nodeName: String, mut nodeList: Option<Rc<RefCell<NodeList>>>) -> Option<Rc<RefCell<NodeList>>> {
        let mut numberOfNodes: i32 = jcall!(nodeList).getLength();
        for index in (0..numberOfNodes) {
            let mut node: Option<Rc<RefCell<Node>>> = jcall!(nodeList).item(index.clone());
            if jcall!(jcall!(node).getNodeName()).compareTo(nodeName.clone()) == 0 {
                return jcall!(node).getChildNodes();
            }
        }
        panic!("{:?}", format!("{}: {:?}", "Exception", (format!("{}{}", nodeName, String::from(" Node Not Found")))));
    }
    

    // throws Exception
    pub fn getChildrenWithoutTextNodes(&mut self, mut nodeName: String, mut nodeList: Option<Rc<RefCell<NodeList>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut childNodeList: Option<Rc<RefCell<NodeList>>> = self.getChildNodeList(nodeName.clone(), nodeList.clone());
        let mut numberOfChildren: i32 = jcall!(childNodeList).getLength();
        for index in (0..numberOfChildren) {
            let mut node: Option<Rc<RefCell<Node>>> = jcall!(childNodeList).item(index.clone());
            if !match (&jcall!(node).getNodeType(), &unsafe { crate::org::w3c::dom::Node::Node_TEXT_NODE.clone() }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                jcall!(list).add(crate::java_into_object(node.clone()));
            }
        }
        return list.clone();
    }
    

    // throws Exception
    pub fn getWithoutTextNodes(&mut self, mut nodeList: Option<Rc<RefCell<NodeList>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut numberOfChildren: i32 = jcall!(nodeList).getLength();
        for index in (0..numberOfChildren) {
            let mut node: Option<Rc<RefCell<Node>>> = jcall!(nodeList).item(index.clone());
            if !match (&jcall!(node).getNodeType(), &unsafe { crate::org::w3c::dom::Node::Node_TEXT_NODE.clone() }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                jcall!(list).add(crate::java_into_object(node.clone()));
            }
        }
        return list.clone();
    }
    

    // throws Exception
    pub fn searchNodeList(&mut self, mut nodeName: String, mut nodeList: Option<Rc<RefCell<NodeList>>>) -> Option<Rc<RefCell<Node>>> {
        let mut numberOfNodes: i32 = jcall!(nodeList).getLength();
        for index in (0..numberOfNodes) {
            let mut node: Option<Rc<RefCell<Node>>> = jcall!(nodeList).item(index.clone());
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("NodeName: "), jcall!(node).getNodeName()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("searchNodeList"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            if jcall!(jcall!(node).getNodeName()).compareTo(nodeName.clone()) == 0 {
                return node.clone();
            }
        }
        panic!("{:?}", format!("{}: {:?}", "Exception", (format!("{}{}", nodeName, String::from(" Node Not Found in search")))));
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn searchNodeList(&mut self, mut nodeName: String, mut nodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Option<Rc<RefCell<Node>>> {
        let mut numberOfNodes: i32 = jcall!(nodeList).size();
        for index in (0..numberOfNodes) {
            let mut node: Option<Rc<RefCell<Node>>> = (jcall!(nodeList).get(index.clone())).clone() /* as Node */;
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("NodeName: "), jcall!(node).getNodeName()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("searchNodeList"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            if jcall!(jcall!(node).getNodeName()).compareTo(nodeName.clone()) == 0 {
                return node.clone();
            }
        }
        panic!("{:?}", format!("{}: {:?}", "Exception", (format!("{}{}", nodeName, String::from(" Node Not Found in search")))));
    }
}

