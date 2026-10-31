// 
// AllBinary Open License Version 1 
// Copyright (c) 2006 AllBinary 
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
// package org.allbinary.game.layer
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Font::Font;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
// use crate::javax::microedition::lcdui::Image::Image;
// use crate::javax::microedition::lcdui::NullImage::NullImage;
use crate::org::allbinary::graphics::form::item::ABCustomImageItem::ABCustomImageItem;
use crate::org::allbinary::logic::java::character::CharArrayFactory::CharArrayFactory;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessor;
use crate::org::allbinary::graphics::font::UpdateMyFontInterface::UpdateMyFontInterface;
use crate::org::allbinary::graphics::font::UpdateMyFontProcessor::UpdateMyFontProcessor;
use crate::org::allbinary::logic::math::PrimitiveLongUtil::PrimitiveLongUtil;
use super::RTSInterface::RTSInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TechnologyRTSInterfaceImageItem {
    pub base: ABCustomImageItem,
    pub rtsInterface: Option<Rc<RefCell<dyn RTSInterface>>>,
    pub primitiveLongUtil: Option<Rc<RefCell<PrimitiveLongUtil>>>,
    pub LEVEL: String,
    pub COST: String,
    pub DOLLAR: String,
    pub myFontProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>>,
    pub costString: Vec<char>,
    pub costLength: i32,
    pub levelString: Vec<char>,
    pub levelLength: i32,
    pub adjustedCostLabelY: i32,
    pub adjustedCostX: i32,
    pub adjustedCostY: i32,
    pub adjustedLevelX: i32,
    pub adjustedLevelY: i32,
}

impl UpdateMyFontInterface for TechnologyRTSInterfaceImageItem {
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::updateMeasurement(self, graphics) }
}
impl crate::org::allbinary::graphics::form::item::ABCustomItemInterface::ABCustomItemInterface for TechnologyRTSInterfaceImageItem {
}

impl TechnologyRTSInterfaceImageItem {
    

