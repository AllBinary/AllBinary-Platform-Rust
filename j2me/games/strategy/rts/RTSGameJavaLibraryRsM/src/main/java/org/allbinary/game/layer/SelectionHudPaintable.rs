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
// package org.allbinary.game.layer
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Font::Font;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::animation::Animation::Animation;
use crate::org::allbinary::animation::NullAnimationFactory::NullAnimationFactory;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::graphics::displayable::DisplayInfoSingleton::DisplayInfoSingleton;
use crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessor;
use crate::org::allbinary::graphics::font::UpdateMyFontInterface::UpdateMyFontInterface;
use crate::org::allbinary::graphics::font::UpdateMyFontProcessor::UpdateMyFontProcessor;
use crate::org::allbinary::graphics::paint::InitUpdatePaintable::InitUpdatePaintable;
use crate::org::allbinary::input::motion::button::CommonButtons::CommonButtons;
use crate::org::allbinary::input::motion::button::TouchButtonLocationHelper::TouchButtonLocationHelper;
use crate::org::allbinary::logic::math::PrimitiveLongUtil::PrimitiveLongUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SelectionHudPaintable {
    pub base: InitUpdatePaintable,
    pub y: i32,
    pub primitiveLongUtil: Option<Rc<RefCell<PrimitiveLongUtil>>>,
    pub updateMyFontProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>>,
    pub myFontProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>>,
    pub x: i32,
    pub textX: i32,
    pub imageX: i32,
    pub basicColor: Option<Rc<RefCell<BasicColor>>>,
    pub color: i32,
    pub width: i32,
    pub height: i32,
    pub name: String,
    pub animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>,
    pub backgroundColor: i32,
}

impl UpdateMyFontInterface for SelectionHudPaintable {
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::updateMeasurement(self, graphics) }
}
impl crate::org::allbinary::graphics::paint::InitUpdatePaintableInterface::InitUpdatePaintableInterface for SelectionHudPaintable {
}
impl crate::org::allbinary::init::InitInterface::InitInterface for SelectionHudPaintable {
    fn init(&mut self) { self.base.init() }
}
impl crate::org::allbinary::init::UpdateInterface::UpdateInterface for SelectionHudPaintable {
    fn update(&mut self) { Self::update(self) }
}

