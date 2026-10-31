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
// package org.allbinary.graphics.color
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::jsinterop::annotations::JsType;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::jsinterop::annotations::JsMethod;
use crate::jsinterop::annotations::JsProperty;
use super::BasicColor::BasicColor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicColorFactory {
    pub NULL_COLOR: Option<Rc<RefCell<BasicColor>>>,
    pub TAN: Option<Rc<RefCell<BasicColor>>>,
    pub SKIN_PINK: Option<Rc<RefCell<BasicColor>>>,
    pub RED: Option<Rc<RefCell<BasicColor>>>,
    pub LIGHT_RED: Option<Rc<RefCell<BasicColor>>>,
    pub DARK_GREEN: Option<Rc<RefCell<BasicColor>>>,
    pub GREEN: Option<Rc<RefCell<BasicColor>>>,
    pub LIGHT_GREEN: Option<Rc<RefCell<BasicColor>>>,
    pub PURPLE: Option<Rc<RefCell<BasicColor>>>,
    pub BLUE: Option<Rc<RefCell<BasicColor>>>,
    pub LIGHT_BLUE: Option<Rc<RefCell<BasicColor>>>,
    pub VERY_LIGHT_BLUE: Option<Rc<RefCell<BasicColor>>>,
    pub PUCE: Option<Rc<RefCell<BasicColor>>>,
    pub AQUA: Option<Rc<RefCell<BasicColor>>>,
    pub YELLOW: Option<Rc<RefCell<BasicColor>>>,
    pub PINK: Option<Rc<RefCell<BasicColor>>>,
    pub BLACK: Option<Rc<RefCell<BasicColor>>>,
    pub GREY: Option<Rc<RefCell<BasicColor>>>,
    pub LIGHT_GREY: Option<Rc<RefCell<BasicColor>>>,
    pub DARK_GREY: Option<Rc<RefCell<BasicColor>>>,
    pub ORANGE: Option<Rc<RefCell<BasicColor>>>,
    pub BROWN: Option<Rc<RefCell<BasicColor>>>,
    pub WHITE: Option<Rc<RefCell<BasicColor>>>,
    pub TRANSPARENT_COLOR: Option<Rc<RefCell<BasicColor>>>,
    pub TRANSPARENT_GREY: Option<Rc<RefCell<BasicColor>>>,
    pub TRANSPARENT_BLACK: Option<Rc<RefCell<BasicColor>>>,
    pub TRANSPARENT_RED: Option<Rc<RefCell<BasicColor>>>,
    pub CLEAR_COLOR: Option<Rc<RefCell<BasicColor>>>,
    pub BORDER_COLOR: Option<Rc<RefCell<BasicColor>>>,
}

thread_local! { pub static BasicColorFactory_instance: Option<Rc<RefCell<BasicColorFactory>>> = BasicColorFactory::new(); }

