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
// package org.allbinary.logic.util.cache
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::jsinterop::annotations::JsType;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::jsinterop::annotations::JsMethod;
use crate::jsinterop::annotations::JsProperty;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BaseBasicArrayListCache {
    pub list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}


impl BaseBasicArrayListCache {

    pub fn new_struct() -> Self {
        let mut this = Self {
            list: None,
        };
        this.list = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn log(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(String::from("S: "));
        jcall!(stringBuffer).appendint(jcall!(self.list).size());
        return jcall!(stringBuffer).toString();
    }
}
pub trait BaseBasicArrayListCacheDyn {
    fn as_BaseBasicArrayListCache(&self) -> &BaseBasicArrayListCache;
    fn as_BaseBasicArrayListCache_mut(&mut self) -> &mut BaseBasicArrayListCache;
    fn log(&mut self) -> String { BaseBasicArrayListCache::log(self.as_BaseBasicArrayListCache_mut()) }
}
impl BaseBasicArrayListCacheDyn for BaseBasicArrayListCache {
    fn as_BaseBasicArrayListCache(&self) -> &BaseBasicArrayListCache { self }
    fn as_BaseBasicArrayListCache_mut(&mut self) -> &mut BaseBasicArrayListCache { self }
}


