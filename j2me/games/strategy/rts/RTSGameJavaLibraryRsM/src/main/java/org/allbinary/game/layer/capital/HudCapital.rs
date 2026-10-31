// 
// AllBinary Open License Version 1 
// Copyright (c) 2002 AllBinary 
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
// package org.allbinary.game.layer.capital
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::ag::layer::hud::basic::NumberStringHud::NumberStringHud;
use super::Capital::Capital;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct HudCapital {
    pub base: Capital,
    pub capitalGraphic: Option<Rc<RefCell<dyn crate::org::allbinary::game::ag::layer::hud::basic::NumberStringHud::NumberStringHudDyn>>>,
}


impl HudCapital {
    

    pub fn new_struct(mut moneyGraphic: Option<Rc<RefCell<dyn crate::org::allbinary::game::ag::layer::hud::basic::NumberStringHud::NumberStringHudDyn>>>) -> Self {
        let mut this = Self {
            base: Capital::new_struct(),
            capitalGraphic: None,
        };
        this.capitalGraphic = moneyGraphic.clone();
        this
    }

    pub fn new(mut moneyGraphic: Option<Rc<RefCell<dyn crate::org::allbinary::game::ag::layer::hud::basic::NumberStringHud::NumberStringHudDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(moneyGraphic))))
    }
    

    pub fn removeMoney(&mut self, mut points: i32) {
        self.base.removeMoney(points.clone());
        jcall!(self.capitalGraphic).reduce(points.clone());
    }
    

    pub fn getTotalMoney(&mut self) -> i32 {
        return jcall!(self.capitalGraphic).get();
    }
    

    pub fn addMoney(&mut self, mut points: i32) {
        self.base.addMoney(points.clone());
        jcall!(self.capitalGraphic).add(points.clone());
    }
}
impl std::ops::Deref for HudCapital {
    type Target = Capital;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for HudCapital {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::layer::capital::Capital::CapitalDyn for HudCapital {
    fn as_Capital(&self) -> &crate::org::allbinary::game::layer::capital::Capital::Capital { &self.base }
    fn as_Capital_mut(&mut self) -> &mut crate::org::allbinary::game::layer::capital::Capital::Capital { &mut self.base }
    fn getTotalMoney(&mut self) -> i32 { Self::getTotalMoney(self) }
    fn removeMoney(&mut self, mut points: i32) { Self::removeMoney(self, points) }
    fn addMoney(&mut self, mut points: i32) { Self::addMoney(self, points) }
}


