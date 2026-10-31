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
// package org.allbinary.math
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::jsinterop::annotations::JsType;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::jsinterop::annotations::JsMethod;
use crate::jsinterop::annotations::JsConstructor;
use super::AngleIncrementInfo::AngleIncrementInfo;
use super::FrameUtil::FrameUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AngleInfo {
    pub angleIncrementInfo: Option<Rc<RefCell<AngleIncrementInfo>>>,
    pub angle: i16,
    pub frameUtil: Option<Rc<RefCell<FrameUtil>>>,
}

thread_local! { pub static AngleInfo_ANGLE: String = String::from("Angle: "); }

impl AngleInfo {
    

    pub fn getInstance(mut angleIncrement: i16) -> Option<Rc<RefCell<AngleInfo>>> {
        return AngleInfo::new(jcall!(AngleIncrementInfoFactory::AngleIncrementInfoFactory::getInstance()).getAt(angleIncrement.clone()));
    }
    

    pub fn new_struct(mut angleIncrementInfo: Option<Rc<RefCell<AngleIncrementInfo>>>) -> Self {
        let mut this = Self {
            angleIncrementInfo: None,
            angle: 0,
            frameUtil: None,
        };
        this.angle = 0;
        this.frameUtil = FrameUtil::getInstance();
        this.angleIncrementInfo = angleIncrementInfo.clone();
        this
    }

    pub fn new(mut angleIncrementInfo: Option<Rc<RefCell<AngleIncrementInfo>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(angleIncrementInfo))))
    }
    

    pub fn adjustAngle(&mut self, mut frame: i32) {
        let mut newAngle: i32 = ((jcall!(self.angleIncrementInfo).getAngleIncrement()) as i32) * frame - 90;
        { let java_arguments = (((jcall!(self.frameUtil).adjustAngleToFrameAngle(newAngle.clone())) as i16),); self.setAngle(java_arguments.0) };
    }
    

    pub fn getAngle(&mut self) -> i16 {
        return self.angle.clone();
    }
    

    pub fn setAngle(&mut self, mut angle: i16) {
        self.angle = angle.clone();
    }
    

    pub fn getAngleIncrementInfo(&mut self) -> Option<Rc<RefCell<AngleIncrementInfo>>> {
        return self.angleIncrementInfo.clone();
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(AngleInfo_ANGLE.with(Clone::clone).clone());
        jcall!(stringBuffer).appendshort(self.angle.clone());
        jcall!(stringBuffer).append(jcall!(CommonSeps::getInstance()).SPACE.clone());
        jcall!(stringBuffer).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(self.getAngleIncrementInfo())));
        return jcall!(stringBuffer).toString();
    }
}

