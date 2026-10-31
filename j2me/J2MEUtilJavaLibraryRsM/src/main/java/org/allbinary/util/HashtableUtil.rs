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
// package org.allbinary.util
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use crate::Enumeration;
use super::*;
// use crate::java::util::Enumeration::Enumeration;
use crate::jsinterop::annotations::JsType;
use super::EnumerationUtil::EnumerationUtil;
use super::ABHashtable::ABHashtable;
use super::BasicArrayListS::BasicArrayListS;
use super::BasicArrayList::BasicArrayList;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct HashtableUtil {
    pub enumerationUtil: Option<Rc<RefCell<EnumerationUtil>>>,
}

thread_local! { pub static HashtableUtil_instance: Option<Rc<RefCell<HashtableUtil>>> = HashtableUtil::new(); }

impl HashtableUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            enumerationUtil: None,
        };
        this.enumerationUtil = EnumerationUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<HashtableUtil>>> {
        return HashtableUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn putAll(&mut self, mut fromHashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>, mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>) {
        let mut enumeration: Option<Rc<RefCell<dyn Enumeration>>> = jcall!(fromHashtable).keys();
        let mut keyCanBeNull: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = None;
        let mut valueCanBeNull: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = None;
        while jcall!(self.enumerationUtil).hasMoreElements(enumeration.clone()) {
            keyCanBeNull = jcall!(self.enumerationUtil).nextElement(enumeration.clone());
            valueCanBeNull = jcall!(hashtable).get(keyCanBeNull.clone());
            if valueCanBeNull.is_none() {
            } else {
                jcall!(hashtable).put(keyCanBeNull.clone(), valueCanBeNull.clone());
            }
        }
    }
    

    pub fn getKeysAsArray(&mut self, mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>) -> Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> {
        let mut objectArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = vec![None; (jcall!(hashtable).size()) as usize];
        let mut index: i32 = 0;
        let mut enumeration: Option<Rc<RefCell<dyn Enumeration>>> = jcall!(hashtable).keys();
        while jcall!(self.enumerationUtil).hasMoreElements(enumeration.clone()) {
            objectArray[({ let value = index; index += 1; value }) as usize] = jcall!(self.enumerationUtil).nextElement(enumeration.clone());
        }
        return objectArray.clone();
    }
    

    pub fn getKeysAsList(&mut self, mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListS::new(jcall!(hashtable).size()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut enumeration: Option<Rc<RefCell<dyn Enumeration>>> = jcall!(hashtable).keys();
        while jcall!(self.enumerationUtil).hasMoreElements(enumeration.clone()) {
            jcall!(list).add(crate::java_into_object(jcall!(self.enumerationUtil).nextElement(enumeration.clone())));
        }
        return list.clone();
    }
}

