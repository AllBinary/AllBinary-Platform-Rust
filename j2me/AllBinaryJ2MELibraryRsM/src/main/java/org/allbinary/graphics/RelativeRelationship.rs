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
// package org.allbinary.graphics
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListUtil::BasicArrayListUtil;
use super::GPoint::GPoint;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RelativeRelationship {
    pub typesAllowedList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

thread_local! { pub static RelativeRelationship_NULL_RELATIVE_RELATIONSHIP: Option<Rc<RefCell<dyn RelativeRelationshipDyn>>> = RelativeRelationship::new(jcall!(PointFactory::PointFactory::getInstance()).ZERO_ZERO.clone(), jcall!(BasicArrayListUtil::getInstance()).getImmutableInstance()).map(|reference| reference as Rc<RefCell<dyn RelativeRelationshipDyn>>); }

impl RelativeRelationship {
    

    pub fn new_struct(mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>, mut typesAllowedList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Self {
        let mut this = Self {
            typesAllowedList: None,
            x: 0,
            y: 0,
            z: 0,
        };
        this.x = jcall!(point).getX();
        this.y = jcall!(point).getY();
        this.z = jcall!(point).getZ();
        this.typesAllowedList = typesAllowedList.clone();
        this
    }

    pub fn new(mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>, mut typesAllowedList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(point, typesAllowedList))))
    }
    

    pub fn getX(&mut self) -> i32 {
        return self.x.clone();
    }
    

    pub fn getY(&mut self) -> i32 {
        return self.y.clone();
    }
    

    pub fn getZ(&mut self) -> i32 {
        return self.z.clone();
    }
    

    pub fn getTypesAllowedList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.typesAllowedList.clone();
    }
    

    pub fn setTypesAllowedList(&mut self, mut typesAllowedList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        self.typesAllowedList = typesAllowedList.clone();
    }
}
pub trait RelativeRelationshipDyn {
    fn as_RelativeRelationship(&self) -> &RelativeRelationship;
    fn as_RelativeRelationship_mut(&mut self) -> &mut RelativeRelationship;
    fn getX(&mut self) -> i32 { RelativeRelationship::getX(self.as_RelativeRelationship_mut()) }
    fn getY(&mut self) -> i32 { RelativeRelationship::getY(self.as_RelativeRelationship_mut()) }
    fn getZ(&mut self) -> i32 { RelativeRelationship::getZ(self.as_RelativeRelationship_mut()) }
    fn getTypesAllowedList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { RelativeRelationship::getTypesAllowedList(self.as_RelativeRelationship_mut()) }
    fn setTypesAllowedList(&mut self, mut typesAllowedList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { RelativeRelationship::setTypesAllowedList(self.as_RelativeRelationship_mut(), typesAllowedList) }
}
impl RelativeRelationshipDyn for RelativeRelationship {
    fn as_RelativeRelationship(&self) -> &RelativeRelationship { self }
    fn as_RelativeRelationship_mut(&mut self) -> &mut RelativeRelationship { self }
}


