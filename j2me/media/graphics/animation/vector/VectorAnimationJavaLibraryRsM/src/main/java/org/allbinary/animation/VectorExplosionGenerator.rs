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
use crate::org::allbinary::graphics::PointFactory::PointFactory;
use crate::org::allbinary::graphics::pipeline::RandomRotationFactory::RandomRotationFactory;
use crate::org::allbinary::graphics::pipeline::RandomTranslation::RandomTranslation;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::util::BasicArrayListS::BasicArrayListS;
use super::VectorExplosionType::VectorExplosionType;
use super::VectorAnimationUtil::VectorAnimationUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct VectorExplosionGenerator {
    pub RANDOM: Option<Rc<RefCell<VectorExplosionType>>>,
    pub ROTATION: Option<Rc<RefCell<VectorExplosionType>>>,
    pub vectorAnimationUtil: Option<Rc<RefCell<VectorAnimationUtil>>>,
    pub randomRotationFactory: Option<Rc<RefCell<RandomRotationFactory>>>,
}

thread_local! { pub static VectorExplosionGenerator_instance: Option<Rc<RefCell<VectorExplosionGenerator>>> = VectorExplosionGenerator::new(); }

impl VectorExplosionGenerator {
    

    pub fn getInstance() -> Option<Rc<RefCell<VectorExplosionGenerator>>> {
        return VectorExplosionGenerator_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            RANDOM: None,
            ROTATION: None,
            vectorAnimationUtil: None,
            randomRotationFactory: None,
        };
        this.RANDOM = VectorExplosionType::new();
        this.ROTATION = VectorExplosionType::new();
        this.vectorAnimationUtil = VectorAnimationUtil::getInstance();
        this.randomRotationFactory = RandomRotationFactory::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn createList(&mut self, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut howMuch: i32, mut r#type: Option<Rc<RefCell<VectorExplosionType>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = None;
        let mut size: i32 = jcall!(list).size();
        let mut points: Vec<Vec<i32>> = vec![vec![0; (2) as usize]; (size) as usize];
        for index in (0..size) {
            point = { let java_object = (jcall!(list).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>>().expect("ClassCastException").clone()) };
            points[(index) as usize][(0) as usize] = jcall!(point).getX();
            points[(index) as usize][(1) as usize] = jcall!(point).getY();
        }
        return self.createListFromPoints(list.clone(), points.clone(), howMuch.clone(), r#type.clone());
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn getInstance3(&mut self, mut points: Vec<Vec<i32>>, mut howMuch: i32, mut r#type: Option<Rc<RefCell<VectorExplosionType>>>) -> Vec<Vec<Vec<i32>>> {
        let mut pointsBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.getInstanceStartFrame(points.clone(), howMuch.clone(), r#type.clone(), true);
        let mut tempBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = { let java_object = (jcall!(pointsBasicArrayList).as_BasicArrayList_mut().objectArray[(0) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>>().expect("ClassCastException").clone()) };
        let mut newPoints: Vec<Vec<Vec<i32>>> = { let java_receiver = self.vectorAnimationUtil.clone(); let java_arguments = (pointsBasicArrayList.clone(), jcall!(tempBasicArrayList).size(),); let java_result = jcall!(java_receiver).toAnimationArrayFromListOfPointListWithPointsPerFrame(java_arguments.0, java_arguments.1); java_result };
        return newPoints.clone();
    }
    

    // throws Exception
    pub fn getInstanceStartFrame(&mut self, mut points: Vec<Vec<i32>>, mut howMuch: i32, mut r#type: Option<Rc<RefCell<VectorExplosionType>>>, mut startFrame: bool) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut pointsBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListS::new(howMuch.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            jcall!(pointsBasicArrayList).add(crate::java_into_object(self.createPointsBasicArrayList(points.clone())));
            let mut frameIndex: i32 = 0;
            let mut tempBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = None;
            let mut pointBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = None;
            while frameIndex < howMuch {
                tempBasicArrayList = { let java_object = (jcall!(pointsBasicArrayList).as_BasicArrayList_mut().objectArray[(frameIndex) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>>().expect("ClassCastException").clone()) };
                pointBasicArrayList = self.createListFromPoints(tempBasicArrayList.clone(), points.clone(), howMuch.clone(), r#type.clone());
                jcall!(pointsBasicArrayList).add(crate::java_into_object(pointBasicArrayList.clone()));
                frameIndex += 1;
            }
            if !startFrame {
                jcall!(pointsBasicArrayList).removeAt(0);
            }
            return pointsBasicArrayList.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            panic!("{:?}", e);
            }
        }
    }
    

    // throws Exception
    pub fn createListFromPoints(&mut self, mut tempBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut points: Vec<Vec<i32>>, mut howMuch: i32, mut r#type: Option<Rc<RefCell<VectorExplosionType>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut index: i32 = 0;
        let mut pointBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut sectionBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = None;
        while index < (points.len() as i32) {
            sectionBasicArrayList = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            while points[(index) as usize][(0) as usize] != 1000 {
                jcall!(sectionBasicArrayList).add(crate::java_into_object(jcall!(tempBasicArrayList).as_BasicArrayList_mut().objectArray[(index) as usize].clone()));
                index += 1;
                if index >= (points.len() as i32) {
                    break;
                }
            }
            sectionBasicArrayList = RandomTranslation::getInstance(sectionBasicArrayList.clone(), howMuch.clone());
            if match (&r#type, &self.ROTATION) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                sectionBasicArrayList = jcall!(self.randomRotationFactory).getInstanceList(sectionBasicArrayList.clone(), howMuch.clone());
            }
            let mut size: i32 = jcall!(sectionBasicArrayList).size();
            for index2 in (0..size) {
                jcall!(pointBasicArrayList).add(crate::java_into_object(jcall!(sectionBasicArrayList).as_BasicArrayList_mut().objectArray[(index2) as usize].clone()));
            }
            if index >= (points.len() as i32) {
                break;
            } else if points[(index) as usize][(0) as usize] == 1000 {
                jcall!(pointBasicArrayList).add(crate::java_into_object(jcall!(tempBasicArrayList).as_BasicArrayList_mut().objectArray[(index) as usize].clone()));
                index += 1;
            }
        }
        return pointBasicArrayList.clone();
    }
    

    // throws Exception
    pub fn createPointsBasicArrayList(&mut self, mut points: Vec<Vec<i32>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        if (points.len() as i32) == 0 {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Not Points Provided"))));
        }
        let mut firstPointBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListS::new((points.len() as i32).clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut pointFactory: Option<Rc<RefCell<PointFactory>>> = PointFactory::getInstance();
        for index in (0..(points.len() as i32)) {
            jcall!(firstPointBasicArrayList).add(crate::java_into_object(jcall!(pointFactory).createXY(points[(index) as usize][(0) as usize].clone(), points[(index) as usize][(1) as usize].clone())));
        }
        return firstPointBasicArrayList.clone();
    }
}

