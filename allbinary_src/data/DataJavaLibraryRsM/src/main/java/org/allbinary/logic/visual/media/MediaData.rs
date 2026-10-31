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
// package org.allbinary.logic.visual.media
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::logic::StdUtil::StdUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MediaData {
    pub name: String,
    pub r#type: String,
    pub extension: String,
}

pub static mut MediaData_hashMap: std::cell::LazyCell<Option<Rc<RefCell<HashMap>>>> = std::cell::LazyCell::new(|| jcall!(StdUtil::getInstance()).createHashMap());

impl MediaData {
    

    pub fn new_struct(mut a_Name: String) -> Self {
        let mut this = Self {
            name: String::new(),
            r#type: String::new(),
            extension: String::new(),
        };
        this.name = a_Name.clone();
        this.extension = format!("{}{}", String::from("."), this.name);
        this.r#type = jcall!(MediaTypeData::MediaTypeData::getInstance()).NOT_RESIZABLE_MEDIA.clone();
        this
    }

    pub fn new(mut a_Name: String) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(a_Name))));
            jcall!(unsafe { MediaData_hashMap.clone() }).put(a_Name.clone(), (|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()));
        this
    }
    

    pub fn getName(&mut self) -> String {
        return self.name.clone();
    }
    

    pub fn getType(&mut self) -> String {
        return self.r#type.clone();
    }
    

    pub fn getExtension(&mut self) -> String {
        return self.extension.clone();
    }
    

    pub fn isConvertableTo(&mut self, mut mediaData: Option<Rc<RefCell<MediaData>>>) -> bool {
        if match &(jcall!(MediaDataFactory::MediaDataFactory::getInstance()).GIF) { Some(reference) => std::ptr::addr_eq(&*self as *const Self, reference.as_ptr()), None => false } && match (&mediaData, &jcall!(MediaDataFactory::MediaDataFactory::getInstance()).JPG) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return true;
        }
        return false;
    }
    

    pub fn toString(&mut self) -> String {
        return self.getName();
    }
    

    // throws Exception
    pub fn get(mut a_Name: String) -> Option<Rc<RefCell<MediaData>>> {
        let mut mediaData: Option<Rc<RefCell<MediaData>>> = (jcall!(unsafe { MediaData_hashMap.clone() }).get(a_Name.clone())).clone() /* as MediaData */;
        if mediaData.is_some() {
            return mediaData.clone();
        } else {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("No Such MediaData"))));
        }
    }
    

    pub fn getDefault() -> Option<Rc<RefCell<MediaData>>> {
        return jcall!(MediaDataFactory::MediaDataFactory::getInstance()).JPG.clone();
    }
    

    pub fn toHashMap() -> Option<Rc<RefCell<HashMap>>> {
        return unsafe { MediaData_hashMap.clone() }.clone();
    }
}

