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
// package org.allbinary.animation.text
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::javax::microedition::lcdui::Font::Font;
use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::animation::AnimationBehavior::AnimationBehavior;
use crate::org::allbinary::graphics::font::FontDebugFactory::FontDebugFactory;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::java::lang::RuntimeException::*;
use super::TextAnimation::TextAnimation;
use super::TextInterface::TextInterface;
use super::TextChangeListener::TextChangeListener;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CustomTextAnimation {
    pub base: TextAnimation,
    pub fontDebugFactory: Option<Rc<RefCell<FontDebugFactory>>>,
    pub fontSize: i32,
    pub font: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Font::FontDyn>>>,
    pub lastText: String,
    pub hasChanged: bool,
    pub textWidth: i32,
}

impl TextInterface for CustomTextAnimation {
    fn setTextWithOnMeasure(&mut self, mut text: String, mut textChangeListener: Option<Rc<RefCell<TextChangeListener>>>) { Self::setTextWithOnMeasure(self, text, textChangeListener) }
    fn getText(&mut self) -> String { Self::getText(self) }
    fn setText(&mut self, mut text: String) { Self::setText(self, text) }
    fn getFontHeight(&mut self) -> i32 { Self::getFontHeight(self) }
    fn getWidth(&mut self) -> i32 { Self::getWidth(self) }
}
impl crate::org::allbinary::animation::AnimationInterface::AnimationInterface for CustomTextAnimation {
    fn nextFrame(&mut self) { crate::org::allbinary::animation::text::TextAnimation::TextAnimation::nextFrame(crate::org::allbinary::animation::text::TextAnimation::TextAnimationDyn::as_TextAnimation_mut(self)) }
    fn paintThreedXYZ(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, p2: i32, p3: i32, p4: i32) { crate::org::allbinary::animation::Animation::Animation::paintThreedXYZ(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self), p1, p2, p3, p4) }
}
impl crate::org::allbinary::graphics::paint::LocationPaintableInterface::LocationPaintableInterface for CustomTextAnimation {
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}

impl CustomTextAnimation {
    

