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
// package org.allbinary.layer
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::util::cache::BaseCircularPool::BaseCircularPool;
use crate::org::allbinary::util::CircularIndexUtil::CircularIndexUtil;
use super::AllBinaryLayer::AllBinaryLayer;
use super::AllBinaryLayerFactoryInterface::AllBinaryLayerFactoryInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AllBinaryLayerCircularPool {
    pub base: BaseCircularPool,
}


impl AllBinaryLayerCircularPool {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: BaseCircularPool::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn initAllBinaryLayerCircularPool(&mut self, mut allbinaryLayerFactoryInterface: Option<Rc<RefCell<dyn AllBinaryLayerFactoryInterface>>>, mut total: i32) {
        self.circularIndexUtil = CircularIndexUtil::createInstance(total.clone());
        self.OBJECT_ARRAY = vec![None; (total) as usize];
        for localIndex in (0..total) {
            self.OBJECT_ARRAY[(localIndex) as usize] = crate::java_into_object(jcall!(allbinaryLayerFactoryInterface).getInstance());
        }
    }
}
impl std::ops::Deref for AllBinaryLayerCircularPool {
    type Target = BaseCircularPool;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AllBinaryLayerCircularPool {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait AllBinaryLayerCircularPoolDyn: crate::org::allbinary::logic::util::cache::BaseCircularPool::BaseCircularPoolDyn {
    fn as_AllBinaryLayerCircularPool(&self) -> &AllBinaryLayerCircularPool;
    fn as_AllBinaryLayerCircularPool_mut(&mut self) -> &mut AllBinaryLayerCircularPool;
    fn initAllBinaryLayerCircularPool(&mut self, mut allbinaryLayerFactoryInterface: Option<Rc<RefCell<dyn AllBinaryLayerFactoryInterface>>>, mut total: i32) { AllBinaryLayerCircularPool::initAllBinaryLayerCircularPool(self.as_AllBinaryLayerCircularPool_mut(), allbinaryLayerFactoryInterface, total) }
}
impl AllBinaryLayerCircularPoolDyn for AllBinaryLayerCircularPool {
    fn as_AllBinaryLayerCircularPool(&self) -> &AllBinaryLayerCircularPool { self }
    fn as_AllBinaryLayerCircularPool_mut(&mut self) -> &mut AllBinaryLayerCircularPool { self }
}

impl crate::org::allbinary::logic::util::cache::BaseCircularPool::BaseCircularPoolDyn for AllBinaryLayerCircularPool {
    fn as_BaseCircularPool(&self) -> &crate::org::allbinary::logic::util::cache::BaseCircularPool::BaseCircularPool { &self.base }
    fn as_BaseCircularPool_mut(&mut self) -> &mut crate::org::allbinary::logic::util::cache::BaseCircularPool::BaseCircularPool { &mut self.base }
}


