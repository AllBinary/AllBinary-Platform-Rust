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
use crate::javax::microedition::lcdui::Command::Command;
use crate::javax::microedition::lcdui::CommandListener::CommandListener;
use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::javax::microedition::lcdui::Item::Item;
use crate::org::allbinary::J2MEUtil::J2MEUtil;
use crate::org::allbinary::animation::Animation::Animation;
use crate::org::allbinary::animation::IndexedAnimationBehavior::IndexedAnimationBehavior;
use crate::org::allbinary::animation::special::SpecialAnimation::SpecialAnimation;
use crate::org::allbinary::business::advertisement::GameAdStateFactory::GameAdStateFactory;
use crate::org::allbinary::canvas::BaseGameStatistics::BaseGameStatistics;
use crate::org::allbinary::canvas::CustomGameMenuUtil::CustomGameMenuUtil;
use crate::org::allbinary::canvas::FullScreenUtil::FullScreenUtil;
use crate::org::allbinary::canvas::GameStatisticsFactory::GameStatisticsFactory;
use crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvas;
use crate::org::allbinary::game::GameAdState::GameAdState;
use crate::org::allbinary::game::commands::GameCommandsFactory::GameCommandsFactory;
use crate::org::allbinary::game::configuration::event::ChangedGameFeatureListener::ChangedGameFeatureListener;
use crate::org::allbinary::game::configuration::feature::Features::Features;
use crate::org::allbinary::game::configuration::feature::GameFeatureFactory::GameFeatureFactory;
use crate::org::allbinary::game::configuration::feature::InputFeatureFactory::InputFeatureFactory;
use crate::org::allbinary::game::configuration::feature::MainFeatureFactory::MainFeatureFactory;
use crate::org::allbinary::game::init::BasicBuildGameInitializerFactory::BasicBuildGameInitializerFactory;
use crate::org::allbinary::game::init::GameInitializationUtil::GameInitializationUtil;
use crate::org::allbinary::game::input::GameInputStrings::GameInputStrings;
use crate::org::allbinary::game::input::GameKey::GameKey;
use crate::org::allbinary::game::input::GameKeyFactory::GameKeyFactory;
use crate::org::allbinary::game::input::PlatformInputMappingFactory::PlatformInputMappingFactory;
use crate::org::allbinary::game::input::event::DownGameKeyEventHandler::DownGameKeyEventHandler;
use crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent;
use crate::org::allbinary::game::input::event::GameKeyEventFactory::GameKeyEventFactory;
use crate::org::allbinary::game::input::event::GameKeyEventHandler::GameKeyEventHandler;
use crate::org::allbinary::game::input::event::UpGameKeyEventHandler::UpGameKeyEventHandler;
use crate::org::allbinary::game::input::mapping::InputToGameKeyMapping::InputToGameKeyMapping;
use crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManager;
use crate::org::allbinary::game::layer::SWTUtil::SWTUtil;
use crate::org::allbinary::game::midlet::DemoGameMidlet::DemoGameMidlet;
use crate::org::allbinary::game::score::HighScoreCommands::HighScoreCommands;
use crate::org::allbinary::game::score::HighScores::HighScores;
use crate::org::allbinary::game::score::HighScoresFactoryInterface::HighScoresFactoryInterface;
use crate::org::allbinary::game::score::HighScoresHelper::HighScoresHelper;
use crate::org::allbinary::game::score::HighScoresPaintable::HighScoresPaintable;
use crate::org::allbinary::game::score::NullHighScoresSingletonFactory::NullHighScoresSingletonFactory;
use crate::org::allbinary::game::state::GameState::GameState;
use crate::org::allbinary::game::state::GameStateFactory::GameStateFactory;
use crate::org::allbinary::graphics::Rectangle::Rectangle;
use crate::org::allbinary::graphics::ResizableListenerHandler::ResizableListenerHandler;
use crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvas;
use crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvasFactory::ProgressCanvasFactory;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::graphics::displayable::command::MyCommandsFactory::MyCommandsFactory;
use crate::org::allbinary::graphics::displayable::event::DisplayChangeEvent::DisplayChangeEvent;
use crate::org::allbinary::graphics::displayable::event::DisplayChangeEventHandler::DisplayChangeEventHandler;
use crate::org::allbinary::graphics::displayable::event::DisplayChangeEventListener::DisplayChangeEventListener;
use crate::org::allbinary::graphics::form::FormType::FormType;
use crate::org::allbinary::graphics::form::FormTypeFactory::FormTypeFactory;
use crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionForm;
use crate::org::allbinary::graphics::opengles::CurrentDisplayableFactory::CurrentDisplayableFactory;
use crate::org::allbinary::graphics::opengles::OpenGLFeatureFactory::OpenGLFeatureFactory;
use crate::org::allbinary::graphics::opengles::OpenGLFeatureUtil::OpenGLFeatureUtil;
use crate::org::allbinary::graphics::opengles::OpenGLThreadUtil::OpenGLThreadUtil;
use crate::org::allbinary::graphics::paint::InitUpdatePaintable::InitUpdatePaintable;
use crate::org::allbinary::graphics::paint::NullPaintable::NullPaintable;
use crate::org::allbinary::graphics::paint::Paintable::Paintable;
use crate::org::allbinary::graphics::paint::PaintableInterface::PaintableInterface;
use crate::org::allbinary::graphics::paint::StatePaintable::StatePaintable;
use crate::org::allbinary::graphics::paint::StatePaintableFactory::StatePaintableFactory;
use crate::org::allbinary::input::motion::gesture::observer::BasicMotionGesturesHandler::BasicMotionGesturesHandler;
use crate::org::allbinary::logic::communication::log::ForcedLogUtil::ForcedLogUtil;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::logic::math::SmallIntegerSingletonFactory::SmallIntegerSingletonFactory;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::system::os::OperatingSystemFactory::OperatingSystemFactory;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::logic::system::security::licensing::InApplicationPurchaseFactory::InApplicationPurchaseFactory;
use crate::org::allbinary::logic::system::security::licensing::LockableFeature::LockableFeature;
use crate::org::allbinary::logic::system::security::licensing::LockableFeatureFactory::LockableFeatureFactory;
use crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject;
use crate::org::allbinary::logic::util::event::EventStrings::EventStrings;
use crate::org::allbinary::media::audio::AllBinaryMediaManager::AllBinaryMediaManager;
use crate::org::allbinary::media::audio::EarlySoundsFactory::EarlySoundsFactory;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelper;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::java::lang::Exception::*;
use super::GameCanvasRunnableInterface::GameCanvasRunnableInterface;
use super::MenuListener::MenuListener;
use super::FormUtil::FormUtil;
use super::BasicMenuInputProcessor::BasicMenuInputProcessor;
use super::GameRunnable::GameRunnable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct StartCanvas {
    pub base: RunnableCanvas,
    pub basicColorFactory: Option<Rc<RefCell<BasicColorFactory>>>,
    pub features: Option<Rc<RefCell<Features>>>,
    pub openGLFeatureFactory: Option<Rc<RefCell<OpenGLFeatureFactory>>>,
    pub myCommandsFactory: Option<Rc<RefCell<MyCommandsFactory>>>,
    pub gameAdStateFactory: Option<Rc<RefCell<GameAdStateFactory>>>,
    pub gameStateFactory: Option<Rc<RefCell<GameStateFactory>>>,
    pub gameInputStrings: Option<Rc<RefCell<GameInputStrings>>>,
    pub formUtil: Option<Rc<RefCell<FormUtil>>>,
    pub basicGameDemoPaintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::StatePaintable::StatePaintableDyn>>>,
    pub specialAnimationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::special::SpecialAnimation::SpecialAnimationDyn>>>,
    pub paintedSpecialAnimationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>,
    pub fullScreenUtil: Option<Rc<RefCell<FullScreenUtil>>>,
    pub highScoresHelper: Option<Rc<RefCell<HighScoresHelper>>>,
    pub realHighScoresPaintable: Option<Rc<RefCell<HighScoresPaintable>>>,
    pub highScoresPaintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>,
    pub state: i32,
    pub timeDelayHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>>,
    pub highScoresFactoryInterface: Option<Rc<RefCell<dyn HighScoresFactoryInterface>>>,
    pub menuInputProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::BasicMenuInputProcessor::BasicMenuInputProcessorDyn>>>,
    pub menuForm: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionFormDyn>>>,
    pub gameInitializationInterfaceFactoryInterface: Option<Rc<RefCell<BasicBuildGameInitializerFactory>>>,
    pub initialized: bool,
    pub defaultPaintableInterface: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>,
    pub paintableInterface: Option<Rc<RefCell<dyn PaintableInterface>>>,
    pub overlayPaintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::InitUpdatePaintable::InitUpdatePaintableDyn>>>,
    pub tempWait: i32,
    pub inputToGameKeyMapping: Option<Rc<RefCell<InputToGameKeyMapping>>>,
    pub gameRunnable: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::GameRunnable::GameRunnableDyn>>>,
    pub progressPaintable: Option<Rc<RefCell<dyn PaintableInterface>>>,
    pub isSingleKeyRepeatableProcessing: bool,
    pub NONE: Option<Rc<RefCell<GameKey>>>,
    pub gameKeyEventFactory: Option<Rc<RefCell<GameKeyEventFactory>>>,
    pub SET_STATE: String,
    pub baseGameStatistics: Option<Rc<RefCell<dyn crate::org::allbinary::canvas::BaseGameStatistics::BaseGameStatisticsDyn>>>,
}

