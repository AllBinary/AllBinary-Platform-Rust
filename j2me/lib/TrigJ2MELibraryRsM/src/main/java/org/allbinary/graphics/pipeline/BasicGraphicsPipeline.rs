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
use crate::org::allbinary::math::Angle::Angle;
use crate::org::allbinary::math::NoDecimalTrigTable::NoDecimalTrigTable;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicGraphicsPipeline {
    pub pointBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub noDecimalTrigTable: Option<Rc<RefCell<NoDecimalTrigTable>>>,
}


impl BasicGraphicsPipeline {
    

    pub fn new_struct(mut points: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Self {
        let mut this = Self {
            pointBasicArrayList: None,
            noDecimalTrigTable: None,
        };
        this.noDecimalTrigTable = NoDecimalTrigTable::getInstance();
        this.pointBasicArrayList = points.clone();
        this
    }

    pub fn new(mut points: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(points))))
    }
    

    pub fn setInitMatrix(&mut self, mut points: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        self.pointBasicArrayList = points.clone();
    }
    

    pub fn createMatrix(&mut self) {
    }
    

    // throws Exception
    pub fn translate(&mut self, mut x: i32, mut y: i32) {
        let mut newBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut size: i32 = jcall!(self.pointBasicArrayList).size();
        let mut pointFactory: Option<Rc<RefCell<PointFactory>>> = PointFactory::getInstance();
        let mut secondPoint: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = None;
        for index in (0..size) {
            secondPoint = { let java_object = (jcall!(self.pointBasicArrayList).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>>().expect("ClassCastException").clone()) };
            if jcall!(secondPoint).getX() != 1000 {
                jcall!(newBasicArrayList).add(crate::java_into_object(jcall!(pointFactory).createXY(jcall!(secondPoint).getX() + x, jcall!(secondPoint).getY() + y)));
            } else {
                jcall!(newBasicArrayList).add(crate::java_into_object(secondPoint.clone()));
            }
        }
        self.pointBasicArrayList = newBasicArrayList.clone();
    }
    

    // throws Exception
    pub fn rotateToAngle(&mut self, mut angle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>) {
        { let java_arguments = (((jcall!(angle).getValue()) as i32),); self.rotate(java_arguments.0) };
    }
    

    // throws Exception
    pub fn rotate(&mut self, mut angle: i32) {
        let mut newBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut secondPoint: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = None;
        let mut y: i64 = 0_i64;
        let mut secondX: i64 = 0_i64;
        let mut secondY: i64 = 0_i64;
        let mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = None;
        let mut pointFactory: Option<Rc<RefCell<PointFactory>>> = PointFactory::getInstance();
        let mut size: i32 = jcall!(self.pointBasicArrayList).size();
        let mut sin: i64 = 0_i64;
        let mut cos: i64 = 0_i64;
        for index in (0..size) {
            secondPoint = { let java_object = (jcall!(self.pointBasicArrayList).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>>().expect("ClassCastException").clone()) };
            if jcall!(secondPoint).getX() != 1000 {
                sin = jcall!(self.noDecimalTrigTable).sin(angle.clone());
                cos = jcall!(self.noDecimalTrigTable).cos(angle.clone());
                y = ((jcall!(secondPoint).getY()) as i64) * sin;
                secondX = (((jcall!(secondPoint).getX()) as i64) * cos) - y;
                secondY = (((jcall!(secondPoint).getX()) as i64) * sin) + (((jcall!(secondPoint).getY()) as i64) * cos);
                point = jcall!(pointFactory).createXY(((secondX) as i32) / 10000, ((secondY) as i32) / 10000);
                jcall!(newBasicArrayList).add(crate::java_into_object(point.clone()));
            } else {
                jcall!(newBasicArrayList).add(crate::java_into_object(secondPoint.clone()));
            }
        }
        self.pointBasicArrayList = newBasicArrayList.clone();
    }
    

    // throws Exception
    pub fn mirror(&mut self, mut width: i32) {
        let mut pointFactory: Option<Rc<RefCell<PointFactory>>> = PointFactory::getInstance();
        let mut halfWidth: i32 = (width >> 1);
        let mut newBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut secondPoint: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = None;
        let mut newX: i32 = 0;
        let mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = None;
        let mut size: i32 = jcall!(self.pointBasicArrayList).size();
        for index in (0..size) {
            secondPoint = { let java_object = (jcall!(self.pointBasicArrayList).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>>().expect("ClassCastException").clone()) };
            if jcall!(secondPoint).getX() != 1000 {
                newX = 0;
                if jcall!(secondPoint).getX() > halfWidth {
                    newX = halfWidth - (jcall!(secondPoint).getX() - halfWidth);
                } else {
                    newX = halfWidth + (halfWidth - jcall!(secondPoint).getX());
                }
                point = jcall!(pointFactory).createXY(newX.clone(), jcall!(secondPoint).getY());
                jcall!(newBasicArrayList).add(crate::java_into_object(point.clone()));
            } else {
                jcall!(newBasicArrayList).add(crate::java_into_object(secondPoint.clone()));
            }
        }
        self.pointBasicArrayList = newBasicArrayList.clone();
    }
    

    pub fn getMatrix(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.pointBasicArrayList.clone();
    }
}
pub trait BasicGraphicsPipelineDyn {
    fn as_BasicGraphicsPipeline(&self) -> &BasicGraphicsPipeline;
    fn as_BasicGraphicsPipeline_mut(&mut self) -> &mut BasicGraphicsPipeline;
    fn setInitMatrix(&mut self, mut points: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { BasicGraphicsPipeline::setInitMatrix(self.as_BasicGraphicsPipeline_mut(), points) }
    fn createMatrix(&mut self) { BasicGraphicsPipeline::createMatrix(self.as_BasicGraphicsPipeline_mut()) }
    fn translate(&mut self, mut x: i32, mut y: i32) { BasicGraphicsPipeline::translate(self.as_BasicGraphicsPipeline_mut(), x, y) }
    fn rotateToAngle(&mut self, mut angle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>) { BasicGraphicsPipeline::rotateToAngle(self.as_BasicGraphicsPipeline_mut(), angle) }
    fn rotate(&mut self, mut angle: i32) { BasicGraphicsPipeline::rotate(self.as_BasicGraphicsPipeline_mut(), angle) }
    fn mirror(&mut self, mut width: i32) { BasicGraphicsPipeline::mirror(self.as_BasicGraphicsPipeline_mut(), width) }
    fn getMatrix(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { BasicGraphicsPipeline::getMatrix(self.as_BasicGraphicsPipeline_mut()) }
}
impl BasicGraphicsPipelineDyn for BasicGraphicsPipeline {
    fn as_BasicGraphicsPipeline(&self) -> &BasicGraphicsPipeline { self }
    fn as_BasicGraphicsPipeline_mut(&mut self) -> &mut BasicGraphicsPipeline { self }
}


