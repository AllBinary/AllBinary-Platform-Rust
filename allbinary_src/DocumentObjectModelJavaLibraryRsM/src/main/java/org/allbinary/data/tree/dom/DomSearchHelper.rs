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
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::w3c::dom::Node::Node;
use crate::org::w3c::dom::NodeList::NodeList;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct DomSearchHelper {
}


impl DomSearchHelper {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getNodeNoThrow(mut nodeName: String, mut nodeList: Option<Rc<RefCell<NodeList>>>) -> Option<Rc<RefCell<Node>>> {
        let mut numberOfNodes: i32 = jcall!(nodeList).getLength();
        for index in (0..numberOfNodes) {
            let mut node: Option<Rc<RefCell<Node>>> = jcall!(nodeList).item(index.clone());
            if jcall!(jcall!(node).getNodeName()).compareTo(nodeName.clone()) == 0 {
                return node.clone();
            }
        }
        return None;
    }
    

    // throws Exception
    pub fn getNode(mut nodeName: String, mut nodeList: Option<Rc<RefCell<NodeList>>>) -> Option<Rc<RefCell<Node>>> {
        let mut node: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNodeNoThrow(nodeName.clone(), nodeList.clone());
        if node.is_some() {
            return node.clone();
        } else {
            panic!("{:?}", format!("{}: {:?}", "Exception", (format!("{}{}", nodeName, String::from(" Node Not Found in search")))));
        }
    }
    

    // throws Exception
    pub fn getChildNodeList(mut nodeName: String, mut nodeList: Option<Rc<RefCell<NodeList>>>) -> Option<Rc<RefCell<NodeList>>> {
        let mut node: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(nodeName.clone(), nodeList.clone());
        return jcall!(node).getChildNodes();
    }
    

    // throws Exception
    pub fn getAllNodes(mut nodeName: String, mut nodeList: Option<Rc<RefCell<NodeList>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = DomSearchHelper::getAllNodesNoThrow(nodeName.clone(), nodeList.clone());
        if vector.is_none() {
            panic!("{:?}", format!("{}: {:?}", "Exception", (format!("{}{}", nodeName, String::from(" Node Not Found in search")))));
        } else {
            return vector.clone();
        }
    }
    

    pub fn getAllNodesNoThrow(mut nodeName: String, mut nodeList: Option<Rc<RefCell<NodeList>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut nodeVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut numberOfNodes: i32 = jcall!(nodeList).getLength();
        for index in (0..numberOfNodes) {
            let mut node: Option<Rc<RefCell<Node>>> = jcall!(nodeList).item(index.clone());
            if jcall!(jcall!(node).getNodeName()).compareTo(nodeName.clone()) == 0 {
                jcall!(nodeVector).add(crate::java_into_object(node.clone()));
            }
        }
        return nodeVector.clone();
    }
}

