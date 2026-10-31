// 
// AllBinary Open License Version 1 
// Copyright (c) 2022 AllBinary 
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
use crate::org::allbinary::graphics::displayable::event::DisplayChangeEvent::DisplayChangeEvent;
use crate::org::allbinary::graphics::paint::NullPaintable::NullPaintable;
use super::AllBinaryGameCanvas::AllBinaryGameCanvas;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BaseMenuBehavior {
}

thread_local! { pub static BaseMenuBehavior_instance: Option<Rc<RefCell<dyn BaseMenuBehaviorDyn>>> = BaseMenuBehavior::new().map(|reference| reference as Rc<RefCell<dyn BaseMenuBehaviorDyn>>); }

impl BaseMenuBehavior {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn BaseMenuBehaviorDyn>>> {
        return BaseMenuBehavior_instance.with(Clone::clone).clone();
    }
    

    // throws Exception
    pub fn onDisplayChangeEvent(&mut self, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>, mut displayChangeEvent: Option<Rc<RefCell<DisplayChangeEvent>>>) {
    }
    

    pub fn initSpecialPaint(&mut self, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) {
        jcall!(allBinaryGameCanvas).setNonBotPaintableP(NullPaintable::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>));
    }
    

    // throws Exception
    pub fn initMenu(&mut self, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) {
    }
    

    // throws Exception
    pub fn updateMenu(&mut self, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) {
    }
    

    // throws Exception
    pub fn popupMenu(&mut self, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) {
    }
    

    pub fn closeMenu(&mut self, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) {
    }
}
pub trait BaseMenuBehaviorDyn {
    fn as_BaseMenuBehavior(&self) -> &BaseMenuBehavior;
    fn as_BaseMenuBehavior_mut(&mut self) -> &mut BaseMenuBehavior;
    fn onDisplayChangeEvent(&mut self, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>, mut displayChangeEvent: Option<Rc<RefCell<DisplayChangeEvent>>>) { BaseMenuBehavior::onDisplayChangeEvent(self.as_BaseMenuBehavior_mut(), allBinaryGameCanvas, displayChangeEvent) }
    fn initSpecialPaint(&mut self, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) { BaseMenuBehavior::initSpecialPaint(self.as_BaseMenuBehavior_mut(), allBinaryGameCanvas) }
    fn initMenu(&mut self, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) { BaseMenuBehavior::initMenu(self.as_BaseMenuBehavior_mut(), allBinaryGameCanvas) }
    fn updateMenu(&mut self, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) { BaseMenuBehavior::updateMenu(self.as_BaseMenuBehavior_mut(), allBinaryGameCanvas) }
    fn popupMenu(&mut self, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) { BaseMenuBehavior::popupMenu(self.as_BaseMenuBehavior_mut(), allBinaryGameCanvas) }
    fn closeMenu(&mut self, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) { BaseMenuBehavior::closeMenu(self.as_BaseMenuBehavior_mut(), allBinaryGameCanvas) }
}
impl BaseMenuBehaviorDyn for BaseMenuBehavior {
    fn as_BaseMenuBehavior(&self) -> &BaseMenuBehavior { self }
    fn as_BaseMenuBehavior_mut(&mut self) -> &mut BaseMenuBehavior { self }
}


