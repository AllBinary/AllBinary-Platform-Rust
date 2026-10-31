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
use crate::org::allbinary::graphics::GPoint::GPoint;
use crate::org::allbinary::graphics::PointFactory::PointFactory;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use super::BasicGraphicsPipeline::BasicGraphicsPipeline;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GraphicsPipeline {
    pub base: BasicGraphicsPipeline,
}


impl GraphicsPipeline {
    

    pub fn new_struct(mut points: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Self {
        let mut this = Self {
            base: BasicGraphicsPipeline::new_struct(points.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut points: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(points))))
    }
    

    // throws Exception
    pub fn rotateTheta(&mut self, mut aTheta: f64) {
        let mut newVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut size: i32 = jcall!(self.pointBasicArrayList).size();
        for index in (0..size) {
            let mut secondPoint: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = { let java_object = (jcall!(self.base.pointBasicArrayList).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>>().expect("ClassCastException").clone()) };
            let mut secondX: f64 = (jcall!(secondPoint).getX() * ((aTheta.clone()) as f64).cos()) - (jcall!(secondPoint).getY() * ((aTheta.clone()) as f64).sin());
            let mut secondY: f64 = (jcall!(secondPoint).getX() * ((aTheta.clone()) as f64).sin()) + (jcall!(secondPoint).getY() * ((aTheta.clone()) as f64).cos());
            let mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = jcall!(PointFactory::getInstance()).createXY((((secondX) as i32)).clone(), (((secondY) as i32)).clone());
            jcall!(newVector).add(crate::java_into_object(point.clone()));
        }
        self.pointBasicArrayList = newVector.clone();
    }
}
impl std::ops::Deref for GraphicsPipeline {
    type Target = BasicGraphicsPipeline;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for GraphicsPipeline {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::pipeline::BasicGraphicsPipeline::BasicGraphicsPipelineDyn for GraphicsPipeline {
    fn as_BasicGraphicsPipeline(&self) -> &crate::org::allbinary::graphics::pipeline::BasicGraphicsPipeline::BasicGraphicsPipeline { &self.base }
    fn as_BasicGraphicsPipeline_mut(&mut self) -> &mut crate::org::allbinary::graphics::pipeline::BasicGraphicsPipeline::BasicGraphicsPipeline { &mut self.base }
}


