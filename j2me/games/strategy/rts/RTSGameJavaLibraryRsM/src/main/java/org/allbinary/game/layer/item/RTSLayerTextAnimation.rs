// 
// AllBinary Open License Version 1 
// Copyright (c) 2003 AllBinary 
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
// package org.allbinary.game.layer.item
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::javax::microedition::lcdui::Font::Font;
use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::javax::microedition::lcdui::Image::Image;
use crate::org::allbinary::animation::Animation::Animation;
use crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessor;
use crate::org::allbinary::graphics::font::UpdateMyFontInterface::UpdateMyFontInterface;
use crate::org::allbinary::graphics::font::UpdateMyFontProcessor::UpdateMyFontProcessor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RTSLayerTextAnimation {
    pub base: Animation,
    pub image: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>,
    pub text: String,
    pub myFontProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>>,
    pub fontHeight: i32,
}

impl UpdateMyFontInterface for RTSLayerTextAnimation {
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::updateMeasurement(self, graphics) }
}
impl crate::org::allbinary::animation::AnimationInterface::AnimationInterface for RTSLayerTextAnimation {
    fn nextFrame(&mut self) { crate::org::allbinary::animation::Animation::Animation::nextFrame(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self)) }
    fn paintThreedXYZ(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, p2: i32, p3: i32, p4: i32) { crate::org::allbinary::animation::Animation::Animation::paintThreedXYZ(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self), p1, p2, p3, p4) }
}
impl crate::org::allbinary::graphics::paint::LocationPaintableInterface::LocationPaintableInterface for RTSLayerTextAnimation {
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}

impl RTSLayerTextAnimation {
    

    pub fn new_struct(mut text: String, mut image: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>) -> Self {
        let mut this = Self {
            base: Animation::new_struct(),
            image: None,
            text: String::new(),
            myFontProcessor: None,
            fontHeight: 0,
        };
        this.fontHeight = 0;
        this.text = text.clone();
        this.image = image.clone();
        this
    }

    pub fn new(mut text: String, mut image: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(text.clone(), image.clone()))));
        jcall!(this).myFontProcessor = UpdateMyFontProcessor::new(this.clone().map(|reference| reference as Rc<RefCell<dyn UpdateMyFontInterface>>)).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>);
        this
    }
    

    pub fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
        let mut font: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Font::FontDyn>>> = jcall!(graphics).getFont();
        self.fontHeight = { let java_value = (jcall!(font).getHeight()).clone(); java_value };
        self.myFontProcessor = { let java_value = (MyFontProcessor::getInstance()).clone(); java_value };
    }
    

    pub fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, mut x: i32, mut y: i32) {
        jcall!(self.myFontProcessor).process(graphics.clone());
        { let java_arguments = (graphics.clone(), x.clone(), y.clone(),); self.base.paintXY(java_arguments.0, java_arguments.1, java_arguments.2) };
        let mut adjustedCostY: i32 = jcall!(self.image).getHeight() - self.fontHeight;
        jcall!(graphics).drawString(self.text.clone(), x.clone(), y + adjustedCostY, 0);
    }
}
impl std::ops::Deref for RTSLayerTextAnimation {
    type Target = Animation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for RTSLayerTextAnimation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::animation::Animation::AnimationDyn for RTSLayerTextAnimation {
    fn as_Animation(&self) -> &crate::org::allbinary::animation::Animation::Animation { &self.base }
    fn as_Animation_mut(&mut self) -> &mut crate::org::allbinary::animation::Animation::Animation { &mut self.base }
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for RTSLayerTextAnimation {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::animation::Animation::Animation::set(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self), p1) }
}


