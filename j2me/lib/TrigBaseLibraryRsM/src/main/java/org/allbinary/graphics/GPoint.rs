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
// package org.allbinary.graphics
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::jsinterop::annotations::JsType;
use crate::jsinterop::annotations::JsMethod;
use crate::jsinterop::annotations::JsConstructor;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::math::PositionStrings::PositionStrings;
use crate::org::allbinary::string::CommonSeps::CommonSeps;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GPoint {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

thread_local! { pub static GPoint_POINT_LABEL: String = String::from("Point: "); }

impl GPoint {
    

    pub fn getInstance(mut point: Option<Rc<RefCell<dyn GPointDyn>>>) -> Option<Rc<RefCell<dyn GPointDyn>>> {
        return GPoint::new(jcall!(point).getX(), jcall!(point).getY(), jcall!(point).getZ()).map(|reference| reference as Rc<RefCell<dyn GPointDyn>>);
    }
    

    pub fn new_struct(mut x: i32, mut y: i32, mut z: i32) -> Self {
        let mut this = Self {
            x: 0,
            y: 0,
            z: 0,
        };
        this.x = x.clone();
        this.y = y.clone();
        this.z = z.clone();
        this
    }

    pub fn new(mut x: i32, mut y: i32, mut z: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(x, y, z))))
    }
    

    pub fn getX(&mut self) -> i32 {
        return self.x.clone();
    }
    

    pub fn getY(&mut self) -> i32 {
        return self.y.clone();
    }
    

    pub fn getZ(&mut self) -> i32 {
        return self.z.clone();
    }
    

    pub fn getRawX(&mut self) -> i32 {
        return self.x.clone();
    }
    

    pub fn getRawY(&mut self) -> i32 {
        return self.y.clone();
    }
    

    pub fn getRawZ(&mut self) -> i32 {
        return self.z.clone();
    }
    

    pub fn toString(&mut self) -> String {
        return GPoint::toStringStatic(self.getX(), self.getY(), self.getZ());
    }
    

    pub fn toStringStatic(mut x: i32, mut y: i32, mut z: i32) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut positionStrings: Option<Rc<RefCell<PositionStrings>>> = PositionStrings::getInstance();
        jcall!(stringBuffer).append(GPoint_POINT_LABEL.with(Clone::clone).clone());
        jcall!(stringBuffer).append(jcall!(positionStrings).X_LABEL.clone());
        jcall!(stringBuffer).appendint(x.clone());
        jcall!(stringBuffer).append(jcall!(CommonSeps::getInstance()).SPACE.clone());
        jcall!(stringBuffer).append(jcall!(positionStrings).Y_LABEL.clone());
        jcall!(stringBuffer).appendint(y.clone());
        jcall!(stringBuffer).append(jcall!(CommonSeps::getInstance()).SPACE.clone());
        jcall!(stringBuffer).append(jcall!(positionStrings).Z_LABEL.clone());
        jcall!(stringBuffer).appendint(z.clone());
        return jcall!(stringBuffer).toString();
    }
}
pub trait GPointDyn {
    fn as_GPoint(&self) -> &GPoint;
    fn as_GPoint_mut(&mut self) -> &mut GPoint;
    fn getX(&mut self) -> i32 { GPoint::getX(self.as_GPoint_mut()) }
    fn getY(&mut self) -> i32 { GPoint::getY(self.as_GPoint_mut()) }
    fn getZ(&mut self) -> i32 { GPoint::getZ(self.as_GPoint_mut()) }
    fn getRawX(&mut self) -> i32 { GPoint::getRawX(self.as_GPoint_mut()) }
    fn getRawY(&mut self) -> i32 { GPoint::getRawY(self.as_GPoint_mut()) }
    fn getRawZ(&mut self) -> i32 { GPoint::getRawZ(self.as_GPoint_mut()) }
    fn toString(&mut self) -> String { GPoint::toString(self.as_GPoint_mut()) }
}
impl GPointDyn for GPoint {
    fn as_GPoint(&self) -> &GPoint { self }
    fn as_GPoint_mut(&mut self) -> &mut GPoint { self }
}


