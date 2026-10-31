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
// use crate::javax::microedition::lcdui::Canvas::Canvas;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
// use crate::javax::microedition::lcdui::Image::Image;
// use crate::javax::microedition::lcdui::NullCanvas::NullCanvas;
// use crate::javax::microedition::lcdui::NullImage::NullImage;
use crate::org::allbinary::graphics::Anchor::Anchor;
use crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas;
use crate::org::allbinary::graphics::paint::ProcessPaintable::ProcessPaintable;
use crate::org::allbinary::util::CircularIndexUtil::CircularIndexUtil;
use super::AllBinaryGameCanvas::AllBinaryGameCanvas;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BufferedGameCanvasPaintHelper {
    pub base: ProcessPaintable,
    pub gameCanvas: Option<Rc<RefCell<Canvas>>>,
    pub offScreenImage: Vec<Option<Rc<RefCell<Image>>>>,
    pub previousImage: Option<Rc<RefCell<Image>>>,
    pub circularIndexUtil: Option<Rc<RefCell<CircularIndexUtil>>>,
    pub drawCircularIndexUtil: Option<Rc<RefCell<CircularIndexUtil>>>,
    pub anchor: i32,
}

thread_local! { pub static BufferedGameCanvasPaintHelper_MAX_IMAGES: i32 = 4; }

impl BufferedGameCanvasPaintHelper {
    

    pub fn new_struct(mut gameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) -> Self {
        let mut this = Self {
            base: ProcessPaintable::new_struct(),
            gameCanvas: None,
            offScreenImage: Vec::new(),
            previousImage: None,
            circularIndexUtil: None,
            drawCircularIndexUtil: None,
            anchor: 0,
        };
        this.gameCanvas = crate::javax::microedition::lcdui::NullCanvas::NullCanvas_NULL_CANVAS.with(Clone::clone).clone();
        this.offScreenImage = vec![None; (BufferedGameCanvasPaintHelper_MAX_IMAGES.with(Clone::clone)) as usize];
        this.previousImage = crate::javax::microedition::lcdui::NullImage::NullImage_NULL_IMAGE.with(Clone::clone).clone();
        this.circularIndexUtil = CircularIndexUtil::createInstance(BufferedGameCanvasPaintHelper_MAX_IMAGES.with(Clone::clone).clone());
        this.drawCircularIndexUtil = CircularIndexUtil::createInstanceAt(BufferedGameCanvasPaintHelper_MAX_IMAGES.with(Clone::clone) - 1, BufferedGameCanvasPaintHelper_MAX_IMAGES.with(Clone::clone).clone());
        this.anchor = unsafe { crate::org::allbinary::graphics::Anchor::Anchor_TOP_LEFT.clone() }.clone();
        panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("No Longer Used"))));
        this
    }

    pub fn new(mut gameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(gameCanvas))))
    }
    

    pub fn process(&mut self) {
        let mut myCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn>>> = ((self.gameCanvas).clone() /* as MyCanvas */);
        jcall!(myCanvas).draw(jcall!(self.offScreenImage[(jcall!(self.circularIndexUtil).getIndex()) as usize]).getGraphics());
        jcall!(self.circularIndexUtil).next();
        jcall!(self.drawCircularIndexUtil).next();
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        self.previousImage = self.offScreenImage[(jcall!(self.drawCircularIndexUtil).getIndex()) as usize].clone();
        jcall!(graphics).drawImage(self.previousImage.clone(), 0, 0, self.anchor.clone());
    }
}
impl std::ops::Deref for BufferedGameCanvasPaintHelper {
    type Target = ProcessPaintable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for BufferedGameCanvasPaintHelper {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::paint::Paintable::PaintableDyn for BufferedGameCanvasPaintHelper {
    fn as_Paintable(&self) -> &crate::org::allbinary::graphics::paint::Paintable::Paintable { &self.base.base }
    fn as_Paintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::Paintable::Paintable { &mut self.base.base }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
}
impl crate::org::allbinary::graphics::paint::ProcessPaintable::ProcessPaintableDyn for BufferedGameCanvasPaintHelper {
    fn as_ProcessPaintable(&self) -> &crate::org::allbinary::graphics::paint::ProcessPaintable::ProcessPaintable { &self.base }
    fn as_ProcessPaintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::ProcessPaintable::ProcessPaintable { &mut self.base }
    fn process(&mut self) { Self::process(self) }
}


