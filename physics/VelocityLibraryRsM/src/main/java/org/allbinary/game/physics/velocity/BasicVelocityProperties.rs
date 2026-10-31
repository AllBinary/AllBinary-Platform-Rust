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
use crate::org::allbinary::direction::DirectionUtil::DirectionUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::math::BasicDecimal::BasicDecimal;
use crate::org::allbinary::logic::math::vector::AxisMathVectorUtil::AxisMathVectorUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::math::Angle::Angle;
use crate::org::allbinary::math::PositionStrings::PositionStrings;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use super::BasicVelocityInterface::BasicVelocityInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicVelocityProperties {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub velocityXBasicDecimal: Option<Rc<RefCell<BasicDecimal>>>,
    pub velocityYBasicDecimal: Option<Rc<RefCell<BasicDecimal>>>,
    pub velocityZBasicDecimal: Option<Rc<RefCell<BasicDecimal>>>,
    pub axisMathVectorUtil: Option<Rc<RefCell<AxisMathVectorUtil>>>,
    pub directionUtil: Option<Rc<RefCell<DirectionUtil>>>,
}

impl BasicVelocityInterface for BasicVelocityProperties {
    fn getVelocityXBasicDecimalP(&mut self) -> Option<Rc<RefCell<BasicDecimal>>> { Self::getVelocityXBasicDecimalP(self) }
    fn getVelocityYBasicDecimalP(&mut self) -> Option<Rc<RefCell<BasicDecimal>>> { Self::getVelocityYBasicDecimalP(self) }
    fn zero(&mut self) { Self::zero(self) }
    fn setVelocityWithBigDecimalAndDirection(&mut self, mut magnitudeBasicDecimal: Option<Rc<RefCell<BasicDecimal>>>, mut direction: Option<Rc<RefCell<Direction>>>, mut otherDirection: Option<Rc<RefCell<Direction>>>) { Self::setVelocityWithBigDecimalAndDirection(self, magnitudeBasicDecimal, direction, otherDirection) }
    fn addVelocityWithBigDecimalAndDirection(&mut self, mut magnitudeBasicDecimal: Option<Rc<RefCell<BasicDecimal>>>, mut direction: Option<Rc<RefCell<Direction>>>, mut otherDirection: Option<Rc<RefCell<Direction>>>) { Self::addVelocityWithBigDecimalAndDirection(self, magnitudeBasicDecimal, direction, otherDirection) }
    fn setVelocityWithDirection(&mut self, mut magnitude: i64, mut direction: Option<Rc<RefCell<Direction>>>, mut otherDirection: Option<Rc<RefCell<Direction>>>) { Self::setVelocityWithDirection(self, magnitude, direction, otherDirection) }
    fn addVelocityWithDirection(&mut self, mut magnitude: i64, mut direction: Option<Rc<RefCell<Direction>>>, mut otherDirection: Option<Rc<RefCell<Direction>>>) { Self::addVelocityWithDirection(self, magnitude, direction, otherDirection) }
    fn setVelocityWithBigDecimal(&mut self, mut magnitudeBasicDecimal: Option<Rc<RefCell<BasicDecimal>>>, mut angle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>, mut otherAngle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>) { Self::setVelocityWithBigDecimal(self, magnitudeBasicDecimal, angle, otherAngle) }
    fn addVelocityWithBigDecimal(&mut self, mut magnitudeBasicDecimal: Option<Rc<RefCell<BasicDecimal>>>, mut angle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>, mut otherAngle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>) { Self::addVelocityWithBigDecimal(self, magnitudeBasicDecimal, angle, otherAngle) }
    fn setVelocity(&mut self, mut magnitude: i64, mut angle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>, mut otherAngle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>) { Self::setVelocity(self, magnitude, angle, otherAngle) }
    fn addVelocity(&mut self, mut magnitude: i64, mut angle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>, mut otherAngle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>) { Self::addVelocity(self, magnitude, angle, otherAngle) }
    fn addVelocityi(&mut self, mut magnitude: i64, mut angle: i32, mut otherAngle: i32) { Self::addVelocityi(self, magnitude, angle, otherAngle) }
}

impl BasicVelocityProperties {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            velocityXBasicDecimal: None,
            velocityYBasicDecimal: None,
            velocityZBasicDecimal: None,
            axisMathVectorUtil: None,
            directionUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.axisMathVectorUtil = AxisMathVectorUtil::getInstance();
        this.directionUtil = DirectionUtil::getInstance();
        this.velocityXBasicDecimal = BasicDecimal::new(0);
        this.velocityYBasicDecimal = BasicDecimal::new(0);
        this.velocityZBasicDecimal = BasicDecimal::new(0);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn zero(&mut self) {
        jcall!(self.velocityXBasicDecimal).setint(0);
        jcall!(self.velocityYBasicDecimal).setint(0);
        jcall!(self.velocityZBasicDecimal).setint(0);
    }
    

