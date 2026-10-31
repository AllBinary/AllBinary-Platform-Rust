// 
// AllBinary Open License Version 1 
// Copyright (c) 2022 AllBinary 
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
// package org.allbinary.graphics.displayable
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

pub struct CanvasStrings {
    pub CONSTRUCTOR: String,
    pub PAUSE: String,
    pub UN_PAUSE: String,
    pub SET_FULL_SCREEN_MODE: String,
    pub SIZE_CHANGED: String,
    pub SCALED_IMAGES: String,
    pub PAINT: String,
    pub ON_DISPLAY_CHANGE_EVENT: String,
    pub UPDATE_MEASUREMENT: String,
    pub FD_WIDTH: String,
    pub FD_HEIGHT: String,
    pub EMPTY_CHILD_NAME_LIST: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub ABOUT: String,
    pub OPTIONS: String,
    pub FONT_ATLAS: String,
    pub OBJ_MODEL: String,
    pub MD2_MODEL: String,
    pub _PNG: String,
}

thread_local! { pub static CanvasStrings_instance: Option<Rc<RefCell<CanvasStrings>>> = CanvasStrings::new(); }

impl CanvasStrings {

    pub fn new_struct() -> Self {
        let mut this = Self {
            CONSTRUCTOR: String::new(),
            PAUSE: String::new(),
            UN_PAUSE: String::new(),
            SET_FULL_SCREEN_MODE: String::new(),
            SIZE_CHANGED: String::new(),
            SCALED_IMAGES: String::new(),
            PAINT: String::new(),
            ON_DISPLAY_CHANGE_EVENT: String::new(),
            UPDATE_MEASUREMENT: String::new(),
            FD_WIDTH: String::new(),
            FD_HEIGHT: String::new(),
            EMPTY_CHILD_NAME_LIST: None,
            ABOUT: String::new(),
            OPTIONS: String::new(),
            FONT_ATLAS: String::new(),
            OBJ_MODEL: String::new(),
            MD2_MODEL: String::new(),
            _PNG: String::new(),
        };
        this.CONSTRUCTOR = String::from("MyCanvas::MyCanvas");
        this.PAUSE = String::from("MyCanvas::pause");
        this.UN_PAUSE = String::from("MyCanvas::unPause");
        this.SET_FULL_SCREEN_MODE = String::from("setFullScreenMode");
        this.SIZE_CHANGED = String::from("sizeChanged");
        this.SCALED_IMAGES = String::from("scaledImages");
        this.PAINT = String::from("paint");
        this.ON_DISPLAY_CHANGE_EVENT = String::from("onDisplayChangeEvent");
        this.UPDATE_MEASUREMENT = String::from("updateMeasurement");
        this.FD_WIDTH = String::from(" font/display width: ");
        this.FD_HEIGHT = String::from(" font/display height: ");
        this.EMPTY_CHILD_NAME_LIST = jcall!(BasicArrayListUtil::getInstance()).getImmutableInstance();
        this.ABOUT = String::from("About");
        this.OPTIONS = String::from("Options");
        this.FONT_ATLAS = String::from("/font.png");
        this.OBJ_MODEL = String::from("_obj");
        this.MD2_MODEL = String::from("_md2");
        this._PNG = String::from(".png");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<CanvasStrings>>> {
        return CanvasStrings_instance.with(Clone::clone).clone();
    }
}

