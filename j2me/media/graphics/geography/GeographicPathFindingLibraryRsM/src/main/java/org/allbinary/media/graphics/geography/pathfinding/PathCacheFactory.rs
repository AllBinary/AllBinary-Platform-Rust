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
// package org.allbinary.media.graphics.geography.pathfinding
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListUtil::BasicArrayListUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PathCacheFactory {
    pub basicArrayListUtil: Option<Rc<RefCell<BasicArrayListUtil>>>,
    pub hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>,
}

pub static mut PathCacheFactory_pathFactory: std::cell::LazyCell<Option<Rc<RefCell<PathCacheFactory>>>> = std::cell::LazyCell::new(|| PathCacheFactory::new());

impl PathCacheFactory {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            basicArrayListUtil: None,
            hashtable: None,
        };
        this.basicArrayListUtil = BasicArrayListUtil::getInstance();
        this.hashtable = jcall!(StdUtil::getInstance()).createHashtable();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<PathCacheFactory>>> {
        return unsafe { PathCacheFactory_pathFactory.clone() }.clone();
    }
    

    pub fn getSize(&mut self) -> i32 {
        return jcall!(self.hashtable).size();
    }
    

    pub fn add(&mut self, mut pathId: Option<i32>, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        jcall!(self.hashtable).put(crate::java_into_object((pathId.clone()).expect("NullPointerException")), crate::java_into_object(list.clone()));
    }
    

    pub fn remove(&mut self, mut pathId: Option<i32>) {
        jcall!(self.hashtable).remove(crate::java_into_object((pathId.clone()).expect("NullPointerException")));
    }
    

    // throws Exception
    pub fn removeAll(&mut self) {
        jcall!(self.hashtable).clear();
        if jcall!(self.hashtable).size() > 0 {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Did not clear"))));
        }
    }
    

    // throws Exception
    pub fn getOrCreate(&mut self, mut pathIdInteger: Option<i32>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut listCanBeNull: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(self.hashtable).get(crate::java_into_object((pathIdInteger.clone()).expect("NullPointerException")));
        if listCanBeNull.is_none() {
            listCanBeNull = jcall!(self.basicArrayListUtil).getImmutableInstance();
        }
        return (listCanBeNull).clone() /* as BasicArrayList */;
    }
}

