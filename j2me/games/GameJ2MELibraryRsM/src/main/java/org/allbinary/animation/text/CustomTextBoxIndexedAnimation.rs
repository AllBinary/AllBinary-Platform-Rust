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
// use crate::javax::microedition::lcdui::Font::Font;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::animation::AnimationBehavior::AnimationBehavior;
use crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation;
use crate::org::allbinary::game::input::event::RawKeyEventListener::RawKeyEventListener;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::font::UpdateMyFontInterface::UpdateMyFontInterface;
use crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBox;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use super::TextInterface::TextInterface;
use super::TextChangeListener::TextChangeListener;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CustomTextBoxIndexedAnimation {
    pub base: IndexedAnimation,
    pub customTextBox: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBoxDyn>>>,
    pub lastText: String,
    pub hasChanged: bool,
    pub textChangeListener: Option<Rc<RefCell<TextChangeListener>>>,
}

impl RawKeyEventListener for CustomTextBoxIndexedAnimation {
    fn onEventRaw(&mut self, mut keyCode: i32, mut deviceId: i32, mut repeated: bool) { Self::onEventRaw(self, keyCode, deviceId, repeated) }
}
impl UpdateMyFontInterface for CustomTextBoxIndexedAnimation {
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::updateMeasurement(self, graphics) }
}
// impl TextInterface for CustomTextBoxIndexedAnimation with the methods of the trait
impl crate::org::allbinary::animation::AnimationInterface::AnimationInterface for CustomTextBoxIndexedAnimation {
    fn nextFrame(&mut self) { self.base.nextFrame() }
    fn paintThreedXYZ(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32, mut z: i32) { Self::paintThreedXYZ(self, graphics, x, y, z) }
}
impl crate::org::allbinary::animation::SequentialAnimationInterface::SequentialAnimationInterface for CustomTextBoxIndexedAnimation {
    fn getSize(&mut self) -> i32 { self.base.getSize() }
    fn previousFrame(&mut self) { self.base.previousFrame() }
}
impl crate::org::allbinary::graphics::paint::LocationPaintableInterface::LocationPaintableInterface for CustomTextBoxIndexedAnimation {
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}

impl CustomTextBoxIndexedAnimation {
    

    pub fn new_struct(mut customTextBox: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBoxDyn>>>) -> Self {
        let mut this = Self {
            base: IndexedAnimation::new_struct(AnimationBehavior::getInstance()),
            customTextBox: None,
            lastText: String::new(),
            hasChanged: false,
            textChangeListener: None,
        };
        this.lastText = jcall!(StringUtil::getInstance()).INIT_STRING.clone();
        this.hasChanged = true;
        this.textChangeListener = TextChangeListener::getInstance();
        // super(...) is in the struct literal above
        this.customTextBox = customTextBox.clone();
        this
    }

    pub fn new(mut customTextBox: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBoxDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(customTextBox))))
    }
    

    pub fn setBasicColorP(&mut self, mut basicColor: Option<Rc<RefCell<BasicColor>>>) {
        jcall!(jcall!(self.customTextBox).getTextFieldItem()).setForegroundBasicColorP(basicColor.clone());
    }
    

    pub fn setBackgroundBasicColorP(&mut self, mut basicColor: Option<Rc<RefCell<BasicColor>>>) {
        jcall!(jcall!(self.customTextBox).getTextFieldItem()).setBackgroundBasicColorP(basicColor.clone());
    }
    

    pub fn setTextWithOnMeasure(&mut self, mut text: String, mut textChangeListener: Option<Rc<RefCell<TextChangeListener>>>) {
        self.setText(text.clone());
        self.textChangeListener = textChangeListener.clone();
    }
    

    pub fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        let mut font: Option<Rc<RefCell<Font>>> = jcall!(graphics).getFont();
        jcall!(self.textChangeListener).onMeasure();
        self.textChangeListener = TextChangeListener::getInstance();
    }
    

    pub fn setText(&mut self, mut text: String) {
        if self.lastText != text {
            self.hasChanged = true;
            self.lastText = text.clone();
            jcall!(jcall!(self.customTextBox).getTextFieldItem()).setString(text.clone());
        }
    }
    

    pub fn getText(&mut self) -> String {
        return jcall!(jcall!(self.customTextBox).getTextFieldItem()).getString();
    }
    

    pub fn onEventRaw(&mut self, mut keyCode: i32, mut deviceId: i32, mut repeated: bool) {
        jcall!(self.customTextBox).onEventRaw(keyCode.clone(), deviceId.clone(), repeated.clone());
    }
    

    pub fn getFontHeight(&mut self) -> i32 {
        return jcall!(jcall!(self.customTextBox).getTextFieldItem()).getFontHeight();
    }
    

    pub fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) {
        jcall!(self.customTextBox).paintXY(graphics.clone(), x.clone(), y.clone());
    }
    

    pub fn paintThreedXYZ(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32, mut z: i32) {
    }
}
impl std::ops::Deref for CustomTextBoxIndexedAnimation {
    type Target = IndexedAnimation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for CustomTextBoxIndexedAnimation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::animation::Animation::AnimationDyn for CustomTextBoxIndexedAnimation {
    fn as_Animation(&self) -> &crate::org::allbinary::animation::Animation::Animation { &self.base.base }
    fn as_Animation_mut(&mut self) -> &mut crate::org::allbinary::animation::Animation::Animation { &mut self.base.base }
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
    fn paintThreedXYZ(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32, mut z: i32) { Self::paintThreedXYZ(self, graphics, x, y, z) }
    fn setBasicColorP(&mut self, mut basicColor: Option<Rc<RefCell<BasicColor>>>) { Self::setBasicColorP(self, basicColor) }
    fn setBackgroundBasicColorP(&mut self, mut basicColor: Option<Rc<RefCell<BasicColor>>>) { Self::setBackgroundBasicColorP(self, basicColor) }
}
impl crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn for CustomTextBoxIndexedAnimation {
    fn as_IndexedAnimation(&self) -> &crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &self.base }
    fn as_IndexedAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &mut self.base }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for CustomTextBoxIndexedAnimation {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::animation::Animation::Animation::set(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self), p1) }
}


