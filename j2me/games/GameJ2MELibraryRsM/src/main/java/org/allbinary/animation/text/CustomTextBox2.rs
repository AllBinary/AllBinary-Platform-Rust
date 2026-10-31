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
// package org.allbinary.animation.text
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::CommandListener::CommandListener;
// use crate::javax::microedition::lcdui::Font::Font;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBox;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CustomTextBox2 {
    pub base: CustomTextBox,
}

impl crate::org::allbinary::game::displayable::canvas::MenuListener::MenuListener for CustomTextBox2 {
    fn open(&mut self) { self.base.open() }
    fn close(&mut self) { self.base.close() }
}
impl crate::org::allbinary::game::input::GameKeyEventSourceInterface::GameKeyEventSourceInterface for CustomTextBox2 {
    fn getSourceId(&mut self) -> i32 { self.base.getSourceId() }
}

impl CustomTextBox2 {
    

    pub fn new_struct(mut cmdListener: Option<Rc<RefCell<CommandListener>>>, mut label: String, mut text: String, mut maxSize: i32, mut constraints: i32, mut font: Option<Rc<RefCell<Font>>>, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Self {
        let mut this = Self {
            base: CustomTextBox::new_struct(cmdListener.clone(), label.clone(), text.clone(), maxSize.clone(), constraints.clone(), font.clone(), backgroundBasicColor.clone(), foregroundBasicColor.clone()),
        };
        // super(...) is in the struct literal above
        jcall!(this.getTextFieldItem()).setFocus(true);
        this
    }

    pub fn new(mut cmdListener: Option<Rc<RefCell<CommandListener>>>, mut label: String, mut text: String, mut maxSize: i32, mut constraints: i32, mut font: Option<Rc<RefCell<Font>>>, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(cmdListener, label, text, maxSize, constraints, font, backgroundBasicColor, foregroundBasicColor))))
    }
}
impl std::ops::Deref for CustomTextBox2 {
    type Target = CustomTextBox;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for CustomTextBox2 {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBoxDyn for CustomTextBox2 {
    fn as_CustomTextBox(&self) -> &crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBox { &self.base }
    fn as_CustomTextBox_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBox { &mut self.base }
}
impl crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn for CustomTextBox2 {
    fn as_GameCommandCanvas(&self) -> &crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas { &self.base.base }
    fn as_GameCommandCanvas_mut(&mut self) -> &mut crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas { &mut self.base.base }
    fn keyPressed(&mut self, p1: i32) { crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBox::keyPressed(crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBoxDyn::as_CustomTextBox_mut(self), p1) }
    fn keyReleased(&mut self, p1: i32) { crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBox::keyReleased(crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBoxDyn::as_CustomTextBox_mut(self), p1) }
    fn keyRepeated(&mut self, p1: i32) { crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBox::keyRepeated(crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBoxDyn::as_CustomTextBox_mut(self), p1) }
}
impl crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn for CustomTextBox2 {
    fn as_MyCanvas(&self) -> &crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas { &self.base.base.base }
    fn as_MyCanvas_mut(&mut self) -> &mut crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas { &mut self.base.base.base }
    fn keyPressedByDevice(&mut self, p1: i32, p2: i32) { crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBox::keyPressedByDevice(crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBoxDyn::as_CustomTextBox_mut(self), p1, p2) }
    fn keyRepeatedByDevice(&mut self, p1: i32, p2: i32) { crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas::keyRepeatedByDevice(crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn::as_GameCommandCanvas_mut(self), p1, p2) }
    fn keyReleasedByDevice(&mut self, p1: i32, p2: i32) { crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBox::keyReleasedByDevice(crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBoxDyn::as_CustomTextBox_mut(self), p1, p2) }
}
impl crate::org::allbinary::game::input::event::RawKeyEventListener::RawKeyEventListener for CustomTextBox2 {
    fn onEventRaw(&mut self, p1: i32, p2: i32, p3: bool) { crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBox::onEventRaw(crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBoxDyn::as_CustomTextBox_mut(self), p1, p2, p3) }
}


