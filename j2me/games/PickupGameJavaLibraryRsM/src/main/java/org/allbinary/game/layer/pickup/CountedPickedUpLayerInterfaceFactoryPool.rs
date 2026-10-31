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
// package org.allbinary.game.layer.pickup
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CountedPickedUpLayerInterfaceFactoryPool {
    pub list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}

pub static mut CountedPickedUpLayerInterfaceFactoryPool_SINGLETON: std::cell::LazyCell<Option<Rc<RefCell<CountedPickedUpLayerInterfaceFactoryPool>>>> = std::cell::LazyCell::new(|| CountedPickedUpLayerInterfaceFactoryPool::new());

impl CountedPickedUpLayerInterfaceFactoryPool {
    

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
    

    pub fn getList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.list.clone();
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<CountedPickedUpLayerInterfaceFactoryPool>>> {
        return unsafe { CountedPickedUpLayerInterfaceFactoryPool_SINGLETON.clone() }.clone();
    }
    

    pub fn init() {
        unsafe { *CountedPickedUpLayerInterfaceFactoryPool_SINGLETON = CountedPickedUpLayerInterfaceFactoryPool::new() };
    }
}

