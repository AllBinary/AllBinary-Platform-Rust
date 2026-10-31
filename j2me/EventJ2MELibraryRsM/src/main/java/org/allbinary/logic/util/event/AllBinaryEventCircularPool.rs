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
// package org.allbinary.logic.util.event
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::util::cache::BaseCircularPool::BaseCircularPool;
use crate::org::allbinary::util::CircularIndexUtil::CircularIndexUtil;
use super::AllBinaryEventObjectFactoryInterface::AllBinaryEventObjectFactoryInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AllBinaryEventCircularPool {
    pub base: BaseCircularPool,
}


impl AllBinaryEventCircularPool {
    

    pub fn new_struct(mut max: i32) -> Self {
        let mut this = Self {
            base: BaseCircularPool::new_struct(),
        };
        this.OBJECT_ARRAY = vec![None; (max) as usize];
        this.circularIndexUtil = CircularIndexUtil::createInstance(max.clone());
        this
    }

    pub fn new(mut max: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(max))))
    }
    

    pub fn initAllBinaryEventCircularPool(&mut self, mut allBinaryEventObjectFactoryInterface: Option<Rc<RefCell<dyn AllBinaryEventObjectFactoryInterface>>>) {
        jcall!(self.circularIndexUtil).setIndex(0);
        let mut size: i32 = jcall!(self.circularIndexUtil).getSize();
        for index in (0..size) {
            self.OBJECT_ARRAY[(index) as usize] = crate::java_into_object(jcall!(allBinaryEventObjectFactoryInterface).getInstance());
        }
    }
}
impl std::ops::Deref for AllBinaryEventCircularPool {
    type Target = BaseCircularPool;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AllBinaryEventCircularPool {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::util::cache::BaseCircularPool::BaseCircularPoolDyn for AllBinaryEventCircularPool {
    fn as_BaseCircularPool(&self) -> &crate::org::allbinary::logic::util::cache::BaseCircularPool::BaseCircularPool { &self.base }
    fn as_BaseCircularPool_mut(&mut self) -> &mut crate::org::allbinary::logic::util::cache::BaseCircularPool::BaseCircularPool { &mut self.base }
}