    pub fn new_struct(mut label: String, mut img: Option<Rc<RefCell<Image>>>, mut layout: i32, mut altText: String, mut basicColor: Option<Rc<RefCell<BasicColor>>>, mut rtsInterface: Option<Rc<RefCell<dyn RTSInterface>>>) -> Self {
        let mut this = Self {
            base: ABCustomImageItem::new_struct(label.clone(), img.clone(), layout.clone(), altText.clone(), basicColor.clone(), 0),
            rtsInterface: None,
            primitiveLongUtil: None,
            LEVEL: String::new(),
            COST: String::new(),
            DOLLAR: String::new(),
            myFontProcessor: None,
            costString: Vec::new(),
            costLength: 0,
            levelString: Vec::new(),
            levelLength: 0,
            adjustedCostLabelY: 0,
            adjustedCostX: 0,
            adjustedCostY: 0,
            adjustedLevelX: 0,
            adjustedLevelY: 0,
        };
        this.primitiveLongUtil = PrimitiveLongUtil::createPowerOfTen(10000);
        this.LEVEL = String::from("Level");
        this.COST = String::from("Cost");
        this.DOLLAR = String::from("$");
        this.myFontProcessor = UpdateMyFontProcessor::new(this).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>);
        this.costString = jcall!(CharArrayFactory::getInstance()).getZeroCharArray();
        this.levelString = jcall!(CharArrayFactory::getInstance()).getZeroCharArray();
        // super(...) is in the struct literal above
        this.rtsInterface = rtsInterface.clone();
        this.update();
        this
    }

    pub fn new(mut label: String, mut img: Option<Rc<RefCell<Image>>>, mut layout: i32, mut altText: String, mut basicColor: Option<Rc<RefCell<BasicColor>>>, mut rtsInterface: Option<Rc<RefCell<dyn RTSInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(label, img, layout, altText, basicColor, rtsInterface))))
    }
    

    pub fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        let mut font: Option<Rc<RefCell<Font>>> = jcall!(graphics).getFont();
        let mut fontHeight: i32 = jcall!(font).getHeight();
        let mut imageHeight: i32 = 0;
        let mut image: Option<Rc<RefCell<Image>>> = self.getImage();
        if !match (&image, &crate::javax::microedition::lcdui::NullImage::NullImage_NULL_IMAGE.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            imageHeight = jcall!(image).getHeight();
        }
        self.adjustedCostLabelY = -self.yOffset + imageHeight - (3 * fontHeight);
        self.adjustedCostY = -self.yOffset + imageHeight - (2 * fontHeight);
        self.adjustedCostX = 2 + ((self.DOLLAR.chars().count() as i32) * (fontHeight - 1));
        self.adjustedLevelY = -self.yOffset + imageHeight - fontHeight;
        self.adjustedLevelX = 2 + ((self.LEVEL.chars().count() as i32) * (fontHeight - 1));
        self.myFontProcessor = MyFontProcessor::getInstance();
    }
    

    pub fn getRtsInterface(&mut self) -> Option<Rc<RefCell<dyn RTSInterface>>> {
        return self.rtsInterface.clone();
    }
    

    pub fn update(&mut self) {
        self.costString = { let java_receiver = self.primitiveLongUtil.clone(); let java_arguments = (jcall!(self.getRtsInterface()).getUpgradeCost(),); let java_result = jcall!(java_receiver).getCharArray(java_arguments.0); java_result };
        self.levelString = { let java_receiver = self.primitiveLongUtil.clone(); let java_arguments = (jcall!(self.getRtsInterface()).getLevel(),); let java_result = jcall!(java_receiver).getCharArray(java_arguments.0); java_result };
    }
    

    pub fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) {
        jcall!(self.myFontProcessor).process(graphics.clone());
        self.base.paintXY(graphics.clone(), x.clone(), y.clone());
        let mut xa: i32 = x + 2;
        jcall!(graphics).drawString(self.COST.clone(), xa.clone(), y + self.adjustedCostLabelY, 0);
        jcall!(graphics).drawString(self.DOLLAR.clone(), xa.clone(), y + self.adjustedCostY, 0);
        jcall!(graphics).drawChars(self.costString.clone(), 0, self.costLength.clone(), x + self.adjustedCostX, y + self.adjustedCostY, 0);
        jcall!(graphics).drawString(self.LEVEL.clone(), xa.clone(), y + self.adjustedLevelY, 0);
        jcall!(graphics).drawChars(self.levelString.clone(), 0, self.levelLength.clone(), x + self.adjustedLevelX, y + self.adjustedLevelY, 0);
    }
}
impl std::ops::Deref for TechnologyRTSInterfaceImageItem {
    type Target = ABCustomImageItem;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for TechnologyRTSInterfaceImageItem {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::form::item::ABCustomImageItem::ABCustomImageItemDyn for TechnologyRTSInterfaceImageItem {
    fn as_ABCustomImageItem(&self) -> &crate::org::allbinary::graphics::form::item::ABCustomImageItem::ABCustomImageItem { &self.base }
    fn as_ABCustomImageItem_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::item::ABCustomImageItem::ABCustomImageItem { &mut self.base }
}
impl crate::org::allbinary::graphics::form::item::ImageItem::ImageItemDyn for TechnologyRTSInterfaceImageItem {
    fn as_ImageItem(&self) -> &crate::org::allbinary::graphics::form::item::ImageItem::ImageItem { &self.base.base.base }
    fn as_ImageItem_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::item::ImageItem::ImageItem { &mut self.base.base.base }
}
impl crate::org::microemu::graphics::form::item::CustomImageItem::CustomImageItemDyn for TechnologyRTSInterfaceImageItem {
    fn as_CustomImageItem(&self) -> &crate::org::microemu::graphics::form::item::CustomImageItem::CustomImageItem { &self.base.base }
    fn as_CustomImageItem_mut(&mut self) -> &mut crate::org::microemu::graphics::form::item::CustomImageItem::CustomImageItem { &mut self.base.base }
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::updateMeasurement(self, graphics) }
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}


