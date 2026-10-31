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
// package org.allbinary.graphics.j2me.workarea
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::dom::DomHelper::DomHelper;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use crate::org::w3c::dom::NodeList::NodeList;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct WorkAreaDom {
    pub name: String,
    pub canvasNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}

thread_local! { pub static WorkAreaDom_WORKAREA: String = String::from("workArea"); }
thread_local! { pub static WorkAreaDom_NAME: String = String::from("name"); }
thread_local! { pub static WorkAreaDom_FRAMES: String = String::from("frames"); }

impl WorkAreaDom {
    

    pub fn new_struct(mut document: Option<Rc<RefCell<Document>>>) -> Self {
        let mut this = Self {
            name: String::new(),
            canvasNodeList: None,
        };
        let mut workAreaNodeList: Option<Rc<RefCell<NodeList>>> = jcall!(document).getElementsByTagName(WorkAreaDom_WORKAREA.with(Clone::clone).clone());
        if workAreaNodeList.is_some() {
            let mut numberOfworkAreas: i32 = jcall!(workAreaNodeList).getLength();
            if numberOfworkAreas == 1 {
                let mut numberOfNodes: i32 = jcall!(workAreaNodeList).getLength();
                let mut nameNode: Option<Rc<RefCell<Node>>> = jcall!(DomHelper::getInstance()).searchNodeList(WorkAreaDom_NAME.with(Clone::clone).clone(), jcall!(jcall!(workAreaNodeList).item(0)).getChildNodes());
                let mut nameTextNode: Option<Rc<RefCell<Node>>> = jcall!(nameNode).getFirstChild();
                this.name = jcall!(nameTextNode).getNodeValue();
            } else {
                panic!("{:?}", format!("{}: {:?}", "Exception", (format!("{}{}", String::from("Wrong Number of WorkAreas: "), numberOfworkAreas))));
            }
            this.canvasNodeList = jcall!(DomHelper::getInstance()).getChildrenWithoutTextNodes(WorkAreaDom_FRAMES.with(Clone::clone).clone(), jcall!(jcall!(workAreaNodeList).item(0)).getChildNodes());
        } else {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("workArea Node Not Found"))));
        }
        this
    }

    pub fn new(mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(document))))
    }
    

    pub fn getName(&mut self) -> String {
        return self.name.clone();
    }
    

    pub fn getCanvasNodes(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.canvasNodeList.clone();
    }
}

