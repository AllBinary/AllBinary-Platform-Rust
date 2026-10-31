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
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use super::SourceCollisionHelper::SourceCollisionHelper;
use super::CollisionHelper::CollisionHelper;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CollidableHelperFactory {
}


impl CollidableHelperFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn crate::org::allbinary::game::collision::CollisionHelper::CollisionHelperDyn>>> {
        if jcall!(Features::getInstance()).isFeature(jcall!(GameFeatureFactory::getInstance()).COLLISIONS_WITH_SOURCE.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
            return CollisionHelper::new(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer_NULL_ALLBINARY_LAYER.with(Clone::clone).clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::collision::CollisionHelper::CollisionHelperDyn>>);
        } else {
            return SourceCollisionHelper::new(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer_NULL_ALLBINARY_LAYER.with(Clone::clone).clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::collision::CollisionHelper::CollisionHelperDyn>>);
        }
    }
}

