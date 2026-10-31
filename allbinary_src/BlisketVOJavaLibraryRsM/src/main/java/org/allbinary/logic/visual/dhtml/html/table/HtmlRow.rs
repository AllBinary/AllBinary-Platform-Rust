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
// package org.allbinary.logic.visual.dhtml.html.table
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::visual::dhtml::html::HtmlTag::HtmlTag;
use super::HtmlCell::HtmlCell;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct HtmlRow {
    pub base: HtmlTag,
    pub END: String,
    pub START: String,
    pub ENDTAG: String,
    pub numberOfColumns: i32,
    pub before: String,
    pub after: String,
    pub htmlCellsVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}


impl HtmlRow {
    

    pub fn new_struct(mut before: String, mut after: String) -> Self {
        let mut this = Self {
            base: HtmlTag::new_struct(),
            END: String::new(),
            START: String::new(),
            ENDTAG: String::new(),
            numberOfColumns: 0,
            before: String::new(),
            after: String::new(),
            htmlCellsVector: None,
        };
        this.END = String::from(">");
        this.START = String::from("<tr ");
        this.ENDTAG = String::from("</tr>");
        this.numberOfColumns = 0;
        this.before = before.clone().clone();
        this.after = after.clone().clone();
        this.htmlCellsVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this
    }

    pub fn new(mut before: String, mut after: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(before, after))))
    }
    

    pub fn addCell(&mut self, mut htmlCell: Option<Rc<RefCell<HtmlCell>>>) {
        jcall!(self.htmlCellsVector).add(crate::java_into_object(htmlCell.clone()));
        self.numberOfColumns += 1;
    }
    

    pub fn getNumberOfColumns(&mut self) -> i32 {
        return self.numberOfColumns.clone();
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringUtil: Option<Rc<RefCell<StringUtil>>> = StringUtil::getInstance();
        let mut result: String = jcall!(stringUtil).EMPTY_STRING.clone();
        let mut attributeKeys: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(jcall!(self.otherAttributes).keySet()).toArray();
        let mut attributeSize: i32 = (attributeKeys.len() as i32).clone();
        let mut cellSize: i32 = jcall!(self.htmlCellsVector).size();
        result = self.before.clone();
        result += &self.START.clone();
        result += &String::from(" ");
        for i in (0..attributeSize) {
            let mut key: String = (attributeKeys[(i) as usize]).clone() /* as String */;
            let mut value: String = (jcall!(self.base.otherAttributes).get(key.clone())).clone() /* as String */;
            if true && (value.cmp(&jcall!(stringUtil).EMPTY_STRING.clone()) as i32) != 0 {
                result += &key.clone();
                result += &String::from("=\"");
                result += &value.clone();
                result += &String::from("\" ");
            }
        }
        result += &self.END.clone();
        for i in (0..cellSize) {
            result += &jcall!(jcall!(self.htmlCellsVector).get(i.clone())).toString();
            result += &String::from(" ");
        }
        result += &self.ENDTAG.clone();
        result += &self.after.clone();
        return result.clone();
    }
}
impl std::ops::Deref for HtmlRow {
    type Target = HtmlTag;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for HtmlRow {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::visual::dhtml::html::HtmlTag::HtmlTagDyn for HtmlRow {
    fn as_HtmlTag(&self) -> &crate::org::allbinary::logic::visual::dhtml::html::HtmlTag::HtmlTag { &self.base }
    fn as_HtmlTag_mut(&mut self) -> &mut crate::org::allbinary::logic::visual::dhtml::html::HtmlTag::HtmlTag { &mut self.base }
}


