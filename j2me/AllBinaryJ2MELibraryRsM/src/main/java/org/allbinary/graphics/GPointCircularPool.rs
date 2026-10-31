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
// package org.allbinary.graphics
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::util::cache::BaseCircularPool::BaseCircularPool;
use crate::org::allbinary::util::CircularIndexUtil::CircularIndexUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GPointCircularPool {
    pub base: BaseCircularPool,
}


impl GPointCircularPool {
    

    pub fn new_struct(mut size: i32) -> Self {
        let mut this = Self {
            base: BaseCircularPool::new_struct(),
        };
        this.circularIndexUtil = CircularIndexUtil::createInstance(size.clone());
        this.OBJECT_ARRAY = vec![None; (size) as usize];
        for index in (0..=size - 1).rev() {
            this.OBJECT_ARRAY[(index) as usize] = crate::java_into_object(CustomGPoint::CustomGPoint::getInstance3(0, 0));
        }
        this
    }

    pub fn new(mut size: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(size))))
    }
}
impl std::ops::Deref for GPointCircularPool {
    type Target = BaseCircularPool;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for GPointCircularPool {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::util::cache::BaseCircularPool::BaseCircularPoolDyn for GPointCircularPool {
    fn as_BaseCircularPool(&self) -> &crate::org::allbinary::logic::util::cache::BaseCircularPool::BaseCircularPool { &self.base }
    fn as_BaseCircularPool_mut(&mut self) -> &mut crate::org::allbinary::logic::util::cache::BaseCircularPool::BaseCircularPool { &mut self.base }
}


