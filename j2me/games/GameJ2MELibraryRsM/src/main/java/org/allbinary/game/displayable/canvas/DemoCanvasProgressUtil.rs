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
use crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvas;
use crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvasFactory::ProgressCanvasFactory;
use crate::org::allbinary::graphics::paint::PaintableComposite::PaintableComposite;
use crate::org::allbinary::graphics::paint::PaintableInterface::PaintableInterface;
use super::DemoCanvas::DemoCanvas;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct DemoCanvasProgressUtil {
}


impl DemoCanvasProgressUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn showProgress(mut demoCanvas: Option<Rc<RefCell<DemoCanvas>>>) {
        DemoCanvasProgressUtil::showProgressOptionallyInTheBackground(demoCanvas.clone(), false);
    }
    

    pub fn showProgressOptionallyInTheBackground(mut demoCanvas: Option<Rc<RefCell<DemoCanvas>>>, mut background: bool) {
        let mut progressCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvasDyn>>> = ProgressCanvasFactory::getInstance();
        jcall!(progressCanvas).startBackground(background.clone());
        let mut paintableArray: Vec<Option<Rc<RefCell<dyn PaintableInterface>>>> = vec![None; (2) as usize];
        paintableArray[(0) as usize] = progressCanvas.clone().map(|reference| reference as Rc<RefCell<dyn PaintableInterface>>);
        paintableArray[(1) as usize] = jcall!(demoCanvas).getDefaultPaintableInterface();
        jcall!(demoCanvas).setDefaultPaintableInterface(PaintableComposite::new(paintableArray.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>));
        jcall!(demoCanvas).setPaintableInterface(jcall!(demoCanvas).getDefaultPaintableInterface());
    }
}

