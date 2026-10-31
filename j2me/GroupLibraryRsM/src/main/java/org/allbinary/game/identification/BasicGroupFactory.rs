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
use crate::org::allbinary::logic::NullUtil::NullUtil;
use super::Group::Group;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicGroupFactory {
    pub NAME: String,
    pub NO_SUCH_GROUP: String,
    pub GOOD: Option<Rc<RefCell<Group>>>,
    pub ENEMY: Option<Rc<RefCell<Group>>>,
    pub ENEMY_ON_LEVEL: Option<Rc<RefCell<Group>>>,
    pub NONE: Option<Rc<RefCell<Group>>>,
    pub GOOD_ARRAY: Vec<Option<Rc<RefCell<Group>>>>,
    pub ENEMY_ARRAY: Vec<Option<Rc<RefCell<Group>>>>,
    pub ENEMY_ON_LEVEL_ARRAY: Vec<Option<Rc<RefCell<Group>>>>,
    pub NONE_ARRAY: Vec<Option<Rc<RefCell<Group>>>>,
}

pub static mut BasicGroupFactory_instance: std::cell::LazyCell<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = std::cell::LazyCell::new(|| jcall!(NullUtil::getInstance()).NULL_OBJECT.clone());

impl BasicGroupFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            NAME: String::new(),
            NO_SUCH_GROUP: String::new(),
            GOOD: None,
            ENEMY: None,
            ENEMY_ON_LEVEL: None,
            NONE: None,
            GOOD_ARRAY: Vec::new(),
            ENEMY_ARRAY: Vec::new(),
            ENEMY_ON_LEVEL_ARRAY: Vec::new(),
            NONE_ARRAY: Vec::new(),
        };
        this.NAME = String::from("GROUP_NAME");
        this.NO_SUCH_GROUP = String::from("No Such Group: ");
        this.GOOD = Group::new(String::from("Good Guys"), ((0) as i16));
        this.ENEMY = Group::new(String::from("Bad Guys"), ((1) as i16));
        this.ENEMY_ON_LEVEL = Group::new(String::from("Bad Guys on Level"), ((2) as i16));
        this.NONE = Group::new(String::from("Not On A Team"), ((3) as i16));
        this.GOOD_ARRAY = vec![this.GOOD.clone()];
        this.ENEMY_ARRAY = vec![this.ENEMY.clone()];
        this.ENEMY_ON_LEVEL_ARRAY = vec![this.ENEMY.clone(), this.ENEMY_ON_LEVEL.clone()];
        this.NONE_ARRAY = vec![this.NONE.clone()];
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<BasicGroupFactory>>> {
        if match (&unsafe { BasicGroupFactory_instance.clone() }, &jcall!(NullUtil::getInstance()).NULL_OBJECT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            unsafe { *BasicGroupFactory_instance = BasicGroupFactory::new() };
        }
        return (unsafe { BasicGroupFactory_instance.clone() }).clone() /* as BasicGroupFactory */;
    }
}

