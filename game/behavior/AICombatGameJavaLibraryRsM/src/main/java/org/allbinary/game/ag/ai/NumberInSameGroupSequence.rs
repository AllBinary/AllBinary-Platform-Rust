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
        this.NUMBER_ON_SAME_TEAM_SEQUENCE_ZERO = jcall!(smallIntegerSingletonFactory).getAt(100);
        this.NUMBER_ON_SAME_TEAM_SEQUENCE_ONE = jcall!(smallIntegerSingletonFactory).getAt(101);
        this.NUMBER_ON_SAME_TEAM_SEQUENCE_TWO = jcall!(smallIntegerSingletonFactory).getAt(102);
        this.NUMBER_ON_SAME_TEAM_SEQUENCE_THREE = jcall!(smallIntegerSingletonFactory).getAt(103);
        this.NUMBER_ON_SAME_TEAM_SEQUENCE_FOUR = jcall!(smallIntegerSingletonFactory).getAt(104);
        this.NUMBER_ON_SAME_TEAM_SEQUENCE_FIVE = jcall!(smallIntegerSingletonFactory).getAt(105);
        this.NUMBER_ON_SAME_TEAM_SEQUENCE_SIX = jcall!(smallIntegerSingletonFactory).getAt(106);
        this.NUMBER_ON_SAME_TEAM_SEQUENCE_SEVEN = jcall!(smallIntegerSingletonFactory).getAt(107);
        this.NUMBER_ON_SAME_TEAM_SEQUENCE_EIGHT = jcall!(smallIntegerSingletonFactory).getAt(108);
        this.NUMBER_ON_SAME_TEAM_SEQUENCE_NINE = jcall!(smallIntegerSingletonFactory).getAt(109);
        this.NUMBER_ON_SAME_TEAM_SEQUENCE = vec![this.NUMBER_ON_SAME_TEAM_SEQUENCE_ZERO.clone(), this.NUMBER_ON_SAME_TEAM_SEQUENCE_ONE.clone(), this.NUMBER_ON_SAME_TEAM_SEQUENCE_THREE.clone(), this.NUMBER_ON_SAME_TEAM_SEQUENCE_FOUR.clone(), this.NUMBER_ON_SAME_TEAM_SEQUENCE_FIVE.clone(), this.NUMBER_ON_SAME_TEAM_SEQUENCE_SIX.clone(), this.NUMBER_ON_SAME_TEAM_SEQUENCE_SEVEN.clone(), this.NUMBER_ON_SAME_TEAM_SEQUENCE_EIGHT.clone(), this.NUMBER_ON_SAME_TEAM_SEQUENCE_NINE.clone()];
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

