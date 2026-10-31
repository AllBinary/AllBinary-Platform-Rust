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
// package org.allbinary.logic.math.vector
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::math::BasicDecimal::BasicDecimal;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::math::PositionStrings::PositionStrings;
use crate::org::allbinary::string::CommonSeps::CommonSeps;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct Vertex {
    pub x: Option<Rc<RefCell<BasicDecimal>>>,
    pub y: Option<Rc<RefCell<BasicDecimal>>>,
    pub z: Option<Rc<RefCell<BasicDecimal>>>,
}


impl Vertex {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            x: None,
            y: None,
            z: None,
        };
        this.x = BasicDecimal::new(0);
        this.y = BasicDecimal::new(0);
        this.z = BasicDecimal::new(0);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut positionStrings: Option<Rc<RefCell<PositionStrings>>> = PositionStrings::getInstance();
        jcall!(stringBuffer).append(jcall!(positionStrings).X_LABEL.clone());
        jcall!(stringBuffer).append(jcall!(self.x).toString());
        jcall!(stringBuffer).append(jcall!(CommonSeps::getInstance()).SPACE.clone());
        jcall!(stringBuffer).append(jcall!(positionStrings).Y_LABEL.clone());
        jcall!(stringBuffer).append(jcall!(self.y).toString());
        jcall!(stringBuffer).append(jcall!(CommonSeps::getInstance()).SPACE.clone());
        jcall!(stringBuffer).append(jcall!(positionStrings).Z_LABEL.clone());
        jcall!(stringBuffer).append(jcall!(self.z).toString());
        return jcall!(stringBuffer).toString();
    }
}

