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
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::vector::VectorInfo::VectorInfo;
use super::ProceduralAnimationInterfaceFactoryInterface::ProceduralAnimationInterfaceFactoryInterface;
use super::VectorAnimationFactoryInterface::VectorAnimationFactoryInterface;
use super::VectorExplosionGenerator::VectorExplosionGenerator;
use super::IndexedAnimation::IndexedAnimation;
use super::Animation::Animation;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct IndependentVectorExplosionRotationAnimationFactory {
    pub framePoints: Vec<Vec<Vec<i32>>>,
    pub vectorInfo: Option<Rc<RefCell<VectorInfo>>>,
    pub basicColor: Option<Rc<RefCell<BasicColor>>>,
    pub vectorAnimationFactoryInterface: Option<Rc<RefCell<dyn VectorAnimationFactoryInterface>>>,
    pub vectorExplosionGenerator: Option<Rc<RefCell<VectorExplosionGenerator>>>,
}

impl ProceduralAnimationInterfaceFactoryInterface for IndependentVectorExplosionRotationAnimationFactory {
    fn getInstanceAnimation(&mut self, mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> { Self::getInstanceAnimation(self, animationInterface) }
}
impl crate::org::allbinary::animation::BasicAnimationInterfaceFactoryInterface::BasicAnimationInterfaceFactoryInterface for IndependentVectorExplosionRotationAnimationFactory {
}

impl IndependentVectorExplosionRotationAnimationFactory {
    

    pub fn new_struct(mut vectorInfo: Option<Rc<RefCell<VectorInfo>>>, mut basicColor: Option<Rc<RefCell<BasicColor>>>, mut vectorAnimationFactoryInterface: Option<Rc<RefCell<dyn VectorAnimationFactoryInterface>>>) -> Self {
        let mut this = Self {
            framePoints: Vec::new(),
            vectorInfo: None,
            basicColor: None,
            vectorAnimationFactoryInterface: None,
            vectorExplosionGenerator: None,
        };
        this.framePoints = jcall!(NullUtil::getInstance()).NULL_INT_ARRAY_ARRAY_ARRAY.clone();
        this.vectorExplosionGenerator = VectorExplosionGenerator::getInstance();
        this.vectorInfo = vectorInfo.clone();
        this.basicColor = basicColor.clone();
        this.vectorAnimationFactoryInterface = vectorAnimationFactoryInterface.clone();
        this.init();
        this
    }

    pub fn new(mut vectorInfo: Option<Rc<RefCell<VectorInfo>>>, mut basicColor: Option<Rc<RefCell<BasicColor>>>, mut vectorAnimationFactoryInterface: Option<Rc<RefCell<dyn VectorAnimationFactoryInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(vectorInfo, basicColor, vectorAnimationFactoryInterface))))
    }
    

    // throws Exception
    pub fn init(&mut self) {
        self.framePoints = jcall!(VectorRotationGenerator::VectorRotationGenerator::getInstance()).getInstance1(self.vectorInfo.clone());
    }
    

    // throws Exception
    pub fn getInstanceAnimation(&mut self, mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> {
        let mut frame: i32 = 0;
        if animationInterface.is_some() {
            let mut rotationAnimationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn>>> = (animationInterface).clone() /* as IndexedAnimation */;
            frame = jcall!(rotationAnimationInterface).getFrame();
        }
        let mut points: Vec<Vec<Vec<i32>>> = jcall!(self.vectorExplosionGenerator).getInstance3(self.framePoints[(frame) as usize].clone(), 6, jcall!(self.vectorExplosionGenerator).ROTATION.clone());
        return jcall!(self.vectorAnimationFactoryInterface).getInstance(points.clone(), self.basicColor.clone());
    }
}

