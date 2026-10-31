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
// package org.allbinary.graphics.opengles
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Displayable::Displayable;
use crate::org::allbinary::J2MEUtil::J2MEUtil;
use crate::org::allbinary::game::configuration::feature::Features::Features;
use crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas;
use crate::org::allbinary::game::displayable::canvas::DemoCanvas::DemoCanvas;
use crate::org::allbinary::game::displayable::canvas::GameInputMappingCanvas::GameInputMappingCanvas;
use crate::org::allbinary::game::displayable::canvas::GameInputMappingInstructionsCanvas::GameInputMappingInstructionsCanvas;
use crate::org::allbinary::game::displayable::canvas::GameRunnable::GameRunnable;
use crate::org::allbinary::game::displayable::canvas::NullDisplayable::NullDisplayable;
use crate::org::allbinary::game::displayable::canvas::NullWaitGameRunnable::NullWaitGameRunnable;
use crate::org::allbinary::game::layer::SWTUtil::SWTUtil;
use crate::org::allbinary::game::score::displayable::HighScoresCanvas::HighScoresCanvas;
use crate::org::allbinary::graphics::GraphicsStrings::GraphicsStrings;
use crate::org::allbinary::graphics::displayable::CanvasStrings::CanvasStrings;
use crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::thread::NullRunnable::NullRunnable;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CurrentDisplayableFactory {
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub canvasStrings: Option<Rc<RefCell<CanvasStrings>>>,
    pub stringUtil: Option<Rc<RefCell<StringUtil>>>,
    pub SET_DISPLAYABLE: String,
    pub SET_RUNNABLE: String,
    pub CLEAR_RUNNABLE: String,
    pub SET_USED_RUNNABLE: String,
    pub RUNNABLE: String,
    pub RUNNABLE_WAS_ALREADY_IN_USE: String,
    pub DEFAULT_CHILD_NAME_LIST: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub displayable: Option<Rc<RefCell<Displayable>>>,
    pub openGlReadydisplayable: Option<Rc<RefCell<Displayable>>>,
    pub runnable: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::GameRunnable::GameRunnableDyn>>>,
    pub usedRunnable: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::GameRunnable::GameRunnableDyn>>>,
}

thread_local! { pub static CurrentDisplayableFactory_instance: Option<Rc<RefCell<CurrentDisplayableFactory>>> = CurrentDisplayableFactory::new(); }