impl SelectionHudPaintable {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: InitUpdatePaintable::new_struct(),
            y: 0,
            primitiveLongUtil: None,
            updateMyFontProcessor: None,
            myFontProcessor: None,
            x: 0,
            textX: 0,
            imageX: 0,
            basicColor: None,
            color: 0,
            width: 0,
            height: 0,
            name: String::new(),
            animationInterface: None,
            backgroundColor: 0,
        };
        this.y = jcall!(CommonButtons::getInstance()).STANDARD_BUTTON_SIZE + 17;
        this.updateMyFontProcessor = UpdateMyFontProcessor::new(this).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>);
        this.myFontProcessor = this.updateMyFontProcessor.clone();
        this.basicColor = jcall!(BasicColorFactory::getInstance()).BLACK.clone();
        this.name = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        this.animationInterface = jcall!(NullAnimationFactory::getFactoryInstance()).getInstance(0);
        this.backgroundColor = jcall!(jcall!(BasicColorFactory::getInstance()).GREY).intValue();
        this.update();
        this.primitiveLongUtil = PrimitiveLongUtil::createPowerOfTen(10000);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        let mut commonButtons: Option<Rc<RefCell<CommonButtons>>> = CommonButtons::getInstance();
        let mut font: Option<Rc<RefCell<Font>>> = jcall!(graphics).getFont();
        { let java_arguments = (jcall!(commonButtons).STANDARD_BUTTON_SIZE + jcall!(font).getHeight(),); self.setHeight(java_arguments.0) };
        self.myFontProcessor = MyFontProcessor::getInstance();
    }
    

    pub fn update(&mut self) {
        let mut commonButtons: Option<Rc<RefCell<CommonButtons>>> = CommonButtons::getInstance();
        let mut touchButtonLocationHelper: Option<Rc<RefCell<TouchButtonLocationHelper>>> = TouchButtonLocationHelper::new();
        let mut displayInfoSingleton: Option<Rc<RefCell<DisplayInfoSingleton>>> = DisplayInfoSingleton::getInstance();
        self.x = jcall!(commonButtons).STANDARD_BUTTON_SIZE + jcall!(touchButtonLocationHelper).getColumnsRemainderHalf();
        self.textX = self.getX() + 4;
        self.width = jcall!(displayInfoSingleton).getLastWidth() - self.getX() * 2;
        self.imageX = self.getWidth() + jcall!(touchButtonLocationHelper).getColumnsRemainderHalf() - 10;
        self.myFontProcessor = self.updateMyFontProcessor.clone();
    }
    

    pub fn updateSelectionInfo(&mut self) {
    }
    

    pub fn updateInfo(&mut self) {
    }
    

    pub fn getPrimitiveLongUtil(&mut self) -> Option<Rc<RefCell<PrimitiveLongUtil>>> {
        return self.primitiveLongUtil.clone();
    }
    

    pub fn setAnimationInterface(&mut self, mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>) {
        self.animationInterface = animationInterface.clone();
    }
    

    pub fn getAnimationInterface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> {
        return self.animationInterface.clone();
    }
    

    pub fn setName(&mut self, mut name: String) {
        self.name = name.clone();
    }
    

    pub fn getName(&mut self) -> String {
        return self.name.clone();
    }
    

    pub fn setBasicColorP(&mut self, mut basicColor: Option<Rc<RefCell<BasicColor>>>) {
        self.basicColor = basicColor.clone();
        { let java_arguments = (jcall!(basicColor).intValue(),); self.setColor(java_arguments.0) };
    }
    

    pub fn getBasicColorP(&mut self) -> Option<Rc<RefCell<BasicColor>>> {
        return self.basicColor.clone();
    }
    

    pub fn setColor(&mut self, mut color: i32) {
        self.color = color.clone();
    }
    

    pub fn getColor(&mut self) -> i32 {
        return self.color.clone();
    }
    

    pub fn getWidth(&mut self) -> i32 {
        return self.width.clone();
    }
    

    pub fn setHeight(&mut self, mut height: i32) {
        self.height = height.clone();
    }
    

    pub fn getHeight(&mut self) -> i32 {
        return self.height.clone();
    }
    

    pub fn getX(&mut self) -> i32 {
        return self.x.clone();
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        jcall!(self.myFontProcessor).process(graphics.clone());
        jcall!(graphics).setColor(self.backgroundColor.clone());
        jcall!(graphics).drawRect(self.getX(), self.y.clone(), self.getWidth(), self.getHeight());
        jcall!(graphics).setColor(self.getColor());
        jcall!(graphics).drawString(self.getName(), self.textX.clone(), self.y.clone(), 0);
    }
}
impl std::ops::Deref for SelectionHudPaintable {
    type Target = InitUpdatePaintable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for SelectionHudPaintable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait SelectionHudPaintableDyn: crate::org::allbinary::graphics::paint::InitUpdatePaintable::InitUpdatePaintableDyn {
    fn as_SelectionHudPaintable(&self) -> &SelectionHudPaintable;
    fn as_SelectionHudPaintable_mut(&mut self) -> &mut SelectionHudPaintable;
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { SelectionHudPaintable::updateMeasurement(self.as_SelectionHudPaintable_mut(), graphics) }
    fn updateSelectionInfo(&mut self) { SelectionHudPaintable::updateSelectionInfo(self.as_SelectionHudPaintable_mut()) }
    fn updateInfo(&mut self) { SelectionHudPaintable::updateInfo(self.as_SelectionHudPaintable_mut()) }
    fn getPrimitiveLongUtil(&mut self) -> Option<Rc<RefCell<PrimitiveLongUtil>>> { SelectionHudPaintable::getPrimitiveLongUtil(self.as_SelectionHudPaintable_mut()) }
    fn setAnimationInterface(&mut self, mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>) { SelectionHudPaintable::setAnimationInterface(self.as_SelectionHudPaintable_mut(), animationInterface) }
    fn getAnimationInterface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> { SelectionHudPaintable::getAnimationInterface(self.as_SelectionHudPaintable_mut()) }
    fn setName(&mut self, mut name: String) { SelectionHudPaintable::setName(self.as_SelectionHudPaintable_mut(), name) }
    fn getName(&mut self) -> String { SelectionHudPaintable::getName(self.as_SelectionHudPaintable_mut()) }
    fn getBasicColorP(&mut self) -> Option<Rc<RefCell<BasicColor>>> { SelectionHudPaintable::getBasicColorP(self.as_SelectionHudPaintable_mut()) }
    fn setColor(&mut self, mut color: i32) { SelectionHudPaintable::setColor(self.as_SelectionHudPaintable_mut(), color) }
    fn getColor(&mut self) -> i32 { SelectionHudPaintable::getColor(self.as_SelectionHudPaintable_mut()) }
    fn getWidth(&mut self) -> i32 { SelectionHudPaintable::getWidth(self.as_SelectionHudPaintable_mut()) }
    fn setHeight(&mut self, mut height: i32) { SelectionHudPaintable::setHeight(self.as_SelectionHudPaintable_mut(), height) }
    fn getHeight(&mut self) -> i32 { SelectionHudPaintable::getHeight(self.as_SelectionHudPaintable_mut()) }
    fn getX(&mut self) -> i32 { SelectionHudPaintable::getX(self.as_SelectionHudPaintable_mut()) }
}
impl SelectionHudPaintableDyn for SelectionHudPaintable {
    fn as_SelectionHudPaintable(&self) -> &SelectionHudPaintable { self }
    fn as_SelectionHudPaintable_mut(&mut self) -> &mut SelectionHudPaintable { self }
}

impl crate::org::allbinary::graphics::paint::InitUpdatePaintable::InitUpdatePaintableDyn for SelectionHudPaintable {
    fn as_InitUpdatePaintable(&self) -> &crate::org::allbinary::graphics::paint::InitUpdatePaintable::InitUpdatePaintable { &self.base }
    fn as_InitUpdatePaintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::InitUpdatePaintable::InitUpdatePaintable { &mut self.base }
    fn update(&mut self) { Self::update(self) }
}
impl crate::org::allbinary::graphics::paint::Paintable::PaintableDyn for SelectionHudPaintable {
    fn as_Paintable(&self) -> &crate::org::allbinary::graphics::paint::Paintable::Paintable { &self.base.base }
    fn as_Paintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::Paintable::Paintable { &mut self.base.base }
    fn setBasicColorP(&mut self, mut basicColor: Option<Rc<RefCell<BasicColor>>>) { Self::setBasicColorP(self, basicColor) }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
}


