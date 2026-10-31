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
// package org.allbinary.game.resource
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::ResourceLoadingLevel::ResourceLoadingLevel;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ResourceLoadingLevelFactory {
    pub LOAD_EARLY: Option<Rc<RefCell<ResourceLoadingLevel>>>,
    pub LOAD_TOUCH: Option<Rc<RefCell<ResourceLoadingLevel>>>,
    pub LOAD_GAME: Option<Rc<RefCell<ResourceLoadingLevel>>>,
    pub LEVEL: Option<Rc<RefCell<ResourceLoadingLevel>>>,
    pub MAX_LEVEL: Option<Rc<RefCell<ResourceLoadingLevel>>>,
    pub LOAD_ALL: Option<Rc<RefCell<ResourceLoadingLevel>>>,
    pub RESOURCE_LOADING_LEVEL_ARRAY: Vec<Option<Rc<RefCell<ResourceLoadingLevel>>>>,
}

thread_local! { pub static ResourceLoadingLevelFactory_instance: Option<Rc<RefCell<ResourceLoadingLevelFactory>>> = ResourceLoadingLevelFactory::new(); }

impl ResourceLoadingLevelFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            LOAD_EARLY: None,
            LOAD_TOUCH: None,
            LOAD_GAME: None,
            LEVEL: None,
            MAX_LEVEL: None,
            LOAD_ALL: None,
            RESOURCE_LOADING_LEVEL_ARRAY: Vec::new(),
        };
        this.LOAD_EARLY = ResourceLoadingLevel::new(String::from("Load Early"), i32::MAX.clone());
        this.LOAD_TOUCH = ResourceLoadingLevel::new(String::from("Load Touch"), i32::MAX - 1);
        this.LOAD_GAME = ResourceLoadingLevel::new(String::from("Load Game"), i32::MAX - 2);
        this.LEVEL = ResourceLoadingLevel::new(String::from("Level"), i32::MAX - 99);
        this.MAX_LEVEL = ResourceLoadingLevel::new(String::from("Max Level"), i32::MAX - 100);
        this.LOAD_ALL = ResourceLoadingLevel::new(String::from("Load All"), -1);
        this.RESOURCE_LOADING_LEVEL_ARRAY = vec![this.LOAD_ALL.clone(), this.MAX_LEVEL.clone(), this.LEVEL.clone(), this.LOAD_GAME.clone(), this.LOAD_TOUCH.clone(), this.LOAD_EARLY.clone()];
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<ResourceLoadingLevelFactory>>> {
        return ResourceLoadingLevelFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn getLevelString(&mut self, mut level: i32) -> String {
        for index in (0..(self.RESOURCE_LOADING_LEVEL_ARRAY.len() as i32)) {
            if level == jcall!(self.RESOURCE_LOADING_LEVEL_ARRAY[(index) as usize]).getLevel() {
                return jcall!(self.RESOURCE_LOADING_LEVEL_ARRAY[(index) as usize]).getName();
            }
        }
        return (level.clone()).to_string();
    }
}

