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
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::math::SmallIntegerSingletonFactory::SmallIntegerSingletonFactory;
use crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelper;
use crate::org::allbinary::util::ABHashtable::ABHashtable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InputProbability {
    pub likelyhoodIntegerArray: Vec<Vec<Option<i32>>>,
    pub repeatLikelyhoodIntegerArray: Vec<Option<i32>>,
    pub max: i32,
    pub timeDelayHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>>,
}

pub static mut InputProbability_INPUT_PROBABILITY: std::cell::LazyCell<Option<i32>> = std::cell::LazyCell::new(|| jcall!(SmallIntegerSingletonFactory::getInstance()).getAt(0));

impl InputProbability {
    

    pub fn new_struct(mut likelyhoodIntegerArray: Vec<Vec<Option<i32>>>, mut repeatLikelyhoodIntegerArray: Vec<Option<i32>>, mut max: i32, mut timeDelayHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>>) -> Self {
        let mut this = Self {
            likelyhoodIntegerArray: Vec::new(),
            repeatLikelyhoodIntegerArray: Vec::new(),
            max: 0,
            timeDelayHelper: None,
        };
        this.likelyhoodIntegerArray = likelyhoodIntegerArray.clone();
        this.repeatLikelyhoodIntegerArray = repeatLikelyhoodIntegerArray.clone();
        this.max = max.clone();
        this.timeDelayHelper = timeDelayHelper.clone();
        this
    }

    pub fn new(mut likelyhoodIntegerArray: Vec<Vec<Option<i32>>>, mut repeatLikelyhoodIntegerArray: Vec<Option<i32>>, mut max: i32, mut timeDelayHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(likelyhoodIntegerArray, repeatLikelyhoodIntegerArray, max, timeDelayHelper))))
    }
    

    pub fn getLikelyhoodIntegerArray(&mut self) -> Vec<Vec<Option<i32>>> {
        return self.likelyhoodIntegerArray.clone();
    }
    

    pub fn getRepeatLikelyhoodIntegerArray(&mut self) -> Vec<Option<i32>> {
        return self.repeatLikelyhoodIntegerArray.clone();
    }
    

    pub fn getMax(&mut self) -> i32 {
        return self.max.clone();
    }
    

    pub fn getTimeDelayHelper(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>> {
        return self.timeDelayHelper.clone();
    }
    

    pub fn toHashtable(&mut self) -> Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> {
        let mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> = jcall!(StdUtil::getInstance()).createHashtable();
        jcall!(hashtable).put(crate::java_into_object(unsafe { InputProbability_INPUT_PROBABILITY.clone() }.clone()), None /* Java this: a &mut self method has no Rc of itself */);
        return hashtable.clone();
    }
}

