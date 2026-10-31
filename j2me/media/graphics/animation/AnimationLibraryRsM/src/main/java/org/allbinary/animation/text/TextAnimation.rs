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
// use crate::javax::microedition::lcdui::Font::Font;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::animation::AnimationBehavior::AnimationBehavior;
use crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation;
use crate::org::allbinary::graphics::Anchor::Anchor;
use crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessor;
use crate::org::allbinary::graphics::font::UpdateMyFontInterface::UpdateMyFontInterface;
use crate::org::allbinary::graphics::font::UpdateMyFontProcessor::UpdateMyFontProcessor;
use crate::org::allbinary::graphics::threed::SWTJOGLProcessor::SWTJOGLProcessor;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use super::TextChangeListener::TextChangeListener;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TextAnimation {
    pub base: IndexedAnimation,
    pub updateMyFontProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>>,
    pub myFontProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>>,
    pub textArrayP: Vec<String>,
    pub anchor: i32,
    pub fontHeight: i32,
    pub offsetY: i32,
    pub textChangeListener: Option<Rc<RefCell<TextChangeListener>>>,
}

impl UpdateMyFontInterface for TextAnimation {
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::updateMeasurement(self, graphics) }
}
impl crate::org::allbinary::animation::SequentialAnimationInterface::SequentialAnimationInterface for TextAnimation {
    fn getSize(&mut self) -> i32 { self.base.getSize() }
    fn previousFrame(&mut self) { self.base.previousFrame() }
}
impl crate::org::allbinary::graphics::paint::LocationPaintableInterface::LocationPaintableInterface for TextAnimation {
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}

impl TextAnimation {
    

    pub fn new_struct(mut text: String, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Self {
        let mut this = Self {
            base: IndexedAnimation::new_struct(animationBehavior.clone()),
            updateMyFontProcessor: None,
            myFontProcessor: None,
            textArrayP: Vec::new(),
            anchor: 0,
            fontHeight: 0,
            offsetY: 0,
            textChangeListener: None,
        };
        this.updateMyFontProcessor = UpdateMyFontProcessor::new(this).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>);
        this.myFontProcessor = this.updateMyFontProcessor.clone();
        this.textArrayP = jcall!(StringUtil::getInstance()).ONE_EMPTY_STRING_ARRAY.clone();
        this.anchor = unsafe { crate::org::allbinary::graphics::Anchor::Anchor_TOP_LEFT.clone() }.clone();
        this.fontHeight = 0;
        this.offsetY = 0;
        this.textChangeListener = TextChangeListener::getInstance();
        // super(...) is in the struct literal above
        this.setText(text.clone());
        this
    }

