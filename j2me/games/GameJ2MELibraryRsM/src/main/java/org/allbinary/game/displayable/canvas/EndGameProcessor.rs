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
// package org.allbinary.game.displayable.canvas
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::canvas::Processor::Processor;
use crate::org::allbinary::game::score::HighScores::HighScores;
use crate::org::allbinary::game::score::HighScoresHelperBase::HighScoresHelperBase;
use crate::org::allbinary::game::score::NullHighScoresSingletonFactory::NullHighScoresSingletonFactory;
use crate::org::allbinary::game::state::GameStateFactory::GameStateFactory;
use crate::org::allbinary::graphics::paint::NullPaintable::NullPaintable;
use super::AllBinaryGameCanvas::AllBinaryGameCanvas;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct EndGameProcessor {
    pub base: Processor,
    pub gameStateFactory: Option<Rc<RefCell<GameStateFactory>>>,
    pub gameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>,
    pub WAIT: i64,
}


impl EndGameProcessor {
    

    pub fn new_struct(mut gameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) -> Self {
        let mut this = Self {
            base: Processor::new_struct(),
            gameStateFactory: None,
            gameCanvas: None,
            WAIT: 0_i64,
        };
        this.gameStateFactory = GameStateFactory::getInstance();
        this.WAIT = 5000_i64;
        this.gameCanvas = gameCanvas.clone();
        this
    }

    pub fn new(mut gameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(gameCanvas))))
    }
    

    // throws Exception
    pub fn process(&mut self) {
        if jcall!(self.gameCanvas).isHighScoreSubmitted() {
            if jcall!(jcall!(self.gameCanvas).getGameStateTimeHelper()).isElapsedTNT(self.WAIT.clone()) {
                if match (&jcall!(self.gameCanvas).getGameState(), &jcall!(self.gameStateFactory).SHOW_END_RESULT_GAME_STATE) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                    let mut highScoresBase: Option<Rc<RefCell<dyn crate::org::allbinary::game::score::HighScoresHelperBase::HighScoresHelperBaseDyn>>> = (jcall!(self.gameCanvas).as_AllBinaryGameCanvas_mut().highScoresHelper).clone() /* as HighScoresHelperBase */;
                    let mut highScores: Option<Rc<RefCell<dyn crate::org::allbinary::game::score::HighScores::HighScoresDyn>>> = jcall!(highScoresBase).getSelectedHighScores();
                    if !match (&highScores, &NullHighScoresSingletonFactory::getInstance()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                        jcall!(highScoresBase).selectHighScores();
                        jcall!(jcall!(self.gameCanvas).getRealHighScoresPaintable()).setHighScores(highScores.clone());
                        jcall!(self.gameCanvas).setGameState(jcall!(self.gameStateFactory).SHOW_HIGH_SCORE_GAME_STATE.clone());
                        { let java_receiver = self.gameCanvas.clone(); let java_arguments = (jcall!(self.gameCanvas).getRealHighScoresPaintable(),); jcall!(java_receiver).setHighScoresPaintable(java_arguments.0); };
                    }
                } else if match (&jcall!(self.gameCanvas).getGameState(), &jcall!(self.gameStateFactory).SHOW_HIGH_SCORE_GAME_STATE) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                    jcall!(self.gameCanvas).setGameState(jcall!(self.gameStateFactory).SHOW_END_RESULT_GAME_STATE.clone());
                    { let java_receiver = self.gameCanvas.clone(); let java_arguments = (NullPaintable::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>),); jcall!(java_receiver).setHighScoresPaintable(java_arguments.0); };
                }
            }
        }
    }
}
impl std::ops::Deref for EndGameProcessor {
    type Target = Processor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for EndGameProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::canvas::Processor::ProcessorDyn for EndGameProcessor {
    fn as_Processor(&self) -> &crate::org::allbinary::canvas::Processor::Processor { &self.base }
    fn as_Processor_mut(&mut self) -> &mut crate::org::allbinary::canvas::Processor::Processor { &mut self.base }
    fn process(&mut self) { Self::process(self) }
}


