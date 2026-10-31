// 
// AllBinary Open License Version 1 
// Copyright (c) 2003 AllBinary 
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
// package org.allbinary.game.layer
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::graphics::paint::InitUpdatePaintable::InitUpdatePaintable;
use crate::org::allbinary::graphics::paint::NullInitUpdatePaintable::NullInitUpdatePaintable;
use super::SelectionHudPaintable::SelectionHudPaintable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RTSLayerInfoPaintable {
    pub base: InitUpdatePaintable,
    pub initUpdatePaintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::InitUpdatePaintable::InitUpdatePaintableDyn>>>,
}

impl crate::org::allbinary::graphics::paint::InitUpdatePaintableInterface::InitUpdatePaintableInterface for RTSLayerInfoPaintable {
}
impl crate::org::allbinary::init::InitInterface::InitInterface for RTSLayerInfoPaintable {
    fn init(&mut self) { self.base.init() }
}
impl crate::org::allbinary::init::UpdateInterface::UpdateInterface for RTSLayerInfoPaintable {
    fn update(&mut self) { Self::update(self) }
}

impl RTSLayerInfoPaintable {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: InitUpdatePaintable::new_struct(),
            initUpdatePaintable: None,
        };
        this.initUpdatePaintable = NullInitUpdatePaintable::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn update(&mut self) {
        jcall!(self.initUpdatePaintable).update();
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        jcall!(self.initUpdatePaintable).paint(graphics.clone());
    }
    

    pub fn updateRTSLayerInfoInit(&mut self, mut hudPaintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::InitUpdatePaintable::InitUpdatePaintableDyn>>>) {
        self.initUpdatePaintable = hudPaintable.clone();
    }
    

    pub fn updateRTSLayerInfoSelection(&mut self, mut hudPaintable: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintableDyn>>>) {
        jcall!(hudPaintable).as_InitUpdatePaintable_mut().updateSelectionInfo();
        self.initUpdatePaintable = hudPaintable.clone();
    }
}
impl std::ops::Deref for RTSLayerInfoPaintable {
    type Target = InitUpdatePaintable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for RTSLayerInfoPaintable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::paint::InitUpdatePaintable::InitUpdatePaintableDyn for RTSLayerInfoPaintable {
    fn as_InitUpdatePaintable(&self) -> &crate::org::allbinary::graphics::paint::InitUpdatePaintable::InitUpdatePaintable { &self.base }
    fn as_InitUpdatePaintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::InitUpdatePaintable::InitUpdatePaintable { &mut self.base }
    fn update(&mut self) { Self::update(self) }
}
impl crate::org::allbinary::graphics::paint::Paintable::PaintableDyn for RTSLayerInfoPaintable {
    fn as_Paintable(&self) -> &crate::org::allbinary::graphics::paint::Paintable::Paintable { &self.base.base }
    fn as_Paintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::Paintable::Paintable { &mut self.base.base }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
}