    pub fn new(mut text: String, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(text, animationBehavior))))
    }
    

    pub fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        let mut font: Option<Rc<RefCell<Font>>> = jcall!(graphics).getFont();
        self.fontHeight = jcall!(font).getHeight();
        let mut adjustedFontHeight: i32 = ((-(((self.fontHeight) as f64) * 2.00)) as i32);
        self.offsetY = (if jcall!(SWTJOGLProcessor::getInstance()).isJOGL() { adjustedFontHeight } else { 0 });
        jcall!(self.textChangeListener).onMeasure();
        self.textChangeListener = TextChangeListener::getInstance();
        self.myFontProcessor = MyFontProcessor::getInstance();
    }
    

    // throws Exception
    pub fn nextFrame(&mut self) {
    }
    

    pub fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) {
        jcall!(self.myFontProcessor).process(graphics.clone());
        self.paintXYNoUpdate(graphics.clone(), x.clone(), y.clone());
    }
    

    pub fn paintXYNoUpdate(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) {
        { let java_receiver = self.basicSetColorUtil.clone(); let java_arguments = (graphics.clone(), self.getBasicColorP(), self.getColor(),); jcall!(java_receiver).setBasicColorP3(java_arguments.0, java_arguments.1, java_arguments.2); };
        let mut size: i32 = (self.textArrayP.len() as i32).clone();
        for index in (0..size) {
            jcall!(graphics).drawString(self.textArrayP[(index) as usize].clone(), x.clone(), y + self.offsetY + (index * self.fontHeight), self.anchor.clone());
        }
    }
    

    pub fn setTextWithOnMeasure(&mut self, mut text: String, mut textChangeListener: Option<Rc<RefCell<TextChangeListener>>>) {
        self.setText(text.clone());
        self.textChangeListener = textChangeListener.clone();
    }
    

    pub fn setText(&mut self, mut text: String) {
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        if true && (text.chars().count() as i32) > 0 {
            let mut index: i32 = 0;
            let mut startIndex: i32 = 0;
            let mut endIndex: i32 = 0;
            while index >= 0 {
                startIndex = index.clone();
                index = jcall!(text).indexOf('\n', startIndex.clone());
                endIndex = index.clone();
                if index < 0 {
                    endIndex = (text.chars().count() as i32);
                }
                jcall!(list).add(crate::java_into_object(text[(startIndex) as usize..(endIndex) as usize].to_string()));
                if index < 0 {
                    break;
                }
                index += 1;
            }
        }
        if jcall!(list).size() > 0 {
            let mut textArray: Vec<String> = vec![String::new(); (jcall!(list).size()) as usize];
            let mut size: i32 = jcall!(list).size();
            for index in (0..size) {
                textArray[(index) as usize] = (jcall!(list).get(index.clone())).clone() /* as String */;
            }
            self.textArrayP = textArray.clone();
        } else {
            self.textArrayP = jcall!(StringUtil::getInstance()).ONE_EMPTY_STRING_ARRAY.clone();
        }
        self.myFontProcessor = self.updateMyFontProcessor.clone();
    }
    

    pub fn getTextArray(&mut self) -> Vec<String> {
        return self.textArrayP.clone();
    }
    

    pub fn getFontHeight(&mut self) -> i32 {
        return self.fontHeight.clone();
    }
}
impl std::ops::Deref for TextAnimation {
    type Target = IndexedAnimation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for TextAnimation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait TextAnimationDyn: crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn {
    fn as_TextAnimation(&self) -> &TextAnimation;
    fn as_TextAnimation_mut(&mut self) -> &mut TextAnimation;
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { TextAnimation::updateMeasurement(self.as_TextAnimation_mut(), graphics) }
    fn paintXYNoUpdate(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { TextAnimation::paintXYNoUpdate(self.as_TextAnimation_mut(), graphics, x, y) }
    fn setTextWithOnMeasure(&mut self, mut text: String, mut textChangeListener: Option<Rc<RefCell<TextChangeListener>>>) { TextAnimation::setTextWithOnMeasure(self.as_TextAnimation_mut(), text, textChangeListener) }
    fn setText(&mut self, mut text: String) { TextAnimation::setText(self.as_TextAnimation_mut(), text) }
    fn getTextArray(&mut self) -> Vec<String> { TextAnimation::getTextArray(self.as_TextAnimation_mut()) }
    fn getFontHeight(&mut self) -> i32 { TextAnimation::getFontHeight(self.as_TextAnimation_mut()) }
}
impl TextAnimationDyn for TextAnimation {
    fn as_TextAnimation(&self) -> &TextAnimation { self }
    fn as_TextAnimation_mut(&mut self) -> &mut TextAnimation { self }
}

impl crate::org::allbinary::animation::Animation::AnimationDyn for TextAnimation {
    fn as_Animation(&self) -> &crate::org::allbinary::animation::Animation::Animation { &self.base.base }
    fn as_Animation_mut(&mut self) -> &mut crate::org::allbinary::animation::Animation::Animation { &mut self.base.base }
    fn nextFrame(&mut self) { Self::nextFrame(self) }
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}
impl crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn for TextAnimation {
    fn as_IndexedAnimation(&self) -> &crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &self.base }
    fn as_IndexedAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &mut self.base }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for TextAnimation {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::animation::Animation::Animation::set(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self), p1) }
}


