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
// package org.allbinary.media.image.comparison.motion
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::awt::*;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ConsolidateMotionRectangles {
    pub base: MotionRectangles,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

thread_local! { pub static ConsolidateMotionRectangles_NAME: String = String::from("consolidatedMotionRectangles_"); }

impl ConsolidateMotionRectangles {
    

    pub fn new_struct(mut motionRectangles: Option<Rc<RefCell<dyn crate::org::allbinary::media::image::comparison::motion::MotionRectangles::MotionRectanglesDyn>>>) -> Self {
        let mut this = Self {
            base: MotionRectangles::new_struct(ConsolidateMotionRectangles_NAME.with(Clone::clone).clone(), jcall!(motionRectangles).getImageComparisonResult()),
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        let mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(motionRectangles).getVector();
        let mut size: i32 = jcall!(vector).size();
        for index in (0..size) {
            let mut motionRectangle: Option<Rc<RefCell<MotionRectangle>>> = { let java_object = (jcall!(vector).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<MotionRectangle>>>>().expect("ClassCastException").clone()) };
            jcall!(vector).add(crate::java_into_object(jcall!(motionRectangle).clone()));
        }
        this.consolidateMotionRectangles();
        this
    }

    pub fn new(mut motionRectangles: Option<Rc<RefCell<dyn crate::org::allbinary::media::image::comparison::motion::MotionRectangles::MotionRectanglesDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(motionRectangles))))
    }
    

    pub fn consolidateMotionRectangles(&mut self) {
        let mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.getVector();
        { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Start - Size Before: "), jcall!(vector).size()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("consolidateMotionRectangleConstraints"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        let mut removeVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut size: i32 = jcall!(vector).size();
        for index in (0..size) {
            let mut motionRectangle: Option<Rc<RefCell<MotionRectangle>>> = { let java_object = (jcall!(vector).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<MotionRectangle>>>>().expect("ClassCastException").clone()) };
            for i in (index + 1..jcall!(vector).size()) {
                let mut motionRectangle2: Option<Rc<RefCell<MotionRectangle>>> = { let java_object = (jcall!(vector).get(i.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<MotionRectangle>>>>().expect("ClassCastException").clone()) };
                let mut rectangle: Option<Rc<RefCell<Rectangle>>> = jcall!(motionRectangle).getRectangle();
                let mut rectangle2: Option<Rc<RefCell<Rectangle>>> = jcall!(motionRectangle2).getRectangle();
                let mut x2: i32 = jcall!(rectangle).x + jcall!(rectangle).width;
                let mut y2: i32 = jcall!(rectangle).y + jcall!(rectangle).height;
                let mut rightX: i32 = jcall!(rectangle2).x + jcall!(rectangle2).width;
                let mut bottomY: i32 = jcall!(rectangle2).y + jcall!(rectangle2).height;
                if jcall!(rectangle).x <= jcall!(rectangle2).x && jcall!(rectangle).y <= jcall!(rectangle2).y && x2 >= rightX && y2 >= bottomY {
                    jcall!(removeVector).add(crate::java_into_object(motionRectangle2.clone()));
                }
            }
        }
        self.remove(removeVector.clone());
    }
    

    pub fn remove(&mut self, mut removeVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        let mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.getVector();
        let mut size: i32 = jcall!(removeVector).size();
        for index in (0..size) {
            jcall!(vector).remove(crate::java_into_object(jcall!(removeVector).get(index.clone())));
        }
    }
}
impl std::ops::Deref for ConsolidateMotionRectangles {
    type Target = MotionRectangles;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ConsolidateMotionRectangles {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::media::image::comparison::motion::MotionRectangles::MotionRectanglesDyn for ConsolidateMotionRectangles {
    fn as_MotionRectangles(&self) -> &crate::org::allbinary::media::image::comparison::motion::MotionRectangles::MotionRectangles { &self.base }
    fn as_MotionRectangles_mut(&mut self) -> &mut crate::org::allbinary::media::image::comparison::motion::MotionRectangles::MotionRectangles { &mut self.base }
}


