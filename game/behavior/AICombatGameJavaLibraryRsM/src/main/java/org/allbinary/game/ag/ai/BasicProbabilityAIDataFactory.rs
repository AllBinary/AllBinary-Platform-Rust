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
use crate::javax::microedition::lcdui::Canvas::Canvas;
use crate::org::allbinary::logic::math::SmallIntegerSingletonFactory::SmallIntegerSingletonFactory;
use crate::org::allbinary::time::NoTimeDelayHelper::NoTimeDelayHelper;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use super::InputProbability::InputProbability;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicProbabilityAIDataFactory {
}


impl BasicProbabilityAIDataFactory {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance(&mut self) -> Option<Rc<RefCell<ABHashtable<crate::JavaObject, crate::JavaObject>>>> {
        let mut likelyhoodIntegerArray: Vec<Vec<Option<i32>>> = vec![vec![None; (2) as usize]; (crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM9.with(Clone::clone) + 1) as usize];
        let mut smallIntegerSingletonFactory: Option<Rc<RefCell<SmallIntegerSingletonFactory>>> = SmallIntegerSingletonFactory::getInstance();
        likelyhoodIntegerArray[(crate::javax::microedition::lcdui::Canvas::Canvas_UP.with(Clone::clone)) as usize][(0) as usize] = jcall!(smallIntegerSingletonFactory).getAt(0);
        likelyhoodIntegerArray[(crate::javax::microedition::lcdui::Canvas::Canvas_UP.with(Clone::clone)) as usize][(1) as usize] = jcall!(smallIntegerSingletonFactory).getAt(10);
        likelyhoodIntegerArray[(crate::javax::microedition::lcdui::Canvas::Canvas_DOWN.with(Clone::clone)) as usize][(0) as usize] = jcall!(smallIntegerSingletonFactory).getAt(10);
        likelyhoodIntegerArray[(crate::javax::microedition::lcdui::Canvas::Canvas_DOWN.with(Clone::clone)) as usize][(1) as usize] = jcall!(smallIntegerSingletonFactory).getAt(14);
        likelyhoodIntegerArray[(crate::javax::microedition::lcdui::Canvas::Canvas_LEFT.with(Clone::clone)) as usize][(0) as usize] = jcall!(smallIntegerSingletonFactory).getAt(14);
        likelyhoodIntegerArray[(crate::javax::microedition::lcdui::Canvas::Canvas_LEFT.with(Clone::clone)) as usize][(1) as usize] = jcall!(smallIntegerSingletonFactory).getAt(18);
        likelyhoodIntegerArray[(crate::javax::microedition::lcdui::Canvas::Canvas_RIGHT.with(Clone::clone)) as usize][(0) as usize] = jcall!(smallIntegerSingletonFactory).getAt(18);
        likelyhoodIntegerArray[(crate::javax::microedition::lcdui::Canvas::Canvas_RIGHT.with(Clone::clone)) as usize][(1) as usize] = jcall!(smallIntegerSingletonFactory).getAt(22);
        likelyhoodIntegerArray[(crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM7.with(Clone::clone)) as usize][(0) as usize] = jcall!(smallIntegerSingletonFactory).getAt(22);
        likelyhoodIntegerArray[(crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM7.with(Clone::clone)) as usize][(1) as usize] = jcall!(smallIntegerSingletonFactory).getAt(26);
        likelyhoodIntegerArray[(crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM9.with(Clone::clone)) as usize][(0) as usize] = jcall!(smallIntegerSingletonFactory).getAt(26);
        likelyhoodIntegerArray[(crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM9.with(Clone::clone)) as usize][(1) as usize] = jcall!(smallIntegerSingletonFactory).getAt(30);
        likelyhoodIntegerArray[(crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM1.with(Clone::clone)) as usize][(0) as usize] = jcall!(smallIntegerSingletonFactory).getAt(30);
        likelyhoodIntegerArray[(crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM1.with(Clone::clone)) as usize][(1) as usize] = jcall!(smallIntegerSingletonFactory).getAt(40);
        let mut repeatLikelyhoodIntegerArray: Vec<Option<i32>> = vec![None; (crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM9.with(Clone::clone) + 1) as usize];
        repeatLikelyhoodIntegerArray[(crate::javax::microedition::lcdui::Canvas::Canvas_UP.with(Clone::clone)) as usize] = jcall!(smallIntegerSingletonFactory).getAt(0);
        repeatLikelyhoodIntegerArray[(crate::javax::microedition::lcdui::Canvas::Canvas_DOWN.with(Clone::clone)) as usize] = jcall!(smallIntegerSingletonFactory).getAt(0);
        repeatLikelyhoodIntegerArray[(crate::javax::microedition::lcdui::Canvas::Canvas_LEFT.with(Clone::clone)) as usize] = jcall!(smallIntegerSingletonFactory).getAt(0);
        repeatLikelyhoodIntegerArray[(crate::javax::microedition::lcdui::Canvas::Canvas_RIGHT.with(Clone::clone)) as usize] = jcall!(smallIntegerSingletonFactory).getAt(0);
        repeatLikelyhoodIntegerArray[(crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM7.with(Clone::clone)) as usize] = jcall!(smallIntegerSingletonFactory).getAt(0);
        repeatLikelyhoodIntegerArray[(crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM9.with(Clone::clone)) as usize] = jcall!(smallIntegerSingletonFactory).getAt(0);
        repeatLikelyhoodIntegerArray[(crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM1.with(Clone::clone)) as usize] = jcall!(smallIntegerSingletonFactory).getAt(10);
        let mut inputProbability: Option<Rc<RefCell<InputProbability>>> = InputProbability::new(likelyhoodIntegerArray.clone(), repeatLikelyhoodIntegerArray.clone(), 42, crate::org::allbinary::time::NoTimeDelayHelper::NoTimeDelayHelper_SINGLETON.with(Clone::clone).clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>));
        return jcall!(inputProbability).toHashtable();
    }
}

