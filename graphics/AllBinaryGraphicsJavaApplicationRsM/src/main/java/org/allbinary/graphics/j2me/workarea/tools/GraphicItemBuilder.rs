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
// package org.allbinary.graphics.j2me.workarea.tools
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::w3c::dom::Node::Node;
use super::LinesGraphicItem::LinesGraphicItem;
use super::GraphicItemInterface::GraphicItemInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GraphicItemBuilder {
}


impl GraphicItemBuilder {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn getInstance(mut node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<dyn GraphicItemInterface>>> {
        let mut graphicItemName: String = jcall!(node).getNodeName();
        if (graphicItemName.cmp(&jcall!(PointsDomUtil::PointsDomUtil::getInstance()).LINES.clone()) as i32) == 0 {
            return LinesGraphicItem::new1(node.clone()).map(|reference| reference as Rc<RefCell<dyn GraphicItemInterface>>);
        } else {
            panic!("{:?}", format!("{}: {:?}", "Exception", (format!("{}{}", String::from("Graphic Item Does Not Exist: "), graphicItemName))));
        }
    }
}

