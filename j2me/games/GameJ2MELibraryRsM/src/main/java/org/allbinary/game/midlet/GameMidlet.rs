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
// package org.allbinary.game.midlet
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use crate::Enumeration;
use super::*;
// use crate::java::util::Enumeration::Enumeration;
// use crate::javax::microedition::lcdui::Canvas::Canvas;
// use crate::javax::microedition::lcdui::Command::Command;
// use crate::javax::microedition::lcdui::CommandListener::CommandListener;
// use crate::javax::microedition::lcdui::Displayable::Displayable;
// use crate::javax::microedition::lcdui::NullCanvas::NullCanvas;
// use crate::javax::microedition::midlet::MIDletStateChangeException::MIDletStateChangeException;
use crate::org::allbinary::J2MEUtil::J2MEUtil;
use crate::org::allbinary::TsUtil::TsUtil;
use crate::org::allbinary::business::advertisement::GameAdStateFactory::GameAdStateFactory;
use crate::org::allbinary::canvas::FullScreenUtil::FullScreenUtil;
use crate::org::allbinary::canvas::GameStatisticsFactory::GameStatisticsFactory;
use crate::org::allbinary::debug::DebugFactory::DebugFactory;
use crate::org::allbinary::debug::DebugInterface::DebugInterface;
use crate::org::allbinary::game::GameAdState::GameAdState;
use crate::org::allbinary::game::GameInfo::GameInfo;
use crate::org::allbinary::game::GameMode::GameMode;
use crate::org::allbinary::game::GameStrings::GameStrings;
use crate::org::allbinary::game::GameTypeFactory::GameTypeFactory;
use crate::org::allbinary::game::PlayerTypesFactory::PlayerTypesFactory;
use crate::org::allbinary::game::commands::GameCommandsFactory::GameCommandsFactory;
use crate::org::allbinary::game::configuration::GameOptionsForm::GameOptionsForm;
use crate::org::allbinary::game::configuration::GameOptionsFormFactory::GameOptionsFormFactory;
use crate::org::allbinary::game::configuration::InGameFeatures::InGameFeatures;
use crate::org::allbinary::game::configuration::InGameOptionsForm::InGameOptionsForm;
use crate::org::allbinary::game::configuration::InGameOptionsFormFactory::InGameOptionsFormFactory;
use crate::org::allbinary::game::configuration::LoadGameForm::LoadGameForm;
use crate::org::allbinary::game::configuration::event::ChangedGameFeatureListener::ChangedGameFeatureListener;
use crate::org::allbinary::game::configuration::event::GameFeatureEventHandler::GameFeatureEventHandler;
use crate::org::allbinary::game::configuration::feature::Features::Features;
use crate::org::allbinary::game::configuration::feature::GameFeatureFormUtil::GameFeatureFormUtil;
use crate::org::allbinary::game::configuration::feature::MainFeatureFactory::MainFeatureFactory;
use crate::org::allbinary::game::configuration::persistance::GamePersistanceSingleton::GamePersistanceSingleton;
use crate::org::allbinary::game::configuration::persistance::KeyValuePersistance::KeyValuePersistance;
use crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas;
use crate::org::allbinary::game::displayable::canvas::BasicPaintablesCanvas::BasicPaintablesCanvas;
use crate::org::allbinary::game::displayable::canvas::DemoCanvas::DemoCanvas;
use crate::org::allbinary::game::displayable::canvas::GameCanvasRunnableInterface::GameCanvasRunnableInterface;
use crate::org::allbinary::game::displayable::canvas::GameEventHandlerUtil::GameEventHandlerUtil;
use crate::org::allbinary::game::displayable::canvas::GameInputMappingCanvas::GameInputMappingCanvas;
use crate::org::allbinary::game::displayable::canvas::GameInputMappingInstructionsCanvas::GameInputMappingInstructionsCanvas;
use crate::org::allbinary::game::displayable::canvas::MenuListener::MenuListener;
use crate::org::allbinary::game::displayable::canvas::NullGameCanvasRunnable::NullGameCanvasRunnable;
use crate::org::allbinary::game::input::TextNotificationUtil::TextNotificationUtil;
use crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManager;
use crate::org::allbinary::game::layer::hud::event::GameNotificationEventHandler::GameNotificationEventHandler;
use crate::org::allbinary::game::paint::help::HelpPaintable::HelpPaintable;
use crate::org::allbinary::game::score::HighScoreCommands::HighScoreCommands;
use crate::org::allbinary::game::score::HighScoreCommandsFactory::HighScoreCommandsFactory;
use crate::org::allbinary::game::score::displayable::HighScoreTextBox::HighScoreTextBox;
use crate::org::allbinary::game::score::displayable::HighScoreUtil::HighScoreUtil;
use crate::org::allbinary::game::score::displayable::HighScoresCanvas::HighScoresCanvas;
use crate::org::allbinary::game::state::GameStateFactory::GameStateFactory;
use crate::org::allbinary::graphics::ResizableListenerHandler::ResizableListenerHandler;
use crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvas;
use crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvasFactory::ProgressCanvasFactory;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::graphics::color::ColorChangeEventHandler::ColorChangeEventHandler;
use crate::org::allbinary::graphics::displayable::CanvasStrings::CanvasStrings;
use crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas;
use crate::org::allbinary::graphics::displayable::command::MyCommandsFactory::MyCommandsFactory;
use crate::org::allbinary::graphics::displayable::screen::AboutCommandProcessor::AboutCommandProcessor;
use crate::org::allbinary::graphics::displayable::screen::AboutPaintableFactory::AboutPaintableFactory;
use crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandForm;
use crate::org::allbinary::graphics::displayable::screen::WebCommandProcessor::WebCommandProcessor;
use crate::org::allbinary::input::AllBinarySensorManager::AllBinarySensorManager;
use crate::org::allbinary::input::event::VirtualKeyboardEventHandler::VirtualKeyboardEventHandler;
use crate::org::allbinary::input::motion::gesture::observer::BasicMotionGesturesHandler::BasicMotionGesturesHandler;
use crate::org::allbinary::logic::communication::log::ForcedLogUtil::ForcedLogUtil;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::logic::math::SmallIntegerSingletonFactory::SmallIntegerSingletonFactory;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::system::security::licensing::ClientInformationFactory::ClientInformationFactory;
use crate::org::allbinary::logic::system::security::licensing::InApplicationPurchaseFactory::InApplicationPurchaseFactory;
use crate::org::allbinary::logic::system::security::licensing::LockableFeature::LockableFeature;
use crate::org::allbinary::logic::system::security::licensing::LockableFeatureFactory::LockableFeatureFactory;
use crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject;
use crate::org::allbinary::logic::util::event::EventStrings::EventStrings;
use crate::org::allbinary::media::audio::AllBinaryMediaManager::AllBinaryMediaManager;
use crate::org::allbinary::midlet::MidletStrings::MidletStrings;
use crate::org::allbinary::midlet::ProgressMidlet::ProgressMidlet;
use crate::org::allbinary::thread::NullThread::NullThread;
use crate::org::allbinary::thread::ThreadFactoryUtil::ThreadFactoryUtil;
use crate::org::allbinary::thread::ThreadUtil::ThreadUtil;
use crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelper;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::EnumerationUtil::EnumerationUtil;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use crate::java::lang::Thread::*;
use super::GameMidletStateFactory::GameMidletStateFactory;
use super::DemoGameMidletEvent::DemoGameMidletEvent;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GameMidlet {
    pub base: ProgressMidlet,
    pub enumerationUtil: Option<Rc<RefCell<EnumerationUtil>>>,
    pub basicColorFactory: Option<Rc<RefCell<BasicColorFactory>>>,
    pub features: Option<Rc<RefCell<Features>>>,
    pub midletStrings: Option<Rc<RefCell<MidletStrings>>>,
    pub myCommandsFactory: Option<Rc<RefCell<MyCommandsFactory>>>,
    pub gameStrings: Option<Rc<RefCell<GameStrings>>>,
    pub gameAdStateFactory: Option<Rc<RefCell<GameAdStateFactory>>>,
    pub gameStateFactory: Option<Rc<RefCell<GameStateFactory>>>,
    pub tsUtil: Option<Rc<RefCell<TsUtil>>>,
    pub DISPLAYABLE: String,
    pub COMMAND_NAME: String,
    pub NO_COMMAND: String,
    pub NO_DISPLAYABLE: String,
    pub COMMAND_ACTION: String,
    pub PAUSE_APP_BACKGROUND: String,
    pub UN_PAUSE_APP_BACKGROUND: String,
    pub aboutCommandProcessor: Option<Rc<RefCell<AboutCommandProcessor>>>,
    pub webCommandProcessor: Option<Rc<RefCell<WebCommandProcessor>>>,
    pub gameMidletStateFactory: Option<Rc<RefCell<GameMidletStateFactory>>>,
    pub gameStartTimeHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>>,
    pub fullScreenUtil: Option<Rc<RefCell<FullScreenUtil>>>,
    pub debugInterface: Option<Rc<RefCell<dyn DebugInterface>>>,
    pub allbinaryGameCanvasRunnableInterface: Option<Rc<RefCell<dyn GameCanvasRunnableInterface>>>,
    pub thread: Option<Rc<RefCell<Thread>>>,
    pub loadGameForm: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandFormDyn>>>,
    pub isFullScreen: bool,
    pub resized: bool,
    pub startedBefore: bool,
}

