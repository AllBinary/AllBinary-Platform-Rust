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
// package org.allbinary.business.category
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::data::tree::dom::DomNodeHelper::DomNodeHelper;
use crate::org::allbinary::data::tree::dom::DomSearchHelper::DomSearchHelper;
use crate::org::allbinary::logic::io::path::AbPath::AbPath;
use crate::org::allbinary::logic::io::path::AbPathData::AbPathData;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::apache::commons::lang3::StringUtils::StringUtils;
use crate::org::w3c::dom::Attr::Attr;
use crate::org::w3c::dom::NamedNodeMap::NamedNodeMap;
use crate::org::w3c::dom::Node::Node;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CategoryUtil {
}


impl CategoryUtil {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // synchronized
    pub fn getPathLevel(mut categoryPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>) -> i32 {
        let mut count: i32 = StringUtils::countMatches(jcall!(categoryPath).toString(), jcall!(AbPathData::getInstance()).SEPARATOR.clone());
        return count.clone();
    }
    

    // synchronized
    pub fn getNameFromNode(mut node: Option<Rc<RefCell<Node>>>) -> String {
        let mut attributes: Option<Rc<RefCell<NamedNodeMap>>> = jcall!(node).getAttributes();
        let mut attrNode: Option<Rc<RefCell<Attr>>> = (jcall!(attributes).getNamedItem(jcall!(CategoryData::CategoryData::getInstance()).LABEL.clone())).clone() /* as Attr */;
        if attrNode.is_some() {
            return jcall!(attrNode).getValue();
        } else {
            let mut labelNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNodeNoThrow(jcall!(CategoryData::CategoryData::getInstance()).LABEL.clone(), jcall!(node).getChildNodes());
            if labelNode.is_some() {
                return DomNodeHelper::getTextNodeValue(labelNode.clone());
            }
        }
        return jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
    }
}

