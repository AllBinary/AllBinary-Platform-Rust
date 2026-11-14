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
use crate::org::allbinary::AndroidUtil::AndroidUtil;
use crate::org::allbinary::J2MEUtil::J2MEUtil;
use crate::org::allbinary::animation::Animation::Animation;
use crate::org::allbinary::animation::NullAnimationFactory::NullAnimationFactory;
use crate::org::allbinary::animation::vector::RectangleFilledAnimation::RectangleFilledAnimation;
use crate::org::allbinary::game::configuration::feature::Features::Features;
use crate::org::allbinary::game::layer::SWTUtil::SWTUtil;
use crate::org::allbinary::graphics::GPoint::GPoint;
use crate::org::allbinary::graphics::Rectangle::Rectangle;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::graphics::color::BasicColorSetUtil::BasicColorSetUtil;
use crate::org::allbinary::graphics::draw::DrawVerticalStringUtil::DrawVerticalStringUtil;
use crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessor;
use crate::org::allbinary::graphics::font::UpdateMyFontInterface::UpdateMyFontInterface;
use crate::org::allbinary::graphics::font::UpdateMyFontProcessor::UpdateMyFontProcessor;
use crate::org::allbinary::graphics::opengles::OpenGLFeatureFactory::OpenGLFeatureFactory;
use crate::org::allbinary::graphics::opengles::OpenGLFeatureUtil::OpenGLFeatureUtil;
use crate::org::allbinary::graphics::paint::Paintable::Paintable;
use crate::org::allbinary::graphics::threed::SWTJOGLProcessor::SWTJOGLProcessor;
use crate::java::lang::Exception::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicPopupMenuPaintable {
    pub base: Paintable,
    pub basicSetColorUtil: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::color::BasicColorSetUtil::BasicColorSetUtilDyn>>>,
    pub drawStringUtil: Option<Rc<RefCell<DrawVerticalStringUtil>>>,
    pub label: String,
    pub foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>,
    pub myFontProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>>,
    pub rectangle: Option<Rc<RefCell<Rectangle>>>,
    pub BORDER: i32,
    pub heightOffset: i32,
    pub offset: i32,
    pub animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>,
}

thread_local! { pub static BasicPopupMenuPaintable_NAME: String = String::from("MENU"); }
impl UpdateMyFontInterface for BasicPopupMenuPaintable {
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::updateMeasurement(self, graphics) }
}

impl BasicPopupMenuPaintable {
    

