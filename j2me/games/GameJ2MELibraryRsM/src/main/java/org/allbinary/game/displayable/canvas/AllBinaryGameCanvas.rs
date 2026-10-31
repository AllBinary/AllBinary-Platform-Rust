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
use crate::Runnable;
use super::*;
// use crate::javax::microedition::lcdui::ChoiceGroup::ChoiceGroup;
// use crate::javax::microedition::lcdui::CommandListener::CommandListener;
// use crate::javax::microedition::lcdui::Font::Font;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
// use crate::javax::microedition::lcdui::Item::Item;
// use crate::javax::microedition::lcdui::NullCommandListener::NullCommandListener;
use crate::org::allbinary::AndroidUtil::AndroidUtil;
use crate::org::allbinary::J2MEUtil::J2MEUtil;
use crate::org::allbinary::business::advertisement::GameAdStateFactory::GameAdStateFactory;
use crate::org::allbinary::canvas::BaseGameStatistics::BaseGameStatistics;
use crate::org::allbinary::canvas::GameStatisticsFactory::GameStatisticsFactory;
use crate::org::allbinary::canvas::Processor::Processor;
use crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvas;
use crate::org::allbinary::debug::DebugFactory::DebugFactory;
use crate::org::allbinary::debug::NoDebug::NoDebug;
use crate::org::allbinary::game::GameAdState::GameAdState;
use crate::org::allbinary::game::GameInfo::GameInfo;
use crate::org::allbinary::game::GameStrings::GameStrings;
use crate::org::allbinary::game::GameTypeFactory::GameTypeFactory;
use crate::org::allbinary::game::Intermission::Intermission;
use crate::org::allbinary::game::IntermissionCompositeInterface::IntermissionCompositeInterface;
use crate::org::allbinary::game::IntermissionEnableListenerInterface::IntermissionEnableListenerInterface;
use crate::org::allbinary::game::IntermissionInterface::IntermissionInterface;
use crate::org::allbinary::game::PlayerTypesFactory::PlayerTypesFactory;
use crate::org::allbinary::game::commands::GameCommandsFactory::GameCommandsFactory;
use crate::org::allbinary::game::configuration::InGameFeatures::InGameFeatures;
use crate::org::allbinary::game::configuration::InGameOptionsForm::InGameOptionsForm;
use crate::org::allbinary::game::configuration::event::ChangedGameFeatureListener::ChangedGameFeatureListener;
use crate::org::allbinary::game::configuration::event::GameInitializedEvent::GameInitializedEvent;
use crate::org::allbinary::game::configuration::event::GameInitializedEventHandler::GameInitializedEventHandler;
use crate::org::allbinary::game::configuration::feature::Features::Features;
use crate::org::allbinary::game::configuration::feature::GameFeatureFactory::GameFeatureFactory;
use crate::org::allbinary::game::configuration::feature::GameFeatureUtil::GameFeatureUtil;
use crate::org::allbinary::game::configuration::feature::InputFeatureFactory::InputFeatureFactory;
use crate::org::allbinary::game::configuration::feature::MainFeatureFactory::MainFeatureFactory;
use crate::org::allbinary::game::configuration::feature::SensorFeatureFactory::SensorFeatureFactory;
use crate::org::allbinary::game::configuration::feature::TouchFeatureFactory::TouchFeatureFactory;
use crate::org::allbinary::game::displayable::GameLevelDisplayChangeEventListenersFactory::GameLevelDisplayChangeEventListenersFactory;
use crate::org::allbinary::game::init::BasicBuildGameInitializerFactory::BasicBuildGameInitializerFactory;
use crate::org::allbinary::game::init::GameInitializationUtil::GameInitializationUtil;
use crate::org::allbinary::game::input::GameInputStrings::GameInputStrings;
use crate::org::allbinary::game::input::InputProcessor::InputProcessor;
use crate::org::allbinary::game::input::NoPlayerGameInput::NoPlayerGameInput;
use crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInput;
use crate::org::allbinary::game::input::event::GameKeyEventHandler::GameKeyEventHandler;
use crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManager;
use crate::org::allbinary::game::layer::SWTUtil::SWTUtil;
use crate::org::allbinary::game::paint::ColorFillBasePaintable::ColorFillBasePaintable;
use crate::org::allbinary::game::paint::ColorFillPaintableFactory::ColorFillPaintableFactory;
use crate::org::allbinary::game::resource::ResourceLoadingLevel::ResourceLoadingLevel;
use crate::org::allbinary::game::resource::ResourceLoadingLevelFactory::ResourceLoadingLevelFactory;
use crate::org::allbinary::game::score::HighScore::HighScore;
use crate::org::allbinary::game::score::HighScoresHelperBaseInterface::HighScoresHelperBaseInterface;
use crate::org::allbinary::game::score::HighScoresFactoryInterface::HighScoresFactoryInterface;
use crate::org::allbinary::game::score::HighScoresPaintable::HighScoresPaintable;
use crate::org::allbinary::game::score::NullHighScoresSingletonFactory::NullHighScoresSingletonFactory;
use crate::org::allbinary::game::score::displayable::HighScoreTextBox::HighScoreTextBox;
use crate::org::allbinary::game::state::GameState::GameState;
use crate::org::allbinary::game::state::GameStateFactory::GameStateFactory;
use crate::org::allbinary::graphics::GraphicsStrings::GraphicsStrings;
use crate::org::allbinary::graphics::Rectangle::Rectangle;
use crate::org::allbinary::graphics::ScreenCapture::ScreenCapture;
use crate::org::allbinary::graphics::ScreenCaptureFactory::ScreenCaptureFactory;
use crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvas;
use crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvasFactory::ProgressCanvasFactory;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::graphics::color::BasicColorSetUtil::BasicColorSetUtil;
use crate::org::allbinary::graphics::displayable::CanvasStrings::CanvasStrings;
use crate::org::allbinary::graphics::displayable::DisplayInfoSingleton::DisplayInfoSingleton;
use crate::org::allbinary::graphics::displayable::GameTickDisplayInfoSingleton::GameTickDisplayInfoSingleton;
use crate::org::allbinary::graphics::displayable::command::MyCommandsFactory::MyCommandsFactory;
use crate::org::allbinary::graphics::displayable::event::DisplayChangeEvent::DisplayChangeEvent;
use crate::org::allbinary::graphics::displayable::event::DisplayChangeEventHandler::DisplayChangeEventHandler;
use crate::org::allbinary::graphics::displayable::event::DisplayChangeEventListener::DisplayChangeEventListener;
use crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessor;
use crate::org::allbinary::graphics::font::UpdateMyFontInterface::UpdateMyFontInterface;
use crate::org::allbinary::graphics::font::UpdateMyFontProcessor::UpdateMyFontProcessor;
use crate::org::allbinary::graphics::form::CommandCurrentSelectionFormFactory::CommandCurrentSelectionFormFactory;
use crate::org::allbinary::graphics::form::FormPaintable::FormPaintable;
use crate::org::allbinary::graphics::form::FormType::FormType;
use crate::org::allbinary::graphics::form::FormTypeFactory::FormTypeFactory;
use crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionForm;
use crate::org::allbinary::graphics::form::ScrollSelectionFormNoneFactory::ScrollSelectionFormNoneFactory;
use crate::org::allbinary::graphics::form::item::CommandTextItemArrayFactory::CommandTextItemArrayFactory;
use crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItem;
use crate::org::allbinary::graphics::opengles::CurrentDisplayableFactory::CurrentDisplayableFactory;
use crate::org::allbinary::graphics::opengles::OpenGLFeatureFactory::OpenGLFeatureFactory;
use crate::org::allbinary::graphics::opengles::OpenGLFeatureUtil::OpenGLFeatureUtil;
use crate::org::allbinary::graphics::opengles::OpenGLThreadUtil::OpenGLThreadUtil;
use crate::org::allbinary::graphics::paint::InitUpdatePaintable::InitUpdatePaintable;
use crate::org::allbinary::graphics::paint::NullInitUpdatePaintable::NullInitUpdatePaintable;
use crate::org::allbinary::graphics::paint::NullPaintable::NullPaintable;
use crate::org::allbinary::graphics::paint::Paintable::Paintable;
use crate::org::allbinary::graphics::paint::PaintableInterface::PaintableInterface;
use crate::org::allbinary::image::opengles::OpenGLImageSpecificFactory::OpenGLImageSpecificFactory;
use crate::org::allbinary::input::gyro::SensorGameUpdateProcessor::SensorGameUpdateProcessor;
use crate::org::allbinary::input::gyro::SingleSensorGameUpdateProcessor::SingleSensorGameUpdateProcessor;
use crate::org::allbinary::input::motion::button::BaseTouchInput::BaseTouchInput;
use crate::org::allbinary::input::motion::button::NoButtonsTouchInputFactory::NoButtonsTouchInputFactory;
use crate::org::allbinary::input::motion::button::TouchButtonFactory::TouchButtonFactory;
use crate::org::allbinary::input::motion::button::TouchButtonsPaintableFactory::TouchButtonsPaintableFactory;
use crate::org::allbinary::input::motion::button::TouchScreenFactory::TouchScreenFactory;
use crate::org::allbinary::input::motion::gesture::observer::BasicMotionGesturesHandler::BasicMotionGesturesHandler;
use crate::org::allbinary::logic::ABSystemWrapper::ABSystemWrapper;
use crate::org::allbinary::logic::communication::log::ForcedLogUtil::ForcedLogUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::system::os::OperatingSystemFactory::OperatingSystemFactory;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject;
use crate::org::allbinary::logic::util::event::EventStrings::EventStrings;
use crate::org::allbinary::media::audio::AllBinaryMediaManager::AllBinaryMediaManager;
use crate::org::allbinary::media::audio::PlayerQueue::PlayerQueue;
use crate::org::allbinary::media::audio::PrimaryPlayerQueueFactory::PrimaryPlayerQueueFactory;
use crate::org::allbinary::media::audio::SecondaryPlayerQueueFactory::SecondaryPlayerQueueFactory;
use crate::org::allbinary::media::audio::SelectSound::SelectSound;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::thread::SecondaryThreadPool::SecondaryThreadPool;
use crate::org::allbinary::time::GameTickTimeDelayHelper::GameTickTimeDelayHelper;
use crate::org::allbinary::time::GameTickTimeDelayHelperFactory::GameTickTimeDelayHelperFactory;
use crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelper;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::util::BasicArrayListUtil::BasicArrayListUtil;
use crate::java::lang::Thread::*;
use super::AllBinaryGameCanvasInterface::AllBinaryGameCanvasInterface;
use super::GameCanvasRunnableInterface::GameCanvasRunnableInterface;
use super::MenuListener::MenuListener;
use super::PopupMenuInterface::PopupMenuInterface;
use super::GameCanvasRunnable::GameCanvasRunnable;
use super::GameCanvasPauseRunnable::GameCanvasPauseRunnable;
use super::FormUtil::FormUtil;
use super::MyFormUtil::MyFormUtil;
use super::EndGameInfo::EndGameInfo;
use super::BasicMenuInputProcessor::BasicMenuInputProcessor;
use super::DemoGameBehavior::DemoGameBehavior;
use super::BaseMenuBehavior::BaseMenuBehavior;
use super::BasicPopupMenuPaintable::BasicPopupMenuPaintable;
use super::PopupMenuInputProcessor::PopupMenuInputProcessor;
use super::GameLimitedCommandTextItemArrayFactory::GameLimitedCommandTextItemArrayFactory;
use super::PopupCommandFormInputProcessor::PopupCommandFormInputProcessor;
use super::GameProcessor::GameProcessor;
use super::CheatGameInputProcessor::CheatGameInputProcessor;
use super::EndGameProcessor::EndGameProcessor;
use super::StartIntermissionProcessor::StartIntermissionProcessor;
use super::EndGamePaintable::EndGamePaintable;
use super::DemoPaintableInterface::DemoPaintableInterface;
use super::GameCanvasInputProcessor::GameCanvasInputProcessor;
use super::FormInputProcessor::FormInputProcessor;
use super::SaveHighScoreRunnable::SaveHighScoreRunnable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AllBinaryGameCanvas {
    pub base: RunnableCanvas,
    pub basicColorFactory: Option<Rc<RefCell<BasicColorFactory>>>,
    pub basicSetColorUtil: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::color::BasicColorSetUtil::BasicColorSetUtilDyn>>>,
    pub touchFeatureFactory: Option<Rc<RefCell<TouchFeatureFactory>>>,
    pub openGLFeatureFactory: Option<Rc<RefCell<OpenGLFeatureFactory>>>,
    pub touchButtonFactory: Option<Rc<RefCell<TouchButtonFactory>>>,
    pub gameAdStateFactory: Option<Rc<RefCell<GameAdStateFactory>>>,
    pub gameStrings: Option<Rc<RefCell<GameStrings>>>,
    pub gameInputStrings: Option<Rc<RefCell<GameInputStrings>>>,
    pub gameStateFactory: Option<Rc<RefCell<GameStateFactory>>>,
    pub gameTickTimeDelayHelper: Option<Rc<RefCell<GameTickTimeDelayHelper>>>,
    pub gameTickDisplayInfoSingleton: Option<Rc<RefCell<GameTickDisplayInfoSingleton>>>,
    pub gameRunnable: Option<Rc<RefCell<GameCanvasRunnable>>>,
    pub gamePauseRunnable: Option<Rc<RefCell<GameCanvasPauseRunnable>>>,
    pub formUtil: Option<Rc<RefCell<FormUtil>>>,
    pub myFormUtil: Option<Rc<RefCell<MyFormUtil>>>,
    pub updateMyFontProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>>,
    pub myFontProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>>,
    pub gameSpecificPaintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>,
    pub sensorGameUpdateProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::input::gyro::SensorGameUpdateProcessor::SensorGameUpdateProcessorDyn>>>,
    pub endGameInfo: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::EndGameInfo::EndGameInfoDyn>>>,
    pub startIntermissionInterface: Option<Rc<RefCell<dyn IntermissionInterface>>>,
    pub endLevelIntermissionInterface: Option<Rc<RefCell<dyn IntermissionInterface>>>,
    pub gameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>,
    pub gameState: Option<Rc<RefCell<GameState>>>,
    pub gameOver: bool,
    pub initialized: bool,
    pub gameStateTimeHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>>,
    pub highScoresHelper: Option<Rc<RefCell<dyn HighScoresHelperBaseInterface>>>,
    pub highScoreSubmitted: bool,
    pub realHighScoresPaintable: Option<Rc<RefCell<HighScoresPaintable>>>,
    pub highScoresPaintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>,
    pub localPlayerGameInputList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub isCheating: bool,
    pub hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>,
    pub isSingleKeyRepeatableProcessing: bool,
    pub gameInitializationInterfaceFactoryInterface: Option<Rc<RefCell<BasicBuildGameInitializerFactory>>>,
    pub touchButtonsPaintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>,
    pub touchPaintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>,
    pub cheatProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInputDyn>>>,
    pub gameInputProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::canvas::Processor::ProcessorDyn>>>,
    pub endGameProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::canvas::Processor::ProcessorDyn>>>,
    pub realEndGameProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::canvas::Processor::ProcessorDyn>>>,
    pub startIntermissionProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::canvas::Processor::ProcessorDyn>>>,
    pub realStartIntermissionProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::canvas::Processor::ProcessorDyn>>>,
    pub endGamePaintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>,
    pub endGameStatePaintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>,
    pub nonBotPaintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>,
    pub intermissionPaintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>,
    pub startIntermissionPaintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::InitUpdatePaintable::InitUpdatePaintableDyn>>>,
    pub mainStateProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::canvas::Processor::ProcessorDyn>>>,
    pub processGameProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::canvas::Processor::ProcessorDyn>>>,
    pub highScoresFactoryInterface: Option<Rc<RefCell<dyn HighScoresFactoryInterface>>>,
    pub startLevel: i32,
    pub mainMenuInputProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::BasicMenuInputProcessor::BasicMenuInputProcessorDyn>>>,
    pub popupMenuInputProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::BasicMenuInputProcessor::BasicMenuInputProcessorDyn>>>,
    pub menuInputProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::BasicMenuInputProcessor::BasicMenuInputProcessorDyn>>>,
    pub menuForm: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionFormDyn>>>,
    pub formPaintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>,
    pub openMenuPaintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>,
    pub menuPaintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>,
    pub currentTouchInputFactory: Option<Rc<RefCell<dyn crate::org::allbinary::input::motion::button::BaseTouchInput::BaseTouchInputDyn>>>,
    pub colorFillPaintable: Option<Rc<RefCell<dyn crate::org::allbinary::game::paint::ColorFillBasePaintable::ColorFillBasePaintableDyn>>>,
    pub baseGameStatistics: Option<Rc<RefCell<dyn crate::org::allbinary::canvas::BaseGameStatistics::BaseGameStatisticsDyn>>>,
    pub primaryPlayerQueue: Option<Rc<RefCell<PlayerQueue>>>,
    pub secondaryPlayerQueue: Option<Rc<RefCell<PlayerQueue>>>,
    pub BUILD_GAME: String,
    pub gameTypeFactory: Option<Rc<RefCell<GameTypeFactory>>>,
    pub screenCapture: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::ScreenCapture::ScreenCaptureDyn>>>,
    pub basicMotionGesturesHandler: Option<Rc<RefCell<BasicMotionGesturesHandler>>>,
    pub gameKeyEventHandler: Option<Rc<RefCell<GameKeyEventHandler>>>,
    pub gameBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::DemoGameBehavior::DemoGameBehaviorDyn>>>,
    pub menuBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::BaseMenuBehavior::BaseMenuBehaviorDyn>>>,
    pub progressPaintable: Option<Rc<RefCell<dyn PaintableInterface>>>,
    pub fontHeightP: i32,
    pub gameCanvasStartListener: Option<Rc<RefCell<dyn DemoPaintableInterface>>>,
    pub rawGameInputProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::InputProcessor::InputProcessorDyn>>>,
    pub rawInputProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::InputProcessor::InputProcessorDyn>>>,
    pub inputProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::InputProcessor::InputProcessorDyn>>>,
    pub YIELD_SLEEP: i64,
}

