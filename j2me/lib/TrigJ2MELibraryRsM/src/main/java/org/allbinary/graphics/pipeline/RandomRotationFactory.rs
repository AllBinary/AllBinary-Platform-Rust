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
use crate::org::allbinary::math::AngleFactory::AngleFactory;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use super::BasicGraphicsPipeline::BasicGraphicsPipeline;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RandomRotationFactory {
    pub myRandomFactory: Option<Rc<RefCell<MyRandomFactory>>>,
    pub angleFactory: Option<Rc<RefCell<AngleFactory>>>,
}

thread_local! { pub static RandomRotationFactory_instance: Option<Rc<RefCell<RandomRotationFactory>>> = RandomRotationFactory::new(); }

impl RandomRotationFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            myRandomFactory: None,
            angleFactory: None,
        };
        this.myRandomFactory = MyRandomFactory::getInstance();
        this.angleFactory = AngleFactory::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<RandomRotationFactory>>> {
        return RandomRotationFactory_instance.with(Clone::clone).clone();
    }
    

    // throws Exception
    pub fn getInstanceList(&mut self, mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut howMuch: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut graphicsPipe: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::pipeline::BasicGraphicsPipeline::BasicGraphicsPipelineDyn>>> = BasicGraphicsPipeline::new(vector.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::pipeline::BasicGraphicsPipeline::BasicGraphicsPipelineDyn>>);
        jcall!(graphicsPipe).createMatrix();
        jcall!(graphicsPipe).rotateToAngle({ let java_receiver = self.angleFactory.clone(); let java_arguments = (self.getNextRandomAngle(howMuch.clone()),); let java_result = jcall!(java_receiver).getAt(java_arguments.0); java_result });
        return jcall!(graphicsPipe).getMatrix();
    }
    

    pub fn getNextRandomAngle(&mut self, mut howMuch: i32) -> i32 {
        return jcall!(self.myRandomFactory).getNextInt(howMuch * 10);
    }
}

