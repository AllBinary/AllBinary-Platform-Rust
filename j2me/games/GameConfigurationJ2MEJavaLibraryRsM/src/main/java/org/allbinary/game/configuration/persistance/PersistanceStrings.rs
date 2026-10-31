// 
// AllBinary Open License Version 1 
// Copyright (c) 2025 AllBinary 
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
// package org.allbinary.game.configuration.persistance
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PersistanceStrings {
    pub SAVING: String,
    pub NUMBER_OF_RECORDS: String,
    pub LOADING_ID: String,
    pub LOAD_ALL: String,
    pub DELETING_WITH_ID: String,
    pub CLOSING_RECORDSTORE: String,
    pub NOT_SAVING: String,
    pub ERROR_LOADING: String,
    pub ERROR_LOADING_ID: String,
    pub ID: String,
    pub GAME_ACTION_INPUT: String,
}

thread_local! { pub static PersistanceStrings_instance: Option<Rc<RefCell<PersistanceStrings>>> = PersistanceStrings::new(); }

impl PersistanceStrings {

    pub fn new_struct() -> Self {
        let mut this = Self {
            SAVING: String::new(),
            NUMBER_OF_RECORDS: String::new(),
            LOADING_ID: String::new(),
            LOAD_ALL: String::new(),
            DELETING_WITH_ID: String::new(),
            CLOSING_RECORDSTORE: String::new(),
            NOT_SAVING: String::new(),
            ERROR_LOADING: String::new(),
            ERROR_LOADING_ID: String::new(),
            ID: String::new(),
            GAME_ACTION_INPUT: String::new(),
        };
        this.SAVING = String::from("Saving: ");
        this.NUMBER_OF_RECORDS = String::from("Number of records: ");
        this.LOADING_ID = String::from("Loading data with id: ");
        this.LOAD_ALL = String::from("loadAll");
        this.DELETING_WITH_ID = String::from("Deleting data with id: ");
        this.CLOSING_RECORDSTORE = String::from("Closing RecordStore");
        this.NOT_SAVING = String::from("Not Saving: ");
        this.ERROR_LOADING = String::from("Error Loading gameActionInput: ");
        this.ERROR_LOADING_ID = String::from("Error Loading id: ");
        this.ID = String::from(" id: ");
        this.GAME_ACTION_INPUT = String::from(" GameActionInput: ");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<PersistanceStrings>>> {
        return PersistanceStrings_instance.with(Clone::clone).clone();
    }
}

