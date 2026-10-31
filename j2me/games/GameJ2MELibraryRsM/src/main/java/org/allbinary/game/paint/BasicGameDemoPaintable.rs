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
// package org.allbinary.game.paint
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::graphics::paint::NullPaintable::NullPaintable;
use crate::org::allbinary::graphics::paint::Paintable::Paintable;
use crate::org::allbinary::graphics::paint::StatePaintable::StatePaintable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicGameDemoPaintable {
    pub base: StatePaintable,
    pub mainDemoStatePaintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>,
    pub ownershipPaintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>,
    pub helpPaintableInterface: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>,
    pub currentStatePaintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>,
}


impl BasicGameDemoPaintable {
    

    pub fn new_struct(mut mainDemoStatePaintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>, mut ownershipPaintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>, mut helpPaintableInterface: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>) -> Self {
        let mut this = Self {
            base: StatePaintable::new_struct(),
            mainDemoStatePaintable: None,
            ownershipPaintable: None,
            helpPaintableInterface: None,
            currentStatePaintable: None,
        };
        this.currentStatePaintable = NullPaintable::getInstance();
        this.mainDemoStatePaintable = mainDemoStatePaintable.clone();
        { let java_arguments = (NullPaintable::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>),); this.setCurrentStatePaintable(java_arguments.0) };
        this.ownershipPaintable = ownershipPaintable.clone();
        this.helpPaintableInterface = helpPaintableInterface.clone();
        this
    }

    pub fn new(mut mainDemoStatePaintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>, mut ownershipPaintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>, mut helpPaintableInterface: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(mainDemoStatePaintable, ownershipPaintable, helpPaintableInterface))))
    }
    

    pub fn setState(&mut self, mut state: i32) {
        if state == 0 {
            { let java_arguments = (self.getMainDemoStatePaintable(),); self.setCurrentStatePaintable(java_arguments.0) };
        } else if state == 1 {
            self.setCurrentStatePaintable(self.helpPaintableInterface.clone());
        } else {
            self.setCurrentStatePaintable(self.ownershipPaintable.clone());
        }
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        jcall!(self.getCurrentStatePaintable()).paint(graphics.clone());
    }
    

    pub fn getHelpPaintableInterface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>> {
        return self.helpPaintableInterface.clone();
    }
    

    pub fn setCurrentStatePaintable(&mut self, mut currentStatePaintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>) {
        self.currentStatePaintable = currentStatePaintable.clone();
    }
    

    pub fn getCurrentStatePaintable(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>> {
        return self.currentStatePaintable.clone();
    }
    

    pub fn getMainDemoStatePaintable(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>> {
        return self.mainDemoStatePaintable.clone();
    }
}
impl std::ops::Deref for BasicGameDemoPaintable {
    type Target = StatePaintable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for BasicGameDemoPaintable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait BasicGameDemoPaintableDyn: crate::org::allbinary::graphics::paint::StatePaintable::StatePaintableDyn {
    fn as_BasicGameDemoPaintable(&self) -> &BasicGameDemoPaintable;
    fn as_BasicGameDemoPaintable_mut(&mut self) -> &mut BasicGameDemoPaintable;
    fn getHelpPaintableInterface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>> { BasicGameDemoPaintable::getHelpPaintableInterface(self.as_BasicGameDemoPaintable_mut()) }
    fn setCurrentStatePaintable(&mut self, mut currentStatePaintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>) { BasicGameDemoPaintable::setCurrentStatePaintable(self.as_BasicGameDemoPaintable_mut(), currentStatePaintable) }
    fn getCurrentStatePaintable(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>> { BasicGameDemoPaintable::getCurrentStatePaintable(self.as_BasicGameDemoPaintable_mut()) }
    fn getMainDemoStatePaintable(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>> { BasicGameDemoPaintable::getMainDemoStatePaintable(self.as_BasicGameDemoPaintable_mut()) }
}
impl BasicGameDemoPaintableDyn for BasicGameDemoPaintable {
    fn as_BasicGameDemoPaintable(&self) -> &BasicGameDemoPaintable { self }
    fn as_BasicGameDemoPaintable_mut(&mut self) -> &mut BasicGameDemoPaintable { self }
}

impl crate::org::allbinary::graphics::paint::Paintable::PaintableDyn for BasicGameDemoPaintable {
    fn as_Paintable(&self) -> &crate::org::allbinary::graphics::paint::Paintable::Paintable { &self.base.base }
    fn as_Paintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::Paintable::Paintable { &mut self.base.base }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
}
impl crate::org::allbinary::graphics::paint::StatePaintable::StatePaintableDyn for BasicGameDemoPaintable {
    fn as_StatePaintable(&self) -> &crate::org::allbinary::graphics::paint::StatePaintable::StatePaintable { &self.base }
    fn as_StatePaintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::StatePaintable::StatePaintable { &mut self.base }
}
impl crate::org::allbinary::graphics::displayable::StateControlledInterface::StateControlledInterface for BasicGameDemoPaintable {
    fn setState(&mut self, mut state: i32) { Self::setState(self, state) }
}


