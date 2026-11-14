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
// package org.allbinary.game.displayable.canvas
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::javax::microedition::lcdui::Font::Font;
use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::graphics::Anchor::Anchor;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::graphics::color::BasicColorSetUtil::BasicColorSetUtil;
use crate::org::allbinary::graphics::displayable::DisplayInfoSingleton::DisplayInfoSingleton;
use crate::org::allbinary::graphics::font::FontDebugFactory::FontDebugFactory;
use crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessor;
use crate::org::allbinary::graphics::font::UpdateMyFontInterface::UpdateMyFontInterface;
use crate::org::allbinary::graphics::font::UpdateMyFontProcessor::UpdateMyFontProcessor;
use crate::org::allbinary::graphics::paint::InitUpdatePaintable::InitUpdatePaintable;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use super::AllBinaryGameCanvas::AllBinaryGameCanvas;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct StartIntermissionPaintable {
    pub base: InitUpdatePaintable,
    pub fontDebugFactory: Option<Rc<RefCell<FontDebugFactory>>>,
    pub basicSetColorUtil: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::color::BasicColorSetUtil::BasicColorSetUtilDyn>>>,
    pub gameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>,
    pub stringArray: Vec<String>,
    pub fontSize: i32,
    pub font: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Font::FontDyn>>>,
    pub lastWidth: Vec<i32>,
    pub updateMyFontProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>>,
    pub myFontProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>>,
    pub basicColor: Option<Rc<RefCell<BasicColor>>>,
    pub color: i32,
    pub lineYOffsetArray: Vec<i32>,
    pub hasChanged: bool,
    pub anchor: i32,
    pub fontHeightP: i32,
    pub BEGIN_LEVEL: String,
    pub EMPTY_STRING: String,
}

impl UpdateMyFontInterface for StartIntermissionPaintable {
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::updateMeasurement(self, graphics) }
}
impl crate::org::allbinary::graphics::paint::InitUpdatePaintableInterface::InitUpdatePaintableInterface for StartIntermissionPaintable {
}
impl crate::org::allbinary::init::InitInterface::InitInterface for StartIntermissionPaintable {
    fn init(&mut self) { crate::org::allbinary::graphics::paint::InitUpdatePaintable::InitUpdatePaintable::init(crate::org::allbinary::graphics::paint::InitUpdatePaintable::InitUpdatePaintableDyn::as_InitUpdatePaintable_mut(self)) }
}
impl crate::org::allbinary::init::UpdateInterface::UpdateInterface for StartIntermissionPaintable {
    fn update(&mut self) { Self::update(self) }
}

impl StartIntermissionPaintable {
    

