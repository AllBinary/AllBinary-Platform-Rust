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
// package org.allbinary.game.layer
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::graphics::paint::Paintable::Paintable;
use crate::org::allbinary::graphics::paint::PaintableInterface::PaintableInterface;
use crate::org::allbinary::layer::Layer::Layer;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PaintableLayerComposite {
    pub base: Paintable,
    pub paintableArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::layer::Layer::LayerDyn>>>>,
}

impl PaintableInterface for PaintableLayerComposite {
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
    fn paintThreed(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paintThreed(self, graphics) }
}

impl PaintableLayerComposite {
    

    pub fn new_struct(mut paintableArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::layer::Layer::LayerDyn>>>>) -> Self {
        let mut this = Self {
            base: Paintable::new_struct(),
            paintableArray: Vec::new(),
        };
        this.paintableArray = paintableArray.clone();
        this
    }

    pub fn new(mut paintableArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::layer::Layer::LayerDyn>>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(paintableArray))))
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        {
            let mut index: i32 = (self.paintableArray.len() as i32).clone();
            while { index -= 1; index } >= 0 {
            jcall!(self.paintableArray[(index) as usize]).paint(graphics.clone());
            }
        }
    }
    

    pub fn paintThreed(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        let mut paintableInterface: Option<Rc<RefCell<dyn PaintableInterface>>> = None;
        {
            let mut index: i32 = (self.paintableArray.len() as i32).clone();
            while { index -= 1; index } >= 0 {
            paintableInterface = (self.paintableArray[(index) as usize]).clone() /* as PaintableInterface */;
            jcall!(paintableInterface).paintThreed(graphics.clone());
            }
        }
    }
}
impl std::ops::Deref for PaintableLayerComposite {
    type Target = Paintable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for PaintableLayerComposite {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::paint::Paintable::PaintableDyn for PaintableLayerComposite {
    fn as_Paintable(&self) -> &crate::org::allbinary::graphics::paint::Paintable::Paintable { &self.base }
    fn as_Paintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::Paintable::Paintable { &mut self.base }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
    fn paintThreed(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paintThreed(self, graphics) }
}


