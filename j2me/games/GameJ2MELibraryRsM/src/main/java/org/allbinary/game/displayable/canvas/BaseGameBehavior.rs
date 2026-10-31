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
use crate::org::allbinary::business::advertisement::GameAdStateFactory::GameAdStateFactory;
use crate::org::allbinary::game::GameAdState::GameAdState;
use crate::org::allbinary::game::configuration::feature::Features::Features;
use crate::org::allbinary::graphics::opengles::CurrentDisplayableFactory::CurrentDisplayableFactory;
use crate::org::allbinary::graphics::opengles::OpenGLFeatureFactory::OpenGLFeatureFactory;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use super::DemoGameBehavior::DemoGameBehavior;
use super::AllBinaryGameCanvas::AllBinaryGameCanvas;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BaseGameBehavior {
    pub base: DemoGameBehavior,
}

thread_local! { pub static BaseGameBehavior_instanceC: Option<Rc<RefCell<BaseGameBehavior>>> = BaseGameBehavior::new(); }

impl BaseGameBehavior {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: DemoGameBehavior::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::DemoGameBehavior::DemoGameBehaviorDyn>>> {
        return BaseGameBehavior_instanceC.with(Clone::clone).clone();
    }
    

    // throws Exception
    pub fn init(&mut self) {
        let mut gameAdState: Option<Rc<RefCell<GameAdState>>> = jcall!(GameAdStateFactory::getInstance()).getCurrentInstance();
        jcall!(gameAdState).playingAdState();
    }
    

    pub fn pause(&mut self, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) {
        let mut features: Option<Rc<RefCell<Features>>> = Features::getInstance();
        if jcall!(features).isDefault(jcall!(OpenGLFeatureFactory::getInstance()).OPENGL_AS_GAME_THREAD.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
            let mut currentDisplayableFactory: Option<Rc<RefCell<CurrentDisplayableFactory>>> = CurrentDisplayableFactory::getInstance();
            jcall!(currentDisplayableFactory).setRunnable(jcall!(allBinaryGameCanvas).as_AllBinaryGameCanvas_mut().gamePauseRunnable.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::GameRunnable::GameRunnableDyn>>));
        }
    }
    

    pub fn unPause(&mut self, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) {
        if jcall!(Features::getInstance()).isDefault(jcall!(OpenGLFeatureFactory::getInstance()).OPENGL_AS_GAME_THREAD.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
            let mut currentDisplayableFactory: Option<Rc<RefCell<CurrentDisplayableFactory>>> = CurrentDisplayableFactory::getInstance();
            jcall!(currentDisplayableFactory).setRunnable(jcall!(allBinaryGameCanvas).as_AllBinaryGameCanvas_mut().gameRunnable.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::GameRunnable::GameRunnableDyn>>));
            jcall!(currentDisplayableFactory).setDisplayable(allBinaryGameCanvas.clone());
        }
    }
    

    // throws Exception
    pub fn updateTouch(&mut self, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) {
        jcall!(allBinaryGameCanvas).updateTouch2();
    }
    

    // throws Exception
    pub fn updateScreenButtonPaintable(&mut self, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) {
        jcall!(allBinaryGameCanvas).updateScreenButtonPaintable2();
    }
    

    // throws Exception
    pub fn setGameState(&mut self, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) {
        jcall!(allBinaryGameCanvas).updateGameState();
    }
    

    pub fn removeAllGameKeyInputListeners(&mut self, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) {
        jcall!(allBinaryGameCanvas).removeAllGameKeyInputListeners2();
    }
    

    // throws Exception
    pub fn updateEndGameProcessor(&mut self, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) {
        jcall!(allBinaryGameCanvas).updateEndGameProcessor2();
    }
    

    // throws Exception
    pub fn buildGame(&mut self, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) {
        jcall!(allBinaryGameCanvas).buildGame2();
    }
    

    // throws Exception
    pub fn run(&mut self, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) {
        jcall!(allBinaryGameCanvas).run2();
    }
    

    // throws Exception
    pub fn setHighScore(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>, mut name: String, mut score: i64, mut autoSubmit: bool, mut isLast: bool) {
        jcall!(allBinaryGameCanvas).setHighScore2(abeClientInformation.clone(), name.clone(), score.clone(), autoSubmit.clone(), isLast.clone());
    }
}
impl std::ops::Deref for BaseGameBehavior {
    type Target = DemoGameBehavior;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for BaseGameBehavior {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::displayable::canvas::DemoGameBehavior::DemoGameBehaviorDyn for BaseGameBehavior {
    fn as_DemoGameBehavior(&self) -> &crate::org::allbinary::game::displayable::canvas::DemoGameBehavior::DemoGameBehavior { &self.base }
    fn as_DemoGameBehavior_mut(&mut self) -> &mut crate::org::allbinary::game::displayable::canvas::DemoGameBehavior::DemoGameBehavior { &mut self.base }
    fn init(&mut self) { Self::init(self) }
    fn pause(&mut self, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) { Self::pause(self, allBinaryGameCanvas) }
    fn unPause(&mut self, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) { Self::unPause(self, allBinaryGameCanvas) }
    fn updateTouch(&mut self, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) { Self::updateTouch(self, allBinaryGameCanvas) }
    fn updateScreenButtonPaintable(&mut self, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) { Self::updateScreenButtonPaintable(self, allBinaryGameCanvas) }
    fn setGameState(&mut self, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) { Self::setGameState(self, allBinaryGameCanvas) }
    fn removeAllGameKeyInputListeners(&mut self, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) { Self::removeAllGameKeyInputListeners(self, allBinaryGameCanvas) }
    fn updateEndGameProcessor(&mut self, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) { Self::updateEndGameProcessor(self, allBinaryGameCanvas) }
    fn buildGame(&mut self, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) { Self::buildGame(self, allBinaryGameCanvas) }
    fn run(&mut self, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) { Self::run(self, allBinaryGameCanvas) }
    fn setHighScore(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>, mut name: String, mut score: i64, mut autoSubmit: bool, mut isLast: bool) { Self::setHighScore(self, abeClientInformation, allBinaryGameCanvas, name, score, autoSubmit, isLast) }
}