    pub fn new_struct(mut gameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>, mut stringArray: Vec<String>, mut basicColor: Option<Rc<RefCell<BasicColor>>>, mut font: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Font::FontDyn>>>) -> Self {
        let mut this = Self {
            base: InitUpdatePaintable::new_struct(),
            fontDebugFactory: None,
            basicSetColorUtil: None,
            gameCanvas: None,
            stringArray: Vec::new(),
            fontSize: 0,
            font: None,
            lastWidth: Vec::new(),
            updateMyFontProcessor: None,
            myFontProcessor: None,
            basicColor: None,
            color: 0,
            lineYOffsetArray: Vec::new(),
            hasChanged: false,
            anchor: 0,
            fontHeightP: 0,
            BEGIN_LEVEL: String::new(),
            EMPTY_STRING: String::new(),
        };
        this.fontDebugFactory = FontDebugFactory::getInstance();
        this.basicSetColorUtil = BasicColorSetUtil::getInstance();
        this.myFontProcessor = this.updateMyFontProcessor.clone();
        this.basicColor = { let java_field = jcall!(BasicColorFactory::getInstance()).BLACK.clone(); java_field }.clone();
        this.lineYOffsetArray = { let java_field = jcall!(NullUtil::getInstance()).NULL_INT_ARRAY.clone(); java_field }.clone();
        this.hasChanged = true;
        this.anchor = unsafe { crate::org::allbinary::graphics::Anchor::Anchor_TOP_LEFT.clone() }.clone();
        this.BEGIN_LEVEL = String::from("Begin Level ");
        this.EMPTY_STRING = { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone();
        this.gameCanvas = gameCanvas.clone();
        this.stringArray = stringArray.clone();
        this.lastWidth = { let java_value = (vec![0; ((this.stringArray.len() as i32)) as usize]).clone(); java_value };
        this.setBasicColorP(basicColor.clone());
        this.color = { let java_value = (jcall!(basicColor).intValue()).clone(); java_value };
        this.fontSize = { let java_value = (jcall!(font).getSize()).clone(); java_value };
        this.font = font.clone();
        this
    }

    pub fn new(mut gameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>, mut stringArray: Vec<String>, mut basicColor: Option<Rc<RefCell<BasicColor>>>, mut font: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Font::FontDyn>>>) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(gameCanvas.clone(), stringArray.clone(), basicColor.clone(), font.clone()))));
        jcall!(this).updateMyFontProcessor = UpdateMyFontProcessor::new(this.clone().map(|reference| reference as Rc<RefCell<dyn UpdateMyFontInterface>>)).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>);
        this
    }
    

    pub fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
        let mut font: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Font::FontDyn>>> = jcall!(graphics).getFont();
        self.fontHeightP = { let java_value = (jcall!(font).getHeight()).clone(); java_value };
        self.myFontProcessor = { let java_value = (MyFontProcessor::getInstance()).clone(); java_value };
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
        jcall!(self.myFontProcessor).process(graphics.clone());
        let mut existingFont: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Font::FontDyn>>> = jcall!(graphics).getFont();
        jcall!(self.fontDebugFactory).setFont(self.font.clone(), graphics.clone());
        let mut displayInfo: Option<Rc<RefCell<DisplayInfoSingleton>>> = DisplayInfoSingleton::getInstance();
        jcall!(self.basicSetColorUtil).setBasicColorP3(graphics.clone(), self.basicColor.clone(), self.color.clone());
        let mut beginWidth: i32 = 0;
        for index in (0..=(self.lineYOffsetArray.len() as i32) - 1).rev() {
            if self.hasChanged {
                { let java_index = (index) as usize; self.lastWidth[java_index] = (jcall!(jcall!(graphics).getFont()).stringWidth(self.stringArray[(index) as usize].clone()) >> 1); };
            }
            beginWidth = { let java_value = (self.lastWidth[(index) as usize]).clone(); java_value }.clone();
            jcall!(graphics).drawString(self.stringArray[(index) as usize].clone(), jcall!(displayInfo).getLastHalfWidth() - beginWidth, jcall!(displayInfo).getLastHalfHeight() - self.lineYOffsetArray[(index) as usize], self.anchor.clone());
        }
        self.hasChanged = false;
        jcall!(self.fontDebugFactory).setFont(existingFont.clone(), graphics.clone());
    }
    

    pub fn update(&mut self) {
        let mut level: i32 = jcall!(jcall!(jcall!(self.gameCanvas).getLayerManager()).getGameInfo()).getCurrentLevel();
        self.stringArray[(0) as usize] = jcall!(StringMaker::new()).append(self.BEGIN_LEVEL.clone()).appendint(level.clone()).toString();
        for index in (1..=(self.stringArray.len() as i32) - 1).rev() {
            { let java_index = (index) as usize; self.stringArray[java_index] = self.EMPTY_STRING.clone(); };
        }
    }
    

    pub fn setBasicColorP(&mut self, mut basicColor: Option<Rc<RefCell<BasicColor>>>) {
        self.basicColor = basicColor.clone();
    }
    

    pub fn getBasicColorP(&mut self) -> Option<Rc<RefCell<BasicColor>>> {
        return self.basicColor.clone();
    }
}
impl std::ops::Deref for StartIntermissionPaintable {
    type Target = InitUpdatePaintable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for StartIntermissionPaintable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::paint::InitUpdatePaintable::InitUpdatePaintableDyn for StartIntermissionPaintable {
    fn as_InitUpdatePaintable(&self) -> &crate::org::allbinary::graphics::paint::InitUpdatePaintable::InitUpdatePaintable { &self.base }
    fn as_InitUpdatePaintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::InitUpdatePaintable::InitUpdatePaintable { &mut self.base }
    fn update(&mut self) { Self::update(self) }
}
impl crate::org::allbinary::graphics::paint::Paintable::PaintableDyn for StartIntermissionPaintable {
    fn as_Paintable(&self) -> &crate::org::allbinary::graphics::paint::Paintable::Paintable { &self.base.base }
    fn as_Paintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::Paintable::Paintable { &mut self.base.base }
    fn setBasicColorP(&mut self, mut basicColor: Option<Rc<RefCell<BasicColor>>>) { Self::setBasicColorP(self, basicColor) }
}
impl crate::org::allbinary::graphics::paint::PaintableInterface::PaintableInterface for StartIntermissionPaintable {
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::paint(self, graphics) }
    fn paintThreed(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { crate::org::allbinary::graphics::paint::Paintable::Paintable::paintThreed(crate::org::allbinary::graphics::paint::Paintable::PaintableDyn::as_Paintable_mut(self), p1) }
}


