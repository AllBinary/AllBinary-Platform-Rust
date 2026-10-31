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
// package org.allbinary.animation
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::graphics::PointFactory::PointFactory;
use crate::org::allbinary::graphics::pipeline::BasicGraphicsPipeline::BasicGraphicsPipeline;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListS::BasicArrayListS;
use super::VectorAnimationUtil::VectorAnimationUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct VectorMirrorGenerator {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub vectorAnimationUtil: Option<Rc<RefCell<VectorAnimationUtil>>>,
}

thread_local! { pub static VectorMirrorGenerator_instance: Option<Rc<RefCell<VectorMirrorGenerator>>> = VectorMirrorGenerator::new(); }

impl VectorMirrorGenerator {
    

    pub fn getInstance() -> Option<Rc<RefCell<VectorMirrorGenerator>>> {
        return VectorMirrorGenerator_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            vectorAnimationUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.vectorAnimationUtil = VectorAnimationUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn getInstance2(&mut self, mut points: Vec<Vec<Vec<i32>>>, mut width: i32) -> Vec<Vec<Vec<i32>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut pointFactory: Option<Rc<RefCell<PointFactory>>> = PointFactory::getInstance();
            let mut pointBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = None;
            let mut nextPoints: Vec<Vec<i32>> = Vec::new();
            let mut size2: i32 = 0;
            let mut size: i32 = (points.len() as i32).clone();
            let mut pointsBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListS::new(size.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            for frame in (0..size) {
                nextPoints = points[(frame) as usize].clone();
                size2 = (nextPoints.len() as i32).clone();
                pointBasicArrayList = BasicArrayListS::new(size2.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
                for index in (0..size2) {
                    jcall!(pointBasicArrayList).add(crate::java_into_object(jcall!(pointFactory).createXY(nextPoints[(index) as usize][(0) as usize].clone(), nextPoints[(index) as usize][(1) as usize].clone())));
                }
                jcall!(pointsBasicArrayList).add(crate::java_into_object(pointBasicArrayList.clone()));
            }
            for index in (0..(points.len() as i32)) {
                jcall!(pointsBasicArrayList).add(crate::java_into_object(self.getInstance2({ let java_object = (jcall!(pointsBasicArrayList).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>>().expect("ClassCastException").clone()) }, width.clone())));
            }
            let mut newPoints: Vec<Vec<Vec<i32>>> = jcall!(self.vectorAnimationUtil).toAnimationArrayFromListOfPointList(pointsBasicArrayList.clone());
            return newPoints.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(self.logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).GET_INSTANCE.clone(), crate::java_into_object(e.clone()));
            return jcall!(NullUtil::getInstance()).NULL_INT_ARRAY_ARRAY_ARRAY.clone();
            }
        }
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn getInstance2(&mut self, mut pointBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut width: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut graphicsPipe: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::pipeline::BasicGraphicsPipeline::BasicGraphicsPipelineDyn>>> = BasicGraphicsPipeline::new(pointBasicArrayList.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::pipeline::BasicGraphicsPipeline::BasicGraphicsPipelineDyn>>);
        jcall!(graphicsPipe).createMatrix();
        jcall!(graphicsPipe).mirror(width.clone());
        return jcall!(graphicsPipe).getMatrix();
    }
}