pub static StartCanvas_id: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 0);
pub static StartCanvas_BOT_GAME_STATS: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Bot Game Statistics: "));
// impl GameCanvasRunnableInterface for StartCanvas with the methods of the trait
impl MenuListener for StartCanvas {
    fn open(&mut self) { Self::open(self) }
    fn close(&mut self) { Self::close(self) }
}
impl crate::org::allbinary::game::input::GameKeyEventSourceInterface::GameKeyEventSourceInterface for StartCanvas {
    fn getSourceId(&mut self) -> i32 { Self::getSourceId(self) }
}
impl DisplayChangeEventListener for StartCanvas {
    fn onDisplayChangeEvent(&mut self, mut displayChangeEvent: Option<Rc<RefCell<DisplayChangeEvent>>>) { Self::onDisplayChangeEvent(self, displayChangeEvent) }
}
impl crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface for StartCanvas {
    fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { Self::onEvent(self, eventObject) }
}
impl crate::org::allbinary::thread::RunnableInterface::RunnableInterface for StartCanvas {
    fn isRunning(&mut self) -> bool { crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvas::isRunning(crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvasDyn::as_RunnableCanvas_mut(self)) }
    fn setRunning(&mut self, mut running: bool) { Self::setRunning(self, running) }
    fn setThread(&mut self, p1: Option<Rc<RefCell<crate::java::lang::Thread::Thread>>>) { crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvas::setThread(crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvasDyn::as_RunnableCanvas_mut(self), p1) }
}
impl crate::Runnable for StartCanvas {
    fn run(&mut self) { Self::run(self) }
}

impl StartCanvas {
    

