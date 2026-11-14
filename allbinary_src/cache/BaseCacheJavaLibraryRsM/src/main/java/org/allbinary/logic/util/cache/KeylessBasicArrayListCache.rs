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
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListUtil::BasicArrayListUtil;
use crate::java::lang::Exception::*;
use super::IndexedBasicArrayListCache::IndexedBasicArrayListCache;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct KeylessBasicArrayListCache {
    pub base: IndexedBasicArrayListCache,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub basicArrayListUtil: Option<Rc<RefCell<BasicArrayListUtil>>>,
}


impl KeylessBasicArrayListCache {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: IndexedBasicArrayListCache::new_struct(),
            logUtil: None,
            basicArrayListUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.basicArrayListUtil = BasicArrayListUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn get(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if self.index >= jcall!(self.list).size() {
                self.addDefault();
            }
            let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = { let java_object = ({ let java_arguments = ({ let value = self.index; self.index += 1; value },); self.getAt(java_arguments.0) }).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>>().expect("ClassCastException").clone()) };
            return Some(list.clone());
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).GET.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            return jcall!(self.basicArrayListUtil).getImmutableInstance();
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    pub fn clear(&mut self) {
        self.base.clear();
        let mut basicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = None;
        for index in (0..={ let java_bound = jcall!(self.list).size() - 1; java_bound }).rev() {
            basicArrayList = { let java_value = (({ let java_object = (jcall!(self.list).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>>().expect("ClassCastException").clone()) })).clone(); java_value };
            jcall!(basicArrayList).clear();
        }
    }
    

    pub fn log(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(String::from("S: "));
        jcall!(stringBuffer).appendint(jcall!(self.list).size());
        let mut S_LABEL: String = String::from(" s: ");
        let mut basicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = None;
        for index in (0..={ let java_bound = jcall!(self.list).size() - 1; java_bound }).rev() {
            jcall!(stringBuffer).append(S_LABEL.clone());
            basicArrayList = { let java_value = (({ let java_object = (jcall!(self.list).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>>().expect("ClassCastException").clone()) })).clone(); java_value };
            jcall!(stringBuffer).appendint(jcall!(basicArrayList).size());
        }
        return jcall!(stringBuffer).toString();
    }
}
impl std::ops::Deref for KeylessBasicArrayListCache {
    type Target = IndexedBasicArrayListCache;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for KeylessBasicArrayListCache {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait KeylessBasicArrayListCacheDyn: crate::org::allbinary::logic::util::cache::IndexedBasicArrayListCache::IndexedBasicArrayListCacheDyn {
    fn as_KeylessBasicArrayListCache(&self) -> &KeylessBasicArrayListCache;
    fn as_KeylessBasicArrayListCache_mut(&mut self) -> &mut KeylessBasicArrayListCache;
    fn get(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { KeylessBasicArrayListCache::get(self.as_KeylessBasicArrayListCache_mut()) }
}
impl KeylessBasicArrayListCacheDyn for KeylessBasicArrayListCache {
    fn as_KeylessBasicArrayListCache(&self) -> &KeylessBasicArrayListCache { self }
    fn as_KeylessBasicArrayListCache_mut(&mut self) -> &mut KeylessBasicArrayListCache { self }
}

impl crate::org::allbinary::logic::util::cache::BaseBasicArrayListCache::BaseBasicArrayListCacheDyn for KeylessBasicArrayListCache {
    fn as_BaseBasicArrayListCache(&self) -> &crate::org::allbinary::logic::util::cache::BaseBasicArrayListCache::BaseBasicArrayListCache { &self.base.base }
    fn as_BaseBasicArrayListCache_mut(&mut self) -> &mut crate::org::allbinary::logic::util::cache::BaseBasicArrayListCache::BaseBasicArrayListCache { &mut self.base.base }
    fn log(&mut self) -> String { Self::log(self) }
}
impl crate::org::allbinary::logic::util::cache::IndexedBasicArrayListCache::IndexedBasicArrayListCacheDyn for KeylessBasicArrayListCache {
    fn as_IndexedBasicArrayListCache(&self) -> &crate::org::allbinary::logic::util::cache::IndexedBasicArrayListCache::IndexedBasicArrayListCache { &self.base }
    fn as_IndexedBasicArrayListCache_mut(&mut self) -> &mut crate::org::allbinary::logic::util::cache::IndexedBasicArrayListCache::IndexedBasicArrayListCache { &mut self.base }
}
impl crate::org::allbinary::logic::util::cache::CacheInterface::CacheInterface for KeylessBasicArrayListCache {
    fn add(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::cache::CacheableInterface::CacheableInterface>>>) { crate::org::allbinary::logic::util::cache::IndexedBasicArrayListCache::IndexedBasicArrayListCache::add(crate::org::allbinary::logic::util::cache::IndexedBasicArrayListCache::IndexedBasicArrayListCacheDyn::as_IndexedBasicArrayListCache_mut(self), p1) }
    fn addArray(&mut self, p1: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::cache::CacheableInterface::CacheableInterface>>>>) { crate::org::allbinary::logic::util::cache::IndexedBasicArrayListCache::IndexedBasicArrayListCache::addArray(crate::org::allbinary::logic::util::cache::IndexedBasicArrayListCache::IndexedBasicArrayListCacheDyn::as_IndexedBasicArrayListCache_mut(self), p1) }
    fn clear(&mut self) { Self::clear(self) }
}


