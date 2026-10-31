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
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::w3c::dom::Node::Node;
use crate::org::w3c::dom::NodeList::NodeList;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct DomNodeHelper {
}


impl DomNodeHelper {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getFirstChildElement(mut parentNode: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
        let mut node: Option<Rc<RefCell<Node>>> = jcall!(parentNode).getFirstChild();
        if node.is_some() && !match (&jcall!(node).getNodeType(), &unsafe { crate::org::w3c::dom::Node::Node_ELEMENT_NODE.clone() }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            let mut nodeList: Option<Rc<RefCell<NodeList>>> = jcall!(parentNode).getChildNodes();
            for index in (0..jcall!(nodeList).getLength()) {
                node = jcall!(nodeList).item(index.clone());
                if jcall!(node).getNodeType() == 1 {
                    break;
                }
            }
        }
        return node.clone();
    }
    

    pub fn getTextNodeValue(mut node: Option<Rc<RefCell<Node>>>) -> String {
        let mut nodeTextNode: Option<Rc<RefCell<Node>>> = jcall!(node).getFirstChild();
        if nodeTextNode.is_some() {
            return jcall!(nodeTextNode).getNodeValue();
        } else {
            return jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        }
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn getTextNodeValue2(mut nodeName: String, mut nodeList: Option<Rc<RefCell<NodeList>>>) -> String {
        let mut node: Option<Rc<RefCell<Node>>> = DomSearchHelper::DomSearchHelper::getNode(nodeName.clone(), nodeList.clone());
        return DomNodeHelper::getTextNodeValue(node.clone());
    }
    

    pub fn getTextNodesValue(mut node: Option<Rc<RefCell<Node>>>) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut nodeList: Option<Rc<RefCell<NodeList>>> = jcall!(node).getChildNodes();
        for index in (0..jcall!(nodeList).getLength()) {
            let mut nodeTextNode: Option<Rc<RefCell<Node>>> = jcall!(nodeList).item(index.clone());
            if match (&jcall!(nodeTextNode).getNodeType(), &unsafe { crate::org::w3c::dom::Node::Node_TEXT_NODE.clone() }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                if nodeTextNode.is_some() {
                    jcall!(stringBuffer).append(jcall!(nodeTextNode).getNodeValue());
                }
            }
        }
        return jcall!(stringBuffer).toString();
    }
}

