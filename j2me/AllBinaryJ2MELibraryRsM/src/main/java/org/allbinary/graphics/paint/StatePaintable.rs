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
// package org.allbinary.graphics.paint
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::graphics::displayable::StateControlledInterface::StateControlledInterface;
use super::Paintable::Paintable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct StatePaintable {
    pub base: Paintable,
}

impl StateControlledInterface for StatePaintable {
    fn setState(&mut self, mut state: i32) { Self::setState(self, state) }
}

impl StatePaintable {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: Paintable::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn setState(&mut self, mut state: i32) {
    }
}
impl std::ops::Deref for StatePaintable {
    type Target = Paintable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for StatePaintable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait StatePaintableDyn: crate::org::allbinary::graphics::paint::Paintable::PaintableDyn + crate::org::allbinary::graphics::displayable::StateControlledInterface::StateControlledInterface {
    fn as_StatePaintable(&self) -> &StatePaintable;
    fn as_StatePaintable_mut(&mut self) -> &mut StatePaintable;
}
impl StatePaintableDyn for StatePaintable {
    fn as_StatePaintable(&self) -> &StatePaintable { self }
    fn as_StatePaintable_mut(&mut self) -> &mut StatePaintable { self }
}

impl crate::org::allbinary::graphics::paint::Paintable::PaintableDyn for StatePaintable {
    fn as_Paintable(&self) -> &crate::org::allbinary::graphics::paint::Paintable::Paintable { &self.base }
    fn as_Paintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::Paintable::Paintable { &mut self.base }
}


