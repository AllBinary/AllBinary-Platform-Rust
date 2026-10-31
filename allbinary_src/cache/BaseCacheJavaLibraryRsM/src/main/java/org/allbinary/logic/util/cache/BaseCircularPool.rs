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
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::util::CircularIndexUtil::CircularIndexUtil;
use crate::jsinterop::annotations::JsMethod;
use crate::jsinterop::annotations::JsProperty;
use super::AllBinaryObjectFactoryInterface::AllBinaryObjectFactoryInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BaseCircularPool {
    pub circularIndexUtil: Option<Rc<RefCell<CircularIndexUtil>>>,
    pub OBJECT_ARRAY: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>>,
}


impl BaseCircularPool {

    pub fn new_struct() -> Self {
        let mut this = Self {
            circularIndexUtil: None,
            OBJECT_ARRAY: Vec::new(),
        };
        this.circularIndexUtil = crate::org::allbinary::util::CircularIndexUtil::CircularIndexUtil_NULL_CIRCULAR_INDEX_UTIL.with(Clone::clone).clone();
        this.OBJECT_ARRAY = jcall!(NullUtil::getInstance()).NULL_OBJECT_ARRAY.clone();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    // synchronized
    pub fn getNextInstance(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = self.OBJECT_ARRAY[(jcall!(self.circularIndexUtil).getIndex()) as usize].clone();
        jcall!(self.circularIndexUtil).next();
        return anyType.clone();
    }
    

    pub fn init(&mut self, mut allBinaryObjectFactoryInterface: Option<Rc<RefCell<dyn AllBinaryObjectFactoryInterface>>>) {
        jcall!(self.circularIndexUtil).setIndex(0);
        let mut size: i32 = jcall!(self.circularIndexUtil).getSize();
        for index in (0..size) {
            self.OBJECT_ARRAY[(index) as usize] = jcall!(allBinaryObjectFactoryInterface).getInstance();
        }
    }
    

    // throws Exception
    pub fn getInstance(&mut self, mut index: i32) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return self.OBJECT_ARRAY[(index) as usize].clone();
    }
}
pub trait BaseCircularPoolDyn {
    fn as_BaseCircularPool(&self) -> &BaseCircularPool;
    fn as_BaseCircularPool_mut(&mut self) -> &mut BaseCircularPool;
    fn getNextInstance(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { BaseCircularPool::getNextInstance(self.as_BaseCircularPool_mut()) }
    fn init(&mut self, mut allBinaryObjectFactoryInterface: Option<Rc<RefCell<dyn AllBinaryObjectFactoryInterface>>>) { BaseCircularPool::init(self.as_BaseCircularPool_mut(), allBinaryObjectFactoryInterface) }
    fn getInstance(&mut self, mut index: i32) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { BaseCircularPool::getInstance(self.as_BaseCircularPool_mut(), index) }
}
impl BaseCircularPoolDyn for BaseCircularPool {
    fn as_BaseCircularPool(&self) -> &BaseCircularPool { self }
    fn as_BaseCircularPool_mut(&mut self) -> &mut BaseCircularPool { self }
}


