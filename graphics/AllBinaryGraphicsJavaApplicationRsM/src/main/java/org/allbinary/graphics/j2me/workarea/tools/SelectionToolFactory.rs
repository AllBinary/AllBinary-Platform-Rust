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
// package org.allbinary.graphics.j2me.workarea.tools
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::graphics::j2me::workarea::canvas::CanvasJPanel::CanvasJPanel;
use super::GraphicsItemInterfaceFactoryInterface::GraphicsItemInterfaceFactoryInterface;
use super::SelectionTool::SelectionTool;
use super::GraphicItemInterface::GraphicItemInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SelectionToolFactory {
}

impl GraphicsItemInterfaceFactoryInterface for SelectionToolFactory {
    fn getInstance(&mut self, mut canvasJPanel: Option<Rc<RefCell<CanvasJPanel>>>) -> Option<Rc<RefCell<dyn GraphicItemInterface>>> { Self::getInstance(self, canvasJPanel) }
}

impl SelectionToolFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance(&mut self, mut canvasJPanel: Option<Rc<RefCell<CanvasJPanel>>>) -> Option<Rc<RefCell<dyn GraphicItemInterface>>> {
        return SelectionTool::new(canvasJPanel.clone()).map(|reference| reference as Rc<RefCell<dyn GraphicItemInterface>>);
    }
}

