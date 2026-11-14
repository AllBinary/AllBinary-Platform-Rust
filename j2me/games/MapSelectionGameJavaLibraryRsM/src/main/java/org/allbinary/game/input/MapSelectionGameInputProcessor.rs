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
use crate::javax::microedition::lcdui::Canvas::Canvas;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::logic::system::security::licensing::LockedFeatureNotificationUtil::LockedFeatureNotificationUtil;
use crate::org::allbinary::logic::system::security::licensing::LockedUtil::LockedUtil;
use crate::org::allbinary::canvas::Processor::Processor;
use crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas;
use crate::org::allbinary::game::displayable::canvas::PreGameSelectorPaintable::PreGameSelectorPaintable;
use crate::org::allbinary::game::input::event::GameKeyEventHandler::GameKeyEventHandler;
use crate::org::allbinary::game::input::event::GameKeyEventUtil::GameKeyEventUtil;
use crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::media::audio::SecondaryPlayerQueueFactory::SecondaryPlayerQueueFactory;
use crate::org::allbinary::media::audio::SelectSound::SelectSound;
use crate::org::allbinary::media::graphics::geography::map::racetrack::MultiLevelRaceTrackGeographicMapInterfaceFactoryInterface::MultiLevelRaceTrackGeographicMapInterfaceFactoryInterface;
use crate::org::allbinary::thread::ABRunnable::ABRunnable;
use crate::org::allbinary::thread::ThreadObjectUtil::ThreadObjectUtil;
use crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelper;
use crate::java::lang::Exception::*;
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

pub struct MapSelectionGameInputProcessor {
    pub base: Processor,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub gameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>,
    pub raceTrackGeographicMapInterfaceFactoryInterface: Option<Rc<RefCell<dyn MultiLevelRaceTrackGeographicMapInterfaceFactoryInterface>>>,
    pub preGameSelectorPaintable: Option<Rc<RefCell<PreGameSelectorPaintable>>>,
    pub inputTimeHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>>,
    pub playerGameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInputDyn>>>,
    pub lockedIndex: i32,
    pub abRunnable: Option<Rc<RefCell<dyn crate::org::allbinary::thread::ABRunnable::ABRunnableDyn>>>,
}

impl GameInputProcessorInterface for MapSelectionGameInputProcessor {
    fn onInput(&mut self, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { Self::onInput(self, list) }
}
impl PlayerGameInputCompositeInterface for MapSelectionGameInputProcessor {
    fn getPlayerGameInput(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInputDyn>>> { Self::getPlayerGameInput(self) }
}
impl crate::org::allbinary::game::input::GameInputInterface::GameInputInterface for MapSelectionGameInputProcessor {
    fn processInput(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { Self::processInput(self, layerManager) }
    fn initInputProcessors(&mut self) { Self::initInputProcessors(self) }
}
impl crate::org::allbinary::layer::NamedInterface::NamedInterface for MapSelectionGameInputProcessor {
    fn getName(&mut self) -> String { Self::getName(self) }
}

impl MapSelectionGameInputProcessor {
    

    pub fn new_struct(mut gameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>, mut raceTrackGeographicMapInterfaceFactoryInterface: Option<Rc<RefCell<dyn MultiLevelRaceTrackGeographicMapInterfaceFactoryInterface>>>, mut mapSelectorPaintable: Option<Rc<RefCell<PreGameSelectorPaintable>>>, mut lockedIndex: i32) -> Self {
        let mut this = Self {
            base: Processor::new_struct(),
            logUtil: None,
            gameCanvas: None,
            raceTrackGeographicMapInterfaceFactoryInterface: None,
            preGameSelectorPaintable: None,
            inputTimeHelper: None,
            playerGameInput: None,
            lockedIndex: 0,
            abRunnable: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.inputTimeHelper = TimeDelayHelper::new(650).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>);
        this.gameCanvas = gameCanvas.clone();
        this.raceTrackGeographicMapInterfaceFactoryInterface = raceTrackGeographicMapInterfaceFactoryInterface.clone();
        this.preGameSelectorPaintable = mapSelectorPaintable.clone();
        this
    }