// impl CommandListener for GameMidlet with the methods of the trait

impl GameMidlet {
    

    pub fn new_struct(mut clientInformationFactory: Option<Rc<RefCell<ClientInformationFactory>>>) -> Self {
        let mut this = Self {
            base: ProgressMidlet::new_struct(clientInformationFactory.clone()),
            enumerationUtil: None,
            basicColorFactory: None,
            features: None,
            midletStrings: None,
            myCommandsFactory: None,
            gameStrings: None,
            gameAdStateFactory: None,
            gameStateFactory: None,
            tsUtil: None,
            DISPLAYABLE: String::new(),
            COMMAND_NAME: String::new(),
            NO_COMMAND: String::new(),
            NO_DISPLAYABLE: String::new(),
            COMMAND_ACTION: String::new(),
            PAUSE_APP_BACKGROUND: String::new(),
            UN_PAUSE_APP_BACKGROUND: String::new(),
            aboutCommandProcessor: None,
            webCommandProcessor: None,
            gameMidletStateFactory: None,
            gameStartTimeHelper: None,
            fullScreenUtil: None,
            debugInterface: None,
            allbinaryGameCanvasRunnableInterface: None,
            thread: None,
            loadGameForm: None,
            isFullScreen: false,
            resized: false,
            startedBefore: false,
        };
        this.enumerationUtil = EnumerationUtil::getInstance();
        this.basicColorFactory = BasicColorFactory::getInstance();
        this.features = Features::getInstance();
        this.midletStrings = MidletStrings::getInstance();
        this.myCommandsFactory = MyCommandsFactory::getInstance();
        this.gameStrings = GameStrings::getInstance();
        this.gameAdStateFactory = GameAdStateFactory::getInstance();
        this.gameStateFactory = GameStateFactory::getInstance();
        this.tsUtil = TsUtil::getInstance();
        this.DISPLAYABLE = String::from(" Displayable: ");
        this.COMMAND_NAME = String::from("command Name/Label: ");
        this.NO_COMMAND = String::from("No Command");
        this.NO_DISPLAYABLE = String::from("No Displayable");
        this.COMMAND_ACTION = jcall!(StringMaker::new()).append(String::from("GameMidlet::")).append(jcall!(this.midletStrings).COMMAND_ACTION.clone()).toString();
        this.PAUSE_APP_BACKGROUND = String::from("pauseAppBackground");
        this.UN_PAUSE_APP_BACKGROUND = String::from("unPauseAppBackground");
        this.aboutCommandProcessor = AboutCommandProcessor::getInstance();
        this.webCommandProcessor = WebCommandProcessor::getInstance();
        this.gameMidletStateFactory = GameMidletStateFactory::getInstance();
        this.gameStartTimeHelper = TimeDelayHelper::new(240).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>);
        this.fullScreenUtil = FullScreenUtil::getInstance();
        this.allbinaryGameCanvasRunnableInterface = crate::org::allbinary::game::displayable::canvas::NullGameCanvasRunnable::NullGameCanvasRunnable_NULL_GAME_CANVAS_RUNNABLE.with(Clone::clone).clone();
        this.thread = crate::org::allbinary::thread::NullThread::NullThread_NULL_THREAD.with(Clone::clone).clone();
        this.startedBefore = false;
        // super(...) is in the struct literal above
        jcall!(SmallIntegerSingletonFactory::getInstance()).initWithRange(0x291_u32 as i32, 6);
        this.loadGameForm = CommandForm::getNullCommandForm();
        let mut progressCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvasDyn>>> = ProgressCanvasFactory::getInstance();
        jcall!(GameFeatureEventHandler::getInstance()).addListenerInterface(ChangedGameFeatureListener::getInstance());
        jcall!(GamePersistanceSingleton::getInstance()).clear();
        this.debugInterface = DebugFactory::getInstance().map(|reference| reference as Rc<RefCell<dyn DebugInterface>>);
        this.init();
        this
    }

    pub fn new(mut clientInformationFactory: Option<Rc<RefCell<ClientInformationFactory>>>) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(clientInformationFactory))));
            jcall!(progressCanvas).init((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()));
        this
    }
    

    pub fn init(&mut self) {
    }
    

    // throws Exception
    pub fn setDemo(&mut self) {
        panic!("{:?}", format!("{}: {:?}", "Exception", (jcall!(self.commonStrings).NOT_IMPLEMENTED.clone())));
    }
    

    // throws Exception
    pub fn createGame(&mut self) {
        panic!("{:?}", format!("{}: {:?}", "Exception", (jcall!(self.commonStrings).NOT_IMPLEMENTED.clone())));
    }
    

    // throws Exception
    pub fn mediaShutdown(&mut self) {
        panic!("{:?}", format!("{}: {:?}", "Exception", (jcall!(self.commonStrings).NOT_IMPLEMENTED.clone())));
    }
    

    pub fn stopAll(&mut self) {
    }
    

    pub fn pauseApp(&mut self) {
        self.pauseAppBackground(true);
        let mut gameAdState: Option<Rc<RefCell<GameAdState>>> = jcall!(self.gameAdStateFactory).getCurrentInstance();
        jcall!(jcall!(gameAdState).getAdvertisements()).stopAll();
    }
    

    pub fn pauseAppBackground(&mut self, mut background: bool) {
        jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.PAUSE_APP_BACKGROUND.clone());
        if !match (&self.allbinaryGameCanvasRunnableInterface, &crate::org::allbinary::game::displayable::canvas::NullGameCanvasRunnable::NullGameCanvasRunnable_NULL_GAME_CANVAS_RUNNABLE.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            jcall!(self.allbinaryGameCanvasRunnableInterface).pause();
        } else {
            jcall!(self.logUtil).putF(String::from("<<<<<< Null"), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.PAUSE_APP_BACKGROUND.clone());
        }
        jcall!(AllBinarySensorManager::getInstance()).shutdown();
    }
    

    pub fn unPauseApp(&mut self) {
        self.unPauseAppBackground(true);
        let mut gameAdState: Option<Rc<RefCell<GameAdState>>> = jcall!(self.gameAdStateFactory).getCurrentInstance();
        jcall!(jcall!(gameAdState).getAdvertisements()).startAll();
    }
    

    pub fn unPauseAppBackground(&mut self, mut background: bool) {
        jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.UN_PAUSE_APP_BACKGROUND.clone());
        jcall!(AllBinarySensorManager::getInstance()).init();
        let mut gameCanvasRunnableInterface: Option<Rc<RefCell<dyn GameCanvasRunnableInterface>>> = self.allbinaryGameCanvasRunnableInterface.clone();
        if !match (&gameCanvasRunnableInterface, &crate::org::allbinary::game::displayable::canvas::NullGameCanvasRunnable::NullGameCanvasRunnable_NULL_GAME_CANVAS_RUNNABLE.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            jcall!(gameCanvasRunnableInterface).unPause();
        } else {
            jcall!(self.logUtil).putF(String::from("<<<<<< Null"), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.UN_PAUSE_APP_BACKGROUND.clone());
        }
    }
    

    pub fn destroyAppInRunnable(&mut self, mut unconditional: bool, mut isProgress: bool) {
        let mut progressCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvasDyn>>> = ProgressCanvasFactory::getInstance();
        if isProgress {
            jcall!(progressCanvas).start();
            self.commandAction(jcall!(self.myCommandsFactory).SET_DISPLAYABLE.clone(), progressCanvas.clone());
        }
        self.destroyApp(unconditional.clone());
        if isProgress {
            jcall!(progressCanvas).end();
        }
    }
    

    pub fn destroyApp(&mut self, mut unconditional: bool) {
        let mut METHOD_NAME: String = String::from("GameMidlet::destroyApp");
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            PreLogUtil::put(jcall!(GameStatisticsFactory::getInstance()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), METHOD_NAME.clone());
            let mut gameAdState: Option<Rc<RefCell<GameAdState>>> = jcall!(self.gameAdStateFactory).getCurrentInstance();
            jcall!(jcall!(gameAdState).getAdvertisements()).stopAll();
            if !self.isDestroyed() {
                self.stopGameCanvasRunnableInterface();
                self.setGameCanvasRunnableInterface(crate::org::allbinary::game::displayable::canvas::NullGameCanvasRunnable::NullGameCanvasRunnable_NULL_GAME_CANVAS_RUNNABLE.with(Clone::clone).clone());
                self.mediaShutdown();
            } else {
                jcall!(self.logUtil).putF(String::from("Midlet Managment Error: Midlet Should Only Be Destroyed Once"), crate::java_into_object(String::from(std::any::type_name::<Self>())), METHOD_NAME.clone());
            }
            self.base.destroyApp(true);
            PreLogUtil::put(jcall!(self.commonStrings).END.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), METHOD_NAME.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), METHOD_NAME.clone(), crate::java_into_object(e.clone()));
            }
        }
        jcall!(self.logUtil).putF(jcall!(self.commonStrings).END.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), METHOD_NAME.clone());
    }
    

    // throws MIDletStateChangeException
    pub fn startApp(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut gameAdState: Option<Rc<RefCell<GameAdState>>> = jcall!(self.gameAdStateFactory).getCurrentInstance();
            jcall!(jcall!(gameAdState).getAdvertisements()).startAll();
            let mut START_APP: String = String::from("startApp");
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), START_APP.clone());
            let mut gameCanvasRunnableInterface: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = self.allbinaryGameCanvasRunnableInterface.clone();
            if match (&gameCanvasRunnableInterface, &crate::org::allbinary::game::displayable::canvas::NullGameCanvasRunnable::NullGameCanvasRunnable_NULL_GAME_CANVAS_RUNNABLE.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                jcall!(self.gameMidletStateFactory).setCurrentGameState(jcall!(self.gameStateFactory).NO_GAME_STATE.clone());
                self.setDemo();
            } else {
                jcall!(self.logUtil).putF(String::from("GameCanvasRunnableInterface is available"), crate::java_into_object(String::from(std::any::type_name::<Self>())), START_APP.clone());
                if match (&gameCanvasRunnableInterface, &self.getCurrentDisplayable()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                    self.unPauseAppBackground(false);
                } else {
                    jcall!(self.logUtil).putF(String::from("GameCanvasRunnableInterface is not current displayable"), crate::java_into_object(String::from(std::any::type_name::<Self>())), START_APP.clone());
                }
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("startApp"), crate::java_into_object(e.clone()));
            self.destroyApp(false);
            self.notifyDestroyed();
            }
        }
    }
    

    // synchronized
    pub fn commandAction(&mut self, mut command: Option<Rc<RefCell<Command>>>, mut displayable: Option<Rc<RefCell<Displayable>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut displayableAsString: String = self.NO_DISPLAYABLE.clone();
            if displayable.is_some() {
                displayableAsString = jcall!(displayable).toString();
            }
            let mut label: String = self.NO_COMMAND.clone();
            if command.is_some() {
                label = jcall!(command).getLabel();
            }
            PreLogUtil::put(jcall!(StringMaker::new()).append(self.COMMAND_NAME.clone()).append(label.clone()).append(self.DISPLAYABLE.clone()).append(displayableAsString.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.COMMAND_ACTION.clone());
            let mut gameCommandsFactory: Option<Rc<RefCell<GameCommandsFactory>>> = GameCommandsFactory::getInstance();
            let mut allbinaryGameCanvasRunnableInterface: Option<Rc<RefCell<dyn GameCanvasRunnableInterface>>> = self.allbinaryGameCanvasRunnableInterface.clone();
            let mut gameCanvasRunnableInterface: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = allbinaryGameCanvasRunnableInterface.clone();
            if match (&command, &jcall!(gameCommandsFactory).SHOW_GAME_CANVAS) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                if !match (&jcall!(self.getDisplay()).getCurrent(), &gameCanvasRunnableInterface) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } && jcall!(allbinaryGameCanvasRunnableInterface).getType() != jcall!(crate::org::allbinary::game::displayable::canvas::NullGameCanvasRunnable::NullGameCanvasRunnable_NULL_GAME_CANVAS_RUNNABLE.with(Clone::clone)).getType() {
                    self.setDisplay((gameCanvasRunnableInterface).clone() /* as Displayable */);
                }
                self.unPauseAppBackground(false);
            } else if match (&command, &jcall!(gameCommandsFactory).CLOSE_AND_SHOW_GAME_CANVAS) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                let mut menuListener: Option<Rc<RefCell<dyn MenuListener>>> = (displayable).clone() /* as MenuListener */;
                jcall!(menuListener).close();
                self.setDisplay((gameCanvasRunnableInterface).clone() /* as Displayable */);
                self.unPauseAppBackground(false);
            } else if match (&command, &jcall!(gameCommandsFactory).EXIT_COMMAND) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                if jcall!(jcall!(GameMidletEventHandler::GameMidletEventHandler::getInstance()).getEventListenerInterfaceListP()).size() == 0 {
                    self.exitProgress(true);
                }
                jcall!(GameMidletEventHandler::GameMidletEventHandler::getInstance()).fireEvent(DemoGameMidletEvent::new(None /* Java this: a &mut self method has no Rc of itself */, jcall!(DemoGameMidletStateFactory::DemoGameMidletStateFactory::getInstance()).NONE.clone()));
            } else if match (&command, &jcall!(gameCommandsFactory).EXIT_WITHOUT_PROGRESS_COMMAND) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                self.exitProgress(false);
            } else if match (&command, &jcall!(gameCommandsFactory).START_COMMAND) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } || match (&command, &jcall!(gameCommandsFactory).RESTART_COMMAND) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } || match (&command, &jcall!(gameCommandsFactory).CONTINUE_COMMAND) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                if !match (&jcall!(self.gameMidletStateFactory).getCurrentGameState(), &jcall!(self.gameStateFactory).PLAYING_GAME_STATE) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } || match (&command, &jcall!(gameCommandsFactory).RESTART_COMMAND) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                    if jcall!(self.gameStartTimeHelper).isTimeTNT() {
                        if match (&command, &jcall!(gameCommandsFactory).START_COMMAND) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } && self.isDemoLoading() {
                            jcall!(self.logUtil).putF(String::from("Trying to Start Game Before Loading Complete"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.midletStrings).COMMAND_ACTION.clone());
                        } else {
                            self.startedBefore = true;
                            self.createGame();
                            jcall!(self.gameMidletStateFactory).setCurrentGameState(jcall!(self.gameStateFactory).PLAYING_GAME_STATE.clone());
                        }
                    } else {
                        jcall!(self.logUtil).putF(String::from("Starting Game Too Often"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.midletStrings).COMMAND_ACTION.clone());
                    }
                } else {
                    jcall!(self.logUtil).putF(String::from("Already in playing state"), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.COMMAND_ACTION.clone());
                }
            } else if match (&command, &jcall!(gameCommandsFactory).BUY_COMMAND) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                let mut inApplicationPurchaseFactory: Option<Rc<RefCell<InApplicationPurchaseFactory>>> = InApplicationPurchaseFactory::getInstance();
                let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(LockableFeatureFactory::getInstance()).getList();
                let mut lockableFeature: Option<Rc<RefCell<LockableFeature>>> = { let java_object = (jcall!(list).get(0)).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<LockableFeature>>>>().expect("ClassCastException").clone()) };
                if jcall!(list).size() > 0 && !jcall!(inApplicationPurchaseFactory).isPurchased(lockableFeature.clone()) {
                    jcall!(inApplicationPurchaseFactory).purchase(lockableFeature.clone());
                } else {
                    jcall!(TextNotificationUtil::getInstance()).fireSuccess(String::from("Already Purchased"));
                }
            } else if match (&command, &jcall!(gameCommandsFactory).QUIT_COMMAND) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                if jcall!(self.gameStartTimeHelper).isTimeTNT() {
                    if (displayable).is_some() /* instanceof HighScoreTextBox */ {
                        let mut menuListener: Option<Rc<RefCell<dyn MenuListener>>> = (displayable).clone() /* as MenuListener */;
                        jcall!(menuListener).close();
                    }
                    self.stopGameCanvasRunnableInterface();
                    jcall!(self.gameMidletStateFactory).setCurrentGameState(jcall!(self.gameStateFactory).NO_GAME_STATE.clone());
                    self.setDemo();
                }
            } else if match (&command, &jcall!(self.myCommandsFactory).RESUME_COMMAND) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                self.unPauseAppBackground(false);
            } else if match (&command, &jcall!(self.myCommandsFactory).PAUSE_COMMAND) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                self.pauseAppBackground(false);
            } else if match (&command, &jcall!(self.myCommandsFactory).SET_DISPLAYABLE) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                self.pauseAppBackground(false);
                self.setDisplay(displayable.clone());
            } else if match (&command, &jcall!(gameCommandsFactory).SET_MENU_DISPLAYABLE) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                self.pauseAppBackground(false);
                let mut menuListener: Option<Rc<RefCell<dyn MenuListener>>> = (displayable).clone() /* as MenuListener */;
                jcall!(menuListener).open();
                self.setDisplay(displayable.clone());
            } else if match (&command, &jcall!(gameCommandsFactory).START_TRACE) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                jcall!(self.debugInterface).start();
                let mut gameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>> = (allbinaryGameCanvasRunnableInterface).clone() /* as AllBinaryGameCanvas */;
                jcall!(gameCanvas).addCommand(jcall!(gameCommandsFactory).STOP_TRACE.clone());
                jcall!(gameCanvas).removeCommand(jcall!(gameCommandsFactory).START_TRACE.clone());
            } else if match (&command, &jcall!(gameCommandsFactory).STOP_TRACE) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                self.pauseAppBackground(false);
                jcall!(self.debugInterface).stop();
                self.unPauseAppBackground(false);
                let mut gameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>> = (allbinaryGameCanvasRunnableInterface).clone() /* as AllBinaryGameCanvas */;
                jcall!(gameCanvas).addCommand(jcall!(gameCommandsFactory).START_TRACE.clone());
                jcall!(gameCanvas).removeCommand(jcall!(gameCommandsFactory).STOP_TRACE.clone());
            } else if match (&command, &jcall!(gameCommandsFactory).DEFAULT_OPTIONS) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                let mut tempDisplayable: Option<Rc<RefCell<Displayable>>> = displayable.clone();
                if (tempDisplayable).is_some() /* instanceof GameOptionsForm */ {
                    jcall!(GameFeatureFormUtil::getInstance()).setDefault((tempDisplayable).clone() /* as CommandForm */);
                }
            } else if match (&command, &crate::org::allbinary::game::displayable::canvas::GameInputMappingInstructionsCanvas::GameInputMappingInstructionsCanvas_DISPLAY.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                let mut tempDisplayable: Option<Rc<RefCell<Displayable>>> = displayable.clone();
                if (tempDisplayable).is_some() /* instanceof GameInputMappingCanvas */ {
                    let mut gameInputMappingCanvas: Option<Rc<RefCell<GameInputMappingCanvas>>> = (tempDisplayable).clone() /* as GameInputMappingCanvas */;
                    jcall!(gameInputMappingCanvas).update();
                }
                { let java_arguments = (jcall!(gameCommandsFactory).SET_MENU_DISPLAYABLE.clone(), self.getInputMappingInstructionsCanvas(),); self.commandAction(java_arguments.0, java_arguments.1) };
            } else if jcall!(HighScoreCommandsFactory::getInstance()).isHighScoreCommand(command.clone()) {
                let mut tempDisplayable: Option<Rc<RefCell<Displayable>>> = displayable.clone();
                if (tempDisplayable).is_some() /* instanceof HighScoresCanvas */ {
                    let mut highScoresCanvas: Option<Rc<RefCell<HighScoresCanvas>>> = (tempDisplayable).clone() /* as HighScoresCanvas */;
                    jcall!(highScoresCanvas).updateCommand(command.clone());
                }
            } else if match (&command, &jcall!(HighScoreCommands::getInstance()).DISPLAY) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                { let java_arguments = (jcall!(gameCommandsFactory).SET_MENU_DISPLAYABLE.clone(), self.createHighScoresCanvas(),); self.commandAction(java_arguments.0, java_arguments.1) };
            } else if match (&command, &crate::org::allbinary::game::displayable::canvas::GameInputMappingInstructionsCanvas::GameInputMappingInstructionsCanvas_CLOSE.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                let mut menuListener: Option<Rc<RefCell<dyn MenuListener>>> = (displayable).clone() /* as MenuListener */;
                jcall!(menuListener).close();
                self.commandAction(crate::org::allbinary::game::displayable::canvas::GameInputMappingCanvas::GameInputMappingCanvas_DISPLAY.with(Clone::clone).clone(), crate::javax::microedition::lcdui::NullCanvas::NullCanvas_NULL_CANVAS.with(Clone::clone).clone());
            } else if match (&command, &crate::org::allbinary::game::displayable::canvas::GameInputMappingCanvas::GameInputMappingCanvas_DISPLAY.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                if J2MEUtil::isHTML() {
                    self.pauseAppBackground(false);
                }
                { let java_arguments = (jcall!(gameCommandsFactory).SET_MENU_DISPLAYABLE.clone(), self.getInputMappingCanvas(),); self.commandAction(java_arguments.0, java_arguments.1) };
            } else if match (&command, &crate::org::allbinary::game::displayable::canvas::GameInputMappingCanvas::GameInputMappingCanvas_DEFAULT.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                let mut tempDisplayable: Option<Rc<RefCell<Displayable>>> = displayable.clone();
                if (tempDisplayable).is_some() /* instanceof GameInputMappingCanvas */ {
                    let mut gameInputMappingCanvas: Option<Rc<RefCell<GameInputMappingCanvas>>> = (tempDisplayable).clone() /* as GameInputMappingCanvas */;
                    jcall!(gameInputMappingCanvas).setDefault();
                }
            } else if match (&command, &crate::org::allbinary::game::configuration::InGameOptionsForm::InGameOptionsForm_DISPLAY.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                self.pauseAppBackground(false);
                let mut progressCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvasDyn>>> = ProgressCanvasFactory::getInstance();
                jcall!(progressCanvas).addNormalPortion(50, String::from("In Game Options"));
                let mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>> = self.createGameLayerManager();
                let mut inGameOptionsFormFactory: Option<Rc<RefCell<InGameOptionsFormFactory>>> = InGameOptionsFormFactory::getInstance();
                jcall!(inGameOptionsFormFactory).init(None /* Java this: a &mut self method has no Rc of itself */, InGameFeatures::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::init::Init::InitDyn>>), String::from("Options In Game"), jcall!(layerManager).getBackgroundBasicColor(), jcall!(layerManager).getForegroundBasicColor());
                let mut inGameOptionsForm: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandFormDyn>>> = jcall!(inGameOptionsFormFactory).get();
                jcall!(inGameOptionsForm).setItemStateListener(allbinaryGameCanvasRunnableInterface.clone());
                self.commandAction(jcall!(gameCommandsFactory).SET_MENU_DISPLAYABLE.clone(), inGameOptionsForm.clone());
            } else if match (&command, &crate::org::allbinary::game::configuration::InGameOptionsForm::InGameOptionsForm_DEFAULT.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                let mut inGameOptionsForm: Option<Rc<RefCell<InGameOptionsForm>>> = (jcall!(InGameOptionsFormFactory::getInstance()).get()).clone() /* as InGameOptionsForm */;
                jcall!(GameFeatureFormUtil::getInstance()).setDefault(inGameOptionsForm.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandFormDyn>>));
            } else if match (&command, &jcall!(gameCommandsFactory).DISPLAY_OPTIONS) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                AllBinaryMediaManager::setMuted(true);
                self.stopAll();
                let mut mainFeatureFactory: Option<Rc<RefCell<MainFeatureFactory>>> = MainFeatureFactory::getInstance();
                self.isFullScreen = jcall!(self.features).isFeature(jcall!(mainFeatureFactory).FULL_SCREEN.clone());
                jcall!(ResizableListenerHandler::getInstance()).fireEvent(true);
                self.setResized(false);
                { let java_arguments = (jcall!(gameCommandsFactory).SET_MENU_DISPLAYABLE.clone(), self.getGameOptionsForm(),); self.commandAction(java_arguments.0, java_arguments.1) };
            } else if match (&command, &jcall!(gameCommandsFactory).CLOSE_OPTIONS) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                jcall!(ResizableListenerHandler::getInstance()).fireEvent(false);
                self.commandAction(jcall!(gameCommandsFactory).CLOSE_AND_SHOW_GAME_CANVAS.clone(), displayable.clone());
                let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                jcall!(stringBuffer).append(String::from("Close isFullScreen/change: "));
                jcall!(stringBuffer).appendboolean(self.isFullScreen.clone());
                jcall!(stringBuffer).appendboolean(jcall!(self.fullScreenUtil).isScreenChange(self.isFullScreen.clone()));
                jcall!(stringBuffer).append(String::from(" isResized: "));
                jcall!(stringBuffer).appendboolean(self.isResized());
                PreLogUtil::put(jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.COMMAND_ACTION.clone());
                if self.isResized() || jcall!(self.fullScreenUtil).isScreenChange(self.isFullScreen.clone()) {
                    self.updateFullScreen();
                }
                AllBinaryMediaManager::setMuted(false);
            } else if match (&command, &jcall!(gameCommandsFactory).DISPLAY_LOAD_FORM) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                self.pauseAppBackground(false);
                let mut keyValuePersistance: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::persistance::KeyValuePersistance::KeyValuePersistanceDyn>>> = GamePersistanceSingleton::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::persistance::KeyValuePersistance::KeyValuePersistanceDyn>>);
                jcall!(keyValuePersistance).clear();
                jcall!(keyValuePersistance).loadAll(self.abeClientInformation.clone());
                if match (&self.getLoadGameForm(), &CommandForm::getNullCommandForm()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                    let mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>> = self.createGameLayerManager();
                    { let java_arguments = (LoadGameForm::new(None /* Java this: a &mut self method has no Rc of itself */, String::from("Load Game"), jcall!(layerManager).getBackgroundBasicColor(), jcall!(layerManager).getForegroundBasicColor()),); self.setLoadGameForm(java_arguments.0) };
                } else {
                    jcall!(self.getLoadGameForm()).update();
                }
                { let java_arguments = (jcall!(gameCommandsFactory).SET_MENU_DISPLAYABLE.clone(), self.getLoadGameForm(),); self.commandAction(java_arguments.0, java_arguments.1) };
            } else if match (&command, &jcall!(gameCommandsFactory).LOAD_FILE) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                let mut loadGameForm: Option<Rc<RefCell<LoadGameForm>>> = (self.getLoadGameForm()).clone() /* as LoadGameForm */;
                let mut index: i32 = jcall!(loadGameForm).as_CommandForm_mut().getSelectedId();
                if index != -1 {
                    let mut keyValuePersistance: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::persistance::KeyValuePersistance::KeyValuePersistanceDyn>>> = GamePersistanceSingleton::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::persistance::KeyValuePersistance::KeyValuePersistanceDyn>>);
                    { let java_arguments = (jcall!(keyValuePersistance).get(index.clone()),); self.setStartStateHashtable(java_arguments.0) };
                    let mut menuListener: Option<Rc<RefCell<dyn MenuListener>>> = self.getLoadGameForm().map(|reference| reference as Rc<RefCell<dyn MenuListener>>);
                    jcall!(menuListener).close();
                    PreLogUtil::put(jcall!(BasicMotionGesturesHandler::getInstance()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.COMMAND_ACTION.clone());
                    self.commandAction(jcall!(gameCommandsFactory).START_COMMAND.clone(), crate::javax::microedition::lcdui::NullCanvas::NullCanvas_NULL_CANVAS.with(Clone::clone).clone());
                    PreLogUtil::put(jcall!(BasicMotionGesturesHandler::getInstance()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.COMMAND_ACTION.clone());
                }
            } else if match (&command, &jcall!(gameCommandsFactory).DELETE_FILE) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                let mut loadGameForm: Option<Rc<RefCell<LoadGameForm>>> = (self.getLoadGameForm()).clone() /* as LoadGameForm */;
                let mut index: i32 = jcall!(loadGameForm).as_CommandForm_mut().getSelectedText();
                if index != -1 {
                    let mut keyValuePersistance: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::persistance::KeyValuePersistance::KeyValuePersistanceDyn>>> = GamePersistanceSingleton::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::persistance::KeyValuePersistance::KeyValuePersistanceDyn>>);
                    jcall!(keyValuePersistance).delete(self.abeClientInformation.clone(), index.clone());
                    jcall!(keyValuePersistance).clear();
                    jcall!(keyValuePersistance).loadAllSize(self.abeClientInformation.clone(), 1);
                    jcall!(self.getLoadGameForm()).update();
                }
            } else if match (&command, &jcall!(gameCommandsFactory).SAVE) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                let mut keyValuePersistance: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::persistance::KeyValuePersistance::KeyValuePersistanceDyn>>> = GamePersistanceSingleton::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::persistance::KeyValuePersistance::KeyValuePersistanceDyn>>);
                self.pauseAppBackground(false);
                jcall!(keyValuePersistance).save(self.abeClientInformation.clone(), self.getCurrentStateHashtable());
                self.unPauseAppBackground(false);
            } else if match (&command, &crate::org::allbinary::game::score::displayable::HighScoreUtil::HighScoreUtil_SUBMIT_TEXTBOX_COMMAND.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                jcall!(self.logUtil).putF(String::from("Submitted Score"), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.COMMAND_ACTION.clone());
                if (displayable).is_some() /* instanceof HighScoreTextBox */ {
                    let mut highScoreTextBox: Option<Rc<RefCell<HighScoreTextBox>>> = (displayable).clone() /* as HighScoreTextBox */;
                    jcall!(highScoreTextBox).submitted = true;
                }
                jcall!(allbinaryGameCanvasRunnableInterface).setHighScoreSubmitted(true);
                self.commandAction(jcall!(gameCommandsFactory).CLOSE_AND_SHOW_GAME_CANVAS.clone(), displayable.clone());
            } else if match (&command, &jcall!(gameCommandsFactory).DISPLAY_ABOUT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                { let java_receiver = self.aboutCommandProcessor.clone(); let java_arguments = (None /* Java this: a &mut self method has no Rc of itself */, jcall!(gameCommandsFactory).SET_MENU_DISPLAYABLE.clone(), self.getAboutCanvas(),); jcall!(java_receiver).process(java_arguments.0, java_arguments.1, java_arguments.2); };
            } else if match (&command, &jcall!(gameCommandsFactory).OPEN_WEB_URL) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                jcall!(self.webCommandProcessor).process(None /* Java this: a &mut self method has no Rc of itself */, jcall!(gameCommandsFactory).OPEN_WEB_URL.clone(), crate::javax::microedition::lcdui::NullCanvas::NullCanvas_NULL_CANVAS.with(Clone::clone).clone());
            } else if match (&command, &jcall!(gameCommandsFactory).TOGGLE_KEYBOARD) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                let mut virtualKeyboardEventHandler: Option<Rc<RefCell<VirtualKeyboardEventHandler>>> = VirtualKeyboardEventHandler::getInstance();
                jcall!(virtualKeyboardEventHandler).fireEvent(jcall!(virtualKeyboardEventHandler).SHOW_EVENT.clone());
            } else if { let left = &(jcall!(command).getLabel()); let right = &(jcall!(jcall!(gameCommandsFactory).TOGGLE_FULLSCREEN).getLabel()); let mut left_units = left.encode_utf16(); let mut right_units = right.encode_utf16(); loop { match (left_units.next(), right_units.next()) { (Some(a), Some(b)) if a != b => break a as i32 - b as i32, (Some(_), Some(_)) => {}, (None, None) => break 0, _ => break left.encode_utf16().count() as i32 - right.encode_utf16().count() as i32 } } } == 0 {
                let mut mainFeatureFactory: Option<Rc<RefCell<MainFeatureFactory>>> = MainFeatureFactory::getInstance();
                let mut isFullScreen: bool = jcall!(self.features).isFeature(jcall!(mainFeatureFactory).FULL_SCREEN.clone());
                if isFullScreen {
                    jcall!(self.features).removeDefault(jcall!(mainFeatureFactory).FULL_SCREEN.clone());
                } else {
                    jcall!(self.features).addDefault(jcall!(mainFeatureFactory).FULL_SCREEN.clone());
                }
                self.updateFullScreen();
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(self.commonStrings).EXCEPTION_LABEL.clone()).append(jcall!(command).getLabel()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.midletStrings).COMMAND_ACTION.clone(), crate::java_into_object(e.clone()),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3); };
            if !match (&command, &jcall!(GameCommandsFactory::getInstance()).EXIT_COMMAND) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                self.exitProgress(false);
            }
            }
        }
    }
    

    pub fn updateFullScreen(&mut self) {
        let mut mainFeatureFactory: Option<Rc<RefCell<MainFeatureFactory>>> = MainFeatureFactory::getInstance();
        let mut isFullScreen: bool = jcall!(self.features).isFeature(jcall!(mainFeatureFactory).FULL_SCREEN.clone());
        let mut displayable: Option<Rc<RefCell<Displayable>>> = jcall!(self.getDisplay()).getCurrent();
        if (displayable).is_some() /* instanceof Canvas */ {
            let mut canvas: Option<Rc<RefCell<Canvas>>> = (displayable).clone() /* as Canvas */;
            jcall!(canvas).setFullScreenMode(isFullScreen.clone());
        }
    }
    

    pub fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) {
        ForcedLogUtil::log(jcall!(EventStrings::getInstance()).PERFORMANCE_MESSAGE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())));
    }
    

    // throws Exception
    pub fn getInputMappingInstructionsCanvas(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn>>> {
        return GameInputMappingInstructionsCanvas::new(None /* Java this: a &mut self method has no Rc of itself */, self.createGameLayerManager()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn>>);
    }
    

    // throws Exception
    pub fn getInputMappingCanvas(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn>>> {
        return GameInputMappingCanvas::new(self.abeClientInformation.clone(), None /* Java this: a &mut self method has no Rc of itself */, self.createGameLayerManager(), self.getHelpPaintable()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn>>);
    }
    

    // throws Exception
    pub fn getAboutCanvas(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn>>> {
        return BasicPaintablesCanvas::new(None /* Java this: a &mut self method has no Rc of itself */, jcall!(CanvasStrings::getInstance()).ABOUT.clone(), self.createGameLayerManager(), jcall!(AboutPaintableFactory::getInstance()).paintableArray.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn>>);
    }
    

    // throws Exception
    pub fn getHelpPaintable(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::paint::help::HelpPaintable::HelpPaintableDyn>>> {
        panic!("{:?}", format!("{}: {:?}", "Exception", (jcall!(self.commonStrings).NOT_IMPLEMENTED.clone())));
    }
    

    // throws Exception
    pub fn createHighScoresCanvas(&mut self) -> Option<Rc<RefCell<HighScoresCanvas>>> {
        panic!("{:?}", format!("{}: {:?}", "Exception", (jcall!(self.commonStrings).NOT_IMPLEMENTED.clone())));
    }
    

    // throws Exception
    pub fn startGameCanvasRunnableInterface(&mut self) {
        let mut threadFactoryUtil: Option<Rc<RefCell<ThreadFactoryUtil>>> = ThreadFactoryUtil::getInstance();
        self.thread = jcall!(threadFactoryUtil).getInstanceGameCanvasRunnable(self.allbinaryGameCanvasRunnableInterface.clone());
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("Thread Priority: ")).appendint(((jcall!(self.thread).getPriority()) as i32)).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("startGameCanvasRunnableInterface"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        jcall!(self.allbinaryGameCanvasRunnableInterface).setThread(self.thread.clone());
        jcall!(threadFactoryUtil).start(self.thread.clone());
    }
    

    // throws Exception
    pub fn stopGameCanvasRunnableInterface(&mut self) {
        jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.gameStrings).STOP_GAME_CANVAS_RUNNABLE_INTERFACE.clone());
        jcall!(GameNotificationEventHandler::getInstance()).removeAllListeners();
        jcall!(ColorChangeEventHandler::getInstance()).removeAllListeners();
        GameEventHandlerUtil::removeAllListeners();
        let mut gameCanvasRunnableInterface: Option<Rc<RefCell<dyn GameCanvasRunnableInterface>>> = self.allbinaryGameCanvasRunnableInterface.clone();
        if !match (&gameCanvasRunnableInterface, &crate::org::allbinary::game::displayable::canvas::NullGameCanvasRunnable::NullGameCanvasRunnable_NULL_GAME_CANVAS_RUNNABLE.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("Set Running False: ")).append(jcall!(gameCanvasRunnableInterface).toString()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.gameStrings).STOP_GAME_CANVAS_RUNNABLE_INTERFACE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            jcall!(gameCanvasRunnableInterface).setRunning(false);
        } else {
            jcall!(self.logUtil).putF(String::from("StopGame - Could Not Stop"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.gameStrings).STOP_GAME_CANVAS_RUNNABLE_INTERFACE.clone());
        }
        jcall!(ThreadUtil::getInstance()).join(self.thread.clone());
        let mut progressCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvasDyn>>> = ProgressCanvasFactory::getInstance();
        if { let java_receiver = self.features.clone(); let java_arguments = (jcall!(MainFeatureFactory::getInstance()).LOAD_ALL.clone(),); let java_result = jcall!(java_receiver).isFeature(java_arguments.0); java_result } {
            jcall!(progressCanvas).addNormalPortion(50, String::from("Stopped Game Runnable"));
        } else {
            jcall!(progressCanvas).addNormalPortion(50, String::from("Stopped Main Runnable"));
        }
        jcall!(self.logUtil).putF(jcall!(self.commonStrings).END.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.gameStrings).STOP_GAME_CANVAS_RUNNABLE_INTERFACE.clone());
    }
    

    pub fn getGameCanvasRunnableInterface(&mut self) -> Option<Rc<RefCell<dyn GameCanvasRunnableInterface>>> {
        return self.allbinaryGameCanvasRunnableInterface.clone();
    }
    

    pub fn setGameCanvasRunnableInterface(&mut self, mut gameCanvasRunnableInterface: Option<Rc<RefCell<dyn GameCanvasRunnableInterface>>>) {
        self.allbinaryGameCanvasRunnableInterface = gameCanvasRunnableInterface.clone();
    }
    

    pub fn createGameLayerManager(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>> {
        let mut gameInfo: Option<Rc<RefCell<GameInfo>>> = GameInfo::new(jcall!(GameTypeFactory::getInstance()).SINGLE_PLAYER.clone(), unsafe { crate::org::allbinary::game::GameMode::GameMode_SERVER.clone() }.clone(), jcall!(PlayerTypesFactory::getInstance()).PLAYER_TYPE_ONE.clone(), 1, 1);
        return AllBinaryGameLayerManager::new(jcall!(self.basicColorFactory).BLACK.clone(), jcall!(self.basicColorFactory).WHITE.clone(), gameInfo.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>);
    }
    

    pub fn getGameOptionsForm(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandFormDyn>>> {
        let mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>> = self.createGameLayerManager();
        return jcall!(GameOptionsFormFactory::getInstance()).init(None /* Java this: a &mut self method has no Rc of itself */, String::from("Game Options"), jcall!(layerManager).getBackgroundBasicColor(), jcall!(layerManager).getForegroundBasicColor());
    }
    

    // throws Exception
    pub fn save(&mut self) {
        jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).SAVE.clone());
        let mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> = self.getCurrentStateHashtable();
        jcall!(GamePersistanceSingleton::getInstance()).save(self.abeClientInformation.clone(), hashtable.clone());
    }
    

    // throws Exception
    pub fn getCurrentStateHashtable(&mut self) -> Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> {
        jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getCurrentStateHashtable"));
        let mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> = jcall!(StdUtil::getInstance()).createHashtable();
        if !match (&self.allbinaryGameCanvasRunnableInterface, &crate::org::allbinary::game::displayable::canvas::NullGameCanvasRunnable::NullGameCanvasRunnable_NULL_GAME_CANVAS_RUNNABLE.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            let mut currentHashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> = jcall!(self.allbinaryGameCanvasRunnableInterface).getCurrentStateHashtable();
            let mut enumeration: Option<Rc<RefCell<dyn Enumeration>>> = jcall!(currentHashtable).keys();
            let mut key: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = None;
            while jcall!(self.enumerationUtil).hasMoreElements(enumeration.clone()) {
                key = jcall!(self.enumerationUtil).nextElement(enumeration.clone());
                jcall!(hashtable).put(key.clone(), jcall!(currentHashtable).get(key.clone()));
            }
        }
        return hashtable.clone();
    }
    

    pub fn setLoadGameForm(&mut self, mut loadGameForm: Option<Rc<RefCell<LoadGameForm>>>) {
        self.loadGameForm = loadGameForm.clone();
    }
    

    pub fn getLoadGameForm(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandFormDyn>>> {
        return self.loadGameForm.clone();
    }
    

    pub fn setResized(&mut self, mut resized: bool) {
        self.resized = resized.clone();
    }
    

    pub fn isResized(&mut self) -> bool {
        return self.resized.clone();
    }
    

    pub fn isDemoLoading(&mut self) -> bool {
        if self.startedBefore {
            return false;
        }
        let mut displayable: Option<Rc<RefCell<Displayable>>> = jcall!(self.getDisplay()).getCurrent();
        if (displayable).is_some() /* instanceof DemoCanvas */ {
            let mut demoCanvas: Option<Rc<RefCell<DemoCanvas>>> = (displayable).clone() /* as DemoCanvas */;
            if jcall!(demoCanvas).isDemoLoading() {
                return true;
            }
        }
        return false;
    }
}
impl std::ops::Deref for GameMidlet {
    type Target = ProgressMidlet;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for GameMidlet {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait GameMidletDyn: crate::org::allbinary::midlet::ProgressMidlet::ProgressMidletDyn {
    fn as_GameMidlet(&self) -> &GameMidlet;
    fn as_GameMidlet_mut(&mut self) -> &mut GameMidlet;
    fn init(&mut self) { GameMidlet::init(self.as_GameMidlet_mut()) }
    fn setDemo(&mut self) { GameMidlet::setDemo(self.as_GameMidlet_mut()) }
    fn createGame(&mut self) { GameMidlet::createGame(self.as_GameMidlet_mut()) }
    fn mediaShutdown(&mut self) { GameMidlet::mediaShutdown(self.as_GameMidlet_mut()) }
    fn stopAll(&mut self) { GameMidlet::stopAll(self.as_GameMidlet_mut()) }
    fn pauseAppBackground(&mut self, mut background: bool) { GameMidlet::pauseAppBackground(self.as_GameMidlet_mut(), background) }
    fn unPauseApp(&mut self) { GameMidlet::unPauseApp(self.as_GameMidlet_mut()) }
    fn unPauseAppBackground(&mut self, mut background: bool) { GameMidlet::unPauseAppBackground(self.as_GameMidlet_mut(), background) }
    fn updateFullScreen(&mut self) { GameMidlet::updateFullScreen(self.as_GameMidlet_mut()) }
    fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { GameMidlet::onEvent(self.as_GameMidlet_mut(), eventObject) }
    fn getInputMappingInstructionsCanvas(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn>>> { GameMidlet::getInputMappingInstructionsCanvas(self.as_GameMidlet_mut()) }
    fn getInputMappingCanvas(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn>>> { GameMidlet::getInputMappingCanvas(self.as_GameMidlet_mut()) }
    fn getAboutCanvas(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn>>> { GameMidlet::getAboutCanvas(self.as_GameMidlet_mut()) }
    fn getHelpPaintable(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::paint::help::HelpPaintable::HelpPaintableDyn>>> { GameMidlet::getHelpPaintable(self.as_GameMidlet_mut()) }
    fn createHighScoresCanvas(&mut self) -> Option<Rc<RefCell<HighScoresCanvas>>> { GameMidlet::createHighScoresCanvas(self.as_GameMidlet_mut()) }
    fn startGameCanvasRunnableInterface(&mut self) { GameMidlet::startGameCanvasRunnableInterface(self.as_GameMidlet_mut()) }
    fn stopGameCanvasRunnableInterface(&mut self) { GameMidlet::stopGameCanvasRunnableInterface(self.as_GameMidlet_mut()) }
    fn getGameCanvasRunnableInterface(&mut self) -> Option<Rc<RefCell<dyn GameCanvasRunnableInterface>>> { GameMidlet::getGameCanvasRunnableInterface(self.as_GameMidlet_mut()) }
    fn setGameCanvasRunnableInterface(&mut self, mut gameCanvasRunnableInterface: Option<Rc<RefCell<dyn GameCanvasRunnableInterface>>>) { GameMidlet::setGameCanvasRunnableInterface(self.as_GameMidlet_mut(), gameCanvasRunnableInterface) }
    fn createGameLayerManager(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>> { GameMidlet::createGameLayerManager(self.as_GameMidlet_mut()) }
    fn getGameOptionsForm(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandFormDyn>>> { GameMidlet::getGameOptionsForm(self.as_GameMidlet_mut()) }
    fn save(&mut self) { GameMidlet::save(self.as_GameMidlet_mut()) }
    fn setLoadGameForm(&mut self, mut loadGameForm: Option<Rc<RefCell<LoadGameForm>>>) { GameMidlet::setLoadGameForm(self.as_GameMidlet_mut(), loadGameForm) }
    fn getLoadGameForm(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandFormDyn>>> { GameMidlet::getLoadGameForm(self.as_GameMidlet_mut()) }
    fn setResized(&mut self, mut resized: bool) { GameMidlet::setResized(self.as_GameMidlet_mut(), resized) }
    fn isResized(&mut self) -> bool { GameMidlet::isResized(self.as_GameMidlet_mut()) }
    fn isDemoLoading(&mut self) -> bool { GameMidlet::isDemoLoading(self.as_GameMidlet_mut()) }
}
impl GameMidletDyn for GameMidlet {
    fn as_GameMidlet(&self) -> &GameMidlet { self }
    fn as_GameMidlet_mut(&mut self) -> &mut GameMidlet { self }
}

impl crate::org::allbinary::midlet::AllBinaryMidlet::AllBinaryMidletDyn for GameMidlet {
    fn as_AllBinaryMidlet(&self) -> &crate::org::allbinary::midlet::AllBinaryMidlet::AllBinaryMidlet { &self.base.base }
    fn as_AllBinaryMidlet_mut(&mut self) -> &mut crate::org::allbinary::midlet::AllBinaryMidlet::AllBinaryMidlet { &mut self.base.base }
    fn startApp(&mut self) { Self::startApp(self) }
    fn pauseApp(&mut self) { Self::pauseApp(self) }
    fn destroyAppInRunnable(&mut self, mut unconditional: bool, mut isProgress: bool) { Self::destroyAppInRunnable(self, unconditional, isProgress) }
    fn destroyApp(&mut self, mut unconditional: bool) { Self::destroyApp(self, unconditional) }
    fn getCurrentStateHashtable(&mut self) -> Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> { Self::getCurrentStateHashtable(self) }
    fn commandAction(&mut self, mut command: Option<Rc<RefCell<Command>>>, mut displayable: Option<Rc<RefCell<Displayable>>>) { Self::commandAction(self, command, displayable) }
}
impl crate::org::allbinary::midlet::ProgressMidlet::ProgressMidletDyn for GameMidlet {
    fn as_ProgressMidlet(&self) -> &crate::org::allbinary::midlet::ProgressMidlet::ProgressMidlet { &self.base }
    fn as_ProgressMidlet_mut(&mut self) -> &mut crate::org::allbinary::midlet::ProgressMidlet::ProgressMidlet { &mut self.base }
}


