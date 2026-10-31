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
// package org.allbinary.image
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::khronos::opengles::GL::GL;
// use crate::javax::microedition::lcdui::ChoiceGroupImageFactory::ChoiceGroupImageFactory;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
// use crate::javax::microedition::lcdui::Image::Image;
use crate::org::allbinary::graphics::opengles::OpenGLFeatureFactory::OpenGLFeatureFactory;
use crate::org::allbinary::game::configuration::feature::Features::Features;
use super::PreResourceImageUtil::PreResourceImageUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ChoiceGroupImageUtil {
}

thread_local! { pub static ChoiceGroupImageUtil_instance: Option<Rc<RefCell<ChoiceGroupImageUtil>>> = ChoiceGroupImageUtil::new(); }

impl ChoiceGroupImageUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<ChoiceGroupImageUtil>>> {
        return ChoiceGroupImageUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn init(&mut self) {
        let mut features: Option<Rc<RefCell<Features>>> = Features::getInstance();
        let mut choiceGroupFactory: Option<Rc<RefCell<ChoiceGroupImageFactory>>> = ChoiceGroupImageFactory::getInstance();
        let mut imageArray: Vec<Option<Rc<RefCell<Image>>>> = jcall!(choiceGroupFactory).getImageArray();
        if jcall!(features).isDefault(jcall!(OpenGLFeatureFactory::getInstance()).OPENGL.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
            let mut preResourceImageUtil: Option<Rc<RefCell<PreResourceImageUtil>>> = PreResourceImageUtil::getInstance();
            let mut size: i32 = (imageArray.len() as i32).clone();
            for index in (0..size) {
                imageArray[(index) as usize] = jcall!(preResourceImageUtil).encapsulate(imageArray[(index) as usize].clone());
            }
            jcall!(choiceGroupFactory).init(imageArray.clone());
        }
    }
    

    pub fn update(&mut self, mut gl: Option<Rc<RefCell<dyn GL>>>) {
        let mut features: Option<Rc<RefCell<Features>>> = Features::getInstance();
        let mut choiceGroupFactory: Option<Rc<RefCell<ChoiceGroupImageFactory>>> = ChoiceGroupImageFactory::getInstance();
        let mut imageArray: Vec<Option<Rc<RefCell<Image>>>> = jcall!(choiceGroupFactory).getImageArray();
        if jcall!(features).isDefault(jcall!(OpenGLFeatureFactory::getInstance()).OPENGL.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
            let mut preResourceImageUtil: Option<Rc<RefCell<PreResourceImageUtil>>> = PreResourceImageUtil::getInstance();
            let mut size: i32 = (imageArray.len() as i32).clone();
            for index in (0..size) {
                jcall!(preResourceImageUtil).updateGL(gl.clone(), imageArray[(index) as usize].clone());
            }
        }
        jcall!(choiceGroupFactory).init(imageArray.clone());
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn update(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        let mut features: Option<Rc<RefCell<Features>>> = Features::getInstance();
        let mut choiceGroupFactory: Option<Rc<RefCell<ChoiceGroupImageFactory>>> = ChoiceGroupImageFactory::getInstance();
        let mut imageArray: Vec<Option<Rc<RefCell<Image>>>> = jcall!(choiceGroupFactory).getImageArray();
        if jcall!(features).isDefault(jcall!(OpenGLFeatureFactory::getInstance()).OPENGL.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
            let mut preResourceImageUtil: Option<Rc<RefCell<PreResourceImageUtil>>> = PreResourceImageUtil::getInstance();
            let mut size: i32 = (imageArray.len() as i32).clone();
            for index in (0..size) {
                jcall!(preResourceImageUtil).update(graphics.clone(), imageArray[(index) as usize].clone());
            }
        }
        jcall!(choiceGroupFactory).init(imageArray.clone());
    }
}

