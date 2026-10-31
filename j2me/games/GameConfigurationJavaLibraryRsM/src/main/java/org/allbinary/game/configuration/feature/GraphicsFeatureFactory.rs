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
// package org.allbinary.game.configuration.feature
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use super::GraphicsFeature::GraphicsFeature;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GraphicsFeatureFactory {
    pub NONE: Option<Rc<RefCell<GraphicsFeature>>>,
    pub TRANSPARENT_IMAGE_CREATION: Option<Rc<RefCell<GraphicsFeature>>>,
    pub IMAGE_GRAPHICS: Option<Rc<RefCell<GraphicsFeature>>>,
    pub IMAGE_ROTATION_ON_THE_FLY: Option<Rc<RefCell<GraphicsFeature>>>,
    pub IMAGE_TO_ARRAY_GRAPHICS: Option<Rc<RefCell<GraphicsFeature>>>,
    pub SPRITE_QUARTER_ROTATION_GRAPHICS: Option<Rc<RefCell<GraphicsFeature>>>,
    pub SPRITE_FULL_GRAPHICS: Option<Rc<RefCell<GraphicsFeature>>>,
    pub VECTOR_GRAPHICS: Option<Rc<RefCell<GraphicsFeature>>>,
    pub VECTOR_TO_IMAGE_GRAPHICS: Option<Rc<RefCell<GraphicsFeature>>>,
}

thread_local! { pub static GraphicsFeatureFactory_instance: Option<Rc<RefCell<GraphicsFeatureFactory>>> = GraphicsFeatureFactory::new(); }

impl GraphicsFeatureFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<GraphicsFeatureFactory>>> {
        return GraphicsFeatureFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            NONE: None,
            TRANSPARENT_IMAGE_CREATION: None,
            IMAGE_GRAPHICS: None,
            IMAGE_ROTATION_ON_THE_FLY: None,
            IMAGE_TO_ARRAY_GRAPHICS: None,
            SPRITE_QUARTER_ROTATION_GRAPHICS: None,
            SPRITE_FULL_GRAPHICS: None,
            VECTOR_GRAPHICS: None,
            VECTOR_TO_IMAGE_GRAPHICS: None,
        };
        this.NONE = GraphicsFeature::new(jcall!(StringUtil::getInstance()).NULL_STRING.clone());
        this.TRANSPARENT_IMAGE_CREATION = GraphicsFeature::new(String::from("Transparent Image Creation"));
        this.IMAGE_GRAPHICS = GraphicsFeature::new(String::from("Image Graphics"));
        this.IMAGE_ROTATION_ON_THE_FLY = GraphicsFeature::new(String::from("Image Rotated On The Fly"));
        this.IMAGE_TO_ARRAY_GRAPHICS = GraphicsFeature::new(String::from("Image To Array Graphics"));
        this.SPRITE_QUARTER_ROTATION_GRAPHICS = GraphicsFeature::new(String::from("Sprite Quarter Rotation Graphics"));
        this.SPRITE_FULL_GRAPHICS = GraphicsFeature::new(String::from("Sprite Full Rotation Graphics"));
        this.VECTOR_GRAPHICS = GraphicsFeature::new(String::from("BasicArrayList Graphics"));
        this.VECTOR_TO_IMAGE_GRAPHICS = GraphicsFeature::new(String::from("BasicArrayList To Image Graphics"));
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

