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
// use crate::javax::microedition::lcdui::Canvas::Canvas;
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

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance(&mut self) -> Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> {
        let mut likelyhoodIntegerArray: Vec<Vec<Option<i32>>> = vec![vec![None; (2) as usize]; (unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM9.clone() } + 1) as usize];
        let mut smallIntegerSingletonFactory: Option<Rc<RefCell<SmallIntegerSingletonFactory>>> = SmallIntegerSingletonFactory::getInstance();
        likelyhoodIntegerArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_UP.clone() }) as usize][(0) as usize] = jcall!(smallIntegerSingletonFactory).getAt(0);
        likelyhoodIntegerArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_UP.clone() }) as usize][(1) as usize] = jcall!(smallIntegerSingletonFactory).getAt(10);
        likelyhoodIntegerArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_DOWN.clone() }) as usize][(0) as usize] = jcall!(smallIntegerSingletonFactory).getAt(10);
        likelyhoodIntegerArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_DOWN.clone() }) as usize][(1) as usize] = jcall!(smallIntegerSingletonFactory).getAt(14);
        likelyhoodIntegerArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_LEFT.clone() }) as usize][(0) as usize] = jcall!(smallIntegerSingletonFactory).getAt(14);
        likelyhoodIntegerArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_LEFT.clone() }) as usize][(1) as usize] = jcall!(smallIntegerSingletonFactory).getAt(18);
        likelyhoodIntegerArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_RIGHT.clone() }) as usize][(0) as usize] = jcall!(smallIntegerSingletonFactory).getAt(18);
        likelyhoodIntegerArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_RIGHT.clone() }) as usize][(1) as usize] = jcall!(smallIntegerSingletonFactory).getAt(22);
        likelyhoodIntegerArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM7.clone() }) as usize][(0) as usize] = jcall!(smallIntegerSingletonFactory).getAt(22);
        likelyhoodIntegerArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM7.clone() }) as usize][(1) as usize] = jcall!(smallIntegerSingletonFactory).getAt(26);
        likelyhoodIntegerArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM9.clone() }) as usize][(0) as usize] = jcall!(smallIntegerSingletonFactory).getAt(26);
        likelyhoodIntegerArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM9.clone() }) as usize][(1) as usize] = jcall!(smallIntegerSingletonFactory).getAt(30);
        likelyhoodIntegerArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM1.clone() }) as usize][(0) as usize] = jcall!(smallIntegerSingletonFactory).getAt(30);
        likelyhoodIntegerArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM1.clone() }) as usize][(1) as usize] = jcall!(smallIntegerSingletonFactory).getAt(40);
        let mut repeatLikelyhoodIntegerArray: Vec<Option<i32>> = vec![None; (unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM9.clone() } + 1) as usize];
        repeatLikelyhoodIntegerArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_UP.clone() }) as usize] = jcall!(smallIntegerSingletonFactory).getAt(0);
        repeatLikelyhoodIntegerArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_DOWN.clone() }) as usize] = jcall!(smallIntegerSingletonFactory).getAt(0);
        repeatLikelyhoodIntegerArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_LEFT.clone() }) as usize] = jcall!(smallIntegerSingletonFactory).getAt(0);
        repeatLikelyhoodIntegerArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_RIGHT.clone() }) as usize] = jcall!(smallIntegerSingletonFactory).getAt(0);
        repeatLikelyhoodIntegerArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM7.clone() }) as usize] = jcall!(smallIntegerSingletonFactory).getAt(0);
        repeatLikelyhoodIntegerArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM9.clone() }) as usize] = jcall!(smallIntegerSingletonFactory).getAt(0);
        repeatLikelyhoodIntegerArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM1.clone() }) as usize] = jcall!(smallIntegerSingletonFactory).getAt(10);
        let mut inputProbability: Option<Rc<RefCell<InputProbability>>> = InputProbability::new(likelyhoodIntegerArray.clone(), repeatLikelyhoodIntegerArray.clone(), 42, crate::org::allbinary::time::NoTimeDelayHelper::NoTimeDelayHelper_SINGLETON.with(Clone::clone).clone());
        return jcall!(inputProbability).toHashtable();
    }
}

