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
// package org.allbinary.input.media.image
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::logic::StdUtil::StdUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InputImageType {
    pub name: String,
    pub index: i32,
}

pub static mut InputImageType_hashMap: std::cell::LazyCell<Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>> = std::cell::LazyCell::new(|| jcall!(StdUtil::getInstance()).createHashMap());
pub static InputImageType_TYPE_VECTOR: crate::JavaStatic<Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>> = crate::JavaStatic::new(|| BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>));
pub static mut InputImageType_CAPTURE: std::cell::LazyCell<Option<Rc<RefCell<InputImageType>>>> = std::cell::LazyCell::new(|| InputImageType::new(String::from("Capture"), 0));
pub static mut InputImageType_COMPARISON: std::cell::LazyCell<Option<Rc<RefCell<InputImageType>>>> = std::cell::LazyCell::new(|| InputImageType::new(String::from("Comparison"), 1));
pub static mut InputImageType_MOTION: std::cell::LazyCell<Option<Rc<RefCell<InputImageType>>>> = std::cell::LazyCell::new(|| InputImageType::new(String::from("Motion"), 2));

impl InputImageType {
    

    pub fn new_struct(mut name: String, mut index: i32) -> Self {
        let mut this = Self {
            name: String::new(),
            index: 0,
        };
        this.name = name.clone();
        this.index = index.clone();
        this
    }

    pub fn new(mut name: String, mut index: i32) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(name.clone(), index.clone()))));
            jcall!(InputImageType_TYPE_VECTOR.with(Clone::clone)).add((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()));
            jcall!(unsafe { InputImageType_hashMap.clone() }).put(crate::java_into_object(jcall!(this).getName()), (|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()));
        this
    }
    

    pub fn toString(&mut self) -> String {
        return format!("{}{}", String::from("ImageType: "), self.getName());
    }
    

    pub fn getName(&mut self) -> String {
        return self.name.clone();
    }
    

    pub fn getInstance(mut imageTypeString: String) -> Option<Rc<RefCell<InputImageType>>> {
        return { let java_object = (jcall!(unsafe { InputImageType_hashMap.clone() }).get(crate::java_into_object(imageTypeString.clone()))).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<InputImageType>>>>().expect("ClassCastException").clone()) };
    }
    

    pub fn getAllAsVector() -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return InputImageType_TYPE_VECTOR.with(Clone::clone).clone();
    }
    

    pub fn getIndex(&mut self) -> i32 {
        return self.index.clone();
    }
}

