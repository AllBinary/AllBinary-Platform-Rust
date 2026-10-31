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
// package org.allbinary.string
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::jsinterop::annotations::JsType;
use crate::jsinterop::annotations::JsMethod;
use crate::jsinterop::annotations::JsProperty;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CommonStrings {
    pub EMPTY: String,
    pub CREATE_IMAGE: String,
    pub PLEASE_WAIT: String,
    pub PLEASE_WAIT_FOR_SERVER: String,
    pub PERIODS: String,
    pub LOADING: String,
    pub CONSTRUCTOR: String,
    pub START: String,
    pub END: String,
    pub EXCEPTION: String,
    pub EXCEPTION_LABEL: String,
    pub CLEANUP: String,
    pub INIT: String,
    pub CLOSE: String,
    pub UP: String,
    pub DOWN: String,
    pub LEFT: String,
    pub RIGHT: String,
    pub DIRECTION_NAME: Vec<String>,
    pub CREATE: String,
    pub REMOVE: String,
    pub UPDATE: String,
    pub DELETE: String,
    pub delete: String,
    pub LOAD: String,
    pub SAVE: String,
    pub ADD: String,
    pub DROP: String,
    pub INSERT: String,
    pub VISIT: String,
    pub START_METHOD_NAME: String,
    pub END_METHOD_NAME: String,
    pub GET: String,
    pub PROCESS: String,
    pub GET_INSTANCE: String,
    pub GET_LIST: String,
    pub RUN: String,
    pub START_RUNNABLE: String,
    pub RUNNING: String,
    pub END_RUNNABLE: String,
    pub DISABLE: String,
    pub ENABLE: String,
    pub SUCCESS: String,
    pub FAILURE: String,
    pub NOT_IMPLEMENTED: String,
    pub UNKNOWN: String,
    pub ON_EVENT: String,
    pub IS_VALID: String,
    pub TOSTRING: String,
    pub ADD_LISTENER: String,
    pub REMOVE_LISTENER: String,
    pub SIZE: String,
}

thread_local! { pub static CommonStrings_instance: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::new(); }

impl CommonStrings {

    pub fn new_struct() -> Self {
        let mut this = Self {
            EMPTY: String::new(),
            CREATE_IMAGE: String::new(),
            PLEASE_WAIT: String::new(),
            PLEASE_WAIT_FOR_SERVER: String::new(),
            PERIODS: String::new(),
            LOADING: String::new(),
            CONSTRUCTOR: String::new(),
            START: String::new(),
            END: String::new(),
            EXCEPTION: String::new(),
            EXCEPTION_LABEL: String::new(),
            CLEANUP: String::new(),
            INIT: String::new(),
            CLOSE: String::new(),
            UP: String::new(),
            DOWN: String::new(),
            LEFT: String::new(),
            RIGHT: String::new(),
            DIRECTION_NAME: Vec::new(),
            CREATE: String::new(),
            REMOVE: String::new(),
            UPDATE: String::new(),
            DELETE: String::new(),
            delete: String::new(),
            LOAD: String::new(),
            SAVE: String::new(),
            ADD: String::new(),
            DROP: String::new(),
            INSERT: String::new(),
            VISIT: String::new(),
            START_METHOD_NAME: String::new(),
            END_METHOD_NAME: String::new(),
            GET: String::new(),
            PROCESS: String::new(),
            GET_INSTANCE: String::new(),
            GET_LIST: String::new(),
            RUN: String::new(),
            START_RUNNABLE: String::new(),
            RUNNING: String::new(),
            END_RUNNABLE: String::new(),
            DISABLE: String::new(),
            ENABLE: String::new(),
            SUCCESS: String::new(),
            FAILURE: String::new(),
            NOT_IMPLEMENTED: String::new(),
            UNKNOWN: String::new(),
            ON_EVENT: String::new(),
            IS_VALID: String::new(),
            TOSTRING: String::new(),
            ADD_LISTENER: String::new(),
            REMOVE_LISTENER: String::new(),
            SIZE: String::new(),
        };
        this.EMPTY = String::from("Empty");
        this.CREATE_IMAGE = String::from("createImage");
        this.PLEASE_WAIT = String::from("Please Wait");
        this.PLEASE_WAIT_FOR_SERVER = String::from("Please Wait Getting Data From Server...");
        this.PERIODS = String::from("...");
        this.LOADING = String::from("Loading...");
        this.CONSTRUCTOR = String::from("Constructor");
        this.START = String::from("Start");
        this.END = String::from("End");
        this.EXCEPTION = String::from("Exception");
        this.EXCEPTION_LABEL = String::from("Exception: ");
        this.CLEANUP = String::from("cleanup");
        this.INIT = String::from("init");
        this.CLOSE = String::from("close");
        this.UP = String::from("up");
        this.DOWN = String::from("down");
        this.LEFT = String::from("left");
        this.RIGHT = String::from("right");
        this.DIRECTION_NAME = vec![this.DOWN.clone(), this.LEFT.clone(), this.RIGHT.clone(), this.UP.clone()];
        this.CREATE = String::from("Create");
        this.REMOVE = String::from("Remove");
        this.UPDATE = String::from("Update");
        this.DELETE = String::from("Delete");
        this.delete = String::from("delete");
        this.LOAD = String::from("load");
        this.SAVE = String::from("save");
        this.ADD = String::from("Add");
        this.DROP = String::from("Drop");
        this.INSERT = String::from("Insert");
        this.VISIT = String::from("visit");
        this.START_METHOD_NAME = this.START.clone();
        this.END_METHOD_NAME = String::from("end");
        this.GET = String::from("get");
        this.PROCESS = String::from("process");
        this.GET_INSTANCE = String::from("getInstance");
        this.GET_LIST = String::from("getList");
        this.RUN = String::from("run");
        this.START_RUNNABLE = String::from("Start Runnable");
        this.RUNNING = String::from("Running");
        this.END_RUNNABLE = String::from("End Runnable");
        this.DISABLE = String::from("disable");
        this.ENABLE = String::from("enable");
        this.SUCCESS = String::from("Command Success");
        this.FAILURE = String::from("Command Failed");
        this.NOT_IMPLEMENTED = String::from("Not Implemented");
        this.UNKNOWN = String::from("Unknown");
        this.ON_EVENT = String::from("onEvent");
        this.IS_VALID = String::from("isValid");
        this.TOSTRING = String::from("toString");
        this.ADD_LISTENER = String::from("addListener");
        this.REMOVE_LISTENER = String::from("removeListener");
        this.SIZE = String::from("size");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<CommonStrings>>> {
        return CommonStrings_instance.with(Clone::clone).clone();
    }
}

