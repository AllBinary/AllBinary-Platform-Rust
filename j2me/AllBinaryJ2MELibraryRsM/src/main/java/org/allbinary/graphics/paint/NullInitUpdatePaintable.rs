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
use super::InitUpdatePaintable::InitUpdatePaintable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NullInitUpdatePaintable {
    pub base: InitUpdatePaintable,
}

thread_local! { pub static NullInitUpdatePaintable_SINGLETON: Option<Rc<RefCell<NullInitUpdatePaintable>>> = NullInitUpdatePaintable::new(); }
impl crate::org::allbinary::graphics::paint::InitUpdatePaintableInterface::InitUpdatePaintableInterface for NullInitUpdatePaintable {
}
impl crate::org::allbinary::init::InitInterface::InitInterface for NullInitUpdatePaintable {
    fn init(&mut self) { self.base.init() }
}
impl crate::org::allbinary::init::UpdateInterface::UpdateInterface for NullInitUpdatePaintable {
    fn update(&mut self) { self.base.update() }
}

impl NullInitUpdatePaintable {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: InitUpdatePaintable::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::InitUpdatePaintable::InitUpdatePaintableDyn>>> {
        return NullInitUpdatePaintable_SINGLETON.with(Clone::clone).clone();
    }
}
impl std::ops::Deref for NullInitUpdatePaintable {
    type Target = InitUpdatePaintable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for NullInitUpdatePaintable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::paint::InitUpdatePaintable::InitUpdatePaintableDyn for NullInitUpdatePaintable {
    fn as_InitUpdatePaintable(&self) -> &crate::org::allbinary::graphics::paint::InitUpdatePaintable::InitUpdatePaintable { &self.base }
    fn as_InitUpdatePaintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::InitUpdatePaintable::InitUpdatePaintable { &mut self.base }
}
impl crate::org::allbinary::graphics::paint::Paintable::PaintableDyn for NullInitUpdatePaintable {
    fn as_Paintable(&self) -> &crate::org::allbinary::graphics::paint::Paintable::Paintable { &self.base.base }
    fn as_Paintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::Paintable::Paintable { &mut self.base.base }
}


