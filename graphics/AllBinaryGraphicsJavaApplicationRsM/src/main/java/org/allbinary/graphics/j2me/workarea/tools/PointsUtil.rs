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
// package org.allbinary.graphics.j2me.workarea.tools
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::animation::VectorRotationGenerator::VectorRotationGenerator;
use crate::org::allbinary::graphics::GPoint::GPoint;
use crate::org::allbinary::math::AngleFactory::AngleFactory;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PointsUtil {
}

thread_local! { pub static PointsUtil_instance: Option<Rc<RefCell<PointsUtil>>> = PointsUtil::new(); }

impl PointsUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<PointsUtil>>> {
        return PointsUtil_instance.with(Clone::clone).clone();
    }
    

    // throws Exception
    pub fn doTransforms(&mut self, mut pointVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut angle: Option<f64>, mut fulcrumPoint: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return jcall!(VectorRotationGenerator::getInstance()).getListAtAngle(jcall!(fulcrumPoint).getX(), jcall!(fulcrumPoint).getY(), pointVector.clone(), jcall!(AngleFactory::getInstance()).getAt(((angle).unwrap() as i32)));
    }
}

