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
use crate::org::allbinary::animation::Animation::Animation;
use crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactory;
use crate::org::allbinary::animation::AnimationInterfaceFactoryInterface::AnimationInterfaceFactoryInterface;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::media::ScaleProperties::ScaleProperties;
use super::CustomTextAnimation::CustomTextAnimation;
use super::AdjustCustomTextAnimation::AdjustCustomTextAnimation;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CustomTextAnimationFactory {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub animationBehaviorFactory: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactoryDyn>>>,
    pub basicColor: Option<Rc<RefCell<BasicColor>>>,
    pub text: String,
    pub initScaleHeight: i32,
    pub dx: i32,
    pub dy: i32,
    pub font: Option<Rc<RefCell<Font>>>,
    pub scaleProperties: Option<Rc<RefCell<ScaleProperties>>>,
}

impl AnimationInterfaceFactoryInterface for CustomTextAnimationFactory {
    fn getInstance(&mut self, mut instanceId: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> { Self::getInstance(self, instanceId) }
    fn setInitialScale(&mut self, mut scaleProperties: Option<Rc<RefCell<ScaleProperties>>>) { Self::setInitialScale(self, scaleProperties) }
}
impl crate::org::allbinary::animation::BasicAnimationInterfaceFactoryInterface::BasicAnimationInterfaceFactoryInterface for CustomTextAnimationFactory {
}

impl CustomTextAnimationFactory {
    

    pub fn createDXY(mut text: String, mut fontSize: i32, mut dx: i32, mut dy: i32, mut animationBehaviorFactory: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactoryDyn>>>) -> Option<Rc<RefCell<CustomTextAnimationFactory>>> {
        let mut customTextAnimationFactory: Option<Rc<RefCell<CustomTextAnimationFactory>>> = CustomTextAnimationFactory::new(text.clone(), fontSize.clone(), animationBehaviorFactory.clone());
        jcall!(customTextAnimationFactory).dx = dx.clone();
        jcall!(customTextAnimationFactory).dy = dy.clone();
        return customTextAnimationFactory.clone();
    }
    

    pub fn new_struct(mut text: String, mut fontSize: i32, mut animationBehaviorFactory: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactoryDyn>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            animationBehaviorFactory: None,
            basicColor: None,
            text: String::new(),
            initScaleHeight: 0,
            dx: 0,
            dy: 0,
            font: None,
            scaleProperties: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.basicColor = jcall!(BasicColorFactory::getInstance()).BLACK.clone();
        this.dx = 0;
        this.dy = 0;
        this.scaleProperties = ScaleProperties::new();
        this.text = text.clone();
        jcall!(this.scaleProperties).scaleHeight = fontSize - (fontSize / 4);
        this.initScaleHeight = jcall!(this.scaleProperties).scaleHeight.clone();
        this.animationBehaviorFactory = animationBehaviorFactory.clone();
        this.font = Font::getFont(unsafe { crate::javax::microedition::lcdui::Font::Font_FACE_SYSTEM.clone() }.clone(), unsafe { crate::javax::microedition::lcdui::Font::Font_STYLE_PLAIN.clone() }.clone(), this.initScaleHeight.clone());
        this
    }

    pub fn new(mut text: String, mut fontSize: i32, mut animationBehaviorFactory: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactoryDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(text, fontSize, animationBehaviorFactory))))
    }
    

    // throws Exception
    pub fn getInstance(&mut self, mut instanceId: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> {
        let mut customTextAnimation: Option<Rc<RefCell<dyn crate::org::allbinary::animation::text::CustomTextAnimation::CustomTextAnimationDyn>>> = None;
        if self.dx != 0 || self.dy != 0 {
            customTextAnimation = AdjustCustomTextAnimation::new(self.text.clone(), jcall!(self.scaleProperties).scaleHeight.clone(), self.dx.clone(), self.dy.clone(), jcall!(self.animationBehaviorFactory).getOrCreateInstance()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::text::CustomTextAnimation::CustomTextAnimationDyn>>);
        } else {
            customTextAnimation = CustomTextAnimation::new(self.text.clone(), jcall!(self.scaleProperties).scaleHeight.clone(), jcall!(self.animationBehaviorFactory).getOrCreateInstance()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::text::CustomTextAnimation::CustomTextAnimationDyn>>);
        }
        jcall!(customTextAnimation).setBasicColorP(self.basicColor.clone());
        return customTextAnimation.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>);
    }
    

    pub fn setInitialScale(&mut self, mut scaleProperties: Option<Rc<RefCell<ScaleProperties>>>) {
    }
    

    pub fn getWidth(&mut self) -> i32 {
        return jcall!(self.font).stringWidth(self.text.clone());
    }
    

    pub fn getHeight(&mut self) -> i32 {
        return jcall!(self.font).getHeight();
    }
}

