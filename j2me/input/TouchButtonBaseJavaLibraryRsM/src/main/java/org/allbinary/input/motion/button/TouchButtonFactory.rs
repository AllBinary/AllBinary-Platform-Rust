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
// package org.allbinary.input.motion.button
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListUtil::BasicArrayListUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TouchButtonFactory {
    pub LIST: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub savedList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub saved: bool,
}

thread_local! { pub static TouchButtonFactory_SINGLETON: Option<Rc<RefCell<TouchButtonFactory>>> = TouchButtonFactory::new(); }

impl TouchButtonFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<TouchButtonFactory>>> {
        return TouchButtonFactory_SINGLETON.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            LIST: None,
            list: None,
            savedList: None,
            saved: false,
        };
        this.LIST = jcall!(BasicArrayListUtil::getInstance()).getImmutableInstance();
        this.list = this.LIST.clone();
        this.savedList = this.LIST.clone();
        this.saved = false;
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn defaultList(&mut self) {
        self.list = self.LIST.clone();
        self.toggle(false, self.LIST.clone());
    }
    

    // synchronized
    pub fn toggle(&mut self, mut save: bool, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        if !match (&list, &jcall!(BasicArrayListUtil::getInstance()).getImmutableInstance()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            self.savedList = list.clone();
            if !self.saved {
                self.list = self.savedList.clone();
            }
        } else {
            if save {
                if !self.saved {
                    self.saved = true;
                    self.savedList = self.list.clone();
                    self.list = self.LIST.clone();
                } else {
                }
            } else {
                self.list = self.savedList.clone();
                self.saved = false;
            }
        }
    }
    

    // throws Exception
    pub fn setList(&mut self, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        if list.is_none() {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Null List"))));
        }
        self.list = list.clone();
    }
    

    pub fn getList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.list.clone();
    }
}