impl BasicColorFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            NULL_COLOR: None,
            TAN: None,
            SKIN_PINK: None,
            RED: None,
            LIGHT_RED: None,
            DARK_GREEN: None,
            GREEN: None,
            LIGHT_GREEN: None,
            PURPLE: None,
            BLUE: None,
            LIGHT_BLUE: None,
            VERY_LIGHT_BLUE: None,
            PUCE: None,
            AQUA: None,
            YELLOW: None,
            PINK: None,
            BLACK: None,
            GREY: None,
            LIGHT_GREY: None,
            DARK_GREY: None,
            ORANGE: None,
            BROWN: None,
            WHITE: None,
            TRANSPARENT_COLOR: None,
            TRANSPARENT_GREY: None,
            TRANSPARENT_BLACK: None,
            TRANSPARENT_RED: None,
            CLEAR_COLOR: None,
            BORDER_COLOR: None,
        };
        this.NULL_COLOR = this.createInstanceARGB(255, 255, 255, 255, String::from("null color"));
        this.TAN = this.createInstance(0xE9D8B5_u32 as i32);
        this.SKIN_PINK = this.createInstance(0xFDCECE_u32 as i32);
        this.RED = { let java_arguments = (jcall!(BasicColorUtil::BasicColorUtil::getInstance()).ALPHA.clone(), 0xFF0000_u32 as i32, String::from("red"),); this.createInstanceAN(java_arguments.0, java_arguments.1, java_arguments.2) };
        this.LIGHT_RED = { let java_arguments = (jcall!(BasicColorUtil::BasicColorUtil::getInstance()).ALPHA.clone(), 0xFFA0A0_u32 as i32, String::from("light red"),); this.createInstanceAN(java_arguments.0, java_arguments.1, java_arguments.2) };
        this.DARK_GREEN = this.createInstance(0x00FF00_u32 as i32);
        this.GREEN = { let java_arguments = (jcall!(BasicColorUtil::BasicColorUtil::getInstance()).ALPHA.clone(), 0x00DD00_u32 as i32, String::from("green"),); this.createInstanceAN(java_arguments.0, java_arguments.1, java_arguments.2) };
        this.LIGHT_GREEN = this.createInstance(0x007700_u32 as i32);
        this.PURPLE = { let java_arguments = (jcall!(BasicColorUtil::BasicColorUtil::getInstance()).ALPHA.clone(), 0x400040_u32 as i32, String::from("purple"),); this.createInstanceAN(java_arguments.0, java_arguments.1, java_arguments.2) };
        this.BLUE = { let java_arguments = (jcall!(BasicColorUtil::BasicColorUtil::getInstance()).ALPHA.clone(), 0x0000FF_u32 as i32, String::from("blue"),); this.createInstanceAN(java_arguments.0, java_arguments.1, java_arguments.2) };
        this.LIGHT_BLUE = this.createInstance(0x00aaff_u32 as i32);
        this.VERY_LIGHT_BLUE = this.createInstance(0x000033_u32 as i32);
        this.PUCE = this.createInstance(0xFF00AA_u32 as i32);
        this.AQUA = this.createInstance(0x00FFFF_u32 as i32);
        this.YELLOW = { let java_arguments = (jcall!(BasicColorUtil::BasicColorUtil::getInstance()).ALPHA.clone(), 0xFFFF00_u32 as i32, String::from("yellow"),); this.createInstanceAN(java_arguments.0, java_arguments.1, java_arguments.2) };
        this.PINK = this.createInstance(0xFF00FF_u32 as i32);
        this.BLACK = { let java_arguments = (jcall!(BasicColorUtil::BasicColorUtil::getInstance()).ALPHA.clone(), 0x000000_u32 as i32, String::from("black"),); this.createInstanceAN(java_arguments.0, java_arguments.1, java_arguments.2) };
        this.GREY = { let java_arguments = (jcall!(BasicColorUtil::BasicColorUtil::getInstance()).ALPHA.clone(), 0xAAAAAA_u32 as i32, String::from("grey"),); this.createInstanceAN(java_arguments.0, java_arguments.1, java_arguments.2) };
        this.LIGHT_GREY = this.createInstance(0xC0C0C0_u32 as i32);
        this.DARK_GREY = this.createInstance(0x404040_u32 as i32);
        this.ORANGE = this.createInstance(0xe07718_u32 as i32);
        this.BROWN = this.createInstance(0x956B00_u32 as i32);
        this.WHITE = this.createInstance(0xFFFFFF_u32 as i32);
        this.TRANSPARENT_COLOR = { let java_arguments = (0x00000000_u32 as i32, 0x000000_u32 as i32, jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(),); this.createInstanceAN(java_arguments.0, java_arguments.1, java_arguments.2) };
        this.TRANSPARENT_GREY = { let java_arguments = (0x52000000_u32 as i32, 0xECECEC_u32 as i32, jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(),); this.createInstanceAN(java_arguments.0, java_arguments.1, java_arguments.2) };
        this.TRANSPARENT_BLACK = { let java_arguments = (0x52000000_u32 as i32, 0x000000_u32 as i32, jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(),); this.createInstanceAN(java_arguments.0, java_arguments.1, java_arguments.2) };
        this.TRANSPARENT_RED = { let java_arguments = (0x52000000_u32 as i32, 0xFF0000_u32 as i32, jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(),); this.createInstanceAN(java_arguments.0, java_arguments.1, java_arguments.2) };
        this.CLEAR_COLOR = this.BLACK.clone();
        this.BORDER_COLOR = this.WHITE.clone();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<BasicColorFactory>>> {
        return BasicColorFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn createInstance(&mut self, mut value: i32) -> Option<Rc<RefCell<BasicColor>>> {
        return BasicColor::new(jcall!(BasicColorUtil::BasicColorUtil::getInstance()).ALPHA.clone(), value.clone(), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone());
    }
    

    pub fn createInstanceA(&mut self, mut alphaValue: i32, mut value: i32) -> Option<Rc<RefCell<BasicColor>>> {
        return BasicColor::new(alphaValue.clone(), value.clone(), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone());
    }
    

    pub fn createInstanceAN(&mut self, mut alphaValue: i32, mut value: i32, mut name: String) -> Option<Rc<RefCell<BasicColor>>> {
        return BasicColor::new(alphaValue.clone(), value.clone(), name.clone());
    }
    

    pub fn createInstanceARGB(&mut self, mut alphaValue: i32, mut r: i32, mut g: i32, mut b: i32, mut name: String) -> Option<Rc<RefCell<BasicColor>>> {
        let mut ALPHA_MASK: i32 = 0xFF000000_u32 as i32;
        return BasicColor::new(alphaValue.clone(), ((alphaValue << 24) & ALPHA_MASK) + ((r << 16) & 0x00FF0000_u32 as i32) + ((g << 8) & 0x0000FF00_u32 as i32) + (b & 0x000000FF_u32 as i32), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone());
    }
}

