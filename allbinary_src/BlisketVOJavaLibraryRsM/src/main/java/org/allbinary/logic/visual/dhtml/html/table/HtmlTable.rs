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
use crate::org::allbinary::string::CommonPhoneStrings::CommonPhoneStrings;
use super::HtmlRow::HtmlRow;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct HtmlTable {
    pub base: HtmlTag,
    pub END: String,
    pub START: String,
    pub ENDTAG: String,
    pub CLASS: String,
    pub DIRECTION: String,
    pub ID: String,
    pub LANG: String,
    pub UNITS: String,
    pub BORDER: String,
    pub CELLPADDING: String,
    pub CELLSPACING: String,
    pub COLS: String,
    pub FLOAT: String,
    pub FRAME: String,
    pub RULES: String,
    pub STYLE: String,
    pub BORDERCOLLAPSE: String,
    pub numberOfColumns: i32,
    pub before: String,
    pub border: String,
    pub cellpadding: String,
    pub cellspacing: String,
    pub style: String,
    pub name: String,
    pub after: String,
    pub htmlRowsVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}


impl HtmlTable {
    

    pub fn new_struct(mut before: String, mut cellpadding: String, mut cellspacing: String, mut after: String) -> Self {
        let mut this = Self {
            base: HtmlTag::new_struct(),
            END: String::new(),
            START: String::new(),
            ENDTAG: String::new(),
            CLASS: String::new(),
            DIRECTION: String::new(),
            ID: String::new(),
            LANG: String::new(),
            UNITS: String::new(),
            BORDER: String::new(),
            CELLPADDING: String::new(),
            CELLSPACING: String::new(),
            COLS: String::new(),
            FLOAT: String::new(),
            FRAME: String::new(),
            RULES: String::new(),
            STYLE: String::new(),
            BORDERCOLLAPSE: String::new(),
            numberOfColumns: 0,
            before: String::new(),
            border: String::new(),
            cellpadding: String::new(),
            cellspacing: String::new(),
            style: String::new(),
            name: String::new(),
            after: String::new(),
            htmlRowsVector: None,
        };
        this.END = String::from(">");
        this.START = String::from("<table ");
        this.ENDTAG = String::from("</table>");
        this.CLASS = String::from("class");
        this.DIRECTION = String::from("dir");
        this.ID = String::from("id");
        this.LANG = String::from("lang");
        this.UNITS = String::from("units");
        this.BORDER = String::from("border");
        this.CELLPADDING = String::from("cellpadding");
        this.CELLSPACING = String::from("cellspacing");
        this.COLS = String::from("cols");
        this.FLOAT = String::from("float");
        this.FRAME = String::from("frame");
        this.RULES = String::from("rules");
        this.STYLE = String::from("style");
        this.BORDERCOLLAPSE = String::from("border-collapse: collapse");
        this.numberOfColumns = 0;
        this.before = before.clone();
        this.cellpadding = cellpadding.clone();
        this.cellspacing = cellspacing.clone();
        this.after = after.clone();
        this.htmlRowsVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        { let java_arguments = (this.BORDER.clone(), jcall!(CommonPhoneStrings::getInstance()).ZERO.clone(),); this.addAttribute(java_arguments.0, java_arguments.1) };
        this.addAttribute(this.CELLPADDING.clone(), this.cellpadding.clone());
        this.addAttribute(this.CELLSPACING.clone(), this.cellspacing.clone());
        this.addAttribute(this.STYLE.clone(), this.BORDERCOLLAPSE.clone());
        this
    }

    pub fn new(mut before: String, mut cellpadding: String, mut cellspacing: String, mut after: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(before, cellpadding, cellspacing, after))))
    }
    

    pub fn addRow(&mut self, mut htmlRow: Option<Rc<RefCell<HtmlRow>>>) {
        if self.numberOfColumns < jcall!(htmlRow).getNumberOfColumns() {
            self.numberOfColumns = jcall!(htmlRow).getNumberOfColumns();
        }
        jcall!(self.htmlRowsVector).add(crate::java_into_object(htmlRow.clone()));
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringUtil: Option<Rc<RefCell<StringUtil>>> = StringUtil::getInstance();
        let mut result: String = jcall!(stringUtil).EMPTY_STRING.clone();
        let mut attributeKeys: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(jcall!(self.otherAttributes).keySet()).toArray();
        result = self.before.clone();
        result += &self.START.clone();
        result += &String::from(" ");
        for i in (0..(attributeKeys.len() as i32)) {
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
        let mut size: i32 = jcall!(self.htmlRowsVector).size();
        for i in (0..size) {
            result += &jcall!(jcall!(self.htmlRowsVector).get(i.clone())).toString();
            result += &String::from(" ");
        }
        result += &self.ENDTAG.clone();
        result += &self.after.clone();
        return result.clone();
    }
}
impl std::ops::Deref for HtmlTable {
    type Target = HtmlTag;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for HtmlTable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::visual::dhtml::html::HtmlTag::HtmlTagDyn for HtmlTable {
    fn as_HtmlTag(&self) -> &crate::org::allbinary::logic::visual::dhtml::html::HtmlTag::HtmlTag { &self.base }
    fn as_HtmlTag_mut(&mut self) -> &mut crate::org::allbinary::logic::visual::dhtml::html::HtmlTag::HtmlTag { &mut self.base }
}


