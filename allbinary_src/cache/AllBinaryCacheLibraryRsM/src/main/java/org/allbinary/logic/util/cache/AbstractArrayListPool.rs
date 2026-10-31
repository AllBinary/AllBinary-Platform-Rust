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
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use super::PoolInterface::PoolInterface;
use super::CacheableInterfaceFactoryInterface::CacheableInterfaceFactoryInterface;
use super::CacheableInterface::CacheableInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AbstractArrayListPool {
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub buffers: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub cacheableInterfaceFactoryInterface: Option<Rc<RefCell<dyn CacheableInterfaceFactoryInterface>>>,
}

impl PoolInterface for AbstractArrayListPool {
    fn remove(&mut self, mut key: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<dyn CacheableInterface>>> { Self::remove(self, key) }
}
impl crate::org::allbinary::logic::util::cache::CacheInterface::CacheInterface for AbstractArrayListPool {
    fn add(&mut self, mut cacheableInterface: Option<Rc<RefCell<dyn CacheableInterface>>>) { Self::add(self, cacheableInterface) }
    fn addArray(&mut self, mut cacheableInterfaces: Vec<Option<Rc<RefCell<dyn CacheableInterface>>>>) { Self::addArray(self, cacheableInterfaces) }
    fn clear(&mut self) { Self::clear(self) }
}

impl AbstractArrayListPool {
    

    pub fn new_struct(mut cacheableInterfaceFactoryInterface: Option<Rc<RefCell<dyn CacheableInterfaceFactoryInterface>>>) -> Self {
        let mut this = Self {
            commonStrings: None,
            buffers: None,
            cacheableInterfaceFactoryInterface: None,
        };
        this.commonStrings = CommonStrings::getInstance();
        this.buffers = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.cacheableInterfaceFactoryInterface = cacheableInterfaceFactoryInterface.clone();
        this
    }

    pub fn new(mut cacheableInterfaceFactoryInterface: Option<Rc<RefCell<dyn CacheableInterfaceFactoryInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(cacheableInterfaceFactoryInterface))))
    }
    

    // throws Exception
    pub fn clear(&mut self) {
        panic!("{:?}", format!("{}: {:?}", "Exception", (jcall!(self.commonStrings).NOT_IMPLEMENTED.clone())));
    }
    

    // throws Exception
    pub fn remove(&mut self, mut key: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<dyn CacheableInterface>>> {
        panic!("{:?}", format!("{}: {:?}", "Exception", (jcall!(self.commonStrings).NOT_IMPLEMENTED.clone())));
    }
    

    // throws Exception
    pub fn add(&mut self, mut cacheableInterface: Option<Rc<RefCell<dyn CacheableInterface>>>) {
        jcall!(self.buffers).add(crate::java_into_object(cacheableInterface.clone()));
    }
    

    // throws Exception
    pub fn addArray(&mut self, mut cacheableInterfaces: Vec<Option<Rc<RefCell<dyn CacheableInterface>>>>) {
        for index in (0..(cacheableInterfaces.len() as i32)) {
            jcall!(self.buffers).add(crate::java_into_object(cacheableInterfaces[(index) as usize].clone()));
        }
    }
}
pub trait AbstractArrayListPoolDyn {
    fn as_AbstractArrayListPool(&self) -> &AbstractArrayListPool;
    fn as_AbstractArrayListPool_mut(&mut self) -> &mut AbstractArrayListPool;
    fn clear(&mut self) { AbstractArrayListPool::clear(self.as_AbstractArrayListPool_mut()) }
    fn remove(&mut self, mut key: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<dyn CacheableInterface>>> { AbstractArrayListPool::remove(self.as_AbstractArrayListPool_mut(), key) }
    fn add(&mut self, mut cacheableInterface: Option<Rc<RefCell<dyn CacheableInterface>>>) { AbstractArrayListPool::add(self.as_AbstractArrayListPool_mut(), cacheableInterface) }
    fn addArray(&mut self, mut cacheableInterfaces: Vec<Option<Rc<RefCell<dyn CacheableInterface>>>>) { AbstractArrayListPool::addArray(self.as_AbstractArrayListPool_mut(), cacheableInterfaces) }
}
impl AbstractArrayListPoolDyn for AbstractArrayListPool {
    fn as_AbstractArrayListPool(&self) -> &AbstractArrayListPool { self }
    fn as_AbstractArrayListPool_mut(&mut self) -> &mut AbstractArrayListPool { self }
}


