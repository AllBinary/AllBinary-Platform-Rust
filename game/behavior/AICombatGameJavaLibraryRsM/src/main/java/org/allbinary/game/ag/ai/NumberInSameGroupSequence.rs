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
// package org.allbinary.game.ag.ai
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::math::SmallIntegerSingletonFactory::SmallIntegerSingletonFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NumberInSameGroupSequence {
    pub NUMBER_ON_SAME_TEAM_SEQUENCE_ZERO: Option<i32>,
    pub NUMBER_ON_SAME_TEAM_SEQUENCE_ONE: Option<i32>,
    pub NUMBER_ON_SAME_TEAM_SEQUENCE_TWO: Option<i32>,
    pub NUMBER_ON_SAME_TEAM_SEQUENCE_THREE: Option<i32>,
    pub NUMBER_ON_SAME_TEAM_SEQUENCE_FOUR: Option<i32>,
    pub NUMBER_ON_SAME_TEAM_SEQUENCE_FIVE: Option<i32>,
    pub NUMBER_ON_SAME_TEAM_SEQUENCE_SIX: Option<i32>,
    pub NUMBER_ON_SAME_TEAM_SEQUENCE_SEVEN: Option<i32>,
    pub NUMBER_ON_SAME_TEAM_SEQUENCE_EIGHT: Option<i32>,
    pub NUMBER_ON_SAME_TEAM_SEQUENCE_NINE: Option<i32>,
    pub NUMBER_ON_SAME_TEAM_SEQUENCE: Vec<Option<i32>>,
}


impl NumberInSameGroupSequence {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<NumberInSameGroupSequence>>> {
        return NumberInSameGroupSequence::new();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            NUMBER_ON_SAME_TEAM_SEQUENCE_ZERO: None,
            NUMBER_ON_SAME_TEAM_SEQUENCE_ONE: None,
            NUMBER_ON_SAME_TEAM_SEQUENCE_TWO: None,
            NUMBER_ON_SAME_TEAM_SEQUENCE_THREE: None,
            NUMBER_ON_SAME_TEAM_SEQUENCE_FOUR: None,
            NUMBER_ON_SAME_TEAM_SEQUENCE_FIVE: None,
            NUMBER_ON_SAME_TEAM_SEQUENCE_SIX: None,
            NUMBER_ON_SAME_TEAM_SEQUENCE_SEVEN: None,
            NUMBER_ON_SAME_TEAM_SEQUENCE_EIGHT: None,
            NUMBER_ON_SAME_TEAM_SEQUENCE_NINE: None,
            NUMBER_ON_SAME_TEAM_SEQUENCE: Vec::new(),
        };
        let mut smallIntegerSingletonFactory: Option<Rc<RefCell<SmallIntegerSingletonFactory>>> = SmallIntegerSingletonFactory::getInstance();
        this.NUMBER_ON_SAME_TEAM_SEQUENCE_ZERO = { let java_value = (jcall!(smallIntegerSingletonFactory).getAt(100)).clone(); java_value };
        this.NUMBER_ON_SAME_TEAM_SEQUENCE_ONE = { let java_value = (jcall!(smallIntegerSingletonFactory).getAt(101)).clone(); java_value };
        this.NUMBER_ON_SAME_TEAM_SEQUENCE_TWO = { let java_value = (jcall!(smallIntegerSingletonFactory).getAt(102)).clone(); java_value };
        this.NUMBER_ON_SAME_TEAM_SEQUENCE_THREE = { let java_value = (jcall!(smallIntegerSingletonFactory).getAt(103)).clone(); java_value };
        this.NUMBER_ON_SAME_TEAM_SEQUENCE_FOUR = { let java_value = (jcall!(smallIntegerSingletonFactory).getAt(104)).clone(); java_value };
        this.NUMBER_ON_SAME_TEAM_SEQUENCE_FIVE = { let java_value = (jcall!(smallIntegerSingletonFactory).getAt(105)).clone(); java_value };
        this.NUMBER_ON_SAME_TEAM_SEQUENCE_SIX = { let java_value = (jcall!(smallIntegerSingletonFactory).getAt(106)).clone(); java_value };
        this.NUMBER_ON_SAME_TEAM_SEQUENCE_SEVEN = { let java_value = (jcall!(smallIntegerSingletonFactory).getAt(107)).clone(); java_value };
        this.NUMBER_ON_SAME_TEAM_SEQUENCE_EIGHT = { let java_value = (jcall!(smallIntegerSingletonFactory).getAt(108)).clone(); java_value };
        this.NUMBER_ON_SAME_TEAM_SEQUENCE_NINE = { let java_value = (jcall!(smallIntegerSingletonFactory).getAt(109)).clone(); java_value };
        this.NUMBER_ON_SAME_TEAM_SEQUENCE = vec![this.NUMBER_ON_SAME_TEAM_SEQUENCE_ZERO.clone(), this.NUMBER_ON_SAME_TEAM_SEQUENCE_ONE.clone(), this.NUMBER_ON_SAME_TEAM_SEQUENCE_THREE.clone(), this.NUMBER_ON_SAME_TEAM_SEQUENCE_FOUR.clone(), this.NUMBER_ON_SAME_TEAM_SEQUENCE_FIVE.clone(), this.NUMBER_ON_SAME_TEAM_SEQUENCE_SIX.clone(), this.NUMBER_ON_SAME_TEAM_SEQUENCE_SEVEN.clone(), this.NUMBER_ON_SAME_TEAM_SEQUENCE_EIGHT.clone(), this.NUMBER_ON_SAME_TEAM_SEQUENCE_NINE.clone()];
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

