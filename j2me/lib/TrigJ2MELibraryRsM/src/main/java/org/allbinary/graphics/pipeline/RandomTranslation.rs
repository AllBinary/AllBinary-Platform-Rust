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
// package org.allbinary.graphics.pipeline
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::rand::MyRandomFactory::MyRandomFactory;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use super::BasicGraphicsPipeline::BasicGraphicsPipeline;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RandomTranslation {
}


impl RandomTranslation {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn getInstance(mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut howMuch: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut myRandomFactory: Option<Rc<RefCell<MyRandomFactory>>> = MyRandomFactory::getInstance();
        let mut x: i32 = -jcall!(myRandomFactory).getNextInt(howMuch.clone());
        let mut y: i32 = jcall!(myRandomFactory).getNextInt(howMuch.clone());
        let mut graphicsPipe: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::pipeline::BasicGraphicsPipeline::BasicGraphicsPipelineDyn>>> = BasicGraphicsPipeline::new(vector.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::pipeline::BasicGraphicsPipeline::BasicGraphicsPipelineDyn>>);
        jcall!(graphicsPipe).createMatrix();
        jcall!(graphicsPipe).translate(x.clone(), y.clone());
        return jcall!(graphicsPipe).getMatrix();
    }
}