impl CurrentDisplayableFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<CurrentDisplayableFactory>>> {
        return CurrentDisplayableFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            commonStrings: None,
            canvasStrings: None,
            stringUtil: None,
            SET_DISPLAYABLE: String::new(),
            SET_RUNNABLE: String::new(),
            CLEAR_RUNNABLE: String::new(),
            SET_USED_RUNNABLE: String::new(),
            RUNNABLE: String::new(),
            RUNNABLE_WAS_ALREADY_IN_USE: String::new(),
            DEFAULT_CHILD_NAME_LIST: None,
            displayable: None,
            openGlReadydisplayable: None,
            runnable: None,
            usedRunnable: None,
        };
        this.commonStrings = CommonStrings::getInstance();
        this.canvasStrings = CanvasStrings::getInstance();
        this.stringUtil = StringUtil::getInstance();
        this.SET_DISPLAYABLE = String::from("setDisplayable");
        this.SET_RUNNABLE = String::from("setRunnable");
        this.CLEAR_RUNNABLE = String::from("clearRunnable");
        this.SET_USED_RUNNABLE = String::from("setUsedRunnable");
        this.RUNNABLE = String::from("Runnable: ");
        this.RUNNABLE_WAS_ALREADY_IN_USE = String::from("Runnable was already in use: ");
        this.DEFAULT_CHILD_NAME_LIST = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.displayable = NullDisplayable::getInstance();
        this.openGlReadydisplayable = NullDisplayable::getInstance();
        this.runnable = NullWaitGameRunnable::getInstance();
        this.usedRunnable = NullWaitGameRunnable::getInstance();
        jcall!(this.DEFAULT_CHILD_NAME_LIST).add(crate::java_into_object(jcall!(this.canvasStrings).ABOUT.clone()));
        jcall!(this.DEFAULT_CHILD_NAME_LIST).add(crate::java_into_object(jcall!(this.canvasStrings).OPTIONS.clone()));
        jcall!(this.DEFAULT_CHILD_NAME_LIST).add(crate::java_into_object(crate::org::allbinary::game::displayable::canvas::GameInputMappingCanvas::GameInputMappingCanvas_NAME.with(Clone::clone).clone()));
        jcall!(this.DEFAULT_CHILD_NAME_LIST).add(crate::java_into_object(crate::org::allbinary::game::displayable::canvas::GameInputMappingInstructionsCanvas::GameInputMappingInstructionsCanvas_NAME.with(Clone::clone).clone()));
        jcall!(this.DEFAULT_CHILD_NAME_LIST).add(crate::java_into_object(crate::org::allbinary::game::score::displayable::HighScoresCanvas::HighScoresCanvas_NAME.with(Clone::clone).clone()));
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn setMyCanvas(&mut self, mut displayable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn>>>) {
        if (self.displayable).is_some() /* instanceof MyCanvas */ {
            if jcall!(displayable).hasChild((self.displayable).clone() /* as MyCanvas */) {
                PreLogUtil::put(jcall!(StringMaker::new()).append(String::from("Child Displayable is already set for: ")).append(jcall!(self.stringUtil).toString(crate::java_into_object(displayable.clone()))).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.SET_DISPLAYABLE.clone());
                return;
            }
        }
        self.setDisplayable(displayable.clone());
    }
    

    pub fn setDisplayable(&mut self, mut displayable: Option<Rc<RefCell<Displayable>>>) {
        self.displayable = displayable.clone();
        self.setOpenGlReadydisplayable(displayable.clone());
    }
    

    pub fn setOpenGlReadydisplayable(&mut self, mut openGlReadydisplayable: Option<Rc<RefCell<Displayable>>>) {
        self.openGlReadydisplayable = openGlReadydisplayable.clone();
        self.update();
    }
    

    pub fn getOpenGlReadydisplayable(&mut self) -> Option<Rc<RefCell<Displayable>>> {
        return self.openGlReadydisplayable.clone();
    }
    

    pub fn setRunnable(&mut self, mut runnable: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::GameRunnable::GameRunnableDyn>>>) {
        PreLogUtil::put(jcall!(StringMaker::new()).append(self.RUNNABLE.clone()).append(jcall!(self.stringUtil).toString(crate::java_into_object(runnable.clone()))).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.SET_RUNNABLE.clone());
        self.runnable = runnable.clone();
        if !match (&self.usedRunnable, &self.runnable) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            self.update();
        } else {
            PreLogUtil::put(jcall!(StringMaker::new()).append(self.RUNNABLE_WAS_ALREADY_IN_USE.clone()).append(jcall!(self.stringUtil).toString(crate::java_into_object(runnable.clone()))).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.SET_RUNNABLE.clone());
        }
    }
    

    pub fn clearRunnable(&mut self) {
        PreLogUtil::put(jcall!(StringMaker::new()).append(self.RUNNABLE.clone()).append({ let java_receiver = self.stringUtil.clone(); let java_arguments = (crate::java_into_object(NullRunnable::getInstance()),); let java_result = jcall!(java_receiver).toString(java_arguments.0); java_result }).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.CLEAR_RUNNABLE.clone());
        { let java_arguments = (NullWaitGameRunnable::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::GameRunnable::GameRunnableDyn>>),); self.setUsedRunnable(java_arguments.0) };
    }
    

    pub fn update(&mut self) {
        { // synchronized (self)
            let mut commonSeps: Option<Rc<RefCell<CommonSeps>>> = CommonSeps::getInstance();
            let mut features: Option<Rc<RefCell<Features>>> = Features::getInstance();
            let mut stringMaker: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringMaker).append(jcall!(self.stringUtil).toString(crate::java_into_object(self.openGlReadydisplayable.clone())));
            if unsafe { crate::org::allbinary::game::layer::SWTUtil::SWTUtil_isSWT.clone() } && !jcall!(features).isDefault(jcall!(OpenGLFeatureFactory::OpenGLFeatureFactory::getInstance()).OPENGL.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
                PreLogUtil::put(jcall!(stringMaker).append(jcall!(commonSeps).SPACE.clone()).append(unsafe { crate::org::allbinary::game::layer::SWTUtil::SWTUtil_SWT.clone() }.clone()).append(jcall!(commonSeps).SPACE.clone()).append(self.RUNNABLE.clone()).append({ let java_receiver = self.stringUtil.clone(); let java_arguments = (crate::java_into_object(NullRunnable::getInstance()),); let java_result = jcall!(java_receiver).toString(java_arguments.0); java_result }).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).UPDATE.clone());
                { let java_arguments = (NullWaitGameRunnable::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::GameRunnable::GameRunnableDyn>>),); self.setUsedRunnable(java_arguments.0) };
            } else if J2MEUtil::isHTML() {
                PreLogUtil::put(jcall!(stringMaker).append(jcall!(commonSeps).SPACE.clone()).append(jcall!(GraphicsStrings::getInstance()).HTML.clone()).append(jcall!(commonSeps).SPACE.clone()).append(self.RUNNABLE.clone()).append(jcall!(self.stringUtil).toString(crate::java_into_object(self.runnable.clone()))).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).UPDATE.clone());
                self.setUsedRunnable(self.runnable.clone());
            } else if (self.openGlReadydisplayable).is_some() /* instanceof DemoCanvas */ || (self.openGlReadydisplayable).is_some() /* instanceof AllBinaryGameCanvas */ || jcall!(features).isDefault(jcall!(OpenGLFeatureFactory::OpenGLFeatureFactory::getInstance()).OPENGL.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
                PreLogUtil::put(jcall!(stringMaker).append(jcall!(commonSeps).SPACE.clone()).append(jcall!(jcall!(OpenGLFeatureFactory::OpenGLFeatureFactory::getInstance()).OPENGL).toString()).append(jcall!(commonSeps).SPACE.clone()).append(self.RUNNABLE.clone()).append(jcall!(self.stringUtil).toString(crate::java_into_object(self.runnable.clone()))).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).UPDATE.clone());
                self.setUsedRunnable(self.runnable.clone());
            } else {
                PreLogUtil::put(jcall!(stringMaker).append(self.RUNNABLE.clone()).append({ let java_receiver = self.stringUtil.clone(); let java_arguments = (crate::java_into_object(NullRunnable::getInstance()),); let java_result = jcall!(java_receiver).toString(java_arguments.0); java_result }).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).UPDATE.clone());
                { let java_arguments = (NullWaitGameRunnable::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::GameRunnable::GameRunnableDyn>>),); self.setUsedRunnable(java_arguments.0) };
            }
        }
    }
    

    pub fn getDisplayable(&mut self) -> Option<Rc<RefCell<Displayable>>> {
        return self.displayable.clone();
    }
    

    pub fn getUsedRunnable(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::GameRunnable::GameRunnableDyn>>> {
        return self.usedRunnable.clone();
    }
    

    pub fn setUsedRunnable(&mut self, mut usedRunnable: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::GameRunnable::GameRunnableDyn>>>) {
        PreLogUtil::put(jcall!(StringMaker::new()).append(self.RUNNABLE.clone()).append(jcall!(self.stringUtil).toString(crate::java_into_object(self.runnable.clone()))).append(jcall!(CommonSeps::getInstance()).COLON_SEP.clone()).append(jcall!(self.stringUtil).toString(crate::java_into_object(usedRunnable.clone()))).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.SET_USED_RUNNABLE.clone());
        self.usedRunnable = usedRunnable.clone();
    }
}

