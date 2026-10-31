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
// package org.allbinary.android.input.motion
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer;
use crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelper;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AnalogControllerHelper {
    pub reduceTimeFactor: i32,
    pub SCALE_VALUE: i32,
    pub rightTimeElapsedHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>>,
    pub leftTimeElapsedHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>>,
    pub downTimeElapsedHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>>,
    pub upTimeElapsedHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>>,
}


impl AnalogControllerHelper {
    

    pub fn new_struct(mut reduceTimeFactor: i32) -> Self {
        let mut this = Self {
            reduceTimeFactor: 0,
            SCALE_VALUE: 0,
            rightTimeElapsedHelper: None,
            leftTimeElapsedHelper: None,
            downTimeElapsedHelper: None,
            upTimeElapsedHelper: None,
        };
        this.SCALE_VALUE = jcall!(AnalogControllerConfigurationFactory::AnalogControllerConfigurationFactory::getInstance()).SCALE_VALUE.clone();
        this.rightTimeElapsedHelper = TimeDelayHelper::new(0).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>);
        this.leftTimeElapsedHelper = TimeDelayHelper::new(0).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>);
        this.downTimeElapsedHelper = TimeDelayHelper::new(0).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>);
        this.upTimeElapsedHelper = TimeDelayHelper::new(0).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>);
        this.reduceTimeFactor = reduceTimeFactor.clone();
        this
    }

    pub fn new(mut reduceTimeFactor: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(reduceTimeFactor))))
    }
    

    // throws Exception
    pub fn right(&mut self, mut collidableDestroyableDamageableLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>, mut xAnalogValue: i32) {
        jcall!(self.rightTimeElapsedHelper).as_TimeDelayHelper_mut().delay = ((self.SCALE_VALUE - xAnalogValue) >> self.reduceTimeFactor);
        if jcall!(self.rightTimeElapsedHelper).isTimeTNT() {
            jcall!(collidableDestroyableDamageableLayer).right();
        }
    }
    

    // throws Exception
    pub fn left(&mut self, mut collidableDestroyableDamageableLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>, mut xAnalogValue: i32) {
        jcall!(self.leftTimeElapsedHelper).as_TimeDelayHelper_mut().delay = ((self.SCALE_VALUE + xAnalogValue) >> self.reduceTimeFactor);
        if jcall!(self.leftTimeElapsedHelper).isTimeTNT() {
            jcall!(collidableDestroyableDamageableLayer).left();
        }
    }
    

    // throws Exception
    pub fn up(&mut self, mut collidableDestroyableDamageableLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>, mut xAnalogValue: i32) {
        jcall!(self.upTimeElapsedHelper).as_TimeDelayHelper_mut().delay = ((self.SCALE_VALUE - xAnalogValue) >> self.reduceTimeFactor);
        if jcall!(self.upTimeElapsedHelper).isTimeTNT() {
            jcall!(collidableDestroyableDamageableLayer).up();
        }
    }
    

    // throws Exception
    pub fn down(&mut self, mut collidableDestroyableDamageableLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>, mut xAnalogValue: i32) {
        jcall!(self.downTimeElapsedHelper).as_TimeDelayHelper_mut().delay = ((self.SCALE_VALUE + xAnalogValue) >> self.reduceTimeFactor);
        if jcall!(self.downTimeElapsedHelper).isTimeTNT() {
            jcall!(collidableDestroyableDamageableLayer).down();
        }
    }
}

