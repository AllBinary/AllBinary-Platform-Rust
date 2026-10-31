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
// package org.allbinary.game.displayable.canvas
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::GameTypeFactory::GameTypeFactory;
use crate::org::allbinary::graphics::paint::NullPaintable::NullPaintable;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::AllBinaryGameCanvas::AllBinaryGameCanvas;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct DemoGameBehavior {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

thread_local! { pub static DemoGameBehavior_instance: Option<Rc<RefCell<dyn DemoGameBehaviorDyn>>> = DemoGameBehavior::new().map(|reference| reference as Rc<RefCell<dyn DemoGameBehaviorDyn>>); }

impl DemoGameBehavior {

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn DemoGameBehaviorDyn>>> {
        return DemoGameBehavior_instance.with(Clone::clone).clone();
    }
    

    // throws Exception
    pub fn init(&mut self) {
    }
    

    pub fn pause(&mut self, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) {
    }
    

    pub fn unPause(&mut self, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) {
    }
    

    // throws Exception
    pub fn updateTouch(&mut self, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) {
    }
    

    // throws Exception
    pub fn updateScreenButtonPaintable(&mut self, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) {
        jcall!(allBinaryGameCanvas).setTouchPaintableP(NullPaintable::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>));
    }
    

    // throws Exception
    pub fn setGameState(&mut self, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) {
    }
    

    pub fn removeAllGameKeyInputListeners(&mut self, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) {
    }
    

    // throws Exception
    pub fn updateEndGameProcessor(&mut self, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) {
    }
    

    // throws Exception
    pub fn buildGame(&mut self, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) {
    }
    

    // throws Exception
    pub fn run(&mut self, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) {
        let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(jcall!(GameTypeFactory::getInstance()).BOT).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).RUN.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    // throws Exception
    pub fn setHighScore(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>, mut name: String, mut score: i64, mut autoSubmit: bool, mut isLast: bool) {
    }
}
pub trait DemoGameBehaviorDyn {
    fn as_DemoGameBehavior(&self) -> &DemoGameBehavior;
    fn as_DemoGameBehavior_mut(&mut self) -> &mut DemoGameBehavior;
    fn init(&mut self) { DemoGameBehavior::init(self.as_DemoGameBehavior_mut()) }
    fn pause(&mut self, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) { DemoGameBehavior::pause(self.as_DemoGameBehavior_mut(), allBinaryGameCanvas) }
    fn unPause(&mut self, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) { DemoGameBehavior::unPause(self.as_DemoGameBehavior_mut(), allBinaryGameCanvas) }
    fn updateTouch(&mut self, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) { DemoGameBehavior::updateTouch(self.as_DemoGameBehavior_mut(), allBinaryGameCanvas) }
    fn updateScreenButtonPaintable(&mut self, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) { DemoGameBehavior::updateScreenButtonPaintable(self.as_DemoGameBehavior_mut(), allBinaryGameCanvas) }
    fn setGameState(&mut self, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) { DemoGameBehavior::setGameState(self.as_DemoGameBehavior_mut(), allBinaryGameCanvas) }
    fn removeAllGameKeyInputListeners(&mut self, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) { DemoGameBehavior::removeAllGameKeyInputListeners(self.as_DemoGameBehavior_mut(), allBinaryGameCanvas) }
    fn updateEndGameProcessor(&mut self, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) { DemoGameBehavior::updateEndGameProcessor(self.as_DemoGameBehavior_mut(), allBinaryGameCanvas) }
    fn buildGame(&mut self, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) { DemoGameBehavior::buildGame(self.as_DemoGameBehavior_mut(), allBinaryGameCanvas) }
    fn run(&mut self, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) { DemoGameBehavior::run(self.as_DemoGameBehavior_mut(), allBinaryGameCanvas) }
    fn setHighScore(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>, mut name: String, mut score: i64, mut autoSubmit: bool, mut isLast: bool) { DemoGameBehavior::setHighScore(self.as_DemoGameBehavior_mut(), abeClientInformation, allBinaryGameCanvas, name, score, autoSubmit, isLast) }
}
impl DemoGameBehaviorDyn for DemoGameBehavior {
    fn as_DemoGameBehavior(&self) -> &DemoGameBehavior { self }
    fn as_DemoGameBehavior_mut(&mut self) -> &mut DemoGameBehavior { self }
}


