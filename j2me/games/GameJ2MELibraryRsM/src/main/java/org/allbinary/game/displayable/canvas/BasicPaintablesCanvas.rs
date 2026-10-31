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
// use crate::javax::microedition::lcdui::CommandListener::CommandListener;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManager;
use crate::org::allbinary::game::paint::ColorFillBasePaintable::ColorFillBasePaintable;
use crate::org::allbinary::game::paint::ColorFillPaintableFactory::ColorFillPaintableFactory;
use crate::org::allbinary::graphics::paint::Paintable::Paintable;
use super::GameCommandCanvas::GameCommandCanvas;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicPaintablesCanvas {
    pub base: GameCommandCanvas,
    pub paintableArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>>,
    pub colorFillPaintable: Option<Rc<RefCell<dyn crate::org::allbinary::game::paint::ColorFillBasePaintable::ColorFillBasePaintableDyn>>>,
}

impl crate::org::allbinary::game::displayable::canvas::MenuListener::MenuListener for BasicPaintablesCanvas {
    fn open(&mut self) { self.base.open() }
    fn close(&mut self) { self.base.close() }
}
impl crate::org::allbinary::game::input::GameKeyEventSourceInterface::GameKeyEventSourceInterface for BasicPaintablesCanvas {
    fn getSourceId(&mut self) -> i32 { self.base.getSourceId() }
}

impl BasicPaintablesCanvas {
    

    pub fn new_struct(mut cmdListener: Option<Rc<RefCell<CommandListener>>>, mut name: String, mut allBinaryGameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>, mut paintableArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>>) -> Self {
        let mut this = Self {
            base: GameCommandCanvas::new_struct(cmdListener.clone(), name.clone(), jcall!(allBinaryGameLayerManager).getBackgroundBasicColor(), jcall!(allBinaryGameLayerManager).getForegroundBasicColor()),
            paintableArray: Vec::new(),
            colorFillPaintable: None,
        };
        // super(...) is in the struct literal above
        this.paintableArray = paintableArray.clone();
        this.colorFillPaintable = jcall!(ColorFillPaintableFactory::getInstance()).getInstance2(jcall!(allBinaryGameLayerManager).getBackgroundBasicColor(), false);
        this
    }

    pub fn new(mut cmdListener: Option<Rc<RefCell<CommandListener>>>, mut name: String, mut allBinaryGameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>, mut paintableArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(cmdListener, name, allBinaryGameLayerManager, paintableArray))))
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        jcall!(self.colorFillPaintable).paint(graphics.clone());
        jcall!(graphics).setColor(self.foregroundColor.clone());
        for index in (0..=(self.paintableArray.len() as i32) - 1).rev() {
            jcall!(self.paintableArray[(index) as usize]).paint(graphics.clone());
        }
        self.base.paint(graphics.clone());
    }
}
impl std::ops::Deref for BasicPaintablesCanvas {
    type Target = GameCommandCanvas;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for BasicPaintablesCanvas {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn for BasicPaintablesCanvas {
    fn as_GameCommandCanvas(&self) -> &crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas { &self.base }
    fn as_GameCommandCanvas_mut(&mut self) -> &mut crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas { &mut self.base }
}
impl crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn for BasicPaintablesCanvas {
    fn as_MyCanvas(&self) -> &crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas { &self.base.base }
    fn as_MyCanvas_mut(&mut self) -> &mut crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas { &mut self.base.base }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
    fn keyPressedByDevice(&mut self, p1: i32, p2: i32) { crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas::keyPressedByDevice(crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn::as_GameCommandCanvas_mut(self), p1, p2) }
    fn keyRepeatedByDevice(&mut self, p1: i32, p2: i32) { crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas::keyRepeatedByDevice(crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn::as_GameCommandCanvas_mut(self), p1, p2) }
    fn keyReleasedByDevice(&mut self, p1: i32, p2: i32) { crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas::keyReleasedByDevice(crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn::as_GameCommandCanvas_mut(self), p1, p2) }
}


