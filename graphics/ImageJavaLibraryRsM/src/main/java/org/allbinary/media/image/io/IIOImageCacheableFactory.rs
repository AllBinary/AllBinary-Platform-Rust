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
// package org.allbinary.media.image.io
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::util::cache::CacheableInterface::CacheableInterface;
use crate::org::allbinary::logic::util::cache::CacheableInterfaceFactoryInterface::CacheableInterfaceFactoryInterface;
use super::IIOImageCacheable::IIOImageCacheable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct IIOImageCacheableFactory {
}

impl CacheableInterfaceFactoryInterface for IIOImageCacheableFactory {
    fn getInstance(&mut self, mut key: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<dyn CacheableInterface>>> { Self::getInstance(self, key) }
}

impl IIOImageCacheableFactory {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance(&mut self, mut key: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<dyn CacheableInterface>>> {
        return IIOImageCacheable::new((key).clone() /* as String */).map(|reference| reference as Rc<RefCell<dyn CacheableInterface>>);
    }
}