    pub fn new_struct(mut rectangle: Option<Rc<RefCell<Rectangle>>>, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Self {
        let mut this = Self {
            base: Paintable::new_struct(),
            basicSetColorUtil: None,
            drawStringUtil: None,
            label: String::new(),
            foregroundBasicColor: None,
            myFontProcessor: None,
            rectangle: None,
            BORDER: 0,
            heightOffset: 0,
            offset: 0,
            animationInterface: None,
        };
        this.basicSetColorUtil = BasicColorSetUtil::getInstance();
        this.drawStringUtil = DrawVerticalStringUtil::getInstance();
        this.foregroundBasicColor = foregroundBasicColor.clone();
        this.label = { let java_value = (BasicPopupMenuPaintable_NAME.with(Clone::clone)).clone(); java_value }.clone();
        this.rectangle = rectangle.clone();
        if J2MEUtil::isJ2ME() || J2MEUtil::isHTML() {
            this.animationInterface = { let java_value = (jcall!(NullAnimationFactory::getFactoryInstance()).getInstance(0)).clone(); java_value };
        } else {
            this.animationInterface = { let java_value = (RectangleFilledAnimation::new(jcall!(this.rectangle).getWidth(), jcall!(this.rectangle).getHeight(), { let java_field = jcall!(BasicColorFactory::getInstance()).TRANSPARENT_GREY.clone(); java_field }.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>)).clone(); java_value };
        }
        this.init(rectangle.clone());
        this
    }

    pub fn new(mut rectangle: Option<Rc<RefCell<Rectangle>>>, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(rectangle.clone(), backgroundBasicColor.clone(), foregroundBasicColor.clone()))));
        jcall!(this).myFontProcessor = UpdateMyFontProcessor::new(this.clone().map(|reference| reference as Rc<RefCell<dyn UpdateMyFontInterface>>)).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>);
        this
    }
    

    pub fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
        let mut font: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Font::FontDyn>>> = jcall!(graphics).getFont();
        let mut features: Option<Rc<RefCell<Features>>> = Features::getInstance();
        let mut isOpenGL: bool = jcall!(features).isDefault({ let java_field = jcall!(OpenGLFeatureFactory::getInstance()).OPENGL.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>));
        jcall!(self.drawStringUtil).updateMeasurement(graphics.clone(), self.label.clone());
        let mut BORDER: i32 = 0;
        if J2MEUtil::isHTML() || isOpenGL {
            BORDER = { let java_value = (MyFontProcessor::defaultCharWidth(font.clone()) / 2).clone(); java_value };
        } else if AndroidUtil::isAndroid() || J2MEUtil::isJ2SE() || unsafe { crate::org::allbinary::game::layer::SWTUtil::SWTUtil_isSWT.clone() } {
            BORDER = { let java_value = (MyFontProcessor::defaultCharWidth(font.clone())).clone(); java_value };
        } else {
            BORDER = { let java_value = (MyFontProcessor::defaultCharWidth(font.clone()) * 2).clone(); java_value };
        }
        self.BORDER = BORDER.clone();
        self.heightOffset = { let java_value = (jcall!(self.rectangle).getHeight() - (jcall!(font).getHeight() * (BasicPopupMenuPaintable_NAME.with(Clone::clone).chars().count() as i32))).clone(); java_value };
        if jcall!(OpenGLFeatureUtil::getInstance()).isAnyThreed() {
            self.heightOffset -= jcall!(font).getHeight() + 2;
            if AndroidUtil::isAndroid() {
                self.heightOffset = { let java_value = (jcall!(font).getHeight()).clone(); java_value };
            } else {
                self.heightOffset -= jcall!(font).getHeight() + 2;
            }
        }
        self.offset = { let java_value = ((self.heightOffset >> 1) + ((if jcall!(SWTJOGLProcessor::getInstance()).isJOGL() { 4 } else { 0 }))).clone(); java_value };
        self.myFontProcessor = { let java_value = (MyFontProcessor::getInstance()).clone(); java_value };
    }
    

    // throws Exception
    pub fn init(&mut self, mut rectangle: Option<Rc<RefCell<Rectangle>>>) {
        self.rectangle = rectangle.clone();
        let mut width: i32 = jcall!(self.rectangle).getWidth();
        let mut height: i32 = jcall!(self.rectangle).getHeight();
        if J2MEUtil::isJ2ME() || J2MEUtil::isHTML() {
        } else {
            let mut rectangleFilledAnimation: Option<Rc<RefCell<dyn crate::org::allbinary::animation::vector::RectangleFilledAnimation::RectangleFilledAnimationDyn>>> = (|| -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::vector::RectangleFilledAnimation::RectangleFilledAnimationDyn>>> { panic!("ClassCastException: a Rust dyn Animation cannot be cast to RectangleFilledAnimation") })();
            jcall!(rectangleFilledAnimation).setWidth(width.clone());
            jcall!(rectangleFilledAnimation).setHeight(height.clone());
        }
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
        jcall!(self.myFontProcessor).process(graphics.clone());
        let mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = jcall!(self.rectangle).getPoint();
        let mut x: i32 = jcall!(point).getX();
        let mut y: i32 = jcall!(point).getY();
        let mut width: i32 = jcall!(self.rectangle).getWidth();
        let mut height: i32 = jcall!(self.rectangle).getHeight();
        jcall!(self.animationInterface).paintXY(graphics.clone(), x.clone(), y.clone());
        jcall!(self.basicSetColorUtil).setBasicColorP(graphics.clone(), self.foregroundBasicColor.clone());
        jcall!(self.drawStringUtil).paintVerticle(graphics.clone(), self.label.clone(), x + self.BORDER, y + self.offset, 0);
        jcall!(graphics).drawRect(x.clone(), y.clone(), width.clone(), height.clone());
    }
}
impl std::ops::Deref for BasicPopupMenuPaintable {
    type Target = Paintable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for BasicPopupMenuPaintable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::paint::Paintable::PaintableDyn for BasicPopupMenuPaintable {
    fn as_Paintable(&self) -> &crate::org::allbinary::graphics::paint::Paintable::Paintable { &self.base }
    fn as_Paintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::Paintable::Paintable { &mut self.base }
}
impl crate::org::allbinary::graphics::paint::PaintableInterface::PaintableInterface for BasicPopupMenuPaintable {
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::paint(self, graphics) }
    fn paintThreed(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { crate::org::allbinary::graphics::paint::Paintable::Paintable::paintThreed(crate::org::allbinary::graphics::paint::Paintable::PaintableDyn::as_Paintable_mut(self), p1) }
}


