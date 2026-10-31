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
// package org.allbinary.layer.relationship
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::graphics::PointFactory::PointFactory;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::util::BasicArrayListUtil::BasicArrayListUtil;
use super::RelativeLayerRelationship::RelativeLayerRelationship;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MiddleRelativeLayerRelationship {
    pub base: RelativeLayerRelationship,
}


impl MiddleRelativeLayerRelationship {
    

    pub fn new_struct(mut layer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) -> Self {
        let mut this = Self {
            base: RelativeLayerRelationship::new_struct(layer.clone(), jcall!(PointFactory::getInstance()).createXY(jcall!(layer).getHalfWidth(), jcall!(layer).getHalfHeight()), jcall!(BasicArrayListUtil::getInstance()).getImmutableInstance()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut layer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(layer))))
    }
}
impl std::ops::Deref for MiddleRelativeLayerRelationship {
    type Target = RelativeLayerRelationship;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for MiddleRelativeLayerRelationship {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::layer::relationship::RelativeLayerRelationship::RelativeLayerRelationshipDyn for MiddleRelativeLayerRelationship {
    fn as_RelativeLayerRelationship(&self) -> &crate::org::allbinary::layer::relationship::RelativeLayerRelationship::RelativeLayerRelationship { &self.base }
    fn as_RelativeLayerRelationship_mut(&mut self) -> &mut crate::org::allbinary::layer::relationship::RelativeLayerRelationship::RelativeLayerRelationship { &mut self.base }
}
impl crate::org::allbinary::graphics::RelativeRelationship::RelativeRelationshipDyn for MiddleRelativeLayerRelationship {
    fn as_RelativeRelationship(&self) -> &crate::org::allbinary::graphics::RelativeRelationship::RelativeRelationship { &self.base.base }
    fn as_RelativeRelationship_mut(&mut self) -> &mut crate::org::allbinary::graphics::RelativeRelationship::RelativeRelationship { &mut self.base.base }
    fn getX(&mut self) -> i32 { crate::org::allbinary::layer::relationship::RelativeLayerRelationship::RelativeLayerRelationship::getX(crate::org::allbinary::layer::relationship::RelativeLayerRelationship::RelativeLayerRelationshipDyn::as_RelativeLayerRelationship_mut(self)) }
    fn getY(&mut self) -> i32 { crate::org::allbinary::layer::relationship::RelativeLayerRelationship::RelativeLayerRelationship::getY(crate::org::allbinary::layer::relationship::RelativeLayerRelationship::RelativeLayerRelationshipDyn::as_RelativeLayerRelationship_mut(self)) }
    fn getZ(&mut self) -> i32 { crate::org::allbinary::layer::relationship::RelativeLayerRelationship::RelativeLayerRelationship::getZ(crate::org::allbinary::layer::relationship::RelativeLayerRelationship::RelativeLayerRelationshipDyn::as_RelativeLayerRelationship_mut(self)) }
}


