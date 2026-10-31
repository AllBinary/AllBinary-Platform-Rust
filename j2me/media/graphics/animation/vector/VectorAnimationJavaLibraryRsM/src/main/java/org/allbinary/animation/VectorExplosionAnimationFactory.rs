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
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use super::ProceduralAnimationInterfaceFactoryInterface::ProceduralAnimationInterfaceFactoryInterface;
use super::VectorExplosionGenerator::VectorExplosionGenerator;
use super::AnimationBehaviorFactory::AnimationBehaviorFactory;
use super::VectorAnimationInterface::VectorAnimationInterface;
use super::VectorRotationAnimation::VectorRotationAnimation;
use super::Animation::Animation;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct VectorExplosionAnimationFactory {
    pub vectorExplosionGenerator: Option<Rc<RefCell<VectorExplosionGenerator>>>,
    pub basicColor: Option<Rc<RefCell<BasicColor>>>,
    pub animationBehaviorFactory: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactoryDyn>>>,
}

impl ProceduralAnimationInterfaceFactoryInterface for VectorExplosionAnimationFactory {
    fn getInstanceAnimation(&mut self, mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> { Self::getInstanceAnimation(self, animationInterface) }
}
impl crate::org::allbinary::animation::BasicAnimationInterfaceFactoryInterface::BasicAnimationInterfaceFactoryInterface for VectorExplosionAnimationFactory {
}

impl VectorExplosionAnimationFactory {
    

    pub fn new_struct(mut basicColor: Option<Rc<RefCell<BasicColor>>>, mut animationBehaviorFactory: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactoryDyn>>>) -> Self {
        let mut this = Self {
            vectorExplosionGenerator: None,
            basicColor: None,
            animationBehaviorFactory: None,
        };
        this.vectorExplosionGenerator = VectorExplosionGenerator::getInstance();
        this.basicColor = jcall!(BasicColorFactory::getInstance()).WHITE.clone();
        this.animationBehaviorFactory = animationBehaviorFactory.clone();
        this.setBasicColorP(basicColor.clone());
        this
    }

    pub fn new(mut basicColor: Option<Rc<RefCell<BasicColor>>>, mut animationBehaviorFactory: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactoryDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(basicColor, animationBehaviorFactory))))
    }
    

    // throws Exception
    pub fn getInstanceAnimation(&mut self, mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> {
        let mut vectorRotationAnimationInterface: Option<Rc<RefCell<dyn VectorAnimationInterface>>> = (animationInterface).clone() /* as VectorAnimationInterface */;
        let mut frame: i32 = jcall!(vectorRotationAnimationInterface).getFrame();
        let mut framePoints: Vec<Vec<i32>> = jcall!(vectorRotationAnimationInterface).getPoints(frame.clone());
        let mut points: Vec<Vec<Vec<i32>>> = jcall!(self.vectorExplosionGenerator).getInstance3(framePoints.clone(), 6, jcall!(self.vectorExplosionGenerator).ROTATION.clone());
        return VectorRotationAnimation::new(points.clone(), self.getBasicColorP(), jcall!(self.animationBehaviorFactory).getOrCreateInstance()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>);
    }
    

    pub fn setBasicColorP(&mut self, mut basicColor: Option<Rc<RefCell<BasicColor>>>) {
        self.basicColor = basicColor.clone();
    }
    

    pub fn getBasicColorP(&mut self) -> Option<Rc<RefCell<BasicColor>>> {
        return self.basicColor.clone();
    }
}