    pub fn getVelocityXBasicDecimalP(&mut self) -> Option<Rc<RefCell<BasicDecimal>>> {
        return self.velocityXBasicDecimal.clone();
    }
    

    pub fn getVelocityYBasicDecimalP(&mut self) -> Option<Rc<RefCell<BasicDecimal>>> {
        return self.velocityYBasicDecimal.clone();
    }
    

    pub fn getVelocityZBasicDecimalP(&mut self) -> Option<Rc<RefCell<BasicDecimal>>> {
        return self.velocityZBasicDecimal.clone();
    }
    

    pub fn setVelocityWithBigDecimalAndDirection(&mut self, mut magnitudeBasicDecimal: Option<Rc<RefCell<BasicDecimal>>>, mut direction: Option<Rc<RefCell<Direction>>>, mut otherDirection: Option<Rc<RefCell<Direction>>>) {
        let mut angle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>> = jcall!(self.directionUtil).getAngle(direction.clone());
        let mut otherAngle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>> = jcall!(self.directionUtil).getAngle(otherDirection.clone());
        self.setVelocityWithBigDecimal(magnitudeBasicDecimal.clone(), angle.clone(), otherAngle.clone());
    }
    

    pub fn setVelocityWithDirection(&mut self, mut magnitude: i64, mut direction: Option<Rc<RefCell<Direction>>>, mut otherDirection: Option<Rc<RefCell<Direction>>>) {
        let mut angle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>> = jcall!(self.directionUtil).getAngle(direction.clone());
        let mut otherAngle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>> = jcall!(self.directionUtil).getAngle(otherDirection.clone());
        self.setVelocity(magnitude.clone(), angle.clone(), otherAngle.clone());
    }
    

    pub fn addVelocityWithBigDecimalAndDirection(&mut self, mut magnitudeBasicDecimal: Option<Rc<RefCell<BasicDecimal>>>, mut direction: Option<Rc<RefCell<Direction>>>, mut otherDirection: Option<Rc<RefCell<Direction>>>) {
        let mut angle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>> = jcall!(self.directionUtil).getAngle(direction.clone());
        let mut otherAngle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>> = jcall!(self.directionUtil).getAngle(otherDirection.clone());
        self.addVelocityWithBigDecimal(magnitudeBasicDecimal.clone(), angle.clone(), otherAngle.clone());
    }
    

    pub fn addVelocityWithDirection(&mut self, mut magnitude: i64, mut direction: Option<Rc<RefCell<Direction>>>, mut otherDirection: Option<Rc<RefCell<Direction>>>) {
        let mut angle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>> = jcall!(self.directionUtil).getAngle(direction.clone());
        let mut otherAngle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>> = jcall!(self.directionUtil).getAngle(otherDirection.clone());
        self.addVelocity(magnitude.clone(), angle.clone(), otherAngle.clone());
    }
    

    pub fn setVelocityWithBigDecimal(&mut self, mut magnitudeBasicDecimal: Option<Rc<RefCell<BasicDecimal>>>, mut angle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>, mut otherAngle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>) {
        let mut magnitude: i64 = jcall!(magnitudeBasicDecimal).getUnscaled();
        self.setVelocity(magnitude.clone(), angle.clone(), otherAngle.clone());
    }
    

    pub fn addVelocityWithBigDecimal(&mut self, mut magnitudeBasicDecimal: Option<Rc<RefCell<BasicDecimal>>>, mut angle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>, mut otherAngle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>) {
        let mut magnitude: i64 = jcall!(magnitudeBasicDecimal).getUnscaled();
        self.addVelocity(magnitude.clone(), angle.clone(), otherAngle.clone());
    }
    

    pub fn setVelocity(&mut self, mut magnitude: i64, mut angle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>, mut otherAngle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>) {
        { let java_arguments = (magnitude.clone(), ((jcall!(angle).getValue()) as i32), ((jcall!(otherAngle).getValue()) as i32),); self.setVelocityi(java_arguments.0, java_arguments.1, java_arguments.2) };
    }
    

    pub fn addVelocity(&mut self, mut magnitude: i64, mut angle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>, mut otherAngle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>) {
        { let java_arguments = (magnitude.clone(), ((jcall!(angle).getValue()) as i32), ((jcall!(otherAngle).getValue()) as i32),); self.addVelocityi(java_arguments.0, java_arguments.1, java_arguments.2) };
    }
    