    pub fn new_struct(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut commandListener: Option<Rc<RefCell<dyn CommandListener>>>, mut highScoresFactoryInterface: Option<Rc<RefCell<dyn HighScoresFactoryInterface>>>, mut paintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>, mut overlayPaintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::InitUpdatePaintable::InitUpdatePaintableDyn>>>, mut gameInitializationInterfaceFactoryInterface: Option<Rc<RefCell<BasicBuildGameInitializerFactory>>>, mut isContinue: bool) -> Self {
        let mut this = Self {
            base: RunnableCanvas::new_struct(commandListener.clone(), { let java_field = jcall!(CurrentDisplayableFactory::getInstance()).DEFAULT_CHILD_NAME_LIST.clone(); java_field }.clone(), true),
            basicColorFactory: None,
            features: None,
            openGLFeatureFactory: None,
            myCommandsFactory: None,
            gameAdStateFactory: None,
            gameStateFactory: None,
            gameInputStrings: None,
            formUtil: None,
            basicGameDemoPaintable: None,
            specialAnimationInterface: None,
            paintedSpecialAnimationInterface: None,
            fullScreenUtil: None,
            highScoresHelper: None,
            realHighScoresPaintable: None,
            highScoresPaintable: None,
            state: 0,
            timeDelayHelper: None,
            highScoresFactoryInterface: None,
            menuInputProcessor: None,
            menuForm: None,
            gameInitializationInterfaceFactoryInterface: None,
            initialized: false,
            defaultPaintableInterface: None,
            paintableInterface: None,
            overlayPaintable: None,
            tempWait: 0,
            inputToGameKeyMapping: None,
            gameRunnable: None,
            progressPaintable: None,
            isSingleKeyRepeatableProcessing: false,
            NONE: None,
            gameKeyEventFactory: None,
            SET_STATE: String::new(),
            baseGameStatistics: None,
        };
        this.basicColorFactory = BasicColorFactory::getInstance();
        this.features = Features::getInstance();
        this.openGLFeatureFactory = OpenGLFeatureFactory::getInstance();
        this.myCommandsFactory = MyCommandsFactory::getInstance();
        this.gameAdStateFactory = GameAdStateFactory::getInstance();
        this.gameStateFactory = GameStateFactory::getInstance();
        this.gameInputStrings = GameInputStrings::getInstance();
        this.formUtil = FormUtil::getInstance();
        this.basicGameDemoPaintable = StatePaintableFactory::getInstance();
        this.specialAnimationInterface = SpecialAnimation::getInstance();
        this.paintedSpecialAnimationInterface = SpecialAnimation::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>);
        this.fullScreenUtil = FullScreenUtil::getInstance();
        this.highScoresHelper = HighScoresHelper::new();
        this.realHighScoresPaintable = HighScoresPaintable::new();
        this.highScoresPaintable = NullPaintable::getInstance();
        this.state = 0;
        this.timeDelayHelper = TimeDelayHelper::new(6000).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>);
        this.menuInputProcessor = NoMenuInputProcessor::NoMenuInputProcessor::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::BasicMenuInputProcessor::BasicMenuInputProcessorDyn>>);
        this.menuForm = ScrollSelectionForm::getNullScrollSelectionForm();
        this.defaultPaintableInterface = NullPaintable::getInstance();
        this.paintableInterface = NullPaintable::getInstance().map(|reference| reference as Rc<RefCell<dyn PaintableInterface>>);
        this.tempWait = { let java_field = jcall!(NullWaitGameRunnable::NullWaitGameRunnable::getInstance()).as_GameRunnable_mut().WAIT.clone(); java_field }.clone();
        this.inputToGameKeyMapping = jcall!(jcall!(PlatformInputMappingFactory::getInstance()).getPersistentInputMappingInstance()).getInputMapping();
        this.gameRunnable = NullWaitGameRunnable::NullWaitGameRunnable::getInstance();
        this.progressPaintable = ProgressCanvasFactory::getLazyInstance();
        this.isSingleKeyRepeatableProcessing = { let java_receiver = this.features.clone(); let java_arguments = ({ let java_field = jcall!(InputFeatureFactory::getInstance()).SINGLE_KEY_REPEAT_PRESS.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>),); let java_result = jcall!(java_receiver).isFeature(java_arguments.0); java_result };
        this.NONE = { let java_field = jcall!(GameKeyFactory::getInstance()).NONE.clone(); java_field }.clone();
        this.gameKeyEventFactory = GameKeyEventFactory::getInstance();
        this.SET_STATE = String::from("setState");
        this.baseGameStatistics = GameStatisticsFactory::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::canvas::BaseGameStatistics::BaseGameStatisticsDyn>>);
        // super(...) is in the struct literal above
        { let java_arguments = ({ let java_field = jcall!(NullWaitGameRunnable::NullWaitGameRunnable::getInstance()).as_GameRunnable_mut().WAIT.clone(); java_field }.clone(),); this.setWait(java_arguments.0) };
        this.gameInitializationInterfaceFactoryInterface = gameInitializationInterfaceFactoryInterface.clone();
        this
    }

    pub fn new(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut commandListener: Option<Rc<RefCell<dyn CommandListener>>>, mut highScoresFactoryInterface: Option<Rc<RefCell<dyn HighScoresFactoryInterface>>>, mut paintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>, mut overlayPaintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::InitUpdatePaintable::InitUpdatePaintableDyn>>>, mut gameInitializationInterfaceFactoryInterface: Option<Rc<RefCell<BasicBuildGameInitializerFactory>>>, mut isContinue: bool) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(abeClientInformation.clone(), commandListener.clone(), highScoresFactoryInterface.clone(), paintable.clone(), overlayPaintable.clone(), gameInitializationInterfaceFactoryInterface.clone(), isContinue.clone()))));
            jcall!(GameInitializationUtil::getInstance()).initDemo(abeClientInformation.clone(), this.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn>>), gameInitializationInterfaceFactoryInterface.clone());
            jcall!(ResizableListenerHandler::getInstance()).fireEvent(false);
            jcall!(this).overlayPaintable = overlayPaintable.clone();
            jcall!(this).highScoresFactoryInterface = highScoresFactoryInterface.clone();
            jcall!(this).setDefaultPaintableInterface(paintable.clone());
            { let java_arguments = (jcall!(this).getDefaultPaintableInterface().map(|reference| reference as Rc<RefCell<dyn PaintableInterface>>),); jcall!(this).setPaintableInterface(java_arguments.0) };
            if isContinue {
            { let java_arguments = ({ let java_field = jcall!(GameCommandsFactory::getInstance()).CONTINUE_COMMAND.clone(); java_field }.clone(),); jcall!(this).addCommand(java_arguments.0) };
        }
            if jcall!(ChangedGameFeatureListener::getInstance()).isChangedFeature({ let java_field = jcall!(GameFeatureFactory::getInstance()).SOUND.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
            jcall!(this).mediaInit();
        }
            jcall!(DisplayChangeEventHandler::getInstance()).addListenerInterface(this.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface>>));
        this
    }
    

    pub fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) {
        ForcedLogUtil::log({ let java_field = jcall!(EventStrings::getInstance()).PERFORMANCE_MESSAGE.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())));
    }
    

    pub fn onDisplayChangeEvent(&mut self, mut displayChangeEvent: Option<Rc<RefCell<DisplayChangeEvent>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut scrollSelectionForm: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionFormDyn>>> = self.getMenuForm();
            if scrollSelectionForm.is_some() {
                let mut formType: Option<Rc<RefCell<FormType>>> = jcall!(FormTypeFactory::getInstance()).getFormType();
                let mut rectangle: Option<Rc<RefCell<Rectangle>>> = jcall!(self.formUtil).createFormRectangle();
                jcall!(scrollSelectionForm).init(rectangle.clone(), formType.clone());
            }
            jcall!(self.overlayPaintable).init();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.canvasStrings).ON_DISPLAY_CHANGE_EVENT.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn getCustomCommands(&mut self) -> Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> {
        let mut gameCommandsFactory: Option<Rc<RefCell<GameCommandsFactory>>> = GameCommandsFactory::getInstance();
        if J2MEUtil::isHTML() {
            let mut commandArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = vec![crate::java_into_object({ let java_field = jcall!(gameCommandsFactory).START_COMMAND.clone(); java_field }.clone()), crate::java_into_object({ let java_field = jcall!(HighScoreCommands::getInstance()).DISPLAY.clone(); java_field }.clone()), crate::java_into_object(GameInputMappingCanvas_DISPLAY.with(Clone::clone).clone()), crate::java_into_object({ let java_field = jcall!(gameCommandsFactory).DISPLAY_ABOUT.clone(); java_field }.clone())];
            return commandArray.clone();
        } else {
            let mut commandList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            jcall!(commandList).add(crate::java_into_object({ let java_field = jcall!(gameCommandsFactory).START_COMMAND.clone(); java_field }.clone()));
            let mut inApplicationPurchaseFactory: Option<Rc<RefCell<InApplicationPurchaseFactory>>> = InApplicationPurchaseFactory::getInstance();
            if jcall!(inApplicationPurchaseFactory).isEnabled() {
                let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(LockableFeatureFactory::getInstance()).getList();
                if jcall!(list).size() > 0 && !jcall!(inApplicationPurchaseFactory).isPurchased({ let java_object = (jcall!(list).get(0)).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<LockableFeature>>>>().expect("ClassCastException").clone()) }) {
                    jcall!(commandList).add(crate::java_into_object({ let java_field = jcall!(gameCommandsFactory).BUY_COMMAND.clone(); java_field }.clone()));
                }
            }
            jcall!(commandList).add(crate::java_into_object({ let java_field = jcall!(HighScoreCommands::getInstance()).DISPLAY.clone(); java_field }.clone()));
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let mut isOverScan: bool = jcall!(jcall!(OperatingSystemFactory::getInstance()).getOperatingSystemInstance()).as_GenericOperatingSystem_mut().isOverScan();
                if unsafe { crate::org::allbinary::game::layer::SWTUtil::SWTUtil_isSWT.clone() } {
                    jcall!(commandList).add(crate::java_into_object(GameInputMappingCanvas_DISPLAY.with(Clone::clone).clone()));
                } else if !isOverScan {
                    jcall!(commandList).add(crate::java_into_object({ let java_field = jcall!(gameCommandsFactory).DISPLAY_OPTIONS.clone(); java_field }.clone()));
                    jcall!(commandList).add(crate::java_into_object({ let java_field = jcall!(gameCommandsFactory).DISPLAY_LOAD_FORM.clone(); java_field }.clone()));
                    jcall!(commandList).add(crate::java_into_object(GameInputMappingCanvas_DISPLAY.with(Clone::clone).clone()));
                }
            })) {
                Ok(_) => {}
                Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

                }
            }
            jcall!(commandList).add(crate::java_into_object({ let java_field = jcall!(gameCommandsFactory).DISPLAY_ABOUT.clone(); java_field }.clone()));
            let mut commandArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(commandList).toArray();
            return commandArray.clone();
        }
    }
    

    pub fn initCommands(&mut self, mut cmdListener: Option<Rc<RefCell<dyn CommandListener>>>) {
        self.removeAllCommands();
        let mut commandArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = self.getCustomCommands();
        let mut size: i32 = (commandArray.len() as i32).clone();
        for index in (0..size) {
            self.addCommand({ let java_object = (commandArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>>().expect("ClassCastException").clone()) });
        }
        CustomGameMenuUtil::add(None /* Java this: a &mut self method has no Rc of itself */);
        self.setCommandListener(cmdListener.clone());
    }
    

    // throws Exception
    pub fn initPostPaint(&mut self) {
    }
    

    // throws Exception
    pub fn mediaInit(&mut self) {
        AllBinaryMediaManager::init(EarlySoundsFactory::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::media::audio::SoundsFactoryInterface::SoundsFactoryInterface>>));
    }
    

    pub fn itemStateChanged(&mut self, mut item: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Item::ItemDyn>>>) {
        ForcedLogUtil::log({ let java_field = jcall!(self.commonStrings).NOT_IMPLEMENTED.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())));
    }
    

    // throws Exception
    pub fn initMenu(&mut self) {
        self.close();
        self.open();
    }
    

    pub fn open(&mut self) {
        jcall!(BasicMotionGesturesHandler::getInstance()).addListenerInterface(self.getMenuInputProcessor().map(|_java_reference| -> Rc<RefCell<dyn crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface>> { panic!("ClassCastException: the Rust trait of BasicMenuInputProcessor does not have EventListenerInterface as a base trait") }));
        jcall!(GameKeyEventHandler::getInstance()).addListener(self.getMenuInputProcessor().map(|_java_reference| -> Rc<RefCell<dyn crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface>> { panic!("ClassCastException: the Rust trait of BasicMenuInputProcessor does not have EventListenerInterface as a base trait") }));
    }
    

    pub fn close(&mut self) {
        jcall!(BasicMotionGesturesHandler::getInstance()).removeListener(self.getMenuInputProcessor().map(|_java_reference| -> Rc<RefCell<dyn crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface>> { panic!("ClassCastException: the Rust trait of BasicMenuInputProcessor does not have EventListenerInterface as a base trait") }));
        jcall!(GameKeyEventHandler::getInstance()).removeListener(self.getMenuInputProcessor().map(|_java_reference| -> Rc<RefCell<dyn crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface>> { panic!("ClassCastException: the Rust trait of BasicMenuInputProcessor does not have EventListenerInterface as a base trait") }));
    }
    

    pub fn getSourceId(&mut self) -> i32 {
        return StartCanvas_id.with(Clone::clone).clone();
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
        self.addGameKeyEvent(keyCode.clone(), false);
    }
    

    pub fn keyReleasedByDevice(&mut self, mut keyCode: i32, mut deviceId: i32) {
        self.removeGameKeyEvent(keyCode.clone(), false);
    }
    

    pub fn keyRepeatedByDevice(&mut self, mut keyCode: i32, mut deviceId: i32) {
        if self.isSingleKeyRepeatableProcessing {
            self.addGameKeyEvent(keyCode.clone(), true);
        }
    }
    

    pub fn addGameKeyEvent(&mut self, mut keyCode: i32, mut repeated: bool) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut gameKey: Option<Rc<RefCell<GameKey>>> = jcall!(self.inputToGameKeyMapping).getInstanceForCanvas(None /* Java this: a &mut self method has no Rc of itself */, keyCode.clone());
            if !match (&gameKey, &self.NONE) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                let mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>> = jcall!(self.gameKeyEventFactory).getInstanceForInput(None /* Java this: a &mut self method has no Rc of itself */, gameKey.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>));
                jcall!(DownGameKeyEventHandler::getInstance()).fireEvent(gameKeyEvent.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>));
            } else {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append({ let java_field = jcall!(self.gameInputStrings).NO_KEY.clone(); java_field }.clone()).appendint(keyCode.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.gameInputStrings).ADD_KEY_EVENT.clone(); java_field }.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put(String::from("Key Event Error"), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.gameInputStrings).ADD_KEY_EVENT.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn removeGameKeyEvent(&mut self, mut keyCode: i32, mut repeated: bool) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut gameKey: Option<Rc<RefCell<GameKey>>> = jcall!(self.inputToGameKeyMapping).getInstanceForCanvas(None /* Java this: a &mut self method has no Rc of itself */, keyCode.clone());
            if !match (&gameKey, &self.NONE) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                let mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>> = jcall!(self.gameKeyEventFactory).getInstanceForInput(None /* Java this: a &mut self method has no Rc of itself */, gameKey.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>));
                jcall!(UpGameKeyEventHandler::getInstance()).fireEvent(gameKeyEvent.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>));
            } else {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append({ let java_field = jcall!(self.gameInputStrings).NO_KEY.clone(); java_field }.clone()).appendint(keyCode.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.gameInputStrings).REMOVE_KEY_EVENT.clone(); java_field }.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put(String::from("Key Event Error"), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.gameInputStrings).REMOVE_KEY_EVENT.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    // synchronized
    pub fn pause(&mut self) {
        self.close();
        self.setPaused(true);
    }
    

    // synchronized
    pub fn unPause(&mut self) {
        self.open();
        self.setPaused(false);
    }
    

    pub fn isPausable(&mut self) -> bool {
        if match (&jcall!(CurrentDisplayableFactory::getInstance()).getUsedRunnable(), &NullWaitGameRunnable::NullWaitGameRunnable::getInstance()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return true;
        } else {
            return false;
        }
    }
    

    pub fn isGameOver(&mut self) -> bool {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append({ let java_field = jcall!(self.commonStrings).NOT_IMPLEMENTED.clone(); java_field }.clone()).append(String::from(" since not a game")).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("isGameOver"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        return false;
    }
    

    // throws Exception
    pub fn setLoadStateHashtable(&mut self, mut hashtable: Option<Rc<RefCell<ABHashtable<crate::JavaObject, crate::JavaObject>>>>) {
        jcall!(self.logUtil).putF(String::from("Trying to continue a demo lol - only continue a game canvas not the demo"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setLoadStateHashtable"));
    }
    

    // throws Exception
    pub fn getLoadStateHashtable(&mut self) -> Option<Rc<RefCell<ABHashtable<crate::JavaObject, crate::JavaObject>>>> {
        jcall!(self.logUtil).putF(String::from("Trying to continue a demo lol - only continue a game canvas not the demo"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getLoadStateHashtable"));
        return { let java_field = jcall!(self.stdUtil).NULL_TABLE.clone(); java_field }.clone();
    }
    

    // throws Exception
    pub fn getCurrentStateHashtable(&mut self) -> Option<Rc<RefCell<ABHashtable<crate::JavaObject, crate::JavaObject>>>> {
        jcall!(self.logUtil).putF(String::from("Trying to save the AI lol"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getCurrentStateHashtable"));
        return { let java_field = jcall!(self.stdUtil).NULL_TABLE.clone(); java_field }.clone();
    }
    

    pub fn setHighScoreSubmitted(&mut self, mut isNotUsed: bool) {
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
        jcall!(self.paintableInterface).paint(graphics.clone());
        jcall!(self.paintedSpecialAnimationInterface).paintXY(graphics.clone(), 0, 0);
        jcall!(self.highScoresPaintable).as_Paintable_mut().paint(graphics.clone());
        jcall!(self.getBasicGameDemoPaintable()).as_StatePaintable_mut().paint(graphics.clone());
        jcall!(self.overlayPaintable).as_InitUpdatePaintable_mut().paint(graphics.clone());
        jcall!(self.progressPaintable).paint(graphics.clone());
    }
    

    pub fn paintThreed(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
        jcall!(self.paintableInterface).paintThreed(graphics.clone());
        jcall!(self.paintedSpecialAnimationInterface).paintThreedXYZ(graphics.clone(), 0, 0, 0);
    }
    

    // synchronized
    pub fn processGameOver(&mut self) {
        jcall!(self.logUtil).putF(String::from("Not Implemented since not a game"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setGameOver"));
    }
    

    pub fn demoStateChange(&mut self) {
        let mut newState: i32 = self.state + 1;
        if newState > 2 {
            newState = 0;
        } else if newState == 2 {
            if !jcall!(self.highScoresHelper).isAnyHighScores() {
                newState = 0;
            } else {
                let mut highScores: Option<Rc<RefCell<dyn crate::org::allbinary::game::score::HighScores::HighScoresDyn>>> = jcall!(self.highScoresHelper).getNextHighScores();
                if match (&highScores, &NullHighScoresSingletonFactory::getInstance()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                    newState = 0;
                } else {
                    jcall!(self.getRealHighScoresPaintable()).setHighScores(highScores.clone());
                }
            }
        }
        self.setState(newState.clone());
        self.updateDemoState();
    }
    

    pub fn updateDemoState(&mut self) {
        PreLogUtil::put((jcall!(SmallIntegerSingletonFactory::getInstance()).createInstance(self.state.clone())).expect("NullPointerException").to_string(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.SET_STATE.clone());
        jcall!(self.getBasicGameDemoPaintable()).as_StatePaintable_mut().setState(self.state.clone());
        if self.state == 0 {
            self.highScoresPaintable = { let java_value = (NullPaintable::getInstance()).clone(); java_value };
            self.paintedSpecialAnimationInterface = self.getSpecialAnimationInterface().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>);
            jcall!(self.getSpecialAnimationInterface()).reset();
        } else if self.state == 1 {
            self.paintedSpecialAnimationInterface = { let java_value = (SpecialAnimation::getInstance()).clone(); java_value }.map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>);
        } else if self.state == 2 {
            self.highScoresPaintable = self.getRealHighScoresPaintable().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>);
        }
        let mut gameAdState: Option<Rc<RefCell<GameAdState>>> = jcall!(self.gameAdStateFactory).getCurrentInstance();
        jcall!(gameAdState).processPageAdState();
    }
    

    // throws Exception
    pub fn getNextRandom(&mut self) -> i32 {
        panic!("{:?}", format!("{}: {:?}", "Exception", ({ let java_field = jcall!(self.commonStrings).NOT_IMPLEMENTED.clone(); java_field }.clone())));
    }
    

    // throws Exception
    pub fn createGameLayerManager(&mut self, mut randomValue: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>> {
        panic!("{:?}", format!("{}: {:?}", "Exception", ({ let java_field = jcall!(self.commonStrings).NOT_IMPLEMENTED.clone(); java_field }.clone())));
    }
    

    // throws Exception
    pub fn createRunnable(&mut self, mut randomLevel: i32) -> Option<Rc<RefCell<dyn GameCanvasRunnableInterface>>> {
        panic!("{:?}", format!("{}: {:?}", "Exception", ({ let java_field = jcall!(self.commonStrings).NOT_IMPLEMENTED.clone(); java_field }.clone())));
    }
    

    // throws Exception
    pub fn createCanvas(&mut self) {
        self.highScoresPaintable = { let java_value = (NullPaintable::getInstance()).clone(); java_value };
        let mut randomLevel: i32 = self.getNextRandom();
    }
    

    // throws Exception
    pub fn start(&mut self) {
    }
    

    pub fn preDemoProcess(&mut self) {
        jcall!(self.overlayPaintable).update();
    }
    

    // throws Exception
    pub fn process(&mut self) {
        jcall!(self.getMenuInputProcessor()).processInputList();
        self.preDemoProcess();
        if self.state == 0 {
            let mut indexedAnimationBehavior: Option<Rc<RefCell<IndexedAnimationBehavior>>> = (jcall!(self.getSpecialAnimationInterface()).getAnimationBehavior()).clone().map(|reference| { assert!({ let java_borrowed = reference.borrow(); let java_any: &dyn std::any::Any = &*java_borrowed; java_any.is::<IndexedAnimationBehavior>() }, "ClassCastException: not a IndexedAnimationBehavior"); unsafe { Rc::from_raw(Rc::into_raw(reference) as *const RefCell<IndexedAnimationBehavior>) } });
            if { let java_field = jcall!(indexedAnimationBehavior).loopIndex.clone(); java_field } < 1 {
                jcall!(self.timeDelayHelper).setStartTimeTNT();
            }
            let mut demoGameMidlet: Option<Rc<RefCell<dyn crate::org::allbinary::game::midlet::DemoGameMidlet::DemoGameMidletDyn>>> = (|| -> Option<Rc<RefCell<dyn crate::org::allbinary::game::midlet::DemoGameMidlet::DemoGameMidletDyn>>> { panic!("ClassCastException: a Rust dyn CommandListener cannot be cast to DemoGameMidlet") })();
        }
    }
    

    // throws Exception
    pub fn startDemoGame(&mut self) {
    }
    

    // throws Exception
    pub fn processGame(&mut self) {
        jcall!(self.gameRunnable).as_GameRunnable_mut().run();
        jcall!(self.specialAnimationInterface).nextFrame();
        self.process();
    }
    

    pub fn run(&mut self) {
        jcall!(self.logUtil).putF({ let java_field = jcall!(self.commonStrings).START_RUNNABLE.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).RUN.clone(); java_field }.clone());
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut progressCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvasDyn>>> = ProgressCanvasFactory::getInstance();
            jcall!(progressCanvas).addNormalPortion(50, String::from("Demo Thread"));
            self.setCurrentThread();
            self.setRunning(true);
            if { let java_receiver = self.features.clone(); let java_arguments = ({ let java_field = jcall!(MainFeatureFactory::getInstance()).LOAD_ONDEMAND.clone(); java_field }.clone(),); let java_result = jcall!(java_receiver).isFeature(java_arguments.0); java_result } {
                jcall!(progressCanvas).end();
            } else {
                jcall!(progressCanvas).addNormalPortion(50, String::from("Demo Thread Running"));
            }
            { let java_receiver = self.fullScreenUtil.clone(); let java_arguments = (None /* Java this: a &mut self method has no Rc of itself */, self.getCustomCommandListener(),); jcall!(java_receiver).initOnRun(java_arguments.0, java_arguments.1); };
            self.initMenu();
            self.initPostPaint();
            self.updateDemoState();
            if jcall!(self.features).isDefault({ let java_field = jcall!(self.openGLFeatureFactory).OPENGL_AS_GAME_THREAD.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
                jcall!(OpenGLThreadUtil::getInstance()).onResume();
            }
            if jcall!(self.features).isDefault({ let java_field = jcall!(self.openGLFeatureFactory).OPENGL_AS_GAME_THREAD.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) || J2MEUtil::isHTML() {
            } else {
                while self.isRunning() {
                    jcall!(self.loopTimeHelper).setStartTimeTNT();
                    self.processGame();
                    self.processLoopSleep();
                }
                self.end();
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).RUN.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
        }
        jcall!(self.logUtil).putF({ let java_field = jcall!(self.commonStrings).END_RUNNABLE.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).RUN.clone(); java_field }.clone());
    }
    

    pub fn setRunning(&mut self, mut running: bool) {
        { let java_arguments = (running.clone(),); self.base.setRunning(java_arguments.0) };
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if (jcall!(self.features).isDefault({ let java_field = jcall!(self.openGLFeatureFactory).OPENGL.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) || J2MEUtil::isHTML()) && !running {
                let mut currentDisplayableFactory: Option<Rc<RefCell<CurrentDisplayableFactory>>> = CurrentDisplayableFactory::getInstance();
                jcall!(currentDisplayableFactory).clearRunnable();
                self.end();
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.SET_RUNNING.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    // throws Exception
    pub fn end(&mut self) {
        let mut progressCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvasDyn>>> = ProgressCanvasFactory::getInstance();
        { let java_receiver = self.baseGameStatistics.clone(); let java_arguments = (jcall!(StringMaker::new()).append(StartCanvas_BOT_GAME_STATS.with(Clone::clone).clone()).append(jcall!(self.baseGameStatistics).toString()).append({ let java_field = jcall!(CommonSeps::getInstance()).NEW_LINE.clone(); java_field }.clone()).toString(),); jcall!(java_receiver).add(java_arguments.0); };
        jcall!(self.baseGameStatistics).init();
        if { let java_receiver = self.features.clone(); let java_arguments = ({ let java_field = jcall!(MainFeatureFactory::getInstance()).LOAD_ONDEMAND.clone(); java_field }.clone(),); let java_result = jcall!(java_receiver).isFeature(java_arguments.0); java_result } {
            jcall!(progressCanvas).start();
        }
        jcall!(self.logUtil).putF(String::from("Demo End"), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).RUN.clone(); java_field }.clone());
        self.close();
        jcall!(DisplayChangeEventHandler::getInstance()).removeListener(None /* Java this: a &mut self method has no Rc of itself */);
    }
    

    pub fn setGameState(&mut self, mut gameState: Option<Rc<RefCell<GameState>>>) {
    }
    

    pub fn getGameState(&mut self) -> Option<Rc<RefCell<GameState>>> {
        return { let java_field = jcall!(self.gameStateFactory).PLAYING_GAME_STATE.clone(); java_field }.clone();
    }
    

    pub fn getState(&mut self) -> i32 {
        return self.state.clone();
    }
    

    pub fn setState(&mut self, mut state: i32) {
        self.state = state.clone();
    }
    

    pub fn isHighScoreSubmitted(&mut self) -> bool {
        jcall!(self.logUtil).putF(String::from("Wow the AI got a high score!"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("isHighScoreSubmitted"));
        return false;
    }
    

    pub fn getRealHighScoresPaintable(&mut self) -> Option<Rc<RefCell<HighScoresPaintable>>> {
        return self.realHighScoresPaintable.clone();
    }
    

    pub fn setSpecialAnimationInterface(&mut self, mut specialAnimationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::special::SpecialAnimation::SpecialAnimationDyn>>>) {
        jcall!(specialAnimationInterface).setFrame(0);
        self.specialAnimationInterface = specialAnimationInterface.clone();
    }
    

    pub fn getSpecialAnimationInterface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::special::SpecialAnimation::SpecialAnimationDyn>>> {
        return self.specialAnimationInterface.clone();
    }
    

    pub fn setPaintableInterface(&mut self, mut paintableInterface: Option<Rc<RefCell<dyn PaintableInterface>>>) {
        self.paintableInterface = paintableInterface.clone();
    }
    

    pub fn getPaintableInterface(&mut self) -> Option<Rc<RefCell<dyn PaintableInterface>>> {
        return self.paintableInterface.clone();
    }
    

    pub fn setDefaultPaintableInterface(&mut self, mut defaultPaintableInterface: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>) {
        self.defaultPaintableInterface = defaultPaintableInterface.clone();
    }
    

    pub fn getDefaultPaintableInterface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>> {
        return self.defaultPaintableInterface.clone();
    }
    

    pub fn getHighScoresFactoryInterface(&mut self) -> Option<Rc<RefCell<dyn HighScoresFactoryInterface>>> {
        return self.highScoresFactoryInterface.clone();
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
    

    pub fn isInitialized(&mut self) -> bool {
        return self.initialized.clone();
    }
    

    pub fn getOverlayPaintable(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>> {
        return self.overlayPaintable.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>);
    }
    

    pub fn setBasicGameDemoPaintable(&mut self, mut basicGameDemoPaintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::StatePaintable::StatePaintableDyn>>>) {
        self.basicGameDemoPaintable = basicGameDemoPaintable.clone();
    }
    

    pub fn getBasicGameDemoPaintable(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::StatePaintable::StatePaintableDyn>>> {
        return self.basicGameDemoPaintable.clone();
    }
    

    pub fn setTempWait(&mut self, mut tempWait: i32) {
        self.tempWait = tempWait.clone();
    }
    

    pub fn getTempWait(&mut self) -> i32 {
        return self.tempWait.clone();
    }
    

    pub fn isSingleThread(&mut self) -> bool {
        return jcall!(OpenGLFeatureUtil::getInstance()).isAnyThreed() || unsafe { crate::org::allbinary::game::layer::SWTUtil::SWTUtil_isSWT.clone() };
    }
    

    pub fn isRunningInAnotherThread(&mut self) -> bool {
        let mut features: Option<Rc<RefCell<Features>>> = Features::getInstance();
        let mut openGLFeatureFactory: Option<Rc<RefCell<OpenGLFeatureFactory>>> = OpenGLFeatureFactory::getInstance();
        if jcall!(features).isDefault({ let java_field = jcall!(openGLFeatureFactory).OPENGL_AS_GAME_THREAD.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
            return true;
        } else {
            return self.isRunning();
        }
    }
    

    pub fn getGameInitializationInterfaceFactoryInterface(&mut self) -> Option<Rc<RefCell<BasicBuildGameInitializerFactory>>> {
        return self.gameInitializationInterfaceFactoryInterface.clone();
    }
    

    pub fn getTitle(&mut self) -> String {
        return self.base.getTitle();
    }
    

    pub fn isPaused(&mut self) -> bool {
        return self.base.isPaused();
    }
    

    pub fn getType(&mut self) -> i32 {
        return self.base.getType();
    }
}
impl std::ops::Deref for StartCanvas {
    type Target = RunnableCanvas;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for StartCanvas {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::javax::microedition::lcdui::Canvas::CanvasDyn for StartCanvas {
    fn as_Canvas(&self) -> &crate::javax::microedition::lcdui::Canvas::Canvas { &self.base.base.base }
    fn as_Canvas_mut(&mut self) -> &mut crate::javax::microedition::lcdui::Canvas::Canvas { &mut self.base.base.base }
}
impl crate::javax::microedition::lcdui::Displayable::DisplayableDyn for StartCanvas {
    fn as_Displayable(&self) -> &crate::javax::microedition::lcdui::Displayable::Displayable { &self.base.base.base.base }
    fn as_Displayable_mut(&mut self) -> &mut crate::javax::microedition::lcdui::Displayable::Displayable { &mut self.base.base.base.base }
    fn addCommand(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::addCommand(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn removeCommand(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::removeCommand(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn getWidth(&mut self) -> i32 { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::getWidth(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self)) }
    fn getHeight(&mut self) -> i32 { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::getHeight(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self)) }
    fn getTitle(&mut self) -> String { Self::getTitle(self) }
    fn setCommandListener(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::CommandListener::CommandListener>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::setCommandListener(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn setFullScreenMode(&mut self, p1: bool) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::setFullScreenMode(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn hideNotify(&mut self) { crate::javax::microedition::lcdui::Canvas::Canvas::hideNotify(crate::javax::microedition::lcdui::Canvas::CanvasDyn::as_Canvas_mut(self)) }
    fn keyPressed(&mut self, mut keyCode: i32) { Self::keyPressed(self, keyCode) }
    fn keyRepeated(&mut self, mut keyCode: i32) { Self::keyRepeated(self, keyCode) }
    fn keyReleased(&mut self, mut keyCode: i32) { Self::keyReleased(self, keyCode) }
    fn pointerPressed(&mut self, p1: i32, p2: i32) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::pointerPressed(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1, p2) }
    fn pointerReleased(&mut self, p1: i32, p2: i32) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::pointerReleased(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1, p2) }
    fn pointerDragged(&mut self, p1: i32, p2: i32) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::pointerDragged(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1, p2) }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::paint(self, graphics) }
    fn repaint(&mut self) { crate::javax::microedition::lcdui::Canvas::Canvas::repaint(crate::javax::microedition::lcdui::Canvas::CanvasDyn::as_Canvas_mut(self)) }
    fn repaintXYWH(&mut self, p1: i32, p2: i32, p3: i32, p4: i32) { crate::javax::microedition::lcdui::Canvas::Canvas::repaintXYWH(crate::javax::microedition::lcdui::Canvas::CanvasDyn::as_Canvas_mut(self), p1, p2, p3, p4) }
    fn sizeChanged(&mut self, p1: i32, p2: i32) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::sizeChanged(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1, p2) }
    fn showNotify(&mut self) { crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvas::showNotify(crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvasDyn::as_RunnableCanvas_mut(self)) }
}
impl crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvasDyn for StartCanvas {
    fn as_RunnableCanvas(&self) -> &crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvas { &self.base }
    fn as_RunnableCanvas_mut(&mut self) -> &mut crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvas { &mut self.base }
    fn initCommands(&mut self, mut cmdListener: Option<Rc<RefCell<dyn CommandListener>>>) { Self::initCommands(self, cmdListener) }
    fn setRunning(&mut self, mut running: bool) { Self::setRunning(self, running) }
    fn isPausable(&mut self) -> bool { Self::isPausable(self) }
    fn isSingleThread(&mut self) -> bool { Self::isSingleThread(self) }
    fn run(&mut self) { Self::run(self) }
    fn getType(&mut self) -> i32 { Self::getType(self) }
}
impl crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn for StartCanvas {
    fn as_MyCanvas(&self) -> &crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas { &self.base.base }
    fn as_MyCanvas_mut(&mut self) -> &mut crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas { &mut self.base.base }
    fn isPaused(&mut self) -> bool { Self::isPaused(self) }
    fn pause(&mut self) { Self::pause(self) }
    fn unPause(&mut self) { Self::unPause(self) }
    fn process(&mut self) { Self::process(self) }
    fn keyPressedByDevice(&mut self, mut keyCode: i32, mut deviceId: i32) { Self::keyPressedByDevice(self, keyCode, deviceId) }
    fn keyRepeatedByDevice(&mut self, mut keyCode: i32, mut deviceId: i32) { Self::keyRepeatedByDevice(self, keyCode, deviceId) }
    fn keyReleasedByDevice(&mut self, mut keyCode: i32, mut deviceId: i32) { Self::keyReleasedByDevice(self, keyCode, deviceId) }
}
impl crate::org::allbinary::graphics::displayable::DisplayableInterface::DisplayableInterface for StartCanvas {
    fn addCommand(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::addCommand(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn getHeight(&mut self) -> i32 { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::getHeight(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self)) }
    fn getWidth(&mut self) -> i32 { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::getWidth(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self)) }
    fn removeCommand(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::removeCommand(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn setCommandListener(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::CommandListener::CommandListener>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::setCommandListener(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
}
impl crate::org::allbinary::graphics::displayable::command::MyCommandInterface::MyCommandInterface for StartCanvas {
    fn addCommand(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::addCommand(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn removeAllCommands(&mut self) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::removeAllCommands(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self)) }
}


