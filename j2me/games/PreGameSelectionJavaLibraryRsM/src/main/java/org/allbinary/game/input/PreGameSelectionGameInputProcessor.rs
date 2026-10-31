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
// package org.allbinary.game.input
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Canvas::Canvas;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::logic::system::security::licensing::LockedFeatureNotificationUtil::LockedFeatureNotificationUtil;
use crate::org::allbinary::logic::system::security::licensing::LockedUtil::LockedUtil;
use crate::org::allbinary::canvas::Processor::Processor;
use crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas;
use crate::org::allbinary::game::displayable::canvas::PreGameSelectorPaintable::PreGameSelectorPaintable;
use crate::org::allbinary::game::input::event::GameKeyEventHandler::GameKeyEventHandler;
use crate::org::allbinary::game::input::event::GameKeyEventUtil::GameKeyEventUtil;
use crate::org::allbinary::game::state::GameState::GameState;
use crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::media::audio::SecondaryPlayerQueueFactory::SecondaryPlayerQueueFactory;
use crate::org::allbinary::media::audio::SelectSound::SelectSound;
use crate::org::allbinary::thread::ABRunnable::ABRunnable;
use crate::org::allbinary::thread::ThreadObjectUtil::ThreadObjectUtil;
use crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelper;
use super::GameInputProcessorInterface::GameInputProcessorInterface;
use super::PlayerGameInputCompositeInterface::PlayerGameInputCompositeInterface;
use super::PlayerGameInput::PlayerGameInput;
use super::GameInputProcessorComposite::GameInputProcessorComposite;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PreGameSelectionGameInputProcessor {
    pub base: Processor,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub gameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>,
    pub preGameSelectorPaintable: Option<Rc<RefCell<PreGameSelectorPaintable>>>,
    pub inputTimeHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>>,
    pub nextGameState: Option<Rc<RefCell<GameState>>>,
    pub playerGameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInputDyn>>>,
    pub lockedIndex: i32,
    pub abRunnable: Option<Rc<RefCell<dyn crate::org::allbinary::thread::ABRunnable::ABRunnableDyn>>>,
}

impl GameInputProcessorInterface for PreGameSelectionGameInputProcessor {
    fn onInput(&mut self, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { Self::onInput(self, list) }
}
// impl PlayerGameInputCompositeInterface for PreGameSelectionGameInputProcessor with the methods of the trait

impl PreGameSelectionGameInputProcessor {
    

    pub fn new_struct(mut gameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>, mut preGameSelectorPaintable: Option<Rc<RefCell<PreGameSelectorPaintable>>>, mut nextGameState: Option<Rc<RefCell<GameState>>>, mut lockedIndex: i32) -> Self {
        let mut this = Self {
            base: Processor::new_struct(),
            logUtil: None,
            gameCanvas: None,
            preGameSelectorPaintable: None,
            inputTimeHelper: None,
            nextGameState: None,
            playerGameInput: None,
            lockedIndex: 0,
            abRunnable: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.inputTimeHelper = TimeDelayHelper::new(650).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>);
        this.abRunnable = ABRunnable::new() /* Java anonymous class body: implement it as a named type
        

        pub fn run(&mut self) {
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                this.setRunning(true);
                jcall!(SecondaryPlayerQueueFactory::getInstance()).add(SelectSound::getInstance());
                jcall!(this.gameCanvas).setGameState(this.nextGameState.clone());
                jcall!(GameKeyEventHandler::getInstance()).removeListener(this.getPlayerGameInput().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface>>));
                this.setRunning(false);
            })) {
                Ok(_) => {}
                Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

                this.setRunning(false);
                let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
                let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
                jcall!(logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).RUN.clone(), crate::java_into_object(e.clone()));
                }
            }
        }
*/;
        this.gameCanvas = gameCanvas.clone();
        this.preGameSelectorPaintable = preGameSelectorPaintable.clone();
        this.nextGameState = nextGameState.clone();
        this.lockedIndex = lockedIndex.clone();
        this
    }

    pub fn new(mut gameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>, mut preGameSelectorPaintable: Option<Rc<RefCell<PreGameSelectorPaintable>>>, mut nextGameState: Option<Rc<RefCell<GameState>>>, mut lockedIndex: i32) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(gameCanvas, preGameSelectorPaintable, nextGameState, lockedIndex))));
            this.playerGameInput = GameInputProcessorComposite::new(String::from("Pre Game Selection"), (|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone())).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInputDyn>>);
        this
    }
    

    // throws Exception
    pub fn process(&mut self) {
        jcall!(self.getPlayerGameInput()).update();
    }
    

    // throws Exception
    pub fn onInput(&mut self, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        let mut size: i32 = jcall!(list).size();
        for index in (0..size) {
            let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(list).get(index.clone());
            let mut key: i32 = GameKeyEventUtil::getKey(anyType.clone());
            if key == unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_LEFT.clone() } || key == unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_RIGHT.clone() } || key == unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_UP.clone() } || key == unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_DOWN.clone() } {
                if jcall!(self.inputTimeHelper).isTimeTNT() {
                    jcall!(SecondaryPlayerQueueFactory::getInstance()).add(SelectSound::getInstance());
                    jcall!(jcall!(self.getPreGameSelectorPaintable()).getPreGameSelectionForm()).processInputKey(key.clone());
                    break;
                }
            } else if key == unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM0.clone() } {
                let mut selectedIndex: i32 = jcall!(jcall!(self.preGameSelectorPaintable).getPreGameSelectionForm()).getSelectedIndex();
                if selectedIndex < self.lockedIndex || !jcall!(LockedUtil::getInstance()).isLocked() {
                    if !jcall!(self.abRunnable).isRunning() {
                        jcall!(self.abRunnable).setRunning(true);
                        jcall!(ThreadObjectUtil::getInstance()).processThread(self.abRunnable.clone().map(|reference| reference as Rc<RefCell<dyn crate::Runnable>>));
                    }
                    break;
                } else {
                    jcall!(LockedFeatureNotificationUtil::getInstance()).fire();
                }
            }
        }
        jcall!(list).clear();
    }
    

    pub fn getPreGameSelectorPaintable(&mut self) -> Option<Rc<RefCell<PreGameSelectorPaintable>>> {
        return self.preGameSelectorPaintable.clone();
    }
    

    // throws Exception
    pub fn processInput(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) {
    }
    

    pub fn initInputProcessors(&mut self) {
    }
    

    pub fn getName(&mut self) -> String {
        return self.toString();
    }
    

    pub fn getPlayerGameInput(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInputDyn>>> {
        return self.playerGameInput.clone();
    }
}
impl std::ops::Deref for PreGameSelectionGameInputProcessor {
    type Target = Processor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for PreGameSelectionGameInputProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::canvas::Processor::ProcessorDyn for PreGameSelectionGameInputProcessor {
    fn as_Processor(&self) -> &crate::org::allbinary::canvas::Processor::Processor { &self.base }
    fn as_Processor_mut(&mut self) -> &mut crate::org::allbinary::canvas::Processor::Processor { &mut self.base }
    fn process(&mut self) { Self::process(self) }
}


