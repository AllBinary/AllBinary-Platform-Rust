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
// package org.allbinary.logic.visual.dhtml.html
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use super::HtmlTag::HtmlTag;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct HtmlTags {
    pub htmlTagVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}


impl HtmlTags {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            htmlTagVector: None,
        };
        this.htmlTagVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn get(&mut self, mut index: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::visual::dhtml::html::HtmlTag::HtmlTagDyn>>> {
        return { let java_object = (jcall!(self.htmlTagVector).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::logic::visual::dhtml::html::HtmlTag::HtmlTagDyn>>>>().expect("ClassCastException").clone()) };
    }
    

    pub fn add(&mut self, mut htmlTag: Option<Rc<RefCell<dyn crate::org::allbinary::logic::visual::dhtml::html::HtmlTag::HtmlTagDyn>>>) {
        jcall!(self.htmlTagVector).add(crate::java_into_object(htmlTag.clone()));
    }
    

    pub fn size(&mut self) -> i32 {
        return jcall!(self.htmlTagVector).size();
    }
}

