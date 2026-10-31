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
// package org.allbinary.data.tree.dom.document
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::string::regex::replace::Replace::Replace;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct DocumentToNode {
}

thread_local! { pub static DocumentToNode_XMLROOT: String = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"); }
thread_local! { pub static DocumentToNode_XMLROOTSTART: String = String::from("<?xml"); }
thread_local! { pub static DocumentToNode_XMLROOTEND: String = String::from("?>"); }
thread_local! { pub static DocumentToNode_XMLJSPROOTSTART: String = String::from("<jsp:root"); }
thread_local! { pub static DocumentToNode_XMLEND: String = String::from(">"); }
thread_local! { pub static DocumentToNode_XMLJSPROOTENDSTART: String = String::from("</jsp:root"); }

impl DocumentToNode {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn convertDocumentToNodeString(mut documentString: String) -> String {
        documentString = DocumentToNode::remove(documentString.clone(), DocumentToNode_XMLJSPROOTSTART.with(Clone::clone).clone(), DocumentToNode_XMLEND.with(Clone::clone).clone());
        documentString = DocumentToNode::remove(documentString.clone(), DocumentToNode_XMLJSPROOTENDSTART.with(Clone::clone).clone(), DocumentToNode_XMLEND.with(Clone::clone).clone());
        let mut replaceXmlRoot: Option<Rc<RefCell<Replace>>> = Replace::new2(DocumentToNode_XMLROOT.with(Clone::clone).clone(), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone());
        return jcall!(replaceXmlRoot).all(documentString.clone());
    }
    

    pub fn removeXmlDefinitionFromDocumentString(mut documentString: String) -> String {
        return DocumentToNode::remove(documentString.clone(), DocumentToNode_XMLROOTSTART.with(Clone::clone).clone(), DocumentToNode_XMLROOTEND.with(Clone::clone).clone());
    }
    

    pub fn remove(mut documentString: String, mut start: String, mut end: String) -> String {
        let mut startIndex: i32 = 0;
        while ({ startIndex = documentString.find(&start.clone()).map(|index| index as i32).unwrap_or(-1); startIndex.clone() }) != -1 {
            if startIndex >= 0 {
                let mut endIndex: i32 = jcall!(documentString).indexOf(end.clone(), startIndex.clone());
                if endIndex >= 0 {
                    let mut startString: String = documentString[(0) as usize..(startIndex) as usize].to_string();
                    let mut resultString: String = documentString[(endIndex + 1) as usize..].to_string();
                    return format!("{}{}", startString, resultString);
                }
            }
        }
        return documentString.clone();
    }
}

