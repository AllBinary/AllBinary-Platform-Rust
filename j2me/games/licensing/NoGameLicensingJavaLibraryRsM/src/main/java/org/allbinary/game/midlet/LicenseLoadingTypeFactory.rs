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
// package org.allbinary.game.midlet
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::LicenseLoadingType::LicenseLoadingType;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LicenseLoadingTypeFactory {
    pub INITIAL_LOADING: Option<Rc<RefCell<LicenseLoadingType>>>,
    pub GAME_START: Option<Rc<RefCell<LicenseLoadingType>>>,
    pub LOGIN: Option<Rc<RefCell<LicenseLoadingType>>>,
    pub OTHER: Option<Rc<RefCell<LicenseLoadingType>>>,
}

thread_local! { pub static LicenseLoadingTypeFactory_instance: Option<Rc<RefCell<LicenseLoadingTypeFactory>>> = LicenseLoadingTypeFactory::new(); }

impl LicenseLoadingTypeFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            INITIAL_LOADING: None,
            GAME_START: None,
            LOGIN: None,
            OTHER: None,
        };
        this.INITIAL_LOADING = LicenseLoadingType::new(String::from("Initial Loading"));
        this.GAME_START = LicenseLoadingType::new(String::from("Game Start"));
        this.LOGIN = LicenseLoadingType::new(String::from("Login"));
        this.OTHER = LicenseLoadingType::new(String::from("Other/Usually The First Non Demo Level"));
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getIntance() -> Option<Rc<RefCell<LicenseLoadingTypeFactory>>> {
        return LicenseLoadingTypeFactory_instance.with(Clone::clone).clone();
    }
}

