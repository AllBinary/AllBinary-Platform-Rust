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
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use super::AbstractArrayListPool::AbstractArrayListPool;
use super::CacheableInterfaceFactoryInterface::CacheableInterfaceFactoryInterface;
use super::CacheableInterface::CacheableInterface;
use super::BasicArrayListCacheable::BasicArrayListCacheable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicArrayListPool {
    pub base: AbstractArrayListPool,
}

impl crate::org::allbinary::logic::util::cache::PoolInterface::PoolInterface for BasicArrayListPool {
    fn remove(&mut self, mut key: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<dyn CacheableInterface>>> { Self::remove(self, key) }
}

impl BasicArrayListPool {
    

    pub fn new_struct(mut cacheableInterfaceFactoryInterface: Option<Rc<RefCell<dyn CacheableInterfaceFactoryInterface>>>) -> Self {
        let mut this = Self {
            base: AbstractArrayListPool::new_struct(cacheableInterfaceFactoryInterface.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut cacheableInterfaceFactoryInterface: Option<Rc<RefCell<dyn CacheableInterfaceFactoryInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(cacheableInterfaceFactoryInterface))))
    }
    

    pub fn clear(&mut self) {
        jcall!(self.buffers).clear();
    }
    

    // throws Exception
    pub fn remove(&mut self, mut key: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<dyn CacheableInterface>>> {
        let mut size: i32 = jcall!(self.buffers).size();
        if size > 0 {
            return { let java_object = jcall!(self.buffers).removeAt(size - 1); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn CacheableInterface>>>>().expect("ClassCastException").clone()) };
        } else {
            return jcall!(self.cacheableInterfaceFactoryInterface).getInstance(crate::java_into_object(key.clone()));
        }
    }
    

    // throws Exception
    pub fn addAll(&mut self, mut usedList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        {
            let mut index: i32 = jcall!(usedList).size();
            while { index -= 1; index } >= 0 {
            let mut anyType: Option<Rc<RefCell<dyn CacheableInterface>>> = { let java_object = (jcall!(usedList).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn CacheableInterface>>>>().expect("ClassCastException").clone()) };
            self.add(anyType.clone());
            }
        }
        jcall!(usedList).clear();
    }
    

    // throws Exception
    pub fn releaseUsedBackToPool(&mut self, mut usedList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        {
            let mut index: i32 = jcall!(usedList).size();
            while { index -= 1; index } >= 0 {
            let mut list: Option<Rc<RefCell<BasicArrayListCacheable>>> = { let java_object = (jcall!(usedList).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<BasicArrayListCacheable>>>>().expect("ClassCastException").clone()) };
            if jcall!(list).size() == 0 {
                jcall!(usedList).removeAt(index.clone());
                self.add(list.clone());
            }
            }
        }
    }
}
impl std::ops::Deref for BasicArrayListPool {
    type Target = AbstractArrayListPool;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for BasicArrayListPool {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::util::cache::AbstractArrayListPool::AbstractArrayListPoolDyn for BasicArrayListPool {
    fn as_AbstractArrayListPool(&self) -> &crate::org::allbinary::logic::util::cache::AbstractArrayListPool::AbstractArrayListPool { &self.base }
    fn as_AbstractArrayListPool_mut(&mut self) -> &mut crate::org::allbinary::logic::util::cache::AbstractArrayListPool::AbstractArrayListPool { &mut self.base }
    fn clear(&mut self) { Self::clear(self) }
    fn remove(&mut self, mut key: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<dyn CacheableInterface>>> { Self::remove(self, key) }
}


