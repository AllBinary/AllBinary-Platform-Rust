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
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::jsinterop::annotations::JsMethod;
use crate::jsinterop::annotations::JsConstructor;
use crate::jsinterop::annotations::JsProperty;
use super::BaseBasicArrayListCache::BaseBasicArrayListCache;
use super::CacheInterface::CacheInterface;
use super::CacheableInterface::CacheableInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct IndexedBasicArrayListCache {
    pub base: BaseBasicArrayListCache,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub size: i32,
    pub index: i32,
}

impl CacheInterface for IndexedBasicArrayListCache {
    fn add(&mut self, mut cacheableInterface: Option<Rc<RefCell<dyn CacheableInterface>>>) { Self::add(self, cacheableInterface) }
    fn addArray(&mut self, mut cacheableInterfaces: Vec<Option<Rc<RefCell<dyn CacheableInterface>>>>) { Self::addArray(self, cacheableInterfaces) }
    fn clear(&mut self) { Self::clear(self) }
}

impl IndexedBasicArrayListCache {
    

    // throws Exception
    pub fn createCache(mut size: i32) -> Option<Rc<RefCell<dyn IndexedBasicArrayListCacheDyn>>> {
        let mut indexedBasicArrayListCache: Option<Rc<RefCell<dyn IndexedBasicArrayListCacheDyn>>> = IndexedBasicArrayListCache::new().map(|reference| reference as Rc<RefCell<dyn IndexedBasicArrayListCacheDyn>>);
        jcall!(indexedBasicArrayListCache).init(size.clone());
        return indexedBasicArrayListCache.clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: BaseBasicArrayListCache::new_struct(),
            commonStrings: None,
            size: 0,
            index: 0,
        };
        this.commonStrings = CommonStrings::getInstance();
        this.size = 0;
        this.index = 0;
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn init(&mut self, mut size: i32) {
        if self.size < size {
            jcall!(self.list).ensureCapacity(size.clone());
            for index in (self.size..=size - 1).rev() {
                self.addDefault();
            }
            self.size = size.clone();
        }
    }
    

    pub fn getAt(&mut self, mut index: i32) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return jcall!(self.list).as_BasicArrayList_mut().objectArray[(index) as usize].clone();
    }
    

    // throws Exception
    pub fn addDefault(&mut self) {
    }
    

    // throws Exception
    pub fn addObject(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) {
        jcall!(self.list).add(crate::java_into_object(anyType.clone()));
    }
    

    // throws Exception
    pub fn add(&mut self, mut cacheableInterface: Option<Rc<RefCell<dyn CacheableInterface>>>) {
        jcall!(self.list).add(crate::java_into_object(cacheableInterface.clone()));
    }
    

    // throws Exception
    pub fn addArray(&mut self, mut cacheableInterfaces: Vec<Option<Rc<RefCell<dyn CacheableInterface>>>>) {
        panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("No Imple"))));
    }
    

    pub fn clear(&mut self) {
        self.index = 0;
    }
}
impl std::ops::Deref for IndexedBasicArrayListCache {
    type Target = BaseBasicArrayListCache;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for IndexedBasicArrayListCache {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait IndexedBasicArrayListCacheDyn: crate::org::allbinary::logic::util::cache::BaseBasicArrayListCache::BaseBasicArrayListCacheDyn {
    fn as_IndexedBasicArrayListCache(&self) -> &IndexedBasicArrayListCache;
    fn as_IndexedBasicArrayListCache_mut(&mut self) -> &mut IndexedBasicArrayListCache;
    fn init(&mut self, mut size: i32) { IndexedBasicArrayListCache::init(self.as_IndexedBasicArrayListCache_mut(), size) }
    fn getAt(&mut self, mut index: i32) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { IndexedBasicArrayListCache::getAt(self.as_IndexedBasicArrayListCache_mut(), index) }
    fn addDefault(&mut self) { IndexedBasicArrayListCache::addDefault(self.as_IndexedBasicArrayListCache_mut()) }
    fn addObject(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) { IndexedBasicArrayListCache::addObject(self.as_IndexedBasicArrayListCache_mut(), anyType) }
    fn add(&mut self, mut cacheableInterface: Option<Rc<RefCell<dyn CacheableInterface>>>) { IndexedBasicArrayListCache::add(self.as_IndexedBasicArrayListCache_mut(), cacheableInterface) }
    fn addArray(&mut self, mut cacheableInterfaces: Vec<Option<Rc<RefCell<dyn CacheableInterface>>>>) { IndexedBasicArrayListCache::addArray(self.as_IndexedBasicArrayListCache_mut(), cacheableInterfaces) }
    fn clear(&mut self) { IndexedBasicArrayListCache::clear(self.as_IndexedBasicArrayListCache_mut()) }
}
impl IndexedBasicArrayListCacheDyn for IndexedBasicArrayListCache {
    fn as_IndexedBasicArrayListCache(&self) -> &IndexedBasicArrayListCache { self }
    fn as_IndexedBasicArrayListCache_mut(&mut self) -> &mut IndexedBasicArrayListCache { self }
}

impl crate::org::allbinary::logic::util::cache::BaseBasicArrayListCache::BaseBasicArrayListCacheDyn for IndexedBasicArrayListCache {
    fn as_BaseBasicArrayListCache(&self) -> &crate::org::allbinary::logic::util::cache::BaseBasicArrayListCache::BaseBasicArrayListCache { &self.base }
    fn as_BaseBasicArrayListCache_mut(&mut self) -> &mut crate::org::allbinary::logic::util::cache::BaseBasicArrayListCache::BaseBasicArrayListCache { &mut self.base }
}


