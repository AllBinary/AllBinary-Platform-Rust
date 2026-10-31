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
// package admin.tags
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::servlet::jsp::tagext::Tag::Tag;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use super::HelperTag::HelperTag;
use super::tags::tags;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ParentInventoryTagHelper {
}

thread_local! { pub static ParentInventoryTagHelper_instance: Option<Rc<RefCell<ParentInventoryTagHelper>>> = ParentInventoryTagHelper::new(); }

impl ParentInventoryTagHelper {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<ParentInventoryTagHelper>>> {
        return ParentInventoryTagHelper_instance.with(Clone::clone).clone();
    }
    

    // throws Exception
    pub fn isValid(&mut self, mut tagClass: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut parentTag: Option<Rc<RefCell<Tag>>>) {
        if parentTag.is_none() {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Must have parent tag."))));
        } else if !((parentTag).is_some() /* instanceof HelperTag */) {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(String::from("Must have at least a "));
            jcall!(stringBuffer).append(String::from("tags.HelperTag"));
            jcall!(stringBuffer).append(String::from(" as parent"));
            jcall!(stringBuffer).append(String::from("and not: "));
            jcall!(stringBuffer).append(std::any::type_name_of_val(&parentTag).to_string());
            panic!("{:?}", format!("{}: {:?}", "Exception", (jcall!(stringBuffer).toString())));
        }
    }
}

