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
// package org.allbinary.game.displayable.canvas
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::graphics::displayable::DisplayInfoSingleton::DisplayInfoSingleton;
use crate::org::allbinary::graphics::draw::CanvasDrawLineString::CanvasDrawLineString;
use crate::org::allbinary::graphics::paint::InitUpdatePaintable::InitUpdatePaintable;
use super::PreGameScrollSelectionForm::PreGameScrollSelectionForm;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PreGameSelectorPaintable {
    pub base: InitUpdatePaintable,
    pub title: String,
    pub canvasDrawLineString: Option<Rc<RefCell<CanvasDrawLineString>>>,
    pub preGameScrollSelectionForm: Option<Rc<RefCell<PreGameScrollSelectionForm>>>,
}

impl crate::org::allbinary::graphics::paint::InitUpdatePaintableInterface::InitUpdatePaintableInterface for PreGameSelectorPaintable {
}
impl crate::org::allbinary::init::InitInterface::InitInterface for PreGameSelectorPaintable {
    fn init(&mut self) { self.base.init() }
}
impl crate::org::allbinary::init::UpdateInterface::UpdateInterface for PreGameSelectorPaintable {
    fn update(&mut self) { Self::update(self) }
}

impl PreGameSelectorPaintable {
    

    pub fn new_struct(mut title: String, mut preGameScrollSelectionForm: Option<Rc<RefCell<PreGameScrollSelectionForm>>>) -> Self {
        let mut this = Self {
            base: InitUpdatePaintable::new_struct(),
            title: String::new(),
            canvasDrawLineString: None,
            preGameScrollSelectionForm: None,
        };
        this.title = title.clone();
        let mut displayInfo: Option<Rc<RefCell<DisplayInfoSingleton>>> = DisplayInfoSingleton::getInstance();
        this.canvasDrawLineString = CanvasDrawLineString::new(jcall!(displayInfo).getLastHalfWidth(), 0);
        this.preGameScrollSelectionForm = preGameScrollSelectionForm.clone();
        this
    }

    pub fn new(mut title: String, mut preGameScrollSelectionForm: Option<Rc<RefCell<PreGameScrollSelectionForm>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(title, preGameScrollSelectionForm))))
    }
    

    pub fn update(&mut self) {
        let mut displayInfo: Option<Rc<RefCell<DisplayInfoSingleton>>> = DisplayInfoSingleton::getInstance();
        self.canvasDrawLineString = CanvasDrawLineString::new(jcall!(displayInfo).getLastHalfWidth(), 0);
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        jcall!(graphics).setColor(jcall!(jcall!(BasicColorFactory::getInstance()).WHITE).intValue());
        jcall!(self.canvasDrawLineString).paint(graphics.clone(), self.title.clone(), 0);
        jcall!(self.getPreGameSelectionForm()).paint(graphics.clone());
    }
    

    pub fn getPreGameSelectionForm(&mut self) -> Option<Rc<RefCell<PreGameScrollSelectionForm>>> {
        return self.preGameScrollSelectionForm.clone();
    }
}
impl std::ops::Deref for PreGameSelectorPaintable {
    type Target = InitUpdatePaintable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for PreGameSelectorPaintable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::paint::InitUpdatePaintable::InitUpdatePaintableDyn for PreGameSelectorPaintable {
    fn as_InitUpdatePaintable(&self) -> &crate::org::allbinary::graphics::paint::InitUpdatePaintable::InitUpdatePaintable { &self.base }
    fn as_InitUpdatePaintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::InitUpdatePaintable::InitUpdatePaintable { &mut self.base }
    fn update(&mut self) { Self::update(self) }
}
impl crate::org::allbinary::graphics::paint::Paintable::PaintableDyn for PreGameSelectorPaintable {
    fn as_Paintable(&self) -> &crate::org::allbinary::graphics::paint::Paintable::Paintable { &self.base.base }
    fn as_Paintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::Paintable::Paintable { &mut self.base.base }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
}