    pub fn setVelocityi(&mut self, mut magnitude: i64, mut angle: i32, mut otherAngle: i32) {
        let mut xVector: i64 = (jcall!(self.axisMathVectorUtil).calculateX(magnitude.clone(), angle.clone()) / ((jcall!(self.velocityXBasicDecimal).getScaledFactorValue()) as i64));
        let mut yVector: i64 = (jcall!(self.axisMathVectorUtil).calculateY(magnitude.clone(), angle.clone()) / ((jcall!(self.velocityYBasicDecimal).getScaledFactorValue()) as i64));
        let mut zVector: i64 = (jcall!(self.axisMathVectorUtil).calculateZ(magnitude.clone(), otherAngle.clone()) / ((jcall!(self.velocityZBasicDecimal).getScaledFactorValue()) as i64));
        jcall!(self.velocityXBasicDecimal).setlong(xVector.clone());
        jcall!(self.velocityYBasicDecimal).setlong(yVector.clone());
        jcall!(self.velocityZBasicDecimal).setlong(zVector.clone());
    }
    

    pub fn addVelocityi(&mut self, mut magnitude: i64, mut angle: i32, mut otherAngle: i32) {
        let mut xVector: i64 = (jcall!(self.axisMathVectorUtil).calculateX(magnitude.clone(), angle.clone()) / ((jcall!(self.velocityXBasicDecimal).getScaledFactorValue()) as i64));
        let mut yVector: i64 = (jcall!(self.axisMathVectorUtil).calculateY(magnitude.clone(), angle.clone()) / ((jcall!(self.velocityYBasicDecimal).getScaledFactorValue()) as i64));
        let mut zVector: i64 = (jcall!(self.axisMathVectorUtil).calculateZ(magnitude.clone(), otherAngle.clone()) / ((jcall!(self.velocityZBasicDecimal).getScaledFactorValue()) as i64));
        jcall!(self.velocityXBasicDecimal).addlong(xVector.clone());
        jcall!(self.velocityYBasicDecimal).addlong(yVector.clone());
        jcall!(self.velocityZBasicDecimal).addlong(zVector.clone());
    }
    

    pub fn toString(&mut self) -> String {
        let mut commonSeps: Option<Rc<RefCell<CommonSeps>>> = CommonSeps::getInstance();
        let mut positionStrings: Option<Rc<RefCell<PositionStrings>>> = PositionStrings::getInstance();
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut stringUtil: Option<Rc<RefCell<StringUtil>>> = StringUtil::getInstance();
        jcall!(stringBuffer).append(String::from("Velocity "));
        jcall!(stringBuffer).append(jcall!(positionStrings).X_LABEL.clone());
        jcall!(stringBuffer).append(jcall!(stringUtil).toString(crate::java_into_object(self.velocityXBasicDecimal.clone())));
        jcall!(stringBuffer).append(jcall!(commonSeps).SPACE.clone());
        jcall!(stringBuffer).append(jcall!(positionStrings).Y_LABEL.clone());
        jcall!(stringBuffer).append(jcall!(stringUtil).toString(crate::java_into_object(self.velocityYBasicDecimal.clone())));
        jcall!(stringBuffer).append(jcall!(commonSeps).SPACE.clone());
        jcall!(stringBuffer).append(jcall!(positionStrings).Z_LABEL.clone());
        jcall!(stringBuffer).append(jcall!(stringUtil).toString(crate::java_into_object(self.velocityZBasicDecimal.clone())));
        return jcall!(stringBuffer).toString();
    }
}
pub trait BasicVelocityPropertiesDyn: crate::org::allbinary::game::physics::velocity::BasicVelocityInterface::BasicVelocityInterface {
    fn as_BasicVelocityProperties(&self) -> &BasicVelocityProperties;
    fn as_BasicVelocityProperties_mut(&mut self) -> &mut BasicVelocityProperties;
    fn getVelocityZBasicDecimalP(&mut self) -> Option<Rc<RefCell<BasicDecimal>>> { BasicVelocityProperties::getVelocityZBasicDecimalP(self.as_BasicVelocityProperties_mut()) }
    fn setVelocityi(&mut self, mut magnitude: i64, mut angle: i32, mut otherAngle: i32) { BasicVelocityProperties::setVelocityi(self.as_BasicVelocityProperties_mut(), magnitude, angle, otherAngle) }
}
impl BasicVelocityPropertiesDyn for BasicVelocityProperties {
    fn as_BasicVelocityProperties(&self) -> &BasicVelocityProperties { self }
    fn as_BasicVelocityProperties_mut(&mut self) -> &mut BasicVelocityProperties { self }
}


