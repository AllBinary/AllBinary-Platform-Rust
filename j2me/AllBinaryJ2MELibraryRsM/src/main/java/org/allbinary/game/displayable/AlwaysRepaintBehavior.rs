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
use crate::javax::microedition::lcdui::Canvas::Canvas;
use crate::org::allbinary::game::configuration::feature::Features::Features;
use crate::org::allbinary::graphics::displayable::DisplayInfoSingleton::DisplayInfoSingleton;
use crate::org::allbinary::graphics::opengles::OpenGLFeatureFactory::OpenGLFeatureFactory;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::thread::NullRunnable::NullRunnable;
use crate::java::lang::Thread::*;
use super::RepaintBehavior::RepaintBehavior;
use super::RepaintRunnable::RepaintRunnable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AlwaysRepaintBehavior {
    pub base: RepaintBehavior,
    pub NAME: String,
}

pub static AlwaysRepaintBehavior_instanceC: crate::JavaStatic<Option<Rc<RefCell<AlwaysRepaintBehavior>>>> = crate::JavaStatic::new(|| AlwaysRepaintBehavior::new());

impl AlwaysRepaintBehavior {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: RepaintBehavior::new_struct(),
            NAME: String::new(),
        };
        this.NAME = String::from("AlwaysRepaintBehavior");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<AlwaysRepaintBehavior>>> {
        return AlwaysRepaintBehavior_instanceC.with(Clone::clone).clone();
    }
    

    pub fn repaint(&mut self, mut canvas: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Canvas::CanvasDyn>>>) {
        let mut features: Option<Rc<RefCell<Features>>> = Features::getInstance();
        let mut openGLFeatureFactory: Option<Rc<RefCell<OpenGLFeatureFactory>>> = OpenGLFeatureFactory::getInstance();
        if jcall!(features).isFeature({ let java_field = jcall!(openGLFeatureFactory).OPENGL.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
            jcall!(DisplayInfoSingleton::getInstance()).process();
        } else {
            let mut thread: Option<Rc<RefCell<Thread>>> = Thread::new2(RepaintRunnable::new(canvas.clone()), self.NAME.clone());
            jcall!(thread).start();
        }
    }
    

    pub fn onChangeRepaint(&mut self, mut canvas: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Canvas::CanvasDyn>>>) {
    }
}
impl std::ops::Deref for AlwaysRepaintBehavior {
    type Target = RepaintBehavior;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AlwaysRepaintBehavior {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::displayable::RepaintBehavior::RepaintBehaviorDyn for AlwaysRepaintBehavior {
    fn as_RepaintBehavior(&self) -> &crate::org::allbinary::game::displayable::RepaintBehavior::RepaintBehavior { &self.base }
    fn as_RepaintBehavior_mut(&mut self) -> &mut crate::org::allbinary::game::displayable::RepaintBehavior::RepaintBehavior { &mut self.base }
    fn repaint(&mut self, mut canvas: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Canvas::CanvasDyn>>>) { Self::repaint(self, canvas) }
    fn onChangeRepaint(&mut self, mut canvas: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Canvas::CanvasDyn>>>) { Self::onChangeRepaint(self, canvas) }
}


