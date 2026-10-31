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
// use crate::java::awt::Rectangle::Rectangle;
// use crate::java::awt::image::BufferedImage::BufferedImage;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use super::MotionRectangles::MotionRectangles;
use super::MotionRectangleConstraintsInterface::MotionRectangleConstraintsInterface;
use super::MotionRectangle::MotionRectangle;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ConstrainedMotionRectangles {
    pub base: MotionRectangles,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub motionRectangleConstraintsInterface: Option<Rc<RefCell<dyn MotionRectangleConstraintsInterface>>>,
}

thread_local! { pub static ConstrainedMotionRectangles_NAME: String = String::from("constrainedMotionRectangles_"); }

impl ConstrainedMotionRectangles {
    

    pub fn new_struct(mut motionRectangleConstraintsInterface: Option<Rc<RefCell<dyn MotionRectangleConstraintsInterface>>>, mut motionRectangles: Option<Rc<RefCell<dyn crate::org::allbinary::media::image::comparison::motion::MotionRectangles::MotionRectanglesDyn>>>) -> Self {
        let mut this = Self {
            base: MotionRectangles::new_struct(ConstrainedMotionRectangles_NAME.with(Clone::clone).clone(), jcall!(motionRectangles).getImageComparisonResult()),
            logUtil: None,
            motionRectangleConstraintsInterface: None,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        this.setMotionRectangleConstraintsInterface(motionRectangleConstraintsInterface.clone());
        this
    }

    pub fn new(mut motionRectangleConstraintsInterface: Option<Rc<RefCell<dyn MotionRectangleConstraintsInterface>>>, mut motionRectangles: Option<Rc<RefCell<dyn crate::org::allbinary::media::image::comparison::motion::MotionRectangles::MotionRectanglesDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(motionRectangleConstraintsInterface, motionRectangles))))
    }
    

    // throws Exception
    pub fn applyMotionRectangleConstraints(&mut self, mut motionRectangles: Option<Rc<RefCell<dyn crate::org::allbinary::media::image::comparison::motion::MotionRectangles::MotionRectanglesDyn>>>) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Start - Size Before: "), jcall!(jcall!(motionRectangles).getVector()).size()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("applyMotionRectangleConstraints"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        let mut bufferedImageArray: Vec<Option<Rc<RefCell<BufferedImage>>>> = jcall!(jcall!(motionRectangles).getImageComparisonResult()).getBufferedImages();
        let mut bufferedImage: Option<Rc<RefCell<BufferedImage>>> = bufferedImageArray[(1) as usize].clone();
        let mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut vector2: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(motionRectangles).getVector();
        let mut size: i32 = jcall!(vector2).size();
        for index in (0..size) {
            let mut motionRectangle: Option<Rc<RefCell<MotionRectangle>>> = { let java_object = (jcall!(vector2).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<MotionRectangle>>>>().expect("ClassCastException").clone()) };
            let mut rectangle: Option<Rc<RefCell<Rectangle>>> = jcall!(motionRectangle).getRectangle();
            let mut isTooSmall: bool = jcall!(self.getMotionRectangleConstraintsInterface()).isTooSmall(rectangle.clone());
            if isTooSmall {
                continue;
            }
            let mut isAreaTooSmall: bool = jcall!(self.getMotionRectangleConstraintsInterface()).isAreaTooSmall(rectangle.clone());
            if isAreaTooSmall {
                continue;
            }
            let mut isTooBig: bool = jcall!(self.getMotionRectangleConstraintsInterface()).isTooBig(rectangle.clone());
            if isTooBig {
                continue;
            }
            let mut isValid: bool = jcall!(self.getMotionRectangleConstraintsInterface()).isValid(jcall!(self.getImageComparisonResult()).getFrameTwo(), bufferedImage.clone(), jcall!(motionRectangle).getRectangle());
            if !isValid {
                continue;
            }
            if !isTooSmall && !isAreaTooSmall && !isTooBig && isValid {
                jcall!(vector).add(crate::java_into_object(motionRectangle.clone()));
            }
        }
        self.setVector(vector.clone());
    }
    

    pub fn getMotionRectangleConstraintsInterface(&mut self) -> Option<Rc<RefCell<dyn MotionRectangleConstraintsInterface>>> {
        return self.motionRectangleConstraintsInterface.clone();
    }
    

    pub fn setMotionRectangleConstraintsInterface(&mut self, mut motionRectangleConstraintsInterface: Option<Rc<RefCell<dyn MotionRectangleConstraintsInterface>>>) {
        self.motionRectangleConstraintsInterface = motionRectangleConstraintsInterface.clone();
    }
}
impl std::ops::Deref for ConstrainedMotionRectangles {
    type Target = MotionRectangles;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ConstrainedMotionRectangles {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::media::image::comparison::motion::MotionRectangles::MotionRectanglesDyn for ConstrainedMotionRectangles {
    fn as_MotionRectangles(&self) -> &crate::org::allbinary::media::image::comparison::motion::MotionRectangles::MotionRectangles { &self.base }
    fn as_MotionRectangles_mut(&mut self) -> &mut crate::org::allbinary::media::image::comparison::motion::MotionRectangles::MotionRectangles { &mut self.base }
}