thread_local! { pub static AllBinaryGameCanvas_id: i32 = 0; }
thread_local! { pub static AllBinaryGameCanvas_TYPE: i32 = 2; }
// impl AllBinaryGameCanvasInterface for AllBinaryGameCanvas with the methods of the trait
// impl GameCanvasRunnableInterface for AllBinaryGameCanvas with the methods of the trait
impl MenuListener for AllBinaryGameCanvas {
    fn open(&mut self) { Self::open(self) }
    fn close(&mut self) { Self::close(self) }
}
impl crate::org::allbinary::game::input::GameKeyEventSourceInterface::GameKeyEventSourceInterface for AllBinaryGameCanvas {
    fn getSourceId(&mut self) -> i32 { Self::getSourceId(self) }
}
impl IntermissionCompositeInterface for AllBinaryGameCanvas {
    fn getStartIntermissionInterface(&mut self) -> Option<Rc<RefCell<dyn IntermissionInterface>>> { Self::getStartIntermissionInterface(self) }
    fn getEndLevelIntermissionInterface(&mut self) -> Option<Rc<RefCell<dyn IntermissionInterface>>> { Self::getEndLevelIntermissionInterface(self) }
}
impl IntermissionEnableListenerInterface for AllBinaryGameCanvas {
    fn notifyIntermission(&mut self, mut enable: bool) { Self::notifyIntermission(self, enable) }
}
impl PopupMenuInterface for AllBinaryGameCanvas {
    fn popupMenu(&mut self) { Self::popupMenu(self) }
    fn closeMenu(&mut self) { Self::closeMenu(self) }
    fn toggleMenu(&mut self) { Self::toggleMenu(self) }
}
impl DisplayChangeEventListener for AllBinaryGameCanvas {
    fn onDisplayChangeEvent(&mut self, mut displayChangeEvent: Option<Rc<RefCell<DisplayChangeEvent>>>) { Self::onDisplayChangeEvent(self, displayChangeEvent) }
}
impl crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface for AllBinaryGameCanvas {
    fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { Self::onEvent(self, eventObject) }
}
impl UpdateMyFontInterface for AllBinaryGameCanvas {
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::updateMeasurement(self, graphics) }
}
impl Runnable for AllBinaryGameCanvas {
    fn run(&mut self) { Self::run(self) }
}

impl AllBinaryGameCanvas {
    

