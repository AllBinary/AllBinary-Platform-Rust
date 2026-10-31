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
// package org.allbinary.game.identification
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use super::Group::Group;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GroupFactory {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub NULL_GROUP_ARRAY: Vec<Option<Rc<RefCell<Group>>>>,
    pub list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub index: i32,
}

pub static mut GroupFactory_instance: std::cell::LazyCell<Option<Rc<RefCell<GroupFactory>>>> = std::cell::LazyCell::new(|| GroupFactory::new());

impl GroupFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            NULL_GROUP_ARRAY: Vec::new(),
            list: None,
            index: 0,
        };
        this.logUtil = LogUtil::getInstance();
        this.NULL_GROUP_ARRAY = vec![None; (0) as usize];
        this.list = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.index = 0;
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<GroupFactory>>> {
        return unsafe { GroupFactory_instance.clone() }.clone();
    }
    

    pub fn getNextGroup(&mut self) -> Option<Rc<RefCell<Group>>> {
        let mut group: Option<Rc<RefCell<Group>>> = { let java_object = (jcall!(self.list).as_BasicArrayList_mut().objectArray[(self.index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<Group>>>>().expect("ClassCastException").clone()) };
        self.index += 1;
        return group.clone();
    }
    

    pub fn getNextGroupByName(&mut self, mut name: String) -> Option<Rc<RefCell<Group>>> {
        let mut group: Option<Rc<RefCell<Group>>> = self.getNextGroup();
        jcall!(group).setName(name.clone());
        return group.clone();
    }
    

    pub fn init(&mut self, mut groups: i16, mut nameArray: Vec<String>) {
        let mut stringMaker: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut stringUtil: Option<Rc<RefCell<StringUtil>>> = StringUtil::getInstance();
        let mut TEAM: String = String::from("Team ");
        jcall!(self.list).clear();
        self.index = 0;
        let mut size: i32 = jcall!(self.list).size();
        while size < ((groups) as i32) {
            let mut name: String = jcall!(stringUtil).EMPTY_STRING.clone();
            if size < (nameArray.len() as i32) {
                name = nameArray[(size) as usize].clone();
            } else {
                jcall!(stringMaker).delete(0, jcall!(stringMaker).length());
                name = jcall!(stringMaker).append(TEAM.clone()).appendint(size.clone()).toString();
            }
            jcall!(self.list).add(crate::java_into_object(Group::new(name.clone(), (((size + 3)) as i16))));
            size += 1;
        }
    }
}