    pub fn new_struct(mut text: String, mut fontSize: i32, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Self {
        let mut this = Self {
            base: TextAnimation::new_struct(text.clone(), animationBehavior.clone()),
            fontDebugFactory: None,
            fontSize: 0,
            font: None,
            lastText: String::new(),
            hasChanged: false,
            textWidth: 0,
        };
        this.fontDebugFactory = FontDebugFactory::getInstance();
        this.lastText = { let java_field = jcall!(StringUtil::getInstance()).INIT_STRING.clone(); java_field }.clone();
        this.hasChanged = true;
        // super(...) is in the struct literal above
        this.fontSize = fontSize.clone();
        this.font = { let java_value = (Font::getFont(crate::javax::microedition::lcdui::Font::Font_FACE_SYSTEM.with(Clone::clone).clone(), crate::javax::microedition::lcdui::Font::Font_STYLE_PLAIN.with(Clone::clone).clone(), fontSize.clone())).clone(); java_value };
        this
    }

    pub fn new(mut text: String, mut fontSize: i32, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(text, fontSize, animationBehavior))))
    }
    

    pub fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, mut x: i32, mut y: i32) {
        jcall!(self.myFontProcessor).process(graphics.clone());
        let mut existingFont: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Font::FontDyn>>> = jcall!(graphics).getFont();
        jcall!(self.fontDebugFactory).setFont(self.font.clone(), graphics.clone());
        { let java_arguments = (graphics.clone(), x.clone(), y.clone(),); self.base.paintXYNoUpdate(java_arguments.0, java_arguments.1, java_arguments.2) };
        jcall!(self.fontDebugFactory).setFont(existingFont.clone(), graphics.clone());
    }
    

    pub fn setScale(&mut self, mut scaleX: f32, mut scaleY: f32) {
        if scaleX != scaleY {
            panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
        }
        self.hasChanged = true;
        self.font = { let java_value = (Font::getFont(crate::javax::microedition::lcdui::Font::Font_FACE_SYSTEM.with(Clone::clone).clone(), crate::javax::microedition::lcdui::Font::Font_STYLE_PLAIN.with(Clone::clone).clone(), (((((self.fontSize) as f32) * scaleX)) as i32))).clone(); java_value };
    }
    

    pub fn setTextWithOnMeasure(&mut self, mut text: String, mut textChangeListener: Option<Rc<RefCell<TextChangeListener>>>) {
        { let java_arguments = (text.clone(), textChangeListener.clone(),); self.base.setTextWithOnMeasure(java_arguments.0, java_arguments.1) };
    }
    

    pub fn setText(&mut self, mut text: String) {
        if self.lastText != text {
            self.hasChanged = true;
            self.lastText = text.clone();
            { let java_arguments = (text.clone(),); self.base.setText(java_arguments.0) };
        }
    }
    

    pub fn getText(&mut self) -> String {
        return self.lastText.clone();
    }
    

    pub fn getWidth(&mut self) -> i32 {
        if self.hasChanged {
            self.textWidth = { let java_value = (jcall!(self.font).stringWidth(self.textArrayP[(0) as usize].clone())).clone(); java_value };
            self.hasChanged = false;
        }
        return self.textWidth.clone();
    }
    

    pub fn getFontHeight(&mut self) -> i32 {
        return jcall!(self.font).getHeight();
    }
}
impl std::ops::Deref for CustomTextAnimation {
    type Target = TextAnimation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for CustomTextAnimation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait CustomTextAnimationDyn: crate::org::allbinary::animation::text::TextAnimation::TextAnimationDyn + crate::org::allbinary::animation::text::TextInterface::TextInterface {
    fn as_CustomTextAnimation(&self) -> &CustomTextAnimation;
    fn as_CustomTextAnimation_mut(&mut self) -> &mut CustomTextAnimation;
}
impl CustomTextAnimationDyn for CustomTextAnimation {
    fn as_CustomTextAnimation(&self) -> &CustomTextAnimation { self }
    fn as_CustomTextAnimation_mut(&mut self) -> &mut CustomTextAnimation { self }
}

impl crate::org::allbinary::animation::text::TextAnimation::TextAnimationDyn for CustomTextAnimation {
    fn as_TextAnimation(&self) -> &crate::org::allbinary::animation::text::TextAnimation::TextAnimation { &self.base }
    fn as_TextAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::text::TextAnimation::TextAnimation { &mut self.base }
    fn setTextWithOnMeasure(&mut self, mut text: String, mut textChangeListener: Option<Rc<RefCell<TextChangeListener>>>) { Self::setTextWithOnMeasure(self, text, textChangeListener) }
    fn setText(&mut self, mut text: String) { Self::setText(self, text) }
    fn getFontHeight(&mut self) -> i32 { Self::getFontHeight(self) }
}
impl crate::org::allbinary::animation::Animation::AnimationDyn for CustomTextAnimation {
    fn as_Animation(&self) -> &crate::org::allbinary::animation::Animation::Animation { &self.base.base.base }
    fn as_Animation_mut(&mut self) -> &mut crate::org::allbinary::animation::Animation::Animation { &mut self.base.base.base }
    fn nextFrame(&mut self) { crate::org::allbinary::animation::text::TextAnimation::TextAnimation::nextFrame(crate::org::allbinary::animation::text::TextAnimation::TextAnimationDyn::as_TextAnimation_mut(self)) }
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
    fn setScale(&mut self, mut scaleX: f32, mut scaleY: f32) { Self::setScale(self, scaleX, scaleY) }
}
impl crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn for CustomTextAnimation {
    fn as_IndexedAnimation(&self) -> &crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &self.base.base }
    fn as_IndexedAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &mut self.base.base }
    fn getWidth(&mut self) -> i32 { Self::getWidth(self) }
}
impl crate::org::allbinary::graphics::font::UpdateMyFontInterface::UpdateMyFontInterface for CustomTextAnimation {
    fn updateMeasurement(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { crate::org::allbinary::animation::text::TextAnimation::TextAnimation::updateMeasurement(crate::org::allbinary::animation::text::TextAnimation::TextAnimationDyn::as_TextAnimation_mut(self), p1) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for CustomTextAnimation {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::animation::Animation::Animation::set(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self), p1) }
}


