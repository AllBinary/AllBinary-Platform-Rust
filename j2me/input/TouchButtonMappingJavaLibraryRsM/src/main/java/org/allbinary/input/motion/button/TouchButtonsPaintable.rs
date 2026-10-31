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
// package org.allbinary.input.motion.button
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::color::BasicColorSetUtil::BasicColorSetUtil;
use crate::org::allbinary::graphics::paint::Paintable::Paintable;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use super::TouchButtonFactory::TouchButtonFactory;
use super::TouchButton::TouchButton;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TouchButtonsPaintable {
    pub base: Paintable,
    pub foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>,
    pub foregroundColor: i32,
    pub basicSetColorUtil: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::color::BasicColorSetUtil::BasicColorSetUtilDyn>>>,
    pub touchButtonFactory: Option<Rc<RefCell<TouchButtonFactory>>>,
}


impl TouchButtonsPaintable {
    

    pub fn new_struct(mut basicColor: Option<Rc<RefCell<BasicColor>>>) -> Self {
        let mut this = Self {
            base: Paintable::new_struct(),
            foregroundBasicColor: None,
            foregroundColor: 0,
            basicSetColorUtil: None,
            touchButtonFactory: None,
        };
        this.basicSetColorUtil = BasicColorSetUtil::getInstance();
        this.touchButtonFactory = TouchButtonFactory::getInstance();
        this.foregroundBasicColor = basicColor.clone();
        this.foregroundColor = jcall!(basicColor).intValue();
        this
    }

    pub fn new(mut basicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(basicColor))))
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(self.touchButtonFactory).getList();
        jcall!(self.basicSetColorUtil).setBasicColorP3(graphics.clone(), self.foregroundBasicColor.clone(), self.foregroundColor.clone());
        let mut touchButton: Option<Rc<RefCell<dyn crate::org::allbinary::input::motion::button::TouchButton::TouchButtonDyn>>> = None;
        {
            let mut index: i32 = jcall!(list).size();
            while { index -= 1; index } >= 0 {
            touchButton = { let java_object = (jcall!(list).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::input::motion::button::TouchButton::TouchButtonDyn>>>>().expect("ClassCastException").clone()) };
            jcall!(touchButton).paint(graphics.clone());
            }
        }
        {
            let mut index: i32 = jcall!(list).size();
            while { index -= 1; index } >= 0 {
            touchButton = { let java_object = (jcall!(list).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::input::motion::button::TouchButton::TouchButtonDyn>>>>().expect("ClassCastException").clone()) };
            jcall!(touchButton).paintHint(graphics.clone());
            }
        }
    }
}
impl std::ops::Deref for TouchButtonsPaintable {
    type Target = Paintable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for TouchButtonsPaintable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::paint::Paintable::PaintableDyn for TouchButtonsPaintable {
    fn as_Paintable(&self) -> &crate::org::allbinary::graphics::paint::Paintable::Paintable { &self.base }
    fn as_Paintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::Paintable::Paintable { &mut self.base }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
}


