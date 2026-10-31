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
use super::BaseMenuBehavior::BaseMenuBehavior;
use super::AllBinaryGameCanvas::AllBinaryGameCanvas;
use super::GameCanvasNonBotPaintable::GameCanvasNonBotPaintable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InGameMenuBehavior {
    pub base: BaseMenuBehavior,
}

thread_local! { pub static InGameMenuBehavior_instanceC: Option<Rc<RefCell<InGameMenuBehavior>>> = InGameMenuBehavior::new(); }

impl InGameMenuBehavior {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: BaseMenuBehavior::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<InGameMenuBehavior>>> {
        return InGameMenuBehavior_instanceC.with(Clone::clone).clone();
    }
    

    // throws Exception
    pub fn onDisplayChangeEvent(&mut self, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>, mut displayChangeEvent: Option<Rc<RefCell<DisplayChangeEvent>>>) {
        jcall!(allBinaryGameCanvas).updateMenuFromEvent(displayChangeEvent.clone());
    }
    

    pub fn initSpecialPaint(&mut self, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) {
        jcall!(allBinaryGameCanvas).setNonBotPaintableP(GameCanvasNonBotPaintable::new(allBinaryGameCanvas.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>));
    }
    

    // throws Exception
    pub fn initMenu(&mut self, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) {
        jcall!(allBinaryGameCanvas).initMenu2();
    }
    

    // throws Exception
    pub fn updateMenu(&mut self, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) {
        jcall!(allBinaryGameCanvas).initMenu2();
    }
    

    // throws Exception
    pub fn popupMenu(&mut self, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) {
        jcall!(allBinaryGameCanvas).popupMenu2();
    }
    

    pub fn closeMenu(&mut self, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) {
        jcall!(allBinaryGameCanvas).closeMenu2();
    }
}
impl std::ops::Deref for InGameMenuBehavior {
    type Target = BaseMenuBehavior;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for InGameMenuBehavior {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::displayable::canvas::BaseMenuBehavior::BaseMenuBehaviorDyn for InGameMenuBehavior {
    fn as_BaseMenuBehavior(&self) -> &crate::org::allbinary::game::displayable::canvas::BaseMenuBehavior::BaseMenuBehavior { &self.base }
    fn as_BaseMenuBehavior_mut(&mut self) -> &mut crate::org::allbinary::game::displayable::canvas::BaseMenuBehavior::BaseMenuBehavior { &mut self.base }
    fn onDisplayChangeEvent(&mut self, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>, mut displayChangeEvent: Option<Rc<RefCell<DisplayChangeEvent>>>) { Self::onDisplayChangeEvent(self, allBinaryGameCanvas, displayChangeEvent) }
    fn initSpecialPaint(&mut self, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) { Self::initSpecialPaint(self, allBinaryGameCanvas) }
    fn initMenu(&mut self, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) { Self::initMenu(self, allBinaryGameCanvas) }
    fn updateMenu(&mut self, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) { Self::updateMenu(self, allBinaryGameCanvas) }
    fn popupMenu(&mut self, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) { Self::popupMenu(self, allBinaryGameCanvas) }
    fn closeMenu(&mut self, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) { Self::closeMenu(self, allBinaryGameCanvas) }
}


