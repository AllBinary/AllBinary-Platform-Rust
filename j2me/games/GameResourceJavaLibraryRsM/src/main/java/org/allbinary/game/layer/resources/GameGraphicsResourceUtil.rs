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
// package org.allbinary.game.layer.resources
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::configuration::feature::Features::Features;
use crate::org::allbinary::game::configuration::feature::GraphicsFeature::GraphicsFeature;
use crate::org::allbinary::game::configuration::feature::GraphicsFeatureFactory::GraphicsFeatureFactory;
use crate::org::allbinary::graphics::displayable::CanvasStrings::CanvasStrings;
use crate::org::allbinary::graphics::opengles::OpenGLFeatureUtil::OpenGLFeatureUtil;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::java::lang::Exception::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GameGraphicsResourceUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub SPRITE: String,
    pub QUARTER: String,
    pub name: String,
}

pub static mut GameGraphicsResourceUtil_instance: std::cell::LazyCell<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = std::cell::LazyCell::new(|| { let java_field = jcall!(NullUtil::getInstance()).NULL_OBJECT.clone(); java_field }.clone());

impl GameGraphicsResourceUtil {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<GameGraphicsResourceUtil>>> {
        if match (&unsafe { GameGraphicsResourceUtil_instance.clone() }, &{ let java_field = jcall!(NullUtil::getInstance()).NULL_OBJECT.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            unsafe { *GameGraphicsResourceUtil_instance = crate::java_into_object(GameGraphicsResourceUtil::new()) };
        }
        return { let java_object = (unsafe { GameGraphicsResourceUtil_instance.clone() }).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<GameGraphicsResourceUtil>>>>().expect("ClassCastException").clone()) };
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            SPRITE: String::new(),
            QUARTER: String::new(),
            name: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.SPRITE = String::from("_sprite");
        this.QUARTER = String::from("_quarter");
        this.name = { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            this.name = this.getString();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(this.logUtil).put({ let java_field = jcall!(commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(commonStrings).CONSTRUCTOR.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
        }
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn getString(&mut self) -> String {
        let mut graphicsFeatureFactory: Option<Rc<RefCell<GraphicsFeatureFactory>>> = GraphicsFeatureFactory::getInstance();
        let mut graphicsFeature: Option<Rc<RefCell<GraphicsFeature>>> = { let java_field = jcall!(graphicsFeatureFactory).NONE.clone(); java_field }.clone();
        let mut features: Option<Rc<RefCell<Features>>> = Features::getInstance();
        if jcall!(features).isFeature({ let java_field = jcall!(graphicsFeatureFactory).IMAGE_TO_ARRAY_GRAPHICS.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
            graphicsFeature = { let java_value = ({ let java_field = jcall!(graphicsFeatureFactory).IMAGE_TO_ARRAY_GRAPHICS.clone(); java_field }).clone(); java_value }.clone();
        } else if jcall!(features).isFeature({ let java_field = jcall!(graphicsFeatureFactory).SPRITE_FULL_GRAPHICS.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
            graphicsFeature = { let java_value = ({ let java_field = jcall!(graphicsFeatureFactory).SPRITE_FULL_GRAPHICS.clone(); java_field }).clone(); java_value }.clone();
        } else if jcall!(features).isFeature({ let java_field = jcall!(graphicsFeatureFactory).SPRITE_QUARTER_ROTATION_GRAPHICS.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
            graphicsFeature = { let java_value = ({ let java_field = jcall!(graphicsFeatureFactory).SPRITE_QUARTER_ROTATION_GRAPHICS.clone(); java_field }).clone(); java_value }.clone();
        } else if jcall!(features).isFeature({ let java_field = jcall!(graphicsFeatureFactory).IMAGE_ROTATION_ON_THE_FLY.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
            graphicsFeature = { let java_value = ({ let java_field = jcall!(graphicsFeatureFactory).IMAGE_ROTATION_ON_THE_FLY.clone(); java_field }).clone(); java_value }.clone();
        }
        return self.getStringForFeature(graphicsFeature.clone());
    }
    

    // throws Exception
    pub fn getStringForFeature(&mut self, mut graphicsFeature: Option<Rc<RefCell<GraphicsFeature>>>) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut graphicsFeatureFactory: Option<Rc<RefCell<GraphicsFeatureFactory>>> = GraphicsFeatureFactory::getInstance();
        jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
        let mut features: Option<Rc<RefCell<Features>>> = Features::getInstance();
        if jcall!(features).isFeature({ let java_field = jcall!(graphicsFeatureFactory).VECTOR_GRAPHICS.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
        } else if jcall!(features).isFeature({ let java_field = jcall!(graphicsFeatureFactory).IMAGE_GRAPHICS.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
            if match (&graphicsFeature, &{ let java_field = jcall!(graphicsFeatureFactory).IMAGE_TO_ARRAY_GRAPHICS.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } || match (&graphicsFeature, &{ let java_field = jcall!(graphicsFeatureFactory).IMAGE_ROTATION_ON_THE_FLY.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            } else if match (&graphicsFeature, &{ let java_field = jcall!(graphicsFeatureFactory).SPRITE_FULL_GRAPHICS.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                jcall!(stringBuffer).append(self.SPRITE.clone());
            } else if match (&graphicsFeature, &{ let java_field = jcall!(graphicsFeatureFactory).SPRITE_QUARTER_ROTATION_GRAPHICS.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                jcall!(stringBuffer).append(self.QUARTER.clone());
                jcall!(stringBuffer).append(self.SPRITE.clone());
            } else {
                panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("None/Unknown Sub Image Resource Type Specified"))));
            }
        } else if jcall!(OpenGLFeatureUtil::getInstance()).isAnyThreed() {
            jcall!(stringBuffer).append({ let java_field = jcall!(CanvasStrings::getInstance()).OBJ_MODEL.clone(); java_field }.clone());
        } else {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("None/Unknown Main Image Resource Type Specified"))));
        }
        return jcall!(stringBuffer).toString();
    }
    

    pub fn getName(&mut self) -> String {
        return self.name.clone();
    }
}

