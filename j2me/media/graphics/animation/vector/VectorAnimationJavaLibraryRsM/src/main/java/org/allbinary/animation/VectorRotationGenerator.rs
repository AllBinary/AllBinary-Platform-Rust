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
use crate::org::allbinary::math::Angle::Angle;
use crate::org::allbinary::math::AngleFactory::AngleFactory;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListS::BasicArrayListS;
use crate::org::allbinary::vector::VectorInfo::VectorInfo;
use super::VectorAnimationUtil::VectorAnimationUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct VectorRotationGenerator {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub vectorAnimationUtil: Option<Rc<RefCell<VectorAnimationUtil>>>,
    pub angleFactory: Option<Rc<RefCell<AngleFactory>>>,
}

thread_local! { pub static VectorRotationGenerator_instance: Option<Rc<RefCell<VectorRotationGenerator>>> = VectorRotationGenerator::new(); }

impl VectorRotationGenerator {
    

    pub fn getInstance() -> Option<Rc<RefCell<VectorRotationGenerator>>> {
        return VectorRotationGenerator_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            vectorAnimationUtil: None,
            angleFactory: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.vectorAnimationUtil = VectorAnimationUtil::getInstance();
        this.angleFactory = AngleFactory::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn getInstance1(&mut self, mut vectorRotationInfo: Option<Rc<RefCell<VectorInfo>>>) -> Vec<Vec<Vec<i32>>> {
        return { let java_arguments = (jcall!(vectorRotationInfo).getWidth(), jcall!(vectorRotationInfo).getHeight(), jcall!(vectorRotationInfo).getPoints(), jcall!(vectorRotationInfo).getTotalFrames(),); self.getInstanceFrames(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3) };
    }
    

    pub fn getInstanceFrames(&mut self, mut width: i32, mut height: i32, mut points: Vec<Vec<i32>>, mut frames: i32) -> Vec<Vec<Vec<i32>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut totalAngle: i16 = jcall!(self.angleFactory).TOTAL_ANGLE.clone();
            let mut angleIncrement: i32 = ((totalAngle) as i32) / frames;
            let mut pointsBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListS::new(((totalAngle) as i32) / angleIncrement).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            let mut pointBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListS::new((points.len() as i32).clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            let mut pointFactory: Option<Rc<RefCell<PointFactory>>> = PointFactory::getInstance();
            let mut size: i32 = (points.len() as i32).clone();
            for index in (0..size) {
                jcall!(pointBasicArrayList).add(crate::java_into_object(jcall!(pointFactory).createXY(points[(index) as usize][(0) as usize].clone(), points[(index) as usize][(1) as usize].clone())));
            }
            let mut index: i32 = 0;
            while index < ((totalAngle) as i32) {
                jcall!(pointsBasicArrayList).add(crate::java_into_object({ let java_arguments = (width.clone(), height.clone(), pointBasicArrayList.clone(), jcall!(self.angleFactory).getAt(index.clone()),); self.getListAtAngle(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3) }));
                index += angleIncrement.clone();
            }
            let mut newPoints: Vec<Vec<Vec<i32>>> = { let java_receiver = self.vectorAnimationUtil.clone(); let java_arguments = (pointsBasicArrayList.clone(), jcall!(pointBasicArrayList).size(),); let java_result = jcall!(java_receiver).toAnimationArrayFromListOfPointListWithPointsPerFrame(java_arguments.0, java_arguments.1); java_result };
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
    pub fn getListAtAngle(&mut self, mut width: i32, mut height: i32, mut pointBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut angle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut graphicsPipe: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::pipeline::BasicGraphicsPipeline::BasicGraphicsPipelineDyn>>> = BasicGraphicsPipeline::new(pointBasicArrayList.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::pipeline::BasicGraphicsPipeline::BasicGraphicsPipelineDyn>>);
        jcall!(graphicsPipe).createMatrix();
        jcall!(graphicsPipe).translate(-(width >> 1), -(height >> 1));
        jcall!(graphicsPipe).rotateToAngle(angle.clone());
        jcall!(graphicsPipe).translate((width >> 1).clone(), (height >> 1).clone());
        return jcall!(graphicsPipe).getMatrix();
    }
}

