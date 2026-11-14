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
use crate::org::allbinary::game::layer::CostLayerInterfaceFactoryInterface::CostLayerInterfaceFactoryInterface;
use crate::org::allbinary::game::rts::technology::event::TechEventListenerInterface::TechEventListenerInterface;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::logic::communication::log::ForcedLogUtil::ForcedLogUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::animation::Animation::Animation;
use crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessor;
use crate::org::allbinary::graphics::font::UpdateMyFontInterface::UpdateMyFontInterface;
use crate::org::allbinary::graphics::font::UpdateMyFontProcessor::UpdateMyFontProcessor;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject;
use crate::org::allbinary::logic::math::PrimitiveLongUtil::PrimitiveLongUtil;
use crate::org::allbinary::logic::util::event::EventStrings::EventStrings;
use crate::java::lang::Exception::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RTSLayerCostAnimation {
    pub base: Animation,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub primitiveLongUtil: Option<Rc<RefCell<PrimitiveLongUtil>>>,
    pub DOLLAR: String,
    pub image: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>,
    pub layerInterfaceFactoryInterface: Option<Rc<RefCell<dyn CostLayerInterfaceFactoryInterface>>>,
    pub myFontProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>>,
    pub costString: Vec<char>,
    pub len: i32,
    pub adjustedCostX: i32,
    pub fontHeight: i32,
}

impl TechEventListenerInterface for RTSLayerCostAnimation {
    fn onTechEvent(&mut self, mut event: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { Self::onTechEvent(self, event) }
}
impl crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface for RTSLayerCostAnimation {
    fn onEvent(&mut self, mut event: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { Self::onEvent(self, event) }
}
impl UpdateMyFontInterface for RTSLayerCostAnimation {
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::updateMeasurement(self, graphics) }
}
impl crate::org::allbinary::animation::AnimationInterface::AnimationInterface for RTSLayerCostAnimation {
    fn nextFrame(&mut self) { crate::org::allbinary::animation::Animation::Animation::nextFrame(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self)) }
    fn paintThreedXYZ(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, p2: i32, p3: i32, p4: i32) { crate::org::allbinary::animation::Animation::Animation::paintThreedXYZ(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self), p1, p2, p3, p4) }
}
impl crate::org::allbinary::graphics::paint::LocationPaintableInterface::LocationPaintableInterface for RTSLayerCostAnimation {
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}

impl RTSLayerCostAnimation {
    

    pub fn new_struct(mut image: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>, mut layerInterfaceFactoryInterface: Option<Rc<RefCell<dyn CostLayerInterfaceFactoryInterface>>>) -> Self {
        let mut this = Self {
            base: Animation::new_struct(),
            logUtil: None,
            primitiveLongUtil: None,
            DOLLAR: String::new(),
            image: None,
            layerInterfaceFactoryInterface: None,
            myFontProcessor: None,
            costString: Vec::new(),
            len: 0,
            adjustedCostX: 0,
            fontHeight: 0,
        };
        this.logUtil = LogUtil::getInstance();
        this.primitiveLongUtil = PrimitiveLongUtil::createPowerOfTen(10000);
        this.DOLLAR = String::from("$");
        this.costString = { let java_field = jcall!(NullUtil::getInstance()).NULL_CHAR_ARRAY.clone(); java_field }.clone();
        this.fontHeight = 0;
        this.image = image.clone();
        this.layerInterfaceFactoryInterface = layerInterfaceFactoryInterface.clone();
        this.update();
        this
    }

    pub fn new(mut image: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>, mut layerInterfaceFactoryInterface: Option<Rc<RefCell<dyn CostLayerInterfaceFactoryInterface>>>) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(image.clone(), layerInterfaceFactoryInterface.clone()))));
        jcall!(this).myFontProcessor = UpdateMyFontProcessor::new(this.clone().map(|reference| reference as Rc<RefCell<dyn UpdateMyFontInterface>>)).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>);
        this
    }
    

    pub fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
        let mut font: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Font::FontDyn>>> = jcall!(graphics).getFont();
        self.fontHeight = { let java_value = (jcall!(font).getHeight()).clone(); java_value };
        self.adjustedCostX = { let java_value = (jcall!(font).stringWidth(self.DOLLAR.clone())).clone(); java_value };
        self.myFontProcessor = { let java_value = (MyFontProcessor::getInstance()).clone(); java_value };
    }
    

    pub fn onEvent(&mut self, mut event: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) {
        ForcedLogUtil::log({ let java_field = jcall!(EventStrings::getInstance()).PERFORMANCE_MESSAGE.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())));
    }
    

    pub fn onTechEvent(&mut self, mut event: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.update();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(self.logUtil).put({ let java_field = jcall!(commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("onTechEvent"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    // throws Exception
    pub fn update(&mut self) {
        self.len = 0;
        self.costString = { let java_value = ({ let java_receiver = self.primitiveLongUtil.clone(); let java_arguments = (jcall!(self.layerInterfaceFactoryInterface).getCost(),); let java_result = jcall!(java_receiver).getCharArray(java_arguments.0); java_result }).clone(); java_value };
        self.len = { let java_value = (jcall!(self.primitiveLongUtil).getCurrentTotalDigits()).clone(); java_value };
    }
    

    pub fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, mut x: i32, mut y: i32) {
        jcall!(self.myFontProcessor).process(graphics.clone());
        { let java_arguments = (graphics.clone(), x.clone(), y.clone(),); self.base.paintXY(java_arguments.0, java_arguments.1, java_arguments.2) };
        let mut adjustedCostY: i32 = jcall!(self.image).getHeight() - self.fontHeight;
        let mut xa: i32 = x + 2;
        jcall!(graphics).drawString(self.DOLLAR.clone(), xa.clone(), y + adjustedCostY, 0);
        jcall!(graphics).drawChars(self.costString.clone(), 0, self.len.clone(), x + self.adjustedCostX, y + adjustedCostY, 0);
    }
}
impl std::ops::Deref for RTSLayerCostAnimation {
    type Target = Animation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for RTSLayerCostAnimation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::animation::Animation::AnimationDyn for RTSLayerCostAnimation {
    fn as_Animation(&self) -> &crate::org::allbinary::animation::Animation::Animation { &self.base }
    fn as_Animation_mut(&mut self) -> &mut crate::org::allbinary::animation::Animation::Animation { &mut self.base }
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for RTSLayerCostAnimation {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::animation::Animation::Animation::set(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self), p1) }
}


