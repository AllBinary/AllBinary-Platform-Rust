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
use crate::org::allbinary::graphics::GPoint::GPoint;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct VectorAnimationUtil {
}

thread_local! { pub static VectorAnimationUtil_instance: Option<Rc<RefCell<VectorAnimationUtil>>> = VectorAnimationUtil::new(); }

impl VectorAnimationUtil {
    

    pub fn getInstance() -> Option<Rc<RefCell<VectorAnimationUtil>>> {
        return VectorAnimationUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn toAnimationArrayFromListOfPointListWithPointsPerFrame(&mut self, mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut pointsPerFrame: i32) -> Vec<Vec<Vec<i32>>> {
        let mut size: i32 = jcall!(vector).size();
        let mut points: Vec<Vec<Vec<i32>>> = vec![vec![vec![0; (2) as usize]; (pointsPerFrame) as usize]; (size) as usize];
        let mut nextBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = None;
        let mut framePoints: Vec<Vec<i32>> = Vec::new();
        for index in (0..size) {
            nextBasicArrayList = { let java_object = (jcall!(vector).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>>().expect("ClassCastException").clone()) };
            framePoints = self.toFrameArrayFromPointBasicArrayList(nextBasicArrayList.clone());
            for pointIndex in (0..(framePoints.len() as i32)) {
                points[(index) as usize][(pointIndex) as usize][(0) as usize] = framePoints[(pointIndex) as usize][(0) as usize].clone();
                points[(index) as usize][(pointIndex) as usize][(1) as usize] = framePoints[(pointIndex) as usize][(1) as usize].clone();
            }
        }
        return points.clone();
    }
    

    pub fn toAnimationArrayFromListOfPointList(&mut self, mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Vec<Vec<Vec<i32>>> {
        let mut size: i32 = jcall!(vector).size();
        let mut points: Vec<Vec<Vec<i32>>> = vec![vec![vec![0; (0) as usize]; (0) as usize]; (size) as usize];
        let mut nextBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = None;
        let mut framePoints: Vec<Vec<i32>> = Vec::new();
        for index in (0..size) {
            nextBasicArrayList = { let java_object = (jcall!(vector).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>>().expect("ClassCastException").clone()) };
            framePoints = self.toFrameArrayFromPointBasicArrayList(nextBasicArrayList.clone());
            points[(index) as usize] = vec![vec![0; (2) as usize]; ((framePoints.len() as i32)) as usize];
            for pointIndex in (0..(framePoints.len() as i32)) {
                points[(index) as usize][(pointIndex) as usize][(0) as usize] = framePoints[(pointIndex) as usize][(0) as usize].clone();
                points[(index) as usize][(pointIndex) as usize][(1) as usize] = framePoints[(pointIndex) as usize][(1) as usize].clone();
            }
        }
        return points.clone();
    }
    

    pub fn toFrameArrayFromPointBasicArrayList(&mut self, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Vec<Vec<i32>> {
        let mut points: Vec<Vec<i32>> = vec![vec![0; (2) as usize]; (jcall!(list).size()) as usize];
        let mut size: i32 = jcall!(list).size();
        for index in (0..size) {
            let mut nextPoint: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = { let java_object = (jcall!(list).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>>().expect("ClassCastException").clone()) };
            points[(index) as usize][(0) as usize] = jcall!(nextPoint).getX();
            points[(index) as usize][(1) as usize] = jcall!(nextPoint).getY();
        }
        return points.clone();
    }
}

