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
// use crate::javax::microedition::lcdui::Font::Font;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::game::configuration::feature::Features::Features;
use crate::org::allbinary::graphics::PointFactory::PointFactory;
use crate::org::allbinary::graphics::Rectangle::Rectangle;
use crate::org::allbinary::graphics::RectangleFactory::RectangleFactory;
use crate::org::allbinary::graphics::displayable::DisplayInfoSingleton::DisplayInfoSingleton;
use crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessor;
use crate::org::allbinary::graphics::opengles::OpenGLFeatureFactory::OpenGLFeatureFactory;
use crate::org::allbinary::graphics::threed::SWTJOGLProcessor::SWTJOGLProcessor;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MyFormUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub popupMenuRectangle: Option<Rc<RefCell<Rectangle>>>,
}

thread_local! { pub static MyFormUtil_instance: Option<Rc<RefCell<MyFormUtil>>> = MyFormUtil::new(); }

impl MyFormUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            popupMenuRectangle: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.popupMenuRectangle = crate::org::allbinary::graphics::RectangleFactory::RectangleFactory_SINGLETON.with(Clone::clone).clone();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<MyFormUtil>>> {
        return MyFormUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        let mut font: Option<Rc<RefCell<Font>>> = jcall!(graphics).getFont();
        let mut displayInfo: Option<Rc<RefCell<DisplayInfoSingleton>>> = DisplayInfoSingleton::getInstance();
        let mut features: Option<Rc<RefCell<Features>>> = Features::getInstance();
        let mut isOpenGL: bool = jcall!(features).isDefault(jcall!(OpenGLFeatureFactory::getInstance()).OPENGL.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>));
        let mut TOTAL_CHAR_WIDTH: i32 = (if jcall!(SWTJOGLProcessor::getInstance()).isJOGL() && isOpenGL { 2 } else { 3 });
        let mut width: i32 = MyFontProcessor::defaultStringWidth(font.clone(), TOTAL_CHAR_WIDTH.clone());
        let mut pointFactory: Option<Rc<RefCell<PointFactory>>> = PointFactory::getInstance();
        if jcall!(displayInfo).getLastHeight() < 320 {
            self.popupMenuRectangle = Rectangle::new(jcall!(pointFactory).createXY(0, 25), width.clone(), (jcall!(font).getHeight() * 4) + 2);
        } else {
            self.popupMenuRectangle = Rectangle::new(jcall!(pointFactory).createXY(0, jcall!(displayInfo).getLastHalfHeight() - 70), width.clone(), (jcall!(font).getHeight() * 5).clone());
        }
    }
    

    pub fn getPopupMenuRectangle(&mut self) -> Option<Rc<RefCell<Rectangle>>> {
        return self.popupMenuRectangle.clone();
    }
}

