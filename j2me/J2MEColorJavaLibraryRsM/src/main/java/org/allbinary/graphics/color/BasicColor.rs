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
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::jsinterop::annotations::JsMethod;
use crate::jsinterop::annotations::JsConstructor;
use crate::jsinterop::annotations::JsProperty;
use super::BasicColorUtil::BasicColorUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicColor {
    pub name: String,
    pub value: i32,
    pub redComponent: f32,
    pub greenComponent: f32,
    pub blueComponent: f32,
    pub alphaComponent: f32,
    pub red: i16,
    pub green: i16,
    pub blue: i16,
    pub alpha: i16,
}


impl BasicColor {
    

    pub fn new_struct(mut alphaValue: i32, mut value: i32, mut name: String) -> Self {
        let mut this = Self {
            name: String::new(),
            value: 0,
            redComponent: 0.0_f32,
            greenComponent: 0.0_f32,
            blueComponent: 0.0_f32,
            alphaComponent: 0.0_f32,
            red: 0,
            green: 0,
            blue: 0,
            alpha: 0,
        };
        this.name = name.clone();
        let mut tempValue: i32 = 0;
        let mut basicColorUtil: Option<Rc<RefCell<BasicColorUtil>>> = BasicColorUtil::getInstance();
        if jcall!(basicColorUtil).isAlpha {
            if jcall!(basicColorUtil).ffOpaque {
                tempValue = alphaValue | value;
            } else {
                tempValue = 0x00000000_u32 as i32 | value;
            }
        } else {
            tempValue = value.clone();
        }
        let mut alphaInt: i32 = ((tempValue >> 24) & 255);
        this.alpha = ((alphaInt) as i16);
        this.alphaComponent = (((alphaInt) as f32)) / 255.0_f32;
        let mut redInt: i32 = ((tempValue >> 16) & 255);
        this.red = ((redInt) as i16);
        this.redComponent = (((redInt) as f32)) / 255.0_f32;
        let mut greenInt: i32 = ((tempValue >> 8) & 255);
        this.green = ((greenInt) as i16);
        this.greenComponent = (((greenInt) as f32)) / 255.0_f32;
        let mut blueInt: i32 = ((tempValue) & 255);
        this.blue = ((blueInt) as i16);
        this.blueComponent = (((blueInt) as f32)) / 255.0_f32;
        this.value = tempValue.clone();
        this
    }

    pub fn new(mut alphaValue: i32, mut value: i32, mut name: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(alphaValue, value, name))))
    }
    

    pub fn intValue(&mut self) -> i32 {
        return self.value.clone();
    }
    

    pub fn toInt(&mut self) -> i32 {
        return self.value.clone();
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(String::from("BasicColor: "));
        jcall!(stringBuffer).append(String::from("Alpha: "));
        jcall!(stringBuffer).appendshort(self.alpha.clone());
        jcall!(stringBuffer).append(String::from(" Red: "));
        jcall!(stringBuffer).appendshort(self.red.clone());
        jcall!(stringBuffer).append(String::from(" Green: "));
        jcall!(stringBuffer).appendshort(self.green.clone());
        jcall!(stringBuffer).append(String::from(" Blue: "));
        jcall!(stringBuffer).appendshort(self.blue.clone());
        jcall!(stringBuffer).append(String::from(" Value: "));
        jcall!(stringBuffer).appendint(self.value.clone());
        return jcall!(stringBuffer).toString();
    }
    

    pub fn getName(&mut self) -> String {
        return self.name.clone();
    }
    

    pub fn getRedComponent(&mut self) -> f32 {
        return self.redComponent.clone();
    }
    

    pub fn getGreenComponent(&mut self) -> f32 {
        return self.greenComponent.clone();
    }
    

    pub fn getBlueComponent(&mut self) -> f32 {
        return self.blueComponent.clone();
    }
    

    pub fn getAlphaComponent(&mut self) -> f32 {
        return self.alphaComponent.clone();
    }
}

