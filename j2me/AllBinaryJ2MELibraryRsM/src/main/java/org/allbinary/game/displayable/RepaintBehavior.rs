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
// package org.allbinary.game.displayable
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::thread::ARunnable::ARunnable;
// use crate::javax::microedition::lcdui::Canvas::Canvas;
use crate::org::allbinary::game::configuration::feature::Features::Features;
use crate::org::allbinary::graphics::displayable::DisplayInfoSingleton::DisplayInfoSingleton;
use crate::org::allbinary::graphics::opengles::OpenGLFeatureFactory::OpenGLFeatureFactory;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::thread::NullRunnable::NullRunnable;
use crate::java::lang::Thread::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RepaintBehavior {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub NAME: String,
}

thread_local! { pub static RepaintBehavior_instance: Option<Rc<RefCell<dyn RepaintBehaviorDyn>>> = RepaintBehavior::new().map(|reference| reference as Rc<RefCell<dyn RepaintBehaviorDyn>>); }

impl RepaintBehavior {

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            NAME: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.NAME = String::from("RepaintBehavior");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn RepaintBehaviorDyn>>> {
        return RepaintBehavior_instance.with(Clone::clone).clone();
    }
    

    pub fn repaint(&mut self, mut canvas: Option<Rc<RefCell<Canvas>>>) {
    }
    

    pub fn onChangeRepaint(&mut self, mut canvas: Option<Rc<RefCell<Canvas>>>) {
        let mut features: Option<Rc<RefCell<Features>>> = Features::getInstance();
        let mut openGLFeatureFactory: Option<Rc<RefCell<OpenGLFeatureFactory>>> = OpenGLFeatureFactory::getInstance();
        if jcall!(features).isFeature(jcall!(openGLFeatureFactory).OPENGL.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
            jcall!(DisplayInfoSingleton::getInstance()).process();
        } else {
            let mut thread: Option<Rc<RefCell<Thread>>> = Thread::new2(ARunnable::new() /* Java anonymous class body: implement it as a named type
                

                pub fn run(&mut self) {
                    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        jcall!(canvas).repaint();
                        jcall!(DisplayInfoSingleton::getInstance()).process();
                    })) {
                        Ok(_) => {}
                        Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

                        let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
                        let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
                        jcall!(logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).RUN.clone(), crate::java_into_object(e.clone()));
                        }
                    }
                }
*/, self.NAME.clone());
            jcall!(thread).start();
        }
    }
}
pub trait RepaintBehaviorDyn {
    fn as_RepaintBehavior(&self) -> &RepaintBehavior;
    fn as_RepaintBehavior_mut(&mut self) -> &mut RepaintBehavior;
    fn repaint(&mut self, mut canvas: Option<Rc<RefCell<Canvas>>>) { RepaintBehavior::repaint(self.as_RepaintBehavior_mut(), canvas) }
    fn onChangeRepaint(&mut self, mut canvas: Option<Rc<RefCell<Canvas>>>) { RepaintBehavior::onChangeRepaint(self.as_RepaintBehavior_mut(), canvas) }
}
impl RepaintBehaviorDyn for RepaintBehavior {
    fn as_RepaintBehavior(&self) -> &RepaintBehavior { self }
    fn as_RepaintBehavior_mut(&mut self) -> &mut RepaintBehavior { self }
}