    pub fn new(mut gameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>, mut raceTrackGeographicMapInterfaceFactoryInterface: Option<Rc<RefCell<dyn MultiLevelRaceTrackGeographicMapInterfaceFactoryInterface>>>, mut mapSelectorPaintable: Option<Rc<RefCell<PreGameSelectorPaintable>>>, mut lockedIndex: i32) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(gameCanvas.clone(), raceTrackGeographicMapInterfaceFactoryInterface.clone(), mapSelectorPaintable.clone(), lockedIndex.clone()))));
        jcall!(this).abRunnable = ABRunnable::new() /* Java anonymous class body: implement it as a named type
        

        pub fn run(&mut self) {
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                jcall!(this).setRunning(true);
                jcall!(SecondaryPlayerQueueFactory::getInstance()).add(SelectSound::getInstance());
                let mut track: i32 = jcall!(jcall!(jcall!(this).preGameSelectorPaintable).getPreGameSelectionForm()).getSelectedIndex() + 1;
                let mut wave: i32 = jcall!(jcall!(this).raceTrackGeographicMapInterfaceFactoryInterface).getFirstWaveWithTrack(track.clone());
                jcall!(jcall!(jcall!(jcall!(this).gameCanvas).getLayerManager()).getGameInfo()).setCurrentLevel(wave.clone());
                jcall!(jcall!(this).gameCanvas).buildGameInit(false);
                jcall!(GameKeyEventHandler::getInstance()).removeListener(jcall!(this).getPlayerGameInput().map(|_java_reference| -> Rc<RefCell<dyn crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface>> { panic!("ClassCastException: the Rust trait of PlayerGameInput does not have EventListenerInterface as a base trait") }));
                jcall!(this).setRunning(false);
            })) {
                Ok(_) => {}
                Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

                jcall!(this).setRunning(false);
                let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
                let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
                jcall!(logUtil).put({ let java_field = jcall!(commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(commonStrings).RUN.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
                }
            }
        }
*/.map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::thread::ABRunnable::ABRunnableDyn>>);
            jcall!(this).playerGameInput = GameInputProcessorComposite::new(String::from("Map Selection"), this.clone().map(|reference| reference as Rc<RefCell<dyn GameInputProcessorInterface>>)).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInputDyn>>);
            jcall!(this).lockedIndex = lockedIndex.clone();
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
            let mut key: i32 = GameKeyEventUtil::getKey(crate::java_into_object(anyType.clone()));
            if key == crate::javax::microedition::lcdui::Canvas::Canvas_LEFT.with(Clone::clone) || key == crate::javax::microedition::lcdui::Canvas::Canvas_RIGHT.with(Clone::clone) || key == crate::javax::microedition::lcdui::Canvas::Canvas_UP.with(Clone::clone) || key == crate::javax::microedition::lcdui::Canvas::Canvas_DOWN.with(Clone::clone) {
                if jcall!(self.inputTimeHelper).isTimeTNT() {
                    jcall!(SecondaryPlayerQueueFactory::getInstance()).add(SelectSound::getInstance());
                    jcall!(jcall!(self.preGameSelectorPaintable).getPreGameSelectionForm()).processInputKey(key.clone());
                    break;
                }
            } else if key == crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM0.with(Clone::clone) {
                let mut selectedIndex: i32 = jcall!(jcall!(self.preGameSelectorPaintable).getPreGameSelectionForm()).getSelectedIndex();
                if selectedIndex < self.lockedIndex || !jcall!(LockedUtil::getInstance()).isLocked() {
                    if !jcall!(self.abRunnable).isRunning() {
                        jcall!(self.abRunnable).setRunning(true);
                        jcall!(ThreadObjectUtil::getInstance()).processThread(self.abRunnable.clone().map(|_java_reference| -> Rc<RefCell<dyn crate::Runnable>> { panic!("ClassCastException: the Rust trait of ABRunnable does not have Runnable as a base trait") }));
                    }
                    break;
                } else {
                    jcall!(LockedFeatureNotificationUtil::getInstance()).fire();
                }
            }
            jcall!(list).clear();
        }
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
impl std::ops::Deref for MapSelectionGameInputProcessor {
    type Target = Processor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for MapSelectionGameInputProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::canvas::Processor::ProcessorDyn for MapSelectionGameInputProcessor {
    fn as_Processor(&self) -> &crate::org::allbinary::canvas::Processor::Processor { &self.base }
    fn as_Processor_mut(&mut self) -> &mut crate::org::allbinary::canvas::Processor::Processor { &mut self.base }
    fn process(&mut self) { Self::process(self) }
}


