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
// package org.allbinary.game.physics.velocity
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::direction::Direction::Direction;
use crate::org::allbinary::logic::math::BasicDecimal::BasicDecimal;
use crate::org::allbinary::math::Angle::Angle;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait BasicVelocityInterface {
    

    fn getVelocityXBasicDecimalP(&mut self) -> Option<Rc<RefCell<BasicDecimal>>>;
    

    fn getVelocityYBasicDecimalP(&mut self) -> Option<Rc<RefCell<BasicDecimal>>>;
    

    fn zero(&mut self);
    

    fn setVelocityWithBigDecimalAndDirection(&mut self, magnitudeBasicDecimal: Option<Rc<RefCell<BasicDecimal>>>, direction: Option<Rc<RefCell<Direction>>>, otherDirection: Option<Rc<RefCell<Direction>>>);
    

    fn addVelocityWithBigDecimalAndDirection(&mut self, magnitudeBasicDecimal: Option<Rc<RefCell<BasicDecimal>>>, direction: Option<Rc<RefCell<Direction>>>, otherDirection: Option<Rc<RefCell<Direction>>>);
    

    fn setVelocityWithDirection(&mut self, magnitude: i64, direction: Option<Rc<RefCell<Direction>>>, otherDirection: Option<Rc<RefCell<Direction>>>);
    

    fn addVelocityWithDirection(&mut self, magnitude: i64, direction: Option<Rc<RefCell<Direction>>>, otherDirection: Option<Rc<RefCell<Direction>>>);
    

    fn setVelocityWithBigDecimal(&mut self, magnitudeBasicDecimal: Option<Rc<RefCell<BasicDecimal>>>, angle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>, otherAngle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>);
    

    fn addVelocityWithBigDecimal(&mut self, magnitudeBasicDecimal: Option<Rc<RefCell<BasicDecimal>>>, angle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>, otherAngle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>);
    

    fn setVelocity(&mut self, magnitude: i64, angle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>, otherAngle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>);
    

    fn addVelocity(&mut self, magnitude: i64, angle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>, otherAngle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>);
    

    fn addVelocityi(&mut self, magnitude: i64, angle: i32, otherAngle: i32);

    fn toString(&mut self) -> String { String::from(std::any::type_name::<Self>()) }
}


