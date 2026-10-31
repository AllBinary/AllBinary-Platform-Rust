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
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonPhoneStrings::CommonPhoneStrings;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::jsinterop::annotations::JsMethod;
use crate::jsinterop::annotations::JsConstructor;
use crate::jsinterop::annotations::JsProperty;
use super::Angle::Angle;
use super::NamedAngle::NamedAngle;
use super::FrameUtil::FrameUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AngleFactory {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub TOTAL_ANGLE: i16,
    pub QUARTER_TOTAL_ANGLE: i16,
    pub angleArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>>,
    pub NEGATIVE_ONE: i16,
    pub NOT_ANGLE: Option<Rc<RefCell<NamedAngle>>>,
    pub DOWN: Option<Rc<RefCell<NamedAngle>>>,
    pub UP: Option<Rc<RefCell<NamedAngle>>>,
    pub LEFT: Option<Rc<RefCell<NamedAngle>>>,
    pub RIGHT: Option<Rc<RefCell<NamedAngle>>>,
    pub frameUtil: Option<Rc<RefCell<FrameUtil>>>,
}

thread_local! { pub static AngleFactory_instance: Option<Rc<RefCell<AngleFactory>>> = AngleFactory::new(); }

impl AngleFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<AngleFactory>>> {
        return AngleFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            TOTAL_ANGLE: 0,
            QUARTER_TOTAL_ANGLE: 0,
            angleArray: Vec::new(),
            NEGATIVE_ONE: 0,
            NOT_ANGLE: None,
            DOWN: None,
            UP: None,
            LEFT: None,
            RIGHT: None,
            frameUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.TOTAL_ANGLE = 360;
        this.QUARTER_TOTAL_ANGLE = 90;
        this.angleArray = vec![None; (((this.TOTAL_ANGLE) as i32)) as usize];
        this.NEGATIVE_ONE = ((-1) as i16);
        this.NOT_ANGLE = NamedAngle::new(this.NEGATIVE_ONE.clone(), jcall!(CommonStrings::getInstance()).EMPTY.clone());
        this.frameUtil = FrameUtil::getInstance();
        let mut commonPhoneStrings: Option<Rc<RefCell<CommonPhoneStrings>>> = CommonPhoneStrings::getInstance();
        let mut total: i16 = (((this.angleArray.len() as i32)) as i16);
        this.UP = NamedAngle::new(((0) as i16), jcall!(commonPhoneStrings).UP.clone());
        this.angleArray[(0) as usize] = this.UP.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>);
        for index in (1..90) {
            this.angleArray[(index) as usize] = Angle::new(((index) as i16)).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>);
        }
        this.RIGHT = NamedAngle::new(((90) as i16), jcall!(commonPhoneStrings).RIGHT.clone());
        this.angleArray[(90) as usize] = this.RIGHT.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>);
        for index in (91..180) {
            this.angleArray[(index) as usize] = Angle::new(((index) as i16)).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>);
        }
        this.DOWN = NamedAngle::new(((180) as i16), jcall!(commonPhoneStrings).DOWN.clone());
        this.angleArray[(180) as usize] = this.DOWN.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>);
        for index in (181..270) {
            this.angleArray[(index) as usize] = Angle::new(((index) as i16)).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>);
        }
        this.LEFT = NamedAngle::new(((270) as i16), jcall!(commonPhoneStrings).LEFT.clone());
        this.angleArray[(270) as usize] = this.LEFT.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>);
        for index in (271..total) {
            this.angleArray[(index) as usize] = Angle::new(((index) as i16)).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>);
        }
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getAt(&mut self, mut index: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>> {
        let mut adjustedIndex: i32 = jcall!(self.frameUtil).adjustAngleToFrameAngle(index.clone());
        return self.angleArray[(adjustedIndex) as usize].clone();
    }
    

    pub fn getClosestDirection(&mut self, mut angle: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>> {
        if (angle >= 315 && angle < 360) || (angle >= 0 && angle < 45) {
            return self.UP.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>);
        } else if angle >= 45 && angle < 135 {
            return self.RIGHT.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>);
        } else if angle >= 135 && angle < 225 {
            return self.DOWN.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>);
        } else if angle >= 225 && angle < 315 {
            return self.LEFT.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>);
        }
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getGeneralDirection(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>> {
        return self.NOT_ANGLE.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>);
    }
    

    pub fn main(mut args: Vec<String>) {
        let mut stringMaker: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut angleFactory: Option<Rc<RefCell<AngleFactory>>> = AngleFactory::getInstance();
        for index in (0..360) {
            jcall!(stringMaker).appendint(index.clone()).append(jcall!(CommonSeps::getInstance()).FORWARD_SLASH.clone()).appendshort(((jcall!(jcall!(angleFactory).getClosestDirection(index.clone())).getValue()) as i16)).append(jcall!(CommonSeps::getInstance()).NEW_LINE.clone());
        }
        jcall!(LogUtil::getInstance()).putF(jcall!(stringMaker).toString(), crate::java_into_object(String::from("main")), String::from("main"));
    }
}