    pub fn new_struct(mut commandListener: Option<Rc<RefCell<CommandListener>>>, mut gameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>, mut highScoresFactoryInterface: Option<Rc<RefCell<dyn HighScoresFactoryInterface>>>, mut gameInitializationInterfaceFactoryInterface: Option<Rc<RefCell<BasicBuildGameInitializerFactory>>>, mut buffered: bool) -> Self {
        let mut this = Self {
            base: RunnableCanvas::new_struct(commandListener.clone(), jcall!(CanvasStrings::getInstance()).EMPTY_CHILD_NAME_LIST.clone(), true),
            basicColorFactory: None,
            basicSetColorUtil: None,
            touchFeatureFactory: None,
            openGLFeatureFactory: None,
            touchButtonFactory: None,
            gameAdStateFactory: None,
            gameStrings: None,
            gameInputStrings: None,
            gameStateFactory: None,
            gameTickTimeDelayHelper: None,
            gameTickDisplayInfoSingleton: None,
            gameRunnable: None,
            gamePauseRunnable: None,
            formUtil: None,
            myFormUtil: None,
            updateMyFontProcessor: None,
            myFontProcessor: None,
            gameSpecificPaintable: None,
            sensorGameUpdateProcessor: None,
            endGameInfo: None,
            startIntermissionInterface: None,
            endLevelIntermissionInterface: None,
            gameLayerManager: None,
            gameState: None,
            gameOver: false,
            initialized: false,
            gameStateTimeHelper: None,
            highScoresHelper: None,
            highScoreSubmitted: false,
            realHighScoresPaintable: None,
            highScoresPaintable: None,
            localPlayerGameInputList: None,
            isCheating: false,
            hashtable: None,
            isSingleKeyRepeatableProcessing: false,
            gameInitializationInterfaceFactoryInterface: None,
            touchButtonsPaintable: None,
            touchPaintable: None,
            cheatProcessor: None,
            gameInputProcessor: None,
            endGameProcessor: None,
            realEndGameProcessor: None,
            startIntermissionProcessor: None,
            realStartIntermissionProcessor: None,
            endGamePaintable: None,
            endGameStatePaintable: None,
            nonBotPaintable: None,
            intermissionPaintable: None,
            startIntermissionPaintable: None,
            mainStateProcessor: None,
            processGameProcessor: None,
            highScoresFactoryInterface: None,
            startLevel: 0,
            mainMenuInputProcessor: None,
            popupMenuInputProcessor: None,
            menuInputProcessor: None,
            menuForm: None,
            formPaintable: None,
            openMenuPaintable: None,
            menuPaintable: None,
            currentTouchInputFactory: None,
            colorFillPaintable: None,
            baseGameStatistics: None,
            primaryPlayerQueue: None,
            secondaryPlayerQueue: None,
            BUILD_GAME: String::new(),
            gameTypeFactory: None,
            screenCapture: None,
            basicMotionGesturesHandler: None,
            gameKeyEventHandler: None,
            gameBehavior: None,
            menuBehavior: None,
            progressPaintable: None,
            fontHeightP: 0,
            gameCanvasStartListener: None,
            rawGameInputProcessor: None,
            rawInputProcessor: None,
            inputProcessor: None,
            YIELD_SLEEP: 0_i64,
        };
        this.basicColorFactory = BasicColorFactory::getInstance();
        this.basicSetColorUtil = BasicColorSetUtil::getInstance();
        this.touchFeatureFactory = TouchFeatureFactory::getInstance();
        this.openGLFeatureFactory = OpenGLFeatureFactory::getInstance();
        this.touchButtonFactory = TouchButtonFactory::getInstance();
        this.gameAdStateFactory = GameAdStateFactory::getInstance();
        this.gameStrings = GameStrings::getInstance();
        this.gameInputStrings = GameInputStrings::getInstance();
        this.gameStateFactory = GameStateFactory::getInstance();
        this.gameTickTimeDelayHelper = GameTickTimeDelayHelperFactory::getInstance();
        this.gameTickDisplayInfoSingleton = GameTickDisplayInfoSingleton::getInstance();
        this.gameRunnable = GameCanvasRunnable::new(this);
        this.gamePauseRunnable = GameCanvasPauseRunnable::new(this);
        this.formUtil = FormUtil::getInstance();
        this.myFormUtil = MyFormUtil::getInstance();
        this.updateMyFontProcessor = UpdateMyFontProcessor::new(this).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>);
        this.myFontProcessor = this.updateMyFontProcessor.clone();
        this.gameSpecificPaintable = NullPaintable::getInstance();
        this.sensorGameUpdateProcessor = SingleSensorGameUpdateProcessor::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::input::gyro::SensorGameUpdateProcessor::SensorGameUpdateProcessorDyn>>);
        this.endGameInfo = EndGameInfo::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::EndGameInfo::EndGameInfoDyn>>);
        this.startIntermissionInterface = Intermission::new().map(|reference| reference as Rc<RefCell<dyn IntermissionInterface>>);
        this.endLevelIntermissionInterface = Intermission::new().map(|reference| reference as Rc<RefCell<dyn IntermissionInterface>>);
        this.gameLayerManager = AllBinaryGameLayerManager::getNullInstance();
        this.gameState = jcall!(this.gameStateFactory).NO_GAME_STATE.clone();
        this.gameStateTimeHelper = TimeDelayHelper::new(0).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>);
        this.realHighScoresPaintable = HighScoresPaintable::new();
        this.highScoresPaintable = NullPaintable::getInstance();
        this.localPlayerGameInputList = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.hashtable = jcall!(this.stdUtil).NULL_TABLE.clone();
        this.gameInitializationInterfaceFactoryInterface = unsafe { crate::org::allbinary::game::init::BasicBuildGameInitializerFactory::BasicBuildGameInitializerFactory_NULL_BASE_BUILD_GMAE_INITIALIZER_FACTORY.clone() }.clone();
        this.touchButtonsPaintable = NullPaintable::getInstance();
        this.touchPaintable = NullPaintable::getInstance();
        this.cheatProcessor = NoPlayerGameInput::getInstance();
        this.gameInputProcessor = Processor::getInstance();
        this.endGameProcessor = Processor::getInstance();
        this.realEndGameProcessor = Processor::getInstance();
        this.startIntermissionProcessor = Processor::getInstance();
        this.realStartIntermissionProcessor = Processor::getInstance();
        this.endGamePaintable = NullPaintable::getInstance();
        this.endGameStatePaintable = NullPaintable::getInstance();
        this.nonBotPaintable = NullPaintable::getInstance();
        this.intermissionPaintable = NullPaintable::getInstance();
        this.startIntermissionPaintable = NullInitUpdatePaintable::getInstance();
        this.mainStateProcessor = Processor::getInstance();
        this.processGameProcessor = Processor::getInstance();
        this.mainMenuInputProcessor = NoMenuInputProcessor::NoMenuInputProcessor::getInstance();
        this.popupMenuInputProcessor = NoMenuInputProcessor::NoMenuInputProcessor::getInstance();
        this.menuInputProcessor = NoMenuInputProcessor::NoMenuInputProcessor::getInstance();
        this.menuForm = ScrollSelectionForm::getNullScrollSelectionForm();
        this.formPaintable = NullPaintable::getInstance();
        this.openMenuPaintable = NullPaintable::getInstance();
        this.menuPaintable = NullPaintable::getInstance();
        this.currentTouchInputFactory = NoButtonsTouchInputFactory::getInstance();
        this.colorFillPaintable = jcall!(ColorFillPaintableFactory::getInstance()).getInstance2(jcall!(this.basicColorFactory).BLACK.clone(), true);
        this.baseGameStatistics = GameStatisticsFactory::getInstance();
        this.primaryPlayerQueue = PrimaryPlayerQueueFactory::getInstance();
        this.secondaryPlayerQueue = SecondaryPlayerQueueFactory::getInstance();
        this.BUILD_GAME = String::from("buildGame");
        this.gameTypeFactory = GameTypeFactory::getInstance();
        this.screenCapture = ScreenCaptureFactory::getInstance();
        this.basicMotionGesturesHandler = BasicMotionGesturesHandler::getInstance();
        this.gameKeyEventHandler = GameKeyEventHandler::getInstance();
        this.progressPaintable = ProgressCanvasFactory::getLazyInstance();
        this.gameCanvasStartListener = NullDemoPaintable_NULL_DEMO_PAINTABLE.with(Clone::clone).clone();
        this.rawGameInputProcessor = GameCanvasInputProcessor::new(this).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::InputProcessor::InputProcessorDyn>>);
        this.rawInputProcessor = FormInputProcessor::new(this).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::InputProcessor::InputProcessorDyn>>);
        this.inputProcessor = this.getRawGameInputProcessor();
        this.YIELD_SLEEP = 100_i64;
        // super(...) is in the struct literal above
        this.highScoresHelper = jcall!(highScoresFactoryInterface).createHighScoresHelper();
        this.gameInitializationInterfaceFactoryInterface = gameInitializationInterfaceFactoryInterface.clone();
        this.init(gameLayerManager.clone(), buffered.clone());
        this.highScoresFactoryInterface = highScoresFactoryInterface.clone();
        if match (&jcall!(jcall!(this.gameLayerManager).getGameInfo()).getGameType(), &jcall!(this.gameTypeFactory).BOT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } || match (&jcall!(jcall!(this.gameLayerManager).getGameInfo()).getGameType(), &jcall!(this.gameTypeFactory).NONE) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            this.gameBehavior = DemoGameBehavior::getInstance();
            this.menuBehavior = BaseMenuBehavior::getInstance();
        } else {
            this.gameBehavior = BaseGameBehavior::BaseGameBehavior::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::DemoGameBehavior::DemoGameBehaviorDyn>>);
            this.menuBehavior = this.getInGameMenuBehavior();
        }
        this.initPopupMenu();
        this.initMenu();
        this.initSpecialPaint();
        this
    }

    pub fn new(mut commandListener: Option<Rc<RefCell<CommandListener>>>, mut gameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>, mut highScoresFactoryInterface: Option<Rc<RefCell<dyn HighScoresFactoryInterface>>>, mut gameInitializationInterfaceFactoryInterface: Option<Rc<RefCell<BasicBuildGameInitializerFactory>>>, mut buffered: bool) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(commandListener, gameLayerManager, highScoresFactoryInterface, gameInitializationInterfaceFactoryInterface, buffered))));
            jcall!(DisplayChangeEventHandler::getInstance()).addListenerInterface((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()));
        this
    }
    

    pub fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut font: Option<Rc<RefCell<Font>>> = jcall!(graphics).getFont();
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(self.commonStrings).START.clone()).append(jcall!(self.canvasStrings).FD_WIDTH.clone()).appendint(((MyFontProcessor::defaultCharWidth(font.clone())) as i32)).append(jcall!(self.canvasStrings).FD_HEIGHT.clone()).appendint(((jcall!(font).getHeight()) as i32)).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.canvasStrings).UPDATE_MEASUREMENT.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            self.fontHeightP = jcall!(font).getHeight();
            jcall!(self.myFormUtil).updateMeasurement(graphics.clone());
            let mut popupMenuRectangle: Option<Rc<RefCell<Rectangle>>> = jcall!(self.myFormUtil).getPopupMenuRectangle();
            let mut basicPopupMenuPaintable: Option<Rc<RefCell<BasicPopupMenuPaintable>>> = ((self.getOpenMenuPaintable()).clone() /* as BasicPopupMenuPaintable */);
            jcall!(basicPopupMenuPaintable).init(popupMenuRectangle.clone());
            if !match (&self.getPopupMenuInputProcessor(), &NoMenuInputProcessor::NoMenuInputProcessor::getInstance()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                let mut popupMenuInputProcessor: Option<Rc<RefCell<PopupMenuInputProcessor>>> = ((self.getPopupMenuInputProcessor()).clone() /* as PopupMenuInputProcessor */);
                jcall!(popupMenuInputProcessor).as_BasicMenuInputProcessor_mut().init(popupMenuRectangle.clone());
            }
            self.myFontProcessor = MyFontProcessor::getInstance();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.canvasStrings).ON_DISPLAY_CHANGE_EVENT.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn getInGameMenuBehavior(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::BaseMenuBehavior::BaseMenuBehaviorDyn>>> {
        return InGameMenuBehavior::InGameMenuBehavior::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::BaseMenuBehavior::BaseMenuBehaviorDyn>>);
    }
    

    pub fn setCurrentThread(&mut self) {
        if J2MEUtil::isHTML() {
            self.base.setCurrentThreadFake();
        } else {
            self.base.setCurrentThread();
        }
    }
    

    pub fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) {
        ForcedLogUtil::log(jcall!(EventStrings::getInstance()).PERFORMANCE_MESSAGE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())));
    }
    

    pub fn onDisplayChangeEvent(&mut self, mut displayChangeEvent: Option<Rc<RefCell<DisplayChangeEvent>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(self.commonStrings).START.clone()).append(jcall!(DisplayInfoSingleton::getInstance()).toString()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.canvasStrings).UPDATE_MEASUREMENT.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            self.myFontProcessor = self.updateMyFontProcessor.clone();
            jcall!(self.menuBehavior).onDisplayChangeEvent(None /* Java this: a &mut self method has no Rc of itself */, displayChangeEvent.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.canvasStrings).ON_DISPLAY_CHANGE_EVENT.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    // throws Exception
    pub fn updateMenuFromEvent(&mut self, mut displayChangeEvent: Option<Rc<RefCell<DisplayChangeEvent>>>) {
        let mut formType: Option<Rc<RefCell<FormType>>> = jcall!(FormTypeFactory::getInstance()).getFormType();
        let mut rectangle: Option<Rc<RefCell<Rectangle>>> = jcall!(self.formUtil).createFormRectangle();
        jcall!(self.menuForm).init(rectangle.clone(), formType.clone());
        if !match (&self.currentTouchInputFactory, &NoButtonsTouchInputFactory::getInstance()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            { let java_receiver = self.touchButtonFactory.clone(); let java_arguments = (self.isPaused(), jcall!(self.currentTouchInputFactory).getList(),); jcall!(java_receiver).toggle(java_arguments.0, java_arguments.1); };
        }
        self.myFontProcessor = self.updateMyFontProcessor.clone();
    }
    

    // throws Exception
    pub fn processSleep(&mut self) {
        self.base.processSleep();
        if AllBinaryMediaManager::update() {
            if !jcall!(self.primaryPlayerQueue).process() {
                jcall!(self.secondaryPlayerQueue).process();
            }
        }
        if jcall!(self.menuInputProcessor).processInputList() != -1 {
            jcall!(self.runnableCanvasRefreshHelper).process();
        }
    }
    

    // throws Exception
    pub fn initPopupMenu(&mut self) {
        if match (&self.popupMenuInputProcessor, &NoMenuInputProcessor::NoMenuInputProcessor::getInstance()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            let mut features: Option<Rc<RefCell<Features>>> = Features::getInstance();
            let mut popupMenuRectangle: Option<Rc<RefCell<Rectangle>>> = jcall!(self.myFormUtil).getPopupMenuRectangle();
            if jcall!(features).isFeature(jcall!(self.touchFeatureFactory).TOUCH_ENABLED.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
                { let java_arguments = (BasicPopupMenuPaintable::new(popupMenuRectangle.clone(), jcall!(self.gameLayerManager).getBackgroundBasicColor(), jcall!(self.gameLayerManager).getForegroundBasicColor()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>),); self.setOpenMenuPaintable(java_arguments.0) };
                self.setPopupMenuInputProcessor(PopupMenuInputProcessor::new(BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayListD::BasicArrayListDDyn>>), -1, None /* Java this: a &mut self method has no Rc of itself */, popupMenuRectangle.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::BasicMenuInputProcessor::BasicMenuInputProcessorDyn>>));
            }
        } else {
        }
    }
    

    pub fn initMenu(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.menuBehavior).initMenu(None /* Java this: a &mut self method has no Rc of itself */);
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("initMenu"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    // throws Exception
    pub fn initMenu2(&mut self) {
        self.closeMenu();
        let mut formType: Option<Rc<RefCell<FormType>>> = jcall!(FormTypeFactory::getInstance()).getFormType();
        let mut gameLimitedCommandTextItemArrayFactory: Option<Rc<RefCell<GameLimitedCommandTextItemArrayFactory>>> = GameLimitedCommandTextItemArrayFactory::getInstance();
        let mut commandTextItemArrayFactory: Option<Rc<RefCell<CommandTextItemArrayFactory>>> = jcall!(gameLimitedCommandTextItemArrayFactory).getCommandTextItemArrayFactory();
        let mut items: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>> = jcall!(commandTextItemArrayFactory).getInstance(crate::java_into_object(self.getCommandStack()), jcall!(self.gameLayerManager).getBackgroundBasicColor(), jcall!(self.gameLayerManager).getForegroundBasicColor());
        let mut rectangle: Option<Rc<RefCell<Rectangle>>> = jcall!(self.formUtil).createFormRectangle();
        { let java_arguments = (CommandCurrentSelectionFormFactory::getInstance(jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), items.clone(), rectangle.clone(), formType.clone(), 25, false, jcall!(self.gameLayerManager).getBackgroundBasicColor(), jcall!(self.gameLayerManager).getForegroundBasicColor()),); self.setMenuForm(java_arguments.0) };
        let mut scrollSelectionForm: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionFormDyn>>> = self.getMenuForm();
        let mut features: Option<Rc<RefCell<Features>>> = Features::getInstance();
        if jcall!(features).isFeature(jcall!(self.touchFeatureFactory).TOUCH_ENABLED.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
            self.mainMenuInputProcessor = PopupCommandFormInputProcessor::new(BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayListD::BasicArrayListDDyn>>), -1, None /* Java this: a &mut self method has no Rc of itself */, scrollSelectionForm.clone(), (self.getPopupMenuInputProcessor()).clone() /* as PopupMenuInputProcessor */).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::BasicMenuInputProcessor::BasicMenuInputProcessorDyn>>);
        } else {
        }
        { let java_arguments = (self.getPopupMenuInputProcessor(),); self.setMenuInputProcessor(java_arguments.0) };
        if !match (&scrollSelectionForm, &ScrollSelectionFormNoneFactory::getInstance()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            self.setFormPaintable(FormPaintable::new(scrollSelectionForm.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>));
        } else {
        }
        self.closeMenu();
    }
    

    pub fn updateMenu(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.menuBehavior).updateMenu(None /* Java this: a &mut self method has no Rc of itself */);
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("initMenu"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    // throws Exception
    pub fn updateMenu2(&mut self) {
        let mut scrollSelectionForm: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionFormDyn>>> = self.getMenuForm();
        jcall!(scrollSelectionForm).as_ScrollSelectionForm_mut().deleteAll();
        let mut gameLimitedCommandTextItemArrayFactory: Option<Rc<RefCell<GameLimitedCommandTextItemArrayFactory>>> = GameLimitedCommandTextItemArrayFactory::getInstance();
        let mut commandTextItemArrayFactory: Option<Rc<RefCell<CommandTextItemArrayFactory>>> = jcall!(gameLimitedCommandTextItemArrayFactory).getCommandTextItemArrayFactory();
        let mut items: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>> = jcall!(commandTextItemArrayFactory).getInstance(crate::java_into_object(self.getCommandStack()), jcall!(self.gameLayerManager).getBackgroundBasicColor(), jcall!(self.gameLayerManager).getForegroundBasicColor());
        let mut size: i32 = (items.len() as i32).clone();
        for index in (0..size) {
            jcall!(scrollSelectionForm).append(items[(index) as usize].clone());
        }
        let mut formType: Option<Rc<RefCell<FormType>>> = jcall!(FormTypeFactory::getInstance()).getFormType();
        let mut rectangle: Option<Rc<RefCell<Rectangle>>> = jcall!(self.formUtil).createFormRectangle();
        jcall!(scrollSelectionForm).init(rectangle.clone(), formType.clone());
    }
    

    // synchronized
    pub fn pause(&mut self) {
        jcall!(self.gameBehavior).pause(None /* Java this: a &mut self method has no Rc of itself */);
        self.closeMenu();
        self.base.pause();
        { let java_receiver = self.touchButtonFactory.clone(); let java_arguments = (self.isPaused(), jcall!(BasicArrayListUtil::getInstance()).getImmutableInstance(),); jcall!(java_receiver).toggle(java_arguments.0, java_arguments.1); };
        jcall!(self.tsUtil).gc();
    }
    

    // synchronized
    pub fn unPause(&mut self) {
        jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.gameStrings).UNPAUSE.clone());
        self.closeMenu();
        jcall!(self.tsUtil).gc();
        self.base.unPause();
        { let java_receiver = self.touchButtonFactory.clone(); let java_arguments = (self.isPaused(), jcall!(BasicArrayListUtil::getInstance()).getImmutableInstance(),); jcall!(java_receiver).toggle(java_arguments.0, java_arguments.1); };
        jcall!(self.gameBehavior).unPause(None /* Java this: a &mut self method has no Rc of itself */);
    }
    

    pub fn isPausable(&mut self) -> bool {
        if match (&jcall!(CurrentDisplayableFactory::getInstance()).getUsedRunnable(), &NullWaitGameRunnable::NullWaitGameRunnable::getInstance()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return true;
        } else {
            return false;
        }
    }
    

    // throws Exception
    pub fn popupMenu(&mut self) {
        jcall!(self.menuBehavior).popupMenu(None /* Java this: a &mut self method has no Rc of itself */);
    }
    

    // throws Exception
    pub fn popupMenu2(&mut self) {
        { let java_receiver = self.primaryPlayerQueue.clone(); let java_arguments = (SelectSound::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>),); jcall!(java_receiver).add(java_arguments.0); };
        { let java_arguments = (self.getFormPaintable(),); self.setMenuPaintable(java_arguments.0) };
        self.setMenuInputProcessor(self.mainMenuInputProcessor.clone());
        jcall!(self.basicMotionGesturesHandler).addListenerInterface(self.mainMenuInputProcessor.clone());
        jcall!(self.gameKeyEventHandler).addListener(self.mainMenuInputProcessor.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface>>));
    }
    

    // throws Exception
    pub fn toggleMenu(&mut self) {
        jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.gameStrings).TOGGLE_MENU.clone());
        if match (&self.getMenuPaintable(), &self.getOpenMenuPaintable()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            self.pause();
            self.updateMenu();
            self.popupMenu();
        } else if match (&self.getMenuPaintable(), &self.getFormPaintable()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            { let java_receiver = self.primaryPlayerQueue.clone(); let java_arguments = (SelectSound::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>),); jcall!(java_receiver).add(java_arguments.0); };
            self.unPause();
        }
    }
    

    pub fn closeMenu(&mut self) {
        jcall!(self.menuBehavior).closeMenu(None /* Java this: a &mut self method has no Rc of itself */);
    }
    

    pub fn closeMenu2(&mut self) {
        { let java_arguments = (self.getOpenMenuPaintable(),); self.setMenuPaintable(java_arguments.0) };
        jcall!(self.basicMotionGesturesHandler).removeListener(self.mainMenuInputProcessor.clone());
        jcall!(self.gameKeyEventHandler).removeListener(self.mainMenuInputProcessor.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface>>));
        { let java_arguments = (self.getPopupMenuInputProcessor(),); self.setMenuInputProcessor(java_arguments.0) };
    }
    

    pub fn open(&mut self) {
        jcall!(self.basicMotionGesturesHandler).addListenerInterface(self.menuInputProcessor.clone());
        jcall!(self.gameKeyEventHandler).addListener(self.menuInputProcessor.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface>>));
    }
    

    pub fn close(&mut self) {
        jcall!(self.basicMotionGesturesHandler).removeListener(self.menuInputProcessor.clone());
        jcall!(self.gameKeyEventHandler).removeListener(self.menuInputProcessor.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface>>));
        jcall!(self.primaryPlayerQueue).clear();
        jcall!(self.secondaryPlayerQueue).clear();
    }
    

    // throws Exception
    pub fn processorInit(&mut self) {
        { let java_arguments = (Processor::getInstance(),); self.setMainStateProcessor(java_arguments.0) };
        self.setProcessGameProcessorInit();
    }
    

    pub fn setProcessGameProcessorInit(&mut self) {
        self.setProcessGameProcessor(GameProcessor::new(None /* Java this: a &mut self method has no Rc of itself */).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::canvas::Processor::ProcessorDyn>>));
    }
    

    pub fn initSpecialPaint(&mut self) {
        jcall!(self.menuBehavior).initSpecialPaint(None /* Java this: a &mut self method has no Rc of itself */);
    }
    

    // throws Exception
    pub fn init(&mut self, mut gameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>, mut buffered: bool) {
        jcall!(self.gameStateTimeHelper).setStartTimeTNT();
        self.setHighScoreSubmitted(false);
        self.setLayerManager(gameLayerManager.clone());
        if !buffered {
            let mut BUFF_MESSAGE: String = String::from("XXX Not Buffering Causes Concurrency Issues XXX");
            jcall!(self.logUtil).putF(BUFF_MESSAGE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).CONSTRUCTOR.clone());
        } else {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Buffering is disabled"))));
        }
        { let java_arguments = (Processor::getInstance(),); self.setGameInputProcessor(java_arguments.0) };
        let mut features: Option<Rc<RefCell<Features>>> = Features::getInstance();
        if jcall!(features).isFeature(jcall!(GameFeatureFactory::getInstance()).CHEATING.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
            self.isCheating = true;
            self.cheatProcessor = CheatGameInputProcessor::new(None /* Java this: a &mut self method has no Rc of itself */).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInputDyn>>);
            jcall!(self.gameKeyEventHandler).addListener(self.cheatProcessor.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface>>));
        } else {
            self.isCheating = false;
            self.cheatProcessor = NoPlayerGameInput::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInputDyn>>);
        }
        self.realEndGameProcessor = EndGameProcessor::new(None /* Java this: a &mut self method has no Rc of itself */).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::canvas::Processor::ProcessorDyn>>);
        { let java_arguments = (Processor::getInstance(),); self.setEndGameProcessor(java_arguments.0) };
        self.realStartIntermissionProcessor = StartIntermissionProcessor::new(None /* Java this: a &mut self method has no Rc of itself */).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::canvas::Processor::ProcessorDyn>>);
        self.startIntermissionProcessor = Processor::getInstance();
        self.setEndGameStatePaintable(EndGamePaintable::new(None /* Java this: a &mut self method has no Rc of itself */).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>));
        { let java_arguments = (NullPaintable::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>),); self.setEndGamePaintable(java_arguments.0) };
        { let java_arguments = (NullPaintable::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>),); self.setIntermissionPaintable(java_arguments.0) };
        jcall!(self.getStartIntermissionInterface()).setListener(None /* Java this: a &mut self method has no Rc of itself */);
    }
    

    pub fn notifyIntermission(&mut self, mut enable: bool) {
        if enable {
            self.startIntermissionProcessor = self.realStartIntermissionProcessor.clone();
        } else {
            self.startIntermissionProcessor = Processor::getInstance();
        }
        if enable {
            { let java_arguments = (self.getStartIntermissionPaintable().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>),); self.setIntermissionPaintable(java_arguments.0) };
        } else {
            { let java_arguments = (NullPaintable::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>),); self.setIntermissionPaintable(java_arguments.0) };
        }
    }
    

    // throws Exception
    pub fn mediaInit(&mut self) {
        ForcedLogUtil::log(jcall!(self.commonStrings).NOT_IMPLEMENTED.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())));
    }
    

    // throws Exception
    // synchronized
    pub fn initConfigurable(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>) {
        let mut progressCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvasDyn>>> = ProgressCanvasFactory::getInstance();
        jcall!(progressCanvas).addNormalPortion(50, String::from("Setting Configurables"));
        jcall!(GameInitializationUtil::getInstance()).initGame(abeClientInformation.clone(), None /* Java this: a &mut self method has no Rc of itself */, self.gameInitializationInterfaceFactoryInterface.clone());
        let mut gameFeatureFactory: Option<Rc<RefCell<GameFeatureFactory>>> = GameFeatureFactory::getInstance();
        let mut changedGameFeatureListener: Option<Rc<RefCell<ChangedGameFeatureListener>>> = ChangedGameFeatureListener::getInstance();
        if jcall!(changedGameFeatureListener).isChangedFeature(jcall!(gameFeatureFactory).SOUND.clone()) {
            let mut features: Option<Rc<RefCell<Features>>> = Features::getInstance();
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("Sound Changing To: ")).appendboolean(jcall!(features).isFeature(jcall!(gameFeatureFactory).SOUND.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>))).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("initConfigurable"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            self.mediaInit();
            jcall!(changedGameFeatureListener).remove(jcall!(gameFeatureFactory).SOUND.clone());
        }
        let mut sensorFeatureFactory: Option<Rc<RefCell<SensorFeatureFactory>>> = SensorFeatureFactory::getInstance();
        if jcall!(changedGameFeatureListener).isChangedFeature(jcall!(sensorFeatureFactory).ORIENTATION_SENSORS.clone()) || jcall!(changedGameFeatureListener).isChangedFeature(jcall!(sensorFeatureFactory).NO_ORIENTATION.clone()) || jcall!(changedGameFeatureListener).isChangedFeature(jcall!(sensorFeatureFactory).SIMULATED_ORIENTATION_SENSORS.clone()) {
            jcall!(changedGameFeatureListener).remove(jcall!(sensorFeatureFactory).NO_ORIENTATION.clone());
            jcall!(changedGameFeatureListener).remove(jcall!(sensorFeatureFactory).ORIENTATION_SENSORS.clone());
            jcall!(changedGameFeatureListener).remove(jcall!(sensorFeatureFactory).SIMULATED_ORIENTATION_SENSORS.clone());
        }
        let mut resourceLoadingLevelFactory: Option<Rc<RefCell<ResourceLoadingLevelFactory>>> = ResourceLoadingLevelFactory::getInstance();
        self.loadResourceLoadingLevel(jcall!(resourceLoadingLevelFactory).LOAD_GAME.clone());
    }
    

    // throws Exception
    pub fn initApp(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>) {
        self.initConfigurable(abeClientInformation.clone());
        self.processorInit();
        jcall!(self.sensorGameUpdateProcessor).process(crate::java_into_object(self.gameLayerManager.clone()));
        jcall!(self.sensorGameUpdateProcessor).sendNotifications(crate::java_into_object(self.gameLayerManager.clone()));
        self.initTouch();
    }
    

    // throws Exception
    pub fn initTouch(&mut self) {
        let mut gameInitializedEvent: Option<Rc<RefCell<GameInitializedEvent>>> = jcall!(GameInitializationUtil::getInstance()).EVENT.clone();
        let mut resourceLoadingLevelFactory: Option<Rc<RefCell<ResourceLoadingLevelFactory>>> = ResourceLoadingLevelFactory::getInstance();
        jcall!(gameInitializedEvent).setResourceLoadingLevel(jcall!(resourceLoadingLevelFactory).LOAD_TOUCH.clone());
        jcall!(GameInitializedEventHandler::getInstance()).fireEvent(gameInitializedEvent.clone());
        jcall!(self.touchButtonFactory).defaultList();
        self.updateTouch();
        self.postInitTouch();
    }
    

    // throws Exception
    pub fn updateCurrentTouchInputFactory(&mut self, mut nextTouchInput: Option<Rc<RefCell<dyn crate::org::allbinary::input::motion::button::BaseTouchInput::BaseTouchInputDyn>>>) {
        if !match (&nextTouchInput, &self.currentTouchInputFactory) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            self.currentTouchInputFactory = nextTouchInput.clone();
            PreLogUtil::put(jcall!(self.currentTouchInputFactory).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.gameInputStrings).UDPATE_CURRENT_TOUCH_INPUT_FACTORY.clone());
            { let java_receiver = self.touchButtonFactory.clone(); let java_arguments = (self.isPaused(), jcall!(self.currentTouchInputFactory).getList(),); jcall!(java_receiver).toggle(java_arguments.0, java_arguments.1); };
        }
    }
    

    // throws Exception
    pub fn updateTouch(&mut self) {
        jcall!(self.gameBehavior).updateTouch(None /* Java this: a &mut self method has no Rc of itself */);
    }
    

    // throws Exception
    pub fn updateTouch2(&mut self) {
        let mut features: Option<Rc<RefCell<Features>>> = Features::getInstance();
        if jcall!(features).isFeature(jcall!(self.touchFeatureFactory).AUTO_HIDE_SHOW_SCREEN_BUTTONS.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
            if jcall!(jcall!(self.gameLayerManager).getGameInfo()).getCurrentLevel() - self.getStartLevel() == 1 {
                { let java_arguments = (NullPaintable::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>),); self.setTouchPaintableP(java_arguments.0) };
            }
        }
    }
    

    // throws Exception
    pub fn postInitTouch(&mut self) {
        { let java_arguments = (TouchButtonsPaintableFactory::getInstance(jcall!(self.gameLayerManager).getForegroundBasicColor()),); self.setTouchButtonsPaintable(java_arguments.0) };
        self.updateScreenButtonPaintable();
    }
    

    pub fn initCommands(&mut self, mut cmdListener: Option<Rc<RefCell<CommandListener>>>) {
        self.removeAllCommands();
        self.addCommands();
        self.setCommandListener(cmdListener.clone());
    }
    

    pub fn addCommands(&mut self) {
        let mut gameCommandsFactory: Option<Rc<RefCell<GameCommandsFactory>>> = GameCommandsFactory::getInstance();
        let mut myCommandsFactory: Option<Rc<RefCell<MyCommandsFactory>>> = MyCommandsFactory::getInstance();
        if !match (&DebugFactory::getInstance(), &NoDebug::getInstance()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            self.addCommand(jcall!(gameCommandsFactory).START_TRACE.clone());
        }
        self.addCommand(jcall!(gameCommandsFactory).RESTART_COMMAND.clone());
        self.addCommand(jcall!(myCommandsFactory).PAUSE_COMMAND.clone());
        self.addCommand(jcall!(gameCommandsFactory).QUIT_COMMAND.clone());
        let mut isOverScan: bool = jcall!(jcall!(OperatingSystemFactory::getInstance()).getOperatingSystemInstance()).isOverScan();
        if J2MEUtil::isHTML() {
        } else if unsafe { crate::org::allbinary::game::layer::SWTUtil::SWTUtil_isSWT.clone() } {
        } else if !isOverScan {
            if jcall!(TouchScreenFactory::getInstance()).isTouch() && jcall!(InGameFeatures::new()).isAny() {
                self.addCommand(crate::org::allbinary::game::configuration::InGameOptionsForm::InGameOptionsForm_DISPLAY.with(Clone::clone).clone());
            }
            self.addCommand(jcall!(gameCommandsFactory).SAVE.clone());
            self.addCommand(jcall!(gameCommandsFactory).DISPLAY_LOAD_FORM.clone());
        }
    }
    

    pub fn itemStateChanged(&mut self, mut item: Option<Rc<RefCell<Item>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut itemLabel: String = jcall!(item).getLabel();
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(self.commonLabels).ITEM_LABEL.clone()).append(itemLabel.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("itemStateChanged"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            if (item).is_some() /* instanceof ChoiceGroup */ {
                let mut gameFeatureUtil: Option<Rc<RefCell<GameFeatureUtil>>> = GameFeatureUtil::getInstance();
                if jcall!(gameFeatureUtil).isExclusive(itemLabel.clone()) {
                    jcall!(gameFeatureUtil).updateExclusiveForChoiceGroup((item).clone() /* as ChoiceGroup */);
                } else {
                    jcall!(gameFeatureUtil).updateMultiple((item).clone() /* as ChoiceGroup */);
                }
            }
            jcall!(self.sensorGameUpdateProcessor).process(crate::java_into_object(self.gameLayerManager.clone()));
            self.updateTouch();
            self.updateScreenButtonPaintable();
            jcall!(self.sensorGameUpdateProcessor).sendNotifications(crate::java_into_object(self.gameLayerManager.clone()));
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("itemStateChanged"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    // throws Exception
    pub fn updateScreenButtonPaintable(&mut self) {
        jcall!(self.gameBehavior).updateScreenButtonPaintable(None /* Java this: a &mut self method has no Rc of itself */);
    }
    

    pub fn updateScreenButtonPaintable2(&mut self) {
        let mut features: Option<Rc<RefCell<Features>>> = Features::getInstance();
        if jcall!(features).isFeature(jcall!(self.touchFeatureFactory).AUTO_HIDE_SHOW_SCREEN_BUTTONS.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
            { let java_arguments = (self.getTouchButtonsPaintable(),); self.setTouchPaintableP(java_arguments.0) };
            { let java_arguments = (jcall!(jcall!(self.gameLayerManager).getGameInfo()).getCurrentLevel(),); self.setStartLevel(java_arguments.0) };
        } else if jcall!(features).isFeature(jcall!(self.touchFeatureFactory).SHOW_SCREEN_BUTTONS.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
            { let java_arguments = (self.getTouchButtonsPaintable(),); self.setTouchPaintableP(java_arguments.0) };
        } else if jcall!(features).isFeature(jcall!(self.touchFeatureFactory).HIDE_SCREEN_BUTTONS.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
            { let java_arguments = (NullPaintable::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>),); self.setTouchPaintableP(java_arguments.0) };
        } else {
            { let java_arguments = (NullPaintable::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>),); self.setTouchPaintableP(java_arguments.0) };
        }
    }
    

    pub fn getLayerManager(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>> {
        return self.gameLayerManager.clone();
    }
    

    pub fn setLayerManager(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>) {
        self.gameLayerManager = layerManager.clone();
    }
    

    // synchronized
    pub fn isGameOver(&mut self) -> bool {
        return self.gameOver.clone();
    }
    

    // synchronized
    pub fn setGameOver(&mut self, mut gameOver: bool) {
        self.gameOver = gameOver.clone();
    }
    

    // throws Exception
    pub fn processGameOver(&mut self) {
        PreLogUtil::put(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.gameStrings).SET_GAME_OVER.clone());
        self.setGameOver(true);
        self.removePauseCommand();
        self.setGameState(jcall!(self.gameStateFactory).SHOW_END_RESULT_GAME_STATE.clone());
        { let java_arguments = (self.getEndGameStatePaintable(),); self.setEndGamePaintable(java_arguments.0) };
    }
    

    pub fn isHighScoreSubmitted(&mut self) -> bool {
        return self.highScoreSubmitted.clone();
    }
    

    // throws Exception
    pub fn setHighScoreSubmitted(&mut self, mut highScoreSubmitted: bool) {
        self.highScoreSubmitted = highScoreSubmitted.clone();
        if highScoreSubmitted {
            { let java_receiver = self.highScoresHelper.clone(); let java_arguments = (NullHighScoresSingletonFactory::getInstance(),); jcall!(java_receiver).setSelectedHighScores(java_arguments.0); };
        }
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("isHighScoreSubmitted: ")).appendboolean(highScoreSubmitted.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setHighScoreSubmitted"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn getGameState(&mut self) -> Option<Rc<RefCell<GameState>>> {
        return self.gameState.clone();
    }
    

    // throws Exception
    pub fn setGameState(&mut self, mut gameState: Option<Rc<RefCell<GameState>>>) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(self.gameStrings).GAME_STATE.clone()).append(jcall!(self.stringUtil).toString(crate::java_into_object(gameState.clone()))).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.gameStrings).SET_GAME_STATE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        self.gameState = gameState.clone();
        jcall!(self.gameStateTimeHelper).setStartTimeTNT();
        self.updateEndGameProcessor();
        self.updateGameKeyEventProcessor();
        jcall!(self.gameBehavior).setGameState(None /* Java this: a &mut self method has no Rc of itself */);
    }
    

    // throws Exception
    pub fn updateGameState(&mut self) {
        let mut gameAdState: Option<Rc<RefCell<GameAdState>>> = jcall!(self.gameAdStateFactory).getCurrentInstance();
        jcall!(gameAdState).processAdState(self.gameState.clone(), jcall!(jcall!(self.gameLayerManager).getGameInfo()).getGameType());
        if !match (&self.gameState, &jcall!(self.gameStateFactory).PLAYING_GAME_STATE) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            jcall!(gameAdState).processPageAdState();
        }
    }
    

    pub fn updateGameKeyEventProcessor(&mut self) {
        if !match (&self.getGameState(), &jcall!(self.gameStateFactory).PLAYING_GAME_STATE) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } || self.isCheating {
        } else {
        }
    }
    

    pub fn removeAllGameKeyInputListenersOnBuild(&mut self) {
        self.removeAllGameKeyInputListeners();
    }
    

    pub fn removeAllGameKeyInputListeners(&mut self) {
        jcall!(self.gameBehavior).removeAllGameKeyInputListeners(None /* Java this: a &mut self method has no Rc of itself */);
    }
    

    pub fn removeAllGameKeyInputListeners2(&mut self) {
        jcall!(self.logUtil).putF(String::from("Remove PlayerInput Listeners"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("removeAllGameKeyInputListeners"));
        for index in (0..=jcall!(self.localPlayerGameInputList).size() - 1).rev() {
            let mut playerGameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInputDyn>>> = { let java_object = (jcall!(self.localPlayerGameInputList).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInputDyn>>>>().expect("ClassCastException").clone()) };
            self.removeKeyInputListener(playerGameInput.clone());
            jcall!(playerGameInput).removeNonAIInputGameKeyEvents();
        }
    }
    

    pub fn removeKeyInputListener(&mut self, mut playerGameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInputDyn>>>) {
        jcall!(self.gameKeyEventHandler).removeListener(playerGameInput.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface>>));
    }
    

    // throws Exception
    pub fn updateEndGameProcessor(&mut self) {
        jcall!(self.gameBehavior).updateEndGameProcessor(None /* Java this: a &mut self method has no Rc of itself */);
    }
    

    // throws Exception
    pub fn updateEndGameProcessor2(&mut self) {
        if match (&self.getGameState(), &jcall!(self.gameStateFactory).SHOW_END_RESULT_GAME_STATE) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } || match (&self.getGameState(), &jcall!(self.gameStateFactory).SHOW_HIGH_SCORE_GAME_STATE) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            self.setEndGameProcessor(self.realEndGameProcessor.clone());
        } else {
            { let java_arguments = (Processor::getInstance(),); self.setEndGameProcessor(java_arguments.0) };
        }
    }
    

    // throws Exception
    pub fn buildGameInit(&mut self, mut isPortion: bool) {
    }
    

    // throws Exception
    pub fn cleanupGame(&mut self) {
        jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).CLEANUP.clone());
        let mut progressCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvasDyn>>> = ProgressCanvasFactory::getInstance();
        jcall!(progressCanvas).addNormalPortion(10, String::from("Cleaning Up"));
        jcall!(self.primaryPlayerQueue).clear();
        jcall!(self.secondaryPlayerQueue).clear();
        jcall!(GameLevelDisplayChangeEventListenersFactory::getInstance()).clear();
    }
    

    // throws Exception
    pub fn loadResourceLoadingLevel(&mut self, mut resourceLoadingLevel: Option<Rc<RefCell<ResourceLoadingLevel>>>) {
        let mut gameInitializedEvent: Option<Rc<RefCell<GameInitializedEvent>>> = jcall!(GameInitializationUtil::getInstance()).EVENT.clone();
        jcall!(gameInitializedEvent).setResourceLoadingLevel(resourceLoadingLevel.clone());
        jcall!(GameInitializedEventHandler::getInstance()).fireEvent(gameInitializedEvent.clone());
    }
    

    // throws Exception
    pub fn loadResources(&mut self, mut level: i32) {
        let mut gameInitializedEvent: Option<Rc<RefCell<GameInitializedEvent>>> = jcall!(GameInitializationUtil::getInstance()).EVENT.clone();
        jcall!(gameInitializedEvent).setLevel(level.clone());
        jcall!(GameInitializedEventHandler::getInstance()).fireEvent(gameInitializedEvent.clone());
    }
    

    // throws Exception
    pub fn updateColor(&mut self) {
    }
    

    // throws Exception
    pub fn buildGame(&mut self, mut portion: i32) {
        jcall!(self.screenCapture).endRecording();
        jcall!(self.screenCapture).startRecording();
        let mut progressCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvasDyn>>> = ProgressCanvasFactory::getInstance();
        let mut features: Option<Rc<RefCell<Features>>> = Features::getInstance();
        jcall!(progressCanvas).addNormalPortion(portion.clone(), String::from("Generic Build"));
        self.removeAllGameKeyInputListenersOnBuild();
        self.updateTouch();
        jcall!(progressCanvas).addNormalPortion(portion.clone(), String::from("High Scores"));
        { let java_receiver = self.highScoresFactoryInterface.clone(); let java_arguments = (jcall!(self.gameLayerManager).getGameInfo(), self.highScoresHelper.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::score::HighScoresResultsListener::HighScoresResultsListener>>),); jcall!(java_receiver).fetchHighScores(java_arguments.0, java_arguments.1); };
        { let java_arguments = (NullPaintable::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>),); self.setHighScoresPaintable(java_arguments.0) };
        jcall!(progressCanvas).addNormalPortion(portion.clone(), String::from("Finishing."));
        jcall!(self.getEndGameInfoP()).update(jcall!(self.getLayerManager()).getGameInfo(), None /* Java this: a &mut self method has no Rc of itself */);
        self.isSingleKeyRepeatableProcessing = jcall!(features).isFeature(jcall!(InputFeatureFactory::getInstance()).SINGLE_KEY_REPEAT_PRESS.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>));
        self.setGameOver(false);
        jcall!(progressCanvas).addNormalPortion(portion.clone(), String::from("Finishing.."));
        self.updateColor();
        jcall!(self.getStartIntermissionPaintable()).update();
        if !self.isRunningInAnotherThread() {
            return;
        }
        jcall!(progressCanvas).addNormalPortion(portion.clone(), String::from("Finishing..."));
        if self.isMainCanvas() || !self.isInitialized() {
            if jcall!(features).isFeature(jcall!(MainFeatureFactory::getInstance()).LOAD_ONDEMAND.clone()) {
                if self.getCustomCommandListener().is_some() {
                    jcall!(progressCanvas).end();
                }
            } else {
                jcall!(progressCanvas).end();
            }
            self.setInitialized(true);
        }
        if match (&self.getCustomCommandListener(), &crate::javax::microedition::lcdui::NullCommandListener::NullCommandListener_NULL_COMMAND_LISTENER.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            jcall!(self.logUtil).putF(String::from("Show Game Paintable in DemoCanvas Thread"), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.BUILD_GAME.clone());
            jcall!(self.gameCanvasStartListener).showGamePaintable();
        } else {
            jcall!(self.logUtil).putF(String::from("No GameCanvasStartListener"), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.BUILD_GAME.clone());
        }
        { let java_receiver = self.colorFillPaintable.clone(); let java_arguments = (jcall!(self.gameLayerManager).getBackgroundBasicColor(),); jcall!(java_receiver).setBasicColorP(java_arguments.0); };
        jcall!(self.gameBehavior).buildGame(None /* Java this: a &mut self method has no Rc of itself */);
    }
    

    pub fn buildGame2(&mut self) {
        PreLogUtil::put(jcall!(StringMaker::new()).append(jcall!(self.gameInputStrings).ENABLE_PLAYER_GAME_INPUTS.clone()).appendint(jcall!(self.localPlayerGameInputList).size()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.BUILD_GAME.clone());
        let mut playerGameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInputDyn>>> = None;
        for index in (0..=jcall!(self.localPlayerGameInputList).size() - 1).rev() {
            playerGameInput = { let java_object = (jcall!(self.localPlayerGameInputList).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInputDyn>>>>().expect("ClassCastException").clone()) };
            PreLogUtil::put(jcall!(StringMaker::new()).append(jcall!(self.gameInputStrings).ENABLE_PLAYER_GAME_INPUT.clone()).append(jcall!(playerGameInput).as_PlayerGameInput_mut().toString()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.BUILD_GAME.clone());
            jcall!(playerGameInput).removeNonAIInputGameKeyEvents();
            self.addKeyInputListener(playerGameInput.clone());
        }
    }
    

    pub fn addKeyInputListener(&mut self, mut playerGameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInputDyn>>>) {
        jcall!(GameKeyEventHandler::getInstance()).addListenerForPlayer(playerGameInput.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface>>), jcall!(playerGameInput).getPlayerInputId());
    }
    

    pub fn setGameCanvasStartListener(&mut self, mut gameCanvasStartListener: Option<Rc<RefCell<dyn DemoPaintableInterface>>>) {
        self.gameCanvasStartListener = gameCanvasStartListener.clone();
    }
    

    // throws Exception
    pub fn loadState(&mut self) {
        jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).LOAD.clone());
        let mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> = self.getLoadStateHashtable();
        if hashtable.is_some() && jcall!(hashtable).size() > 0 {
            let mut levelAsString: String = (jcall!(hashtable).get(crate::java_into_object((crate::org::allbinary::game::GameInfo::GameInfo_LEVEL_NAME.with(Clone::clone)).clone() /* as Any */))).clone() /* as String */;
            let mut level: i32 = (((levelAsString.clone()).to_string()).unwrap() as i32);
            let mut gameInfo: Option<Rc<RefCell<GameInfo>>> = jcall!(self.gameLayerManager).getGameInfo();
            jcall!(gameInfo).setCurrentLevel(level.clone());
        }
    }
    

    // throws Exception
    pub fn getLoadStateHashtable(&mut self) -> Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(self.commonLabels).START_LABEL.clone()).append(jcall!(self.stringUtil).toString(crate::java_into_object(self.hashtable.clone()))).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getLoadStateHashtable"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        return self.hashtable.clone();
    }
    

    pub fn setLoadStateHashtable(&mut self, mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(self.commonLabels).START_LABEL.clone()).append(jcall!(self.stringUtil).toString(crate::java_into_object(hashtable.clone()))).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setLoadStateHashtable"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        self.hashtable = hashtable.clone();
    }
    

    pub fn getCurrentStateHashtable(&mut self) -> Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> {
        let mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> = jcall!(self.stdUtil).createHashtable();
        let mut level: i32 = jcall!(jcall!(self.gameLayerManager).getGameInfo()).getCurrentLevel();
        jcall!(hashtable).put(crate::java_into_object(crate::org::allbinary::game::GameInfo::GameInfo_LEVEL_NAME.with(Clone::clone).clone()), crate::java_into_object((level.clone()).to_string()));
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("End: ")).append(jcall!(self.stringUtil).toString(crate::java_into_object(hashtable.clone()))).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getCurrentStateHashtable"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        return hashtable.clone();
    }
    

    pub fn paintGameOver(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        ForcedLogUtil::log(jcall!(self.commonStrings).NOT_IMPLEMENTED.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())));
    }
    

    pub fn draw(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        jcall!(self.colorFillPaintable).paint(graphics.clone());
        { let java_receiver = self.basicSetColorUtil.clone(); let java_arguments = (graphics.clone(), jcall!(self.gameLayerManager).getForegroundBasicColor(),); jcall!(java_receiver).setBasicColorP(java_arguments.0, java_arguments.1); };
        jcall!(self.gameSpecificPaintable).paint(graphics.clone());
    }
    

    pub fn clear(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        jcall!(self.colorFillPaintable).paint(graphics.clone());
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        jcall!(self.myFontProcessor).process(graphics.clone());
        jcall!(self.baseGameStatistics).nextRefresh();
        self.draw(graphics.clone());
        jcall!(self.menuPaintable).paint(graphics.clone());
        jcall!(self.progressPaintable).paint(graphics.clone());
    }
    

    pub fn paintThreed(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
    }
    

    // throws Exception
    pub fn processEndLevelIntermissionGameState(&mut self) {
    }
    

    pub fn nonBotPaint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        jcall!(self.endGamePaintable).paint(graphics.clone());
        jcall!(self.intermissionPaintable).paint(graphics.clone());
        self.paintIntermission(graphics.clone());
        jcall!(self.touchPaintable).paint(graphics.clone());
    }
    

    pub fn paintIntermission(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
    }
    

    pub fn keyPressed(&mut self, mut keyCode: i32) {
        self.keyPressedByDevice(keyCode.clone(), 0);
    }
    

    pub fn keyReleased(&mut self, mut keyCode: i32) {
        self.keyReleasedByDevice(keyCode.clone(), 0);
    }
    

    pub fn keyRepeated(&mut self, mut keyCode: i32) {
        self.keyRepeatedByDevice(keyCode.clone(), 0);
    }
    

    pub fn keyPressedByDevice(&mut self, mut keyCode: i32, mut deviceId: i32) {
        jcall!(self.inputProcessor).keyPressedByDevice(keyCode.clone(), deviceId.clone());
    }
    

    pub fn keyRepeatedByDevice(&mut self, mut keyCode: i32, mut deviceId: i32) {
        if self.isSingleKeyRepeatableProcessing {
            jcall!(self.inputProcessor).keyPressedByDevice(keyCode.clone(), deviceId.clone());
        }
    }
    

    pub fn keyReleasedByDevice(&mut self, mut keyCode: i32, mut deviceId: i32) {
        jcall!(self.inputProcessor).keyReleasedByDevice(None /* Java this: a &mut self method has no Rc of itself */, keyCode.clone(), deviceId.clone());
    }
    

    // throws Exception
    pub fn handleRawKey(&mut self, mut keyCode: i32, mut deviceId: i32, mut repeated: bool) {
    }
    

    pub fn endProgress(&mut self, mut isProgress: bool) -> i32 {
        let mut portion: i32 = 30;
        if isProgress && self.isMainCanvas() {
            let mut myCommandsFactory: Option<Rc<RefCell<MyCommandsFactory>>> = MyCommandsFactory::getInstance();
            let mut progressCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvasDyn>>> = ProgressCanvasFactory::getInstance();
            jcall!(progressCanvas).start();
            jcall!(self.getCustomCommandListener()).commandAction(jcall!(myCommandsFactory).SET_DISPLAYABLE.clone(), progressCanvas.clone());
            portion = 4;
        }
        return portion.clone();
    }
    

    // throws Exception
    pub fn processPlayingGame(&mut self) {
        jcall!(self.cheatProcessor).update();
        jcall!(jcall!(self.sensorGameUpdateProcessor).getInputSensor()).update();
        jcall!(self.gameInputProcessor).process();
        jcall!(self.gameLayerManager).process();
        jcall!(self.startIntermissionProcessor).process();
    }
    

    // throws Exception
    pub fn threadInit(&mut self) {
    }
    

    // throws Exception
    pub fn processGame(&mut self) {
        self.base.process();
        if AllBinaryMediaManager::update() {
            if !jcall!(self.primaryPlayerQueue).process() {
                jcall!(self.secondaryPlayerQueue).process();
            }
        }
        jcall!(self.mainStateProcessor).process();
        jcall!(self.menuInputProcessor).processInputList();
        jcall!(self.endGameProcessor).process();
        jcall!(self.baseGameStatistics).nextFrame();
    }
    

    pub fn notifyDonePainting(&mut self) {
        { // synchronized (self)
            jcall!(self.threadObjectUtil).notifyObject(None /* Java this: a &mut self method has no Rc of itself */);
        }
    }
    

    // throws Exception
    pub fn shouldWait(&mut self) {
        let mut features: Option<Rc<RefCell<Features>>> = Features::getInstance();
        if AndroidUtil::isAndroid() {
        } else if jcall!(features).isFeature(jcall!(self.openGLFeatureFactory).OPENGL_2D.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
        } else {
            if jcall!(features).isDefault(jcall!(self.openGLFeatureFactory).OPENGL.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
                let mut openGLImageSpecificFactory: Option<Rc<RefCell<OpenGLImageSpecificFactory>>> = OpenGLImageSpecificFactory::getInstance();
                while jcall!(openGLImageSpecificFactory).updating {
                    std::thread::sleep(std::time::Duration::from_millis((self.YIELD_SLEEP.clone()) as u64));
                }
            }
        }
    }
    

    pub fn run(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).START_RUNNABLE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).RUN.clone());
            let mut progressCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvasDyn>>> = ProgressCanvasFactory::getInstance();
            jcall!(progressCanvas).addNormalPortion(50, String::from("Game Thread"));
            self.setCurrentThread();
            self.setRunning(true);
            if match (&self.getCustomCommandListener(), &crate::javax::microedition::lcdui::NullCommandListener::NullCommandListener_NULL_COMMAND_LISTENER.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                std::thread::sleep(std::time::Duration::from_millis((self.YIELD_SLEEP.clone()) as u64));
            }
            self.shouldWait();
            self.threadInit();
            if match (&self.getCustomCommandListener(), &crate::javax::microedition::lcdui::NullCommandListener::NullCommandListener_NULL_COMMAND_LISTENER.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                std::thread::sleep(std::time::Duration::from_millis((self.YIELD_SLEEP.clone()) as u64));
            }
            self.open();
            let mut gameAdState: Option<Rc<RefCell<GameAdState>>> = jcall!(self.gameAdStateFactory).getCurrentInstance();
            jcall!(gameAdState).init();
            jcall!(gameAdState).setGameIsReady(true);
            jcall!(self.gameBehavior).run(None /* Java this: a &mut self method has no Rc of itself */);
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).END_RUNNABLE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).RUN.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).RUN.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    // throws Exception
    pub fn run2(&mut self) {
        let mut features: Option<Rc<RefCell<Features>>> = Features::getInstance();
        if jcall!(features).isDefault(jcall!(self.openGLFeatureFactory).OPENGL_AS_GAME_THREAD.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) || J2MEUtil::isHTML() {
            if jcall!(features).isDefault(jcall!(self.openGLFeatureFactory).OPENGL_AS_GAME_THREAD.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(jcall!(self.openGLFeatureFactory).OPENGL_AS_GAME_THREAD).getName(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).RUN.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            if J2MEUtil::isHTML() {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(GraphicsStrings::getInstance()).HTML.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).RUN.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            let mut currentDisplayableFactory: Option<Rc<RefCell<CurrentDisplayableFactory>>> = CurrentDisplayableFactory::getInstance();
            jcall!(currentDisplayableFactory).setRunnable(self.gameRunnable.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::GameRunnable::GameRunnableDyn>>));
            jcall!(currentDisplayableFactory).setDisplayable(None /* Java this: a &mut self method has no Rc of itself */);
            jcall!(OpenGLThreadUtil::getInstance()).onResume();
        } else if jcall!(features).isDefault(jcall!(self.openGLFeatureFactory).OPENGL_AND_GAME_HAVE_DIFFERENT_THREADS.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(jcall!(self.openGLFeatureFactory).OPENGL_AND_GAME_HAVE_DIFFERENT_THREADS).getName(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).RUN.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            jcall!(OpenGLThreadUtil::getInstance()).onResume();
            while self.isRunning() {
                self.run3();
            }
            self.end();
        } else {
            jcall!(self.logUtil).putF(String::from("this thread"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).RUN.clone());
            while self.isRunning() {
                self.run3();
            }
            self.end();
        }
    }
    

    // throws Exception
    pub fn run3(&mut self) {
        { let java_receiver = self.loopTimeHelper.clone(); let java_arguments = (jcall!(self.gameTickTimeDelayHelper).setStartTime(),); jcall!(java_receiver).setStartTime(java_arguments.0); };
        jcall!(self.gameTickDisplayInfoSingleton).update();
        self.processGame();
        self.processLoopSleep();
    }
    

    pub fn setRunning(&mut self, mut running: bool) {
        self.base.setRunning(running.clone());
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut features: Option<Rc<RefCell<Features>>> = Features::getInstance();
            if running {
            } else {
                if jcall!(features).isDefault(jcall!(self.openGLFeatureFactory).OPENGL.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) || unsafe { crate::org::allbinary::game::layer::SWTUtil::SWTUtil_isSWT.clone() } {
                    if !match (&jcall!(jcall!(self.gameLayerManager).getGameInfo()).getGameType(), &jcall!(self.gameTypeFactory).BOT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                        let mut currentDisplayableFactory: Option<Rc<RefCell<CurrentDisplayableFactory>>> = CurrentDisplayableFactory::getInstance();
                        jcall!(currentDisplayableFactory).clearRunnable();
                    }
                    self.end();
                }
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.SET_RUNNING.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    // throws Exception
    pub fn end(&mut self) {
        jcall!(self.screenCapture).endRecording();
        { let java_receiver = self.baseGameStatistics.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(self.baseGameStatistics).toString()).append(jcall!(CommonSeps::getInstance()).NEW_LINE.clone()).toString(),); jcall!(java_receiver).add(java_arguments.0); };
        jcall!(self.baseGameStatistics).init();
        jcall!(self.gameKeyEventHandler).removeListener(self.cheatProcessor.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface>>));
        self.close();
        self.removeAllGameKeyInputListeners();
        self.endGameThread();
    }
    

    // throws Exception
    pub fn endGameThread(&mut self) {
        jcall!(DisplayChangeEventHandler::getInstance()).removeListener(None /* Java this: a &mut self method has no Rc of itself */);
        jcall!(GameLevelDisplayChangeEventListenersFactory::getInstance()).clear();
        let mut gameAdState: Option<Rc<RefCell<GameAdState>>> = jcall!(self.gameAdStateFactory).getCurrentInstance();
        jcall!(gameAdState).notPlayingAdState();
    }
    

    pub fn createHighScore(&mut self, mut score: i64) -> Option<Rc<RefCell<HighScore>>> {
        let mut gameInfo: Option<Rc<RefCell<GameInfo>>> = jcall!(self.gameLayerManager).getGameInfo();
        return HighScore::new(0, String::from("NONE"), GameInfo::new(jcall!(gameInfo).getGameType(), jcall!(gameInfo).getGameMode(), jcall!(PlayerTypesFactory::getInstance()).PLAYER_TYPE_ONE.clone(), 0, 0), score.clone());
    }
    

    // throws Exception
    pub fn setHighScore(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut name: String, mut score: i64, mut autoSubmit: bool, mut isLast: bool) {
        jcall!(self.gameBehavior).setHighScore(abeClientInformation.clone(), None /* Java this: a &mut self method has no Rc of itself */, name.clone(), score.clone(), autoSubmit.clone(), isLast.clone());
    }
    

    // throws Exception
    pub fn setHighScore2(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut name: String, mut score: i64, mut autoSubmit: bool, mut isLast: bool) {
        let mut highScore: Option<Rc<RefCell<HighScore>>> = self.createHighScore(score.clone());
        let mut textBox: Option<Rc<RefCell<HighScoreTextBox>>> = HighScoreTextBox::new(self.highScoresFactoryInterface.clone(), self.highScoresHelper.clone(), abeClientInformation.clone(), jcall!(self.gameLayerManager).getGameInfo(), self.getCustomCommandListener(), name.clone(), highScore.clone(), jcall!(self.gameLayerManager).getBackgroundBasicColor(), jcall!(self.gameLayerManager).getForegroundBasicColor());
        if isLast {
            jcall!(self.getCustomCommandListener()).commandAction(jcall!(GameCommandsFactory::getInstance()).SET_MENU_DISPLAYABLE.clone(), textBox.clone());
        }
        if autoSubmit {
            /* Java local class: move it out of the function
pub struct SaveHighScoreRunnable {
    pub progressCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvasDyn>>>,
}

impl Runnable for SaveHighScoreRunnable {
    fn run(&mut self) { Self::run(self) }
}

impl SaveHighScoreRunnable {

    pub fn new_struct() -> Self {
        let mut this = Self {
            progressCanvas: None,
        };
        this.progressCanvas = ProgressCanvasFactory::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
                

                pub fn run(&mut self) {
                    let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
                    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        if self.base.isLast {
                            jcall!(self.base.textBox).submit();
                        } else {
                            jcall!(self.base.textBox).saveHighScore();
                        }
                    })) {
                        Ok(_) => {}
                        Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

                        let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
                        jcall!(logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("run"), crate::java_into_object(e.clone()));
                        jcall!(self.progressCanvas).end();
                        }
                    }
                }
}
*/
            jcall!(SecondaryThreadPool::getInstance()).runTask(SaveHighScoreRunnable::new());
        }
    }
    

    pub fn getGameStateTimeHelper(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>> {
        return self.gameStateTimeHelper.clone();
    }
    

    pub fn setInitialized(&mut self, mut initialized: bool) {
        self.initialized = initialized.clone();
    }
    

    pub fn isInitialized(&mut self) -> bool {
        return self.initialized.clone();
    }
    

    pub fn getSourceId(&mut self) -> i32 {
        return AllBinaryGameCanvas_id.with(Clone::clone).clone();
    }
    

    pub fn setProgressPaintable(&mut self, mut paintable: Option<Rc<RefCell<dyn PaintableInterface>>>) {
        self.progressPaintable = paintable.clone();
    }
    

    pub fn getStartIntermissionInterface(&mut self) -> Option<Rc<RefCell<dyn IntermissionInterface>>> {
        return self.startIntermissionInterface.clone();
    }
    

    pub fn getEndLevelIntermissionInterface(&mut self) -> Option<Rc<RefCell<dyn IntermissionInterface>>> {
        return self.endLevelIntermissionInterface.clone();
    }
    

    pub fn setTouchPaintableP(&mut self, mut paintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>) {
        self.touchPaintable = paintable.clone();
    }
    

    pub fn getTouchPaintableP(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>> {
        return self.touchPaintable.clone();
    }
    

    pub fn setEndGamePaintable(&mut self, mut endGamePaintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>) {
        self.endGamePaintable = endGamePaintable.clone();
    }
    

    pub fn getEndGamePaintable(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>> {
        return self.endGamePaintable.clone();
    }
    

    pub fn setIntermissionPaintable(&mut self, mut intermissionPaintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>) {
        self.intermissionPaintable = intermissionPaintable.clone();
    }
    

    pub fn getIntermissionPaintable(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>> {
        return self.intermissionPaintable.clone();
    }
    

    pub fn getEndGameInfoP(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::EndGameInfo::EndGameInfoDyn>>> {
        return self.endGameInfo.clone();
    }
    

    pub fn setHighScoresPaintable(&mut self, mut highScoresPaintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>) {
        self.highScoresPaintable = highScoresPaintable.clone();
    }
    

    pub fn getHighScoresPaintable(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>> {
        return self.highScoresPaintable.clone();
    }
    

    pub fn getRealHighScoresPaintable(&mut self) -> Option<Rc<RefCell<HighScoresPaintable>>> {
        return self.realHighScoresPaintable.clone();
    }
    

    pub fn clearPlayerGameInputList(&mut self) {
        let mut playerGameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInputDyn>>> = None;
        for index in (0..=jcall!(self.localPlayerGameInputList).size() - 1).rev() {
            playerGameInput = { let java_object = (jcall!(self.localPlayerGameInputList).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInputDyn>>>>().expect("ClassCastException").clone()) };
            jcall!(self.gameKeyEventHandler).removeListener(playerGameInput.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface>>));
        }
        jcall!(self.localPlayerGameInputList).clear();
    }
    

    pub fn addPlayerGameInput(&mut self, mut playerGameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInputDyn>>>) {
        jcall!(self.localPlayerGameInputList).add(crate::java_into_object(playerGameInput.clone()));
    }
    

    pub fn setMenuInputProcessor(&mut self, mut menuInputProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::BasicMenuInputProcessor::BasicMenuInputProcessorDyn>>>) {
        self.menuInputProcessor = menuInputProcessor.clone();
    }
    

    pub fn getMenuInputProcessor(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::BasicMenuInputProcessor::BasicMenuInputProcessorDyn>>> {
        return self.menuInputProcessor.clone();
    }
    

    pub fn getMenuForm(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionFormDyn>>> {
        return self.menuForm.clone();
    }
    

    pub fn setMenuForm(&mut self, mut menuForm: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionFormDyn>>>) {
        self.menuForm = menuForm.clone();
    }
    

    pub fn getStartLevel(&mut self) -> i32 {
        return self.startLevel.clone();
    }
    

    pub fn setStartLevel(&mut self, mut startLevel: i32) {
        self.startLevel = startLevel.clone();
    }
    

    pub fn setTouchButtonsPaintable(&mut self, mut touchButtonsPaintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>) {
        self.touchButtonsPaintable = touchButtonsPaintable.clone();
    }
    

    pub fn getTouchButtonsPaintable(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>> {
        return self.touchButtonsPaintable.clone();
    }
    

    pub fn setGameInputProcessor(&mut self, mut gameInputProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::canvas::Processor::ProcessorDyn>>>) {
        self.gameInputProcessor = gameInputProcessor.clone();
    }
    

    pub fn getGameInputProcessor(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::canvas::Processor::ProcessorDyn>>> {
        return self.gameInputProcessor.clone();
    }
    

    pub fn setEndGameProcessor(&mut self, mut endGameProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::canvas::Processor::ProcessorDyn>>>) {
        self.endGameProcessor = endGameProcessor.clone();
    }
    

    pub fn getEndGameProcessor(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::canvas::Processor::ProcessorDyn>>> {
        return self.endGameProcessor.clone();
    }
    

    pub fn setEndGameStatePaintable(&mut self, mut endGameStatePaintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>) {
        self.endGameStatePaintable = endGameStatePaintable.clone();
    }
    

    pub fn getEndGameStatePaintable(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>> {
        return self.endGameStatePaintable.clone();
    }
    

    pub fn setNonBotPaintableP(&mut self, mut nonBotPaintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>) {
        self.nonBotPaintable = nonBotPaintable.clone();
    }
    

    pub fn getNonBotPaintableP(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>> {
        return self.nonBotPaintable.clone();
    }
    

    pub fn setStartIntermissionPaintable(&mut self, mut startIntermissionPaintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::InitUpdatePaintable::InitUpdatePaintableDyn>>>) {
        self.startIntermissionPaintable = startIntermissionPaintable.clone();
    }
    

    pub fn getStartIntermissionPaintable(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::InitUpdatePaintable::InitUpdatePaintableDyn>>> {
        return self.startIntermissionPaintable.clone();
    }
    

    pub fn setMainStateProcessor(&mut self, mut mainStateProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::canvas::Processor::ProcessorDyn>>>) {
        self.mainStateProcessor = mainStateProcessor.clone();
    }
    

    pub fn getMainStateProcessor(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::canvas::Processor::ProcessorDyn>>> {
        return self.mainStateProcessor.clone();
    }
    

    pub fn setProcessGameProcessor(&mut self, mut processGameProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::canvas::Processor::ProcessorDyn>>>) {
        self.processGameProcessor = processGameProcessor.clone();
    }
    

    pub fn getProcessGameProcessor(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::canvas::Processor::ProcessorDyn>>> {
        return self.processGameProcessor.clone();
    }
    

    pub fn setOpenMenuPaintable(&mut self, mut openMenuPaintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>) {
        self.openMenuPaintable = openMenuPaintable.clone();
    }
    

    pub fn getOpenMenuPaintable(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>> {
        return self.openMenuPaintable.clone();
    }
    

    pub fn setPopupMenuInputProcessor(&mut self, mut popupMenuInputProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::BasicMenuInputProcessor::BasicMenuInputProcessorDyn>>>) {
        self.popupMenuInputProcessor = popupMenuInputProcessor.clone();
    }
    

    pub fn getPopupMenuInputProcessor(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::BasicMenuInputProcessor::BasicMenuInputProcessorDyn>>> {
        return self.popupMenuInputProcessor.clone();
    }
    

    pub fn getSensorGameUpdateProcessor(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::input::gyro::SensorGameUpdateProcessor::SensorGameUpdateProcessorDyn>>> {
        return self.sensorGameUpdateProcessor.clone();
    }
    

    pub fn getRawGameInputProcessor(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::input::InputProcessor::InputProcessorDyn>>> {
        return self.rawGameInputProcessor.clone();
    }
    

    pub fn getRawInputProcessor(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::input::InputProcessor::InputProcessorDyn>>> {
        return self.rawInputProcessor.clone();
    }
    

    pub fn setInputProcessor(&mut self, mut inputProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::InputProcessor::InputProcessorDyn>>>) {
        self.inputProcessor = inputProcessor.clone();
    }
    

    pub fn getInputProcessor(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::input::InputProcessor::InputProcessorDyn>>> {
        return self.inputProcessor.clone();
    }
    

    pub fn setMenuPaintable(&mut self, mut menuPaintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>) {
        self.menuPaintable = menuPaintable.clone();
    }
    

    pub fn getMenuPaintable(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>> {
        return self.menuPaintable.clone();
    }
    

    pub fn setFormPaintable(&mut self, mut formPaintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>) {
        self.formPaintable = formPaintable.clone();
    }
    

    pub fn getFormPaintable(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>> {
        return self.formPaintable.clone();
    }
    

    pub fn setGameSpecificPaintableP(&mut self, mut gameSpecificPaintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>) {
        self.gameSpecificPaintable = gameSpecificPaintable.clone();
    }
    

    pub fn getGameSpecificPaintableP(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>> {
        return self.gameSpecificPaintable.clone();
    }
    

    pub fn isSingleThread(&mut self) -> bool {
        return jcall!(OpenGLFeatureUtil::getInstance()).isAnyThreed() || unsafe { crate::org::allbinary::game::layer::SWTUtil::SWTUtil_isSWT.clone() };
    }
    

    pub fn isRunningInAnotherThread(&mut self) -> bool {
        let mut features: Option<Rc<RefCell<Features>>> = Features::getInstance();
        let mut openGLFeatureFactory: Option<Rc<RefCell<OpenGLFeatureFactory>>> = OpenGLFeatureFactory::getInstance();
        if jcall!(features).isDefault(jcall!(openGLFeatureFactory).OPENGL_AS_GAME_THREAD.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
            return true;
        } else {
            return self.isRunning();
        }
    }
    

    pub fn getType(&mut self) -> i32 {
        return AllBinaryGameCanvas_TYPE.with(Clone::clone).clone();
    }
}
impl std::ops::Deref for AllBinaryGameCanvas {
    type Target = RunnableCanvas;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AllBinaryGameCanvas {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait AllBinaryGameCanvasDyn: crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvasDyn + crate::org::allbinary::game::displayable::canvas::PopupMenuInterface::PopupMenuInterface + crate::org::allbinary::game::IntermissionCompositeInterface::IntermissionCompositeInterface + crate::org::allbinary::game::IntermissionEnableListenerInterface::IntermissionEnableListenerInterface {
    fn as_AllBinaryGameCanvas(&self) -> &AllBinaryGameCanvas;
    fn as_AllBinaryGameCanvas_mut(&mut self) -> &mut AllBinaryGameCanvas;
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { AllBinaryGameCanvas::updateMeasurement(self.as_AllBinaryGameCanvas_mut(), graphics) }
    fn getInGameMenuBehavior(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::BaseMenuBehavior::BaseMenuBehaviorDyn>>> { AllBinaryGameCanvas::getInGameMenuBehavior(self.as_AllBinaryGameCanvas_mut()) }
    fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { AllBinaryGameCanvas::onEvent(self.as_AllBinaryGameCanvas_mut(), eventObject) }
    fn onDisplayChangeEvent(&mut self, mut displayChangeEvent: Option<Rc<RefCell<DisplayChangeEvent>>>) { AllBinaryGameCanvas::onDisplayChangeEvent(self.as_AllBinaryGameCanvas_mut(), displayChangeEvent) }
    fn updateMenuFromEvent(&mut self, mut displayChangeEvent: Option<Rc<RefCell<DisplayChangeEvent>>>) { AllBinaryGameCanvas::updateMenuFromEvent(self.as_AllBinaryGameCanvas_mut(), displayChangeEvent) }
    fn initPopupMenu(&mut self) { AllBinaryGameCanvas::initPopupMenu(self.as_AllBinaryGameCanvas_mut()) }
    fn initMenu(&mut self) { AllBinaryGameCanvas::initMenu(self.as_AllBinaryGameCanvas_mut()) }
    fn initMenu2(&mut self) { AllBinaryGameCanvas::initMenu2(self.as_AllBinaryGameCanvas_mut()) }
    fn updateMenu(&mut self) { AllBinaryGameCanvas::updateMenu(self.as_AllBinaryGameCanvas_mut()) }
    fn updateMenu2(&mut self) { AllBinaryGameCanvas::updateMenu2(self.as_AllBinaryGameCanvas_mut()) }
    fn popupMenu2(&mut self) { AllBinaryGameCanvas::popupMenu2(self.as_AllBinaryGameCanvas_mut()) }
    fn closeMenu2(&mut self) { AllBinaryGameCanvas::closeMenu2(self.as_AllBinaryGameCanvas_mut()) }
    fn open(&mut self) { AllBinaryGameCanvas::open(self.as_AllBinaryGameCanvas_mut()) }
    fn close(&mut self) { AllBinaryGameCanvas::close(self.as_AllBinaryGameCanvas_mut()) }
    fn processorInit(&mut self) { AllBinaryGameCanvas::processorInit(self.as_AllBinaryGameCanvas_mut()) }
    fn setProcessGameProcessorInit(&mut self) { AllBinaryGameCanvas::setProcessGameProcessorInit(self.as_AllBinaryGameCanvas_mut()) }
    fn initSpecialPaint(&mut self) { AllBinaryGameCanvas::initSpecialPaint(self.as_AllBinaryGameCanvas_mut()) }
    fn init(&mut self, mut gameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>, mut buffered: bool) { AllBinaryGameCanvas::init(self.as_AllBinaryGameCanvas_mut(), gameLayerManager, buffered) }
    fn mediaInit(&mut self) { AllBinaryGameCanvas::mediaInit(self.as_AllBinaryGameCanvas_mut()) }
    fn initConfigurable(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>) { AllBinaryGameCanvas::initConfigurable(self.as_AllBinaryGameCanvas_mut(), abeClientInformation) }
    fn initApp(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>) { AllBinaryGameCanvas::initApp(self.as_AllBinaryGameCanvas_mut(), abeClientInformation) }
    fn initTouch(&mut self) { AllBinaryGameCanvas::initTouch(self.as_AllBinaryGameCanvas_mut()) }
    fn updateCurrentTouchInputFactory(&mut self, mut nextTouchInput: Option<Rc<RefCell<dyn crate::org::allbinary::input::motion::button::BaseTouchInput::BaseTouchInputDyn>>>) { AllBinaryGameCanvas::updateCurrentTouchInputFactory(self.as_AllBinaryGameCanvas_mut(), nextTouchInput) }
    fn updateTouch(&mut self) { AllBinaryGameCanvas::updateTouch(self.as_AllBinaryGameCanvas_mut()) }
    fn updateTouch2(&mut self) { AllBinaryGameCanvas::updateTouch2(self.as_AllBinaryGameCanvas_mut()) }
    fn postInitTouch(&mut self) { AllBinaryGameCanvas::postInitTouch(self.as_AllBinaryGameCanvas_mut()) }
    fn addCommands(&mut self) { AllBinaryGameCanvas::addCommands(self.as_AllBinaryGameCanvas_mut()) }
    fn itemStateChanged(&mut self, mut item: Option<Rc<RefCell<Item>>>) { AllBinaryGameCanvas::itemStateChanged(self.as_AllBinaryGameCanvas_mut(), item) }
    fn updateScreenButtonPaintable(&mut self) { AllBinaryGameCanvas::updateScreenButtonPaintable(self.as_AllBinaryGameCanvas_mut()) }
    fn updateScreenButtonPaintable2(&mut self) { AllBinaryGameCanvas::updateScreenButtonPaintable2(self.as_AllBinaryGameCanvas_mut()) }
    fn getLayerManager(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>> { AllBinaryGameCanvas::getLayerManager(self.as_AllBinaryGameCanvas_mut()) }
    fn setLayerManager(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>) { AllBinaryGameCanvas::setLayerManager(self.as_AllBinaryGameCanvas_mut(), layerManager) }
    fn isGameOver(&mut self) -> bool { AllBinaryGameCanvas::isGameOver(self.as_AllBinaryGameCanvas_mut()) }
    fn setGameOver(&mut self, mut gameOver: bool) { AllBinaryGameCanvas::setGameOver(self.as_AllBinaryGameCanvas_mut(), gameOver) }
    fn processGameOver(&mut self) { AllBinaryGameCanvas::processGameOver(self.as_AllBinaryGameCanvas_mut()) }
    fn isHighScoreSubmitted(&mut self) -> bool { AllBinaryGameCanvas::isHighScoreSubmitted(self.as_AllBinaryGameCanvas_mut()) }
    fn setHighScoreSubmitted(&mut self, mut highScoreSubmitted: bool) { AllBinaryGameCanvas::setHighScoreSubmitted(self.as_AllBinaryGameCanvas_mut(), highScoreSubmitted) }
    fn getGameState(&mut self) -> Option<Rc<RefCell<GameState>>> { AllBinaryGameCanvas::getGameState(self.as_AllBinaryGameCanvas_mut()) }
    fn setGameState(&mut self, mut gameState: Option<Rc<RefCell<GameState>>>) { AllBinaryGameCanvas::setGameState(self.as_AllBinaryGameCanvas_mut(), gameState) }
    fn updateGameState(&mut self) { AllBinaryGameCanvas::updateGameState(self.as_AllBinaryGameCanvas_mut()) }
    fn updateGameKeyEventProcessor(&mut self) { AllBinaryGameCanvas::updateGameKeyEventProcessor(self.as_AllBinaryGameCanvas_mut()) }
    fn removeAllGameKeyInputListenersOnBuild(&mut self) { AllBinaryGameCanvas::removeAllGameKeyInputListenersOnBuild(self.as_AllBinaryGameCanvas_mut()) }
    fn removeAllGameKeyInputListeners(&mut self) { AllBinaryGameCanvas::removeAllGameKeyInputListeners(self.as_AllBinaryGameCanvas_mut()) }
    fn removeAllGameKeyInputListeners2(&mut self) { AllBinaryGameCanvas::removeAllGameKeyInputListeners2(self.as_AllBinaryGameCanvas_mut()) }
    fn removeKeyInputListener(&mut self, mut playerGameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInputDyn>>>) { AllBinaryGameCanvas::removeKeyInputListener(self.as_AllBinaryGameCanvas_mut(), playerGameInput) }
    fn updateEndGameProcessor(&mut self) { AllBinaryGameCanvas::updateEndGameProcessor(self.as_AllBinaryGameCanvas_mut()) }
    fn updateEndGameProcessor2(&mut self) { AllBinaryGameCanvas::updateEndGameProcessor2(self.as_AllBinaryGameCanvas_mut()) }
    fn buildGameInit(&mut self, mut isPortion: bool) { AllBinaryGameCanvas::buildGameInit(self.as_AllBinaryGameCanvas_mut(), isPortion) }
    fn cleanupGame(&mut self) { AllBinaryGameCanvas::cleanupGame(self.as_AllBinaryGameCanvas_mut()) }
    fn loadResourceLoadingLevel(&mut self, mut resourceLoadingLevel: Option<Rc<RefCell<ResourceLoadingLevel>>>) { AllBinaryGameCanvas::loadResourceLoadingLevel(self.as_AllBinaryGameCanvas_mut(), resourceLoadingLevel) }
    fn loadResources(&mut self, mut level: i32) { AllBinaryGameCanvas::loadResources(self.as_AllBinaryGameCanvas_mut(), level) }
    fn updateColor(&mut self) { AllBinaryGameCanvas::updateColor(self.as_AllBinaryGameCanvas_mut()) }
    fn buildGame(&mut self, mut portion: i32) { AllBinaryGameCanvas::buildGame(self.as_AllBinaryGameCanvas_mut(), portion) }
    fn buildGame2(&mut self) { AllBinaryGameCanvas::buildGame2(self.as_AllBinaryGameCanvas_mut()) }
    fn addKeyInputListener(&mut self, mut playerGameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInputDyn>>>) { AllBinaryGameCanvas::addKeyInputListener(self.as_AllBinaryGameCanvas_mut(), playerGameInput) }
    fn setGameCanvasStartListener(&mut self, mut gameCanvasStartListener: Option<Rc<RefCell<dyn DemoPaintableInterface>>>) { AllBinaryGameCanvas::setGameCanvasStartListener(self.as_AllBinaryGameCanvas_mut(), gameCanvasStartListener) }
    fn loadState(&mut self) { AllBinaryGameCanvas::loadState(self.as_AllBinaryGameCanvas_mut()) }
    fn getLoadStateHashtable(&mut self) -> Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> { AllBinaryGameCanvas::getLoadStateHashtable(self.as_AllBinaryGameCanvas_mut()) }
    fn setLoadStateHashtable(&mut self, mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>) { AllBinaryGameCanvas::setLoadStateHashtable(self.as_AllBinaryGameCanvas_mut(), hashtable) }
    fn getCurrentStateHashtable(&mut self) -> Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> { AllBinaryGameCanvas::getCurrentStateHashtable(self.as_AllBinaryGameCanvas_mut()) }
    fn paintGameOver(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { AllBinaryGameCanvas::paintGameOver(self.as_AllBinaryGameCanvas_mut(), graphics) }
    fn clear(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { AllBinaryGameCanvas::clear(self.as_AllBinaryGameCanvas_mut(), graphics) }
    fn paintThreed(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { AllBinaryGameCanvas::paintThreed(self.as_AllBinaryGameCanvas_mut(), graphics) }
    fn processEndLevelIntermissionGameState(&mut self) { AllBinaryGameCanvas::processEndLevelIntermissionGameState(self.as_AllBinaryGameCanvas_mut()) }
    fn nonBotPaint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { AllBinaryGameCanvas::nonBotPaint(self.as_AllBinaryGameCanvas_mut(), graphics) }
    fn paintIntermission(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { AllBinaryGameCanvas::paintIntermission(self.as_AllBinaryGameCanvas_mut(), graphics) }
    fn keyPressed(&mut self, mut keyCode: i32) { AllBinaryGameCanvas::keyPressed(self.as_AllBinaryGameCanvas_mut(), keyCode) }
    fn keyReleased(&mut self, mut keyCode: i32) { AllBinaryGameCanvas::keyReleased(self.as_AllBinaryGameCanvas_mut(), keyCode) }
    fn keyRepeated(&mut self, mut keyCode: i32) { AllBinaryGameCanvas::keyRepeated(self.as_AllBinaryGameCanvas_mut(), keyCode) }
    fn handleRawKey(&mut self, mut keyCode: i32, mut deviceId: i32, mut repeated: bool) { AllBinaryGameCanvas::handleRawKey(self.as_AllBinaryGameCanvas_mut(), keyCode, deviceId, repeated) }
    fn endProgress(&mut self, mut isProgress: bool) -> i32 { AllBinaryGameCanvas::endProgress(self.as_AllBinaryGameCanvas_mut(), isProgress) }
    fn processPlayingGame(&mut self) { AllBinaryGameCanvas::processPlayingGame(self.as_AllBinaryGameCanvas_mut()) }
    fn threadInit(&mut self) { AllBinaryGameCanvas::threadInit(self.as_AllBinaryGameCanvas_mut()) }
    fn processGame(&mut self) { AllBinaryGameCanvas::processGame(self.as_AllBinaryGameCanvas_mut()) }
    fn notifyDonePainting(&mut self) { AllBinaryGameCanvas::notifyDonePainting(self.as_AllBinaryGameCanvas_mut()) }
    fn shouldWait(&mut self) { AllBinaryGameCanvas::shouldWait(self.as_AllBinaryGameCanvas_mut()) }
    fn run2(&mut self) { AllBinaryGameCanvas::run2(self.as_AllBinaryGameCanvas_mut()) }
    fn run3(&mut self) { AllBinaryGameCanvas::run3(self.as_AllBinaryGameCanvas_mut()) }
    fn end(&mut self) { AllBinaryGameCanvas::end(self.as_AllBinaryGameCanvas_mut()) }
    fn endGameThread(&mut self) { AllBinaryGameCanvas::endGameThread(self.as_AllBinaryGameCanvas_mut()) }
    fn createHighScore(&mut self, mut score: i64) -> Option<Rc<RefCell<HighScore>>> { AllBinaryGameCanvas::createHighScore(self.as_AllBinaryGameCanvas_mut(), score) }
    fn setHighScore(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut name: String, mut score: i64, mut autoSubmit: bool, mut isLast: bool) { AllBinaryGameCanvas::setHighScore(self.as_AllBinaryGameCanvas_mut(), abeClientInformation, name, score, autoSubmit, isLast) }
    fn setHighScore2(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut name: String, mut score: i64, mut autoSubmit: bool, mut isLast: bool) { AllBinaryGameCanvas::setHighScore2(self.as_AllBinaryGameCanvas_mut(), abeClientInformation, name, score, autoSubmit, isLast) }
    fn getGameStateTimeHelper(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>> { AllBinaryGameCanvas::getGameStateTimeHelper(self.as_AllBinaryGameCanvas_mut()) }
    fn setInitialized(&mut self, mut initialized: bool) { AllBinaryGameCanvas::setInitialized(self.as_AllBinaryGameCanvas_mut(), initialized) }
    fn isInitialized(&mut self) -> bool { AllBinaryGameCanvas::isInitialized(self.as_AllBinaryGameCanvas_mut()) }
    fn getSourceId(&mut self) -> i32 { AllBinaryGameCanvas::getSourceId(self.as_AllBinaryGameCanvas_mut()) }
    fn setProgressPaintable(&mut self, mut paintable: Option<Rc<RefCell<dyn PaintableInterface>>>) { AllBinaryGameCanvas::setProgressPaintable(self.as_AllBinaryGameCanvas_mut(), paintable) }
    fn setTouchPaintableP(&mut self, mut paintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>) { AllBinaryGameCanvas::setTouchPaintableP(self.as_AllBinaryGameCanvas_mut(), paintable) }
    fn getTouchPaintableP(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>> { AllBinaryGameCanvas::getTouchPaintableP(self.as_AllBinaryGameCanvas_mut()) }
    fn setEndGamePaintable(&mut self, mut endGamePaintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>) { AllBinaryGameCanvas::setEndGamePaintable(self.as_AllBinaryGameCanvas_mut(), endGamePaintable) }
    fn getEndGamePaintable(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>> { AllBinaryGameCanvas::getEndGamePaintable(self.as_AllBinaryGameCanvas_mut()) }
    fn setIntermissionPaintable(&mut self, mut intermissionPaintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>) { AllBinaryGameCanvas::setIntermissionPaintable(self.as_AllBinaryGameCanvas_mut(), intermissionPaintable) }
    fn getIntermissionPaintable(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>> { AllBinaryGameCanvas::getIntermissionPaintable(self.as_AllBinaryGameCanvas_mut()) }
    fn getEndGameInfoP(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::EndGameInfo::EndGameInfoDyn>>> { AllBinaryGameCanvas::getEndGameInfoP(self.as_AllBinaryGameCanvas_mut()) }
    fn setHighScoresPaintable(&mut self, mut highScoresPaintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>) { AllBinaryGameCanvas::setHighScoresPaintable(self.as_AllBinaryGameCanvas_mut(), highScoresPaintable) }
    fn getHighScoresPaintable(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>> { AllBinaryGameCanvas::getHighScoresPaintable(self.as_AllBinaryGameCanvas_mut()) }
    fn getRealHighScoresPaintable(&mut self) -> Option<Rc<RefCell<HighScoresPaintable>>> { AllBinaryGameCanvas::getRealHighScoresPaintable(self.as_AllBinaryGameCanvas_mut()) }
    fn clearPlayerGameInputList(&mut self) { AllBinaryGameCanvas::clearPlayerGameInputList(self.as_AllBinaryGameCanvas_mut()) }
    fn addPlayerGameInput(&mut self, mut playerGameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInputDyn>>>) { AllBinaryGameCanvas::addPlayerGameInput(self.as_AllBinaryGameCanvas_mut(), playerGameInput) }
    fn setMenuInputProcessor(&mut self, mut menuInputProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::BasicMenuInputProcessor::BasicMenuInputProcessorDyn>>>) { AllBinaryGameCanvas::setMenuInputProcessor(self.as_AllBinaryGameCanvas_mut(), menuInputProcessor) }
    fn getMenuInputProcessor(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::BasicMenuInputProcessor::BasicMenuInputProcessorDyn>>> { AllBinaryGameCanvas::getMenuInputProcessor(self.as_AllBinaryGameCanvas_mut()) }
    fn getMenuForm(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionFormDyn>>> { AllBinaryGameCanvas::getMenuForm(self.as_AllBinaryGameCanvas_mut()) }
    fn setMenuForm(&mut self, mut menuForm: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionFormDyn>>>) { AllBinaryGameCanvas::setMenuForm(self.as_AllBinaryGameCanvas_mut(), menuForm) }
    fn getStartLevel(&mut self) -> i32 { AllBinaryGameCanvas::getStartLevel(self.as_AllBinaryGameCanvas_mut()) }
    fn setStartLevel(&mut self, mut startLevel: i32) { AllBinaryGameCanvas::setStartLevel(self.as_AllBinaryGameCanvas_mut(), startLevel) }
    fn setTouchButtonsPaintable(&mut self, mut touchButtonsPaintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>) { AllBinaryGameCanvas::setTouchButtonsPaintable(self.as_AllBinaryGameCanvas_mut(), touchButtonsPaintable) }
    fn getTouchButtonsPaintable(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>> { AllBinaryGameCanvas::getTouchButtonsPaintable(self.as_AllBinaryGameCanvas_mut()) }
    fn setGameInputProcessor(&mut self, mut gameInputProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::canvas::Processor::ProcessorDyn>>>) { AllBinaryGameCanvas::setGameInputProcessor(self.as_AllBinaryGameCanvas_mut(), gameInputProcessor) }
    fn getGameInputProcessor(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::canvas::Processor::ProcessorDyn>>> { AllBinaryGameCanvas::getGameInputProcessor(self.as_AllBinaryGameCanvas_mut()) }
    fn setEndGameProcessor(&mut self, mut endGameProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::canvas::Processor::ProcessorDyn>>>) { AllBinaryGameCanvas::setEndGameProcessor(self.as_AllBinaryGameCanvas_mut(), endGameProcessor) }
    fn getEndGameProcessor(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::canvas::Processor::ProcessorDyn>>> { AllBinaryGameCanvas::getEndGameProcessor(self.as_AllBinaryGameCanvas_mut()) }
    fn setEndGameStatePaintable(&mut self, mut endGameStatePaintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>) { AllBinaryGameCanvas::setEndGameStatePaintable(self.as_AllBinaryGameCanvas_mut(), endGameStatePaintable) }
    fn getEndGameStatePaintable(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>> { AllBinaryGameCanvas::getEndGameStatePaintable(self.as_AllBinaryGameCanvas_mut()) }
    fn setNonBotPaintableP(&mut self, mut nonBotPaintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>) { AllBinaryGameCanvas::setNonBotPaintableP(self.as_AllBinaryGameCanvas_mut(), nonBotPaintable) }
    fn getNonBotPaintableP(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>> { AllBinaryGameCanvas::getNonBotPaintableP(self.as_AllBinaryGameCanvas_mut()) }
    fn setStartIntermissionPaintable(&mut self, mut startIntermissionPaintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::InitUpdatePaintable::InitUpdatePaintableDyn>>>) { AllBinaryGameCanvas::setStartIntermissionPaintable(self.as_AllBinaryGameCanvas_mut(), startIntermissionPaintable) }
    fn getStartIntermissionPaintable(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::InitUpdatePaintable::InitUpdatePaintableDyn>>> { AllBinaryGameCanvas::getStartIntermissionPaintable(self.as_AllBinaryGameCanvas_mut()) }
    fn setMainStateProcessor(&mut self, mut mainStateProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::canvas::Processor::ProcessorDyn>>>) { AllBinaryGameCanvas::setMainStateProcessor(self.as_AllBinaryGameCanvas_mut(), mainStateProcessor) }
    fn getMainStateProcessor(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::canvas::Processor::ProcessorDyn>>> { AllBinaryGameCanvas::getMainStateProcessor(self.as_AllBinaryGameCanvas_mut()) }
    fn setProcessGameProcessor(&mut self, mut processGameProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::canvas::Processor::ProcessorDyn>>>) { AllBinaryGameCanvas::setProcessGameProcessor(self.as_AllBinaryGameCanvas_mut(), processGameProcessor) }
    fn getProcessGameProcessor(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::canvas::Processor::ProcessorDyn>>> { AllBinaryGameCanvas::getProcessGameProcessor(self.as_AllBinaryGameCanvas_mut()) }
    fn setOpenMenuPaintable(&mut self, mut openMenuPaintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>) { AllBinaryGameCanvas::setOpenMenuPaintable(self.as_AllBinaryGameCanvas_mut(), openMenuPaintable) }
    fn getOpenMenuPaintable(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>> { AllBinaryGameCanvas::getOpenMenuPaintable(self.as_AllBinaryGameCanvas_mut()) }
    fn setPopupMenuInputProcessor(&mut self, mut popupMenuInputProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::BasicMenuInputProcessor::BasicMenuInputProcessorDyn>>>) { AllBinaryGameCanvas::setPopupMenuInputProcessor(self.as_AllBinaryGameCanvas_mut(), popupMenuInputProcessor) }
    fn getPopupMenuInputProcessor(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::BasicMenuInputProcessor::BasicMenuInputProcessorDyn>>> { AllBinaryGameCanvas::getPopupMenuInputProcessor(self.as_AllBinaryGameCanvas_mut()) }
    fn getSensorGameUpdateProcessor(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::input::gyro::SensorGameUpdateProcessor::SensorGameUpdateProcessorDyn>>> { AllBinaryGameCanvas::getSensorGameUpdateProcessor(self.as_AllBinaryGameCanvas_mut()) }
    fn getRawGameInputProcessor(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::input::InputProcessor::InputProcessorDyn>>> { AllBinaryGameCanvas::getRawGameInputProcessor(self.as_AllBinaryGameCanvas_mut()) }
    fn getRawInputProcessor(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::input::InputProcessor::InputProcessorDyn>>> { AllBinaryGameCanvas::getRawInputProcessor(self.as_AllBinaryGameCanvas_mut()) }
    fn setInputProcessor(&mut self, mut inputProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::InputProcessor::InputProcessorDyn>>>) { AllBinaryGameCanvas::setInputProcessor(self.as_AllBinaryGameCanvas_mut(), inputProcessor) }
    fn getInputProcessor(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::input::InputProcessor::InputProcessorDyn>>> { AllBinaryGameCanvas::getInputProcessor(self.as_AllBinaryGameCanvas_mut()) }
    fn setMenuPaintable(&mut self, mut menuPaintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>) { AllBinaryGameCanvas::setMenuPaintable(self.as_AllBinaryGameCanvas_mut(), menuPaintable) }
    fn getMenuPaintable(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>> { AllBinaryGameCanvas::getMenuPaintable(self.as_AllBinaryGameCanvas_mut()) }
    fn setFormPaintable(&mut self, mut formPaintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>) { AllBinaryGameCanvas::setFormPaintable(self.as_AllBinaryGameCanvas_mut(), formPaintable) }
    fn getFormPaintable(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>> { AllBinaryGameCanvas::getFormPaintable(self.as_AllBinaryGameCanvas_mut()) }
    fn setGameSpecificPaintableP(&mut self, mut gameSpecificPaintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>) { AllBinaryGameCanvas::setGameSpecificPaintableP(self.as_AllBinaryGameCanvas_mut(), gameSpecificPaintable) }
    fn getGameSpecificPaintableP(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>> { AllBinaryGameCanvas::getGameSpecificPaintableP(self.as_AllBinaryGameCanvas_mut()) }
    fn isRunningInAnotherThread(&mut self) -> bool { AllBinaryGameCanvas::isRunningInAnotherThread(self.as_AllBinaryGameCanvas_mut()) }
}
impl AllBinaryGameCanvasDyn for AllBinaryGameCanvas {
    fn as_AllBinaryGameCanvas(&self) -> &AllBinaryGameCanvas { self }
    fn as_AllBinaryGameCanvas_mut(&mut self) -> &mut AllBinaryGameCanvas { self }
}

impl crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvasDyn for AllBinaryGameCanvas {
    fn as_RunnableCanvas(&self) -> &crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvas { &self.base }
    fn as_RunnableCanvas_mut(&mut self) -> &mut crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvas { &mut self.base }
    fn initCommands(&mut self, mut cmdListener: Option<Rc<RefCell<CommandListener>>>) { Self::initCommands(self, cmdListener) }
    fn setRunning(&mut self, mut running: bool) { Self::setRunning(self, running) }
    fn setCurrentThread(&mut self) { Self::setCurrentThread(self) }
    fn processSleep(&mut self) { Self::processSleep(self) }
    fn isPausable(&mut self) -> bool { Self::isPausable(self) }
    fn isSingleThread(&mut self) -> bool { Self::isSingleThread(self) }
    fn run(&mut self) { Self::run(self) }
    fn getType(&mut self) -> i32 { Self::getType(self) }
}
impl crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn for AllBinaryGameCanvas {
    fn as_MyCanvas(&self) -> &crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas { &self.base.base }
    fn as_MyCanvas_mut(&mut self) -> &mut crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas { &mut self.base.base }
    fn pause(&mut self) { Self::pause(self) }
    fn unPause(&mut self) { Self::unPause(self) }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
    fn draw(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::draw(self, graphics) }
    fn keyPressedByDevice(&mut self, mut keyCode: i32, mut deviceId: i32) { Self::keyPressedByDevice(self, keyCode, deviceId) }
    fn keyRepeatedByDevice(&mut self, mut keyCode: i32, mut deviceId: i32) { Self::keyRepeatedByDevice(self, keyCode, deviceId) }
    fn keyReleasedByDevice(&mut self, mut keyCode: i32, mut deviceId: i32) { Self::keyReleasedByDevice(self, keyCode, deviceId) }
}


