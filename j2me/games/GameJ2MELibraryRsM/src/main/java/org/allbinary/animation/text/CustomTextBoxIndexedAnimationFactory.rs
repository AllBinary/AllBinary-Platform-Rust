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
// use crate::javax::microedition::lcdui::TextField::TextField;
use crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBox;
use crate::org::allbinary::animation::Animation::Animation;
use crate::org::allbinary::animation::AnimationInterfaceFactoryInterface::AnimationInterfaceFactoryInterface;
use crate::org::allbinary::game::canvas::ABToGBUtil::ABToGBUtil;
use crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::media::ScaleProperties::ScaleProperties;
use super::CustomTextBox2::CustomTextBox2;
use super::CustomTextBoxIndexedAnimation::CustomTextBoxIndexedAnimation;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CustomTextBoxIndexedAnimationFactory {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub scaleProperties: Option<Rc<RefCell<ScaleProperties>>>,
    pub maxLength: i32,
}

impl AnimationInterfaceFactoryInterface for CustomTextBoxIndexedAnimationFactory {
    fn getInstance(&mut self, mut instanceId: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> { Self::getInstance(self, instanceId) }
    fn setInitialScale(&mut self, mut scaleProperties: Option<Rc<RefCell<ScaleProperties>>>) { Self::setInitialScale(self, scaleProperties) }
}
impl crate::org::allbinary::animation::BasicAnimationInterfaceFactoryInterface::BasicAnimationInterfaceFactoryInterface for CustomTextBoxIndexedAnimationFactory {
}

impl CustomTextBoxIndexedAnimationFactory {
    

    pub fn new_struct(mut fontSize: i32, mut maxLength: i32) -> Self {
        let mut this = Self {
            logUtil: None,
            scaleProperties: None,
            maxLength: 0,
        };
        this.logUtil = LogUtil::getInstance();
        this.scaleProperties = ScaleProperties::new();
        jcall!(this.scaleProperties).scaleHeight = fontSize.clone();
        this.maxLength = maxLength.clone();
        this
    }

    pub fn new(mut fontSize: i32, mut maxLength: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(fontSize, maxLength))))
    }
    

    // throws Exception
    pub fn getInstance(&mut self, mut instanceId: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> {
        let mut abToGBUtil: Option<Rc<RefCell<ABToGBUtil>>> = ABToGBUtil::getInstance();
        let mut abCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>> = (jcall!(abToGBUtil).abCanvas).clone() /* as AllBinaryGameCanvas */;
        let mut font: Option<Rc<RefCell<Font>>> = Font::getFont(unsafe { crate::javax::microedition::lcdui::Font::Font_FACE_SYSTEM.clone() }.clone(), unsafe { crate::javax::microedition::lcdui::Font::Font_STYLE_PLAIN.clone() }.clone(), jcall!(self.scaleProperties).scaleHeight.clone());
        let mut customTextBox: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBoxDyn>>> = CustomTextBox2::new(jcall!(abCanvas).getCustomCommandListener(), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), self.maxLength.clone(), unsafe { crate::javax::microedition::lcdui::TextField::TextField_ANY.clone() }.clone(), font.clone(), jcall!(jcall!(abCanvas).getLayerManager()).getBackgroundBasicColor(), jcall!(jcall!(abCanvas).getLayerManager()).getForegroundBasicColor()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBoxDyn>>);
        return CustomTextBoxIndexedAnimation::new(customTextBox.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>);
    }
    

    pub fn setInitialScale(&mut self, mut scaleProperties: Option<Rc<RefCell<ScaleProperties>>>) {
        let mut fontSize: i32 = jcall!(self.scaleProperties).scaleHeight.clone();
        self.scaleProperties = scaleProperties.clone();
        if jcall!(self.scaleProperties).scaleHeight < 6 {
            jcall!(self.scaleProperties).scaleHeight = fontSize.clone();
        }
    }
}

