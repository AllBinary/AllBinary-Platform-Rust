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
// package org.allbinary.game.collision
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::configuration::feature::Features::Features;
use crate::org::allbinary::game::configuration::feature::GameFeatureFactory::GameFeatureFactory;
use crate::org::allbinary::graphics::opengles::OpenGLFeatureUtil::OpenGLFeatureUtil;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use super::CollisionProcessor::CollisionProcessor;
use super::Collision2DProcessor::Collision2DProcessor;
use super::CollisionThreedProcessor::CollisionThreedProcessor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LayerCollisionUtil {
    pub collisionProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::game::collision::CollisionProcessor::CollisionProcessorDyn>>>,
}

thread_local! { pub static LayerCollisionUtil_instance: Option<Rc<RefCell<LayerCollisionUtil>>> = LayerCollisionUtil::new(); }

impl LayerCollisionUtil {
    

    pub fn getInstance() -> Option<Rc<RefCell<LayerCollisionUtil>>> {
        return LayerCollisionUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            collisionProcessor: None,
        };
        let mut features: Option<Rc<RefCell<Features>>> = Features::getInstance();
        let mut gameFeatureFactory: Option<Rc<RefCell<GameFeatureFactory>>> = GameFeatureFactory::getInstance();
        if jcall!(OpenGLFeatureUtil::getInstance()).isAnyThreed() && !jcall!(features).isFeature(jcall!(gameFeatureFactory).COLLISIONS_FORCED_TWO_DIMENSIONAL.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
            this.collisionProcessor = CollisionThreedProcessor::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::collision::CollisionProcessor::CollisionProcessorDyn>>);
        } else {
            this.collisionProcessor = Collision2DProcessor::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::collision::CollisionProcessor::CollisionProcessorDyn>>);
        }
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn isCollision(&mut self, mut myLayer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut myLayer2: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) -> bool {
        return jcall!(self.collisionProcessor).isCollision(myLayer.clone(), myLayer2.clone());
    }
}

