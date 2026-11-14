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
// package org.allbinary.direction
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::math::Angle::Angle;
use crate::org::allbinary::math::AngleFactory::AngleFactory;
use super::DirectionFactory::DirectionFactory;
use super::Direction::Direction;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct DirectionUtil {
    pub directionArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>>,
    pub directionArray2: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>>,
}

thread_local! { pub static DirectionUtil_instance: Option<Rc<RefCell<DirectionUtil>>> = DirectionUtil::new(); }

impl DirectionUtil {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<DirectionUtil>>> {
        return DirectionUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            directionArray: Vec::new(),
            directionArray2: Vec::new(),
        };
        this.directionArray = vec![None; (10) as usize];
        this.directionArray2 = vec![None; (10) as usize];
        let mut angleFactory: Option<Rc<RefCell<AngleFactory>>> = AngleFactory::getInstance();
        let mut directionFactory: Option<Rc<RefCell<DirectionFactory>>> = DirectionFactory::getInstance();
        { let java_index = (jcall!({ let java_field = jcall!(directionFactory).LEFT.clone(); java_field }).getValue()) as usize; this.directionArray[java_index] = { let java_field = jcall!(angleFactory).LEFT.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>); };
        { let java_index = (jcall!({ let java_field = jcall!(directionFactory).RIGHT.clone(); java_field }).getValue()) as usize; this.directionArray[java_index] = { let java_field = jcall!(angleFactory).RIGHT.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>); };
        { let java_index = (jcall!({ let java_field = jcall!(directionFactory).UP.clone(); java_field }).getValue()) as usize; this.directionArray[java_index] = { let java_field = jcall!(angleFactory).UP.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>); };
        { let java_index = (jcall!({ let java_field = jcall!(directionFactory).DOWN.clone(); java_field }).getValue()) as usize; this.directionArray[java_index] = { let java_field = jcall!(angleFactory).DOWN.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>); };
        { let java_index = (jcall!({ let java_field = jcall!(directionFactory).LEFT.clone(); java_field }).getValue()) as usize; this.directionArray2[java_index] = { let java_field = jcall!(angleFactory).LEFT.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>); };
        { let java_index = (jcall!({ let java_field = jcall!(directionFactory).RIGHT.clone(); java_field }).getValue()) as usize; this.directionArray2[java_index] = { let java_field = jcall!(angleFactory).RIGHT.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>); };
        { let java_index = (jcall!({ let java_field = jcall!(directionFactory).UP.clone(); java_field }).getValue()) as usize; this.directionArray2[java_index] = { let java_field = jcall!(angleFactory).UP.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>); };
        { let java_index = (jcall!({ let java_field = jcall!(directionFactory).DOWN.clone(); java_field }).getValue()) as usize; this.directionArray2[java_index] = { let java_field = jcall!(angleFactory).DOWN.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>); };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getAngle(&mut self, mut direction: Option<Rc<RefCell<Direction>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>> {
        return self.directionArray2[(jcall!(direction).getValue()) as usize].clone();
    }
    

    pub fn getFrameAngle(&mut self, mut direction: Option<Rc<RefCell<Direction>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>> {
        return self.directionArray[(jcall!(direction).getValue()) as usize].clone();
    }
}

