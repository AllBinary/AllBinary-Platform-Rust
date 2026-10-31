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
use crate::org::allbinary::graphics::GPoint::GPoint;
use crate::org::allbinary::graphics::RelativeRelationship::RelativeRelationship;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RelativeLayerRelationship {
    pub base: RelativeRelationship,
    pub layer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>,
}


impl RelativeLayerRelationship {
    

    pub fn new_struct(mut layer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>, mut typesAllowedList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Self {
        let mut this = Self {
            base: RelativeRelationship::new_struct(point.clone(), typesAllowedList.clone()),
            layer: None,
        };
        this.layer = crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer_NULL_ALLBINARY_LAYER.with(Clone::clone).clone();
        // super(...) is in the struct literal above
        this.layer = layer.clone();
        this
    }

    pub fn new(mut layer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>, mut typesAllowedList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(layer, point, typesAllowedList))))
    }
    

    pub fn getX(&mut self) -> i32 {
        return jcall!(self.layer).getXP() + self.base.getX();
    }
    

    pub fn getY(&mut self) -> i32 {
        return jcall!(self.layer).getYP() + self.base.getY();
    }
    

    pub fn getZ(&mut self) -> i32 {
        return jcall!(self.layer).getZP() + self.base.getZ();
    }
}
impl std::ops::Deref for RelativeLayerRelationship {
    type Target = RelativeRelationship;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for RelativeLayerRelationship {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait RelativeLayerRelationshipDyn: crate::org::allbinary::graphics::RelativeRelationship::RelativeRelationshipDyn {
    fn as_RelativeLayerRelationship(&self) -> &RelativeLayerRelationship;
    fn as_RelativeLayerRelationship_mut(&mut self) -> &mut RelativeLayerRelationship;
}
impl RelativeLayerRelationshipDyn for RelativeLayerRelationship {
    fn as_RelativeLayerRelationship(&self) -> &RelativeLayerRelationship { self }
    fn as_RelativeLayerRelationship_mut(&mut self) -> &mut RelativeLayerRelationship { self }
}

impl crate::org::allbinary::graphics::RelativeRelationship::RelativeRelationshipDyn for RelativeLayerRelationship {
    fn as_RelativeRelationship(&self) -> &crate::org::allbinary::graphics::RelativeRelationship::RelativeRelationship { &self.base }
    fn as_RelativeRelationship_mut(&mut self) -> &mut crate::org::allbinary::graphics::RelativeRelationship::RelativeRelationship { &mut self.base }
    fn getX(&mut self) -> i32 { Self::getX(self) }
    fn getY(&mut self) -> i32 { Self::getY(self) }
    fn getZ(&mut self) -> i32 { Self::getZ(self) }
}


