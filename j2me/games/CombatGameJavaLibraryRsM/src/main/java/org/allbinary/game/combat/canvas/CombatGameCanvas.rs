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
// package org.allbinary.game.combat.canvas
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::javax::microedition::lcdui::CommandListener::CommandListener;
use crate::org::allbinary::game::combat::destroy::DestroyedLayerProcessor::DestroyedLayerProcessor;
use crate::org::allbinary::game::combat::destroy::event::DestroyEventCircularStaticPool::DestroyEventCircularStaticPool;
use crate::org::allbinary::game::combat::destroy::event::DestroyedEventHandler::DestroyedEventHandler;
use crate::org::allbinary::game::configuration::feature::Features::Features;
use crate::org::allbinary::game::configuration::feature::GameFeatureFactory::GameFeatureFactory;
use crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas;
use crate::org::allbinary::game::init::BasicBuildGameInitializerFactory::BasicBuildGameInitializerFactory;
use crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManager;
use crate::org::allbinary::game::layer::drop::DropLayerProcessor::DropLayerProcessor;
use crate::org::allbinary::game::layer::identification::GroupLayerManagerListener::GroupLayerManagerListener;
use crate::org::allbinary::game::score::HighScoresFactoryInterface::HighScoresFactoryInterface;
use crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvasFactory::ProgressCanvasFactory;
use crate::org::allbinary::layer::BasicLayerProcessor::BasicLayerProcessor;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::java::lang::Exception::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CombatGameCanvas {
    pub base: AllBinaryGameCanvas,
}

pub static mut CombatGameCanvas_basicLayerProcessor: std::cell::LazyCell<Vec<Option<Rc<RefCell<dyn crate::org::allbinary::layer::BasicLayerProcessor::BasicLayerProcessorDyn>>>>> = std::cell::LazyCell::new(|| vec![None; (0) as usize]);
impl crate::javax::microedition::lcdui::ItemStateListener::ItemStateListener for CombatGameCanvas {
    fn itemStateChanged(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Item::ItemDyn>>>) { crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas::itemStateChanged(crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn::as_AllBinaryGameCanvas_mut(self), p1) }
}
impl crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvasInterface::AllBinaryGameCanvasInterface for CombatGameCanvas {
    fn getLayerManager(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>> { crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas::getLayerManager(crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn::as_AllBinaryGameCanvas_mut(self)) }
}
impl crate::org::allbinary::game::displayable::canvas::MenuListener::MenuListener for CombatGameCanvas {
    fn open(&mut self) { crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas::open(crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn::as_AllBinaryGameCanvas_mut(self)) }
    fn close(&mut self) { crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas::close(crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn::as_AllBinaryGameCanvas_mut(self)) }
}
impl crate::org::allbinary::thread::RunnableInterface::RunnableInterface for CombatGameCanvas {
    fn isRunning(&mut self) -> bool { crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvas::isRunning(crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvasDyn::as_RunnableCanvas_mut(self)) }
    fn setRunning(&mut self, p1: bool) { crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas::setRunning(crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn::as_AllBinaryGameCanvas_mut(self), p1) }
    fn setThread(&mut self, p1: Option<Rc<RefCell<crate::java::lang::Thread::Thread>>>) { crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvas::setThread(crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvasDyn::as_RunnableCanvas_mut(self), p1) }
}
impl crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface for CombatGameCanvas {
    fn onEvent(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas::onEvent(crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn::as_AllBinaryGameCanvas_mut(self), p1) }
}
impl crate::org::allbinary::game::input::GameKeyEventSourceInterface::GameKeyEventSourceInterface for CombatGameCanvas {
    fn getSourceId(&mut self) -> i32 { crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas::getSourceId(crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn::as_AllBinaryGameCanvas_mut(self)) }
}
impl crate::org::allbinary::game::state::GameStateCompositeInterface::GameStateCompositeInterface for CombatGameCanvas {
    fn setGameState(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::game::state::GameState::GameState>>>) { crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas::setGameState(crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn::as_AllBinaryGameCanvas_mut(self), p1) }
    fn getGameState(&mut self) -> Option<Rc<RefCell<crate::org::allbinary::game::state::GameState::GameState>>> { crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas::getGameState(crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn::as_AllBinaryGameCanvas_mut(self)) }
}
impl crate::org::allbinary::graphics::displayable::event::DisplayChangeEventListener::DisplayChangeEventListener for CombatGameCanvas {
    fn onDisplayChangeEvent(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::graphics::displayable::event::DisplayChangeEvent::DisplayChangeEvent>>>) { crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas::onDisplayChangeEvent(crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn::as_AllBinaryGameCanvas_mut(self), p1) }
}
impl crate::org::allbinary::graphics::paint::PaintableInterface::PaintableInterface for CombatGameCanvas {
    fn paint(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas::paint(crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn::as_AllBinaryGameCanvas_mut(self), p1) }
    fn paintThreed(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas::paintThreed(crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn::as_AllBinaryGameCanvas_mut(self), p1) }
}
impl crate::Runnable for CombatGameCanvas {
    fn run(&mut self) { crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas::run(crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn::as_AllBinaryGameCanvas_mut(self)) }
}

impl CombatGameCanvas {
    

    pub fn new_struct(mut cmdListener: Option<Rc<RefCell<dyn CommandListener>>>, mut gameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>, mut highScoresFactoryInterface: Option<Rc<RefCell<dyn HighScoresFactoryInterface>>>, mut gameInitializationInterfaceFactoryInterface: Option<Rc<RefCell<BasicBuildGameInitializerFactory>>>, mut buffered: bool) -> Self {
        let mut this = Self {
            base: AllBinaryGameCanvas::new_struct(cmdListener.clone(), gameLayerManager.clone(), highScoresFactoryInterface.clone(), gameInitializationInterfaceFactoryInterface.clone(), buffered.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut cmdListener: Option<Rc<RefCell<dyn CommandListener>>>, mut gameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>, mut highScoresFactoryInterface: Option<Rc<RefCell<dyn HighScoresFactoryInterface>>>, mut gameInitializationInterfaceFactoryInterface: Option<Rc<RefCell<BasicBuildGameInitializerFactory>>>, mut buffered: bool) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(cmdListener, gameLayerManager, highScoresFactoryInterface, gameInitializationInterfaceFactoryInterface, buffered))))
    }
    

    pub fn CombatGameCanvas_init(&mut self) {
        jcall!(DestroyEventCircularStaticPool::getInstance()).init(None /* Java this: a &mut self method has no Rc of itself */);
        jcall!(ProgressCanvasFactory::getInstance()).addNormalPortion(50, String::from("Destroy Events"));
    }
    

    // throws Exception
    pub fn initApp(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>) {
        self.CombatGameCanvas_init();
        { let java_arguments = (abeClientInformation.clone(),); self.base.initApp(java_arguments.0) };
    }
    

    pub fn initConfigurablePortion(&mut self, mut portion: i32) {
        DestroyedLayerProcessor::init();
        jcall!(ProgressCanvasFactory::getInstance()).addNormalPortion(portion.clone(), String::from("Basic Processors"));
        let mut features: Option<Rc<RefCell<Features>>> = Features::getInstance();
        let mut gameFeatureFactory: Option<Rc<RefCell<GameFeatureFactory>>> = GameFeatureFactory::getInstance();
        if jcall!(features).isFeature({ let java_field = jcall!(gameFeatureFactory).DROPPED_ITEMS.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) && jcall!(features).isFeature({ let java_field = jcall!(gameFeatureFactory).DROPPED_ITEMS_FROM_DEATH.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
            unsafe { *CombatGameCanvas_basicLayerProcessor = vec![None; (2) as usize] };
            unsafe { (*CombatGameCanvas_basicLayerProcessor)[(0) as usize] = DestroyedLayerProcessor::getInstance() };
            unsafe { (*CombatGameCanvas_basicLayerProcessor)[(1) as usize] = DropLayerProcessor::getInstance() };
        } else {
            unsafe { *CombatGameCanvas_basicLayerProcessor = vec![None; (1) as usize] };
            unsafe { (*CombatGameCanvas_basicLayerProcessor)[(0) as usize] = DestroyedLayerProcessor::getInstance() };
        }
    }
    

    // throws Exception
    pub fn processPlayingGame(&mut self) {
        self.base.processPlayingGame();
        {
            let mut index: i32 = (unsafe { CombatGameCanvas_basicLayerProcessor.clone() }.len() as i32).clone();
            while { index -= 1; index } >= 0 {
            jcall!(unsafe { CombatGameCanvas_basicLayerProcessor.clone() }[(index) as usize]).process(self.gameLayerManager.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>));
            }
        }
    }
    

    // throws Exception
    pub fn cleanupGame(&mut self) {
        self.base.cleanupGame();
        {
            let mut index: i32 = (unsafe { CombatGameCanvas_basicLayerProcessor.clone() }.len() as i32).clone();
            while { index -= 1; index } >= 0 {
            jcall!(jcall!(unsafe { CombatGameCanvas_basicLayerProcessor.clone() }[(index) as usize]).getList()).clear();
            }
        }
        jcall!(GroupLayerManagerListener::getInstance()).clear();
        jcall!(GroupLayerManagerListener::getInstance()).log();
        jcall!(DestroyedEventHandler::getInstance()).removeAllListeners();
        self.cleanupManager();
    }
    

    // throws Exception
    pub fn cleanupManager(&mut self) {
        jcall!(self.gameLayerManager).cleanup();
    }
}
impl std::ops::Deref for CombatGameCanvas {
    type Target = AllBinaryGameCanvas;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for CombatGameCanvas {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait CombatGameCanvasDyn: crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn {
    fn as_CombatGameCanvas(&self) -> &CombatGameCanvas;
    fn as_CombatGameCanvas_mut(&mut self) -> &mut CombatGameCanvas;
    fn CombatGameCanvas_init(&mut self) { CombatGameCanvas::CombatGameCanvas_init(self.as_CombatGameCanvas_mut()) }
    fn initConfigurablePortion(&mut self, mut portion: i32) { CombatGameCanvas::initConfigurablePortion(self.as_CombatGameCanvas_mut(), portion) }
    fn cleanupManager(&mut self) { CombatGameCanvas::cleanupManager(self.as_CombatGameCanvas_mut()) }
}
impl CombatGameCanvasDyn for CombatGameCanvas {
    fn as_CombatGameCanvas(&self) -> &CombatGameCanvas { self }
    fn as_CombatGameCanvas_mut(&mut self) -> &mut CombatGameCanvas { self }
}

impl crate::javax::microedition::lcdui::Canvas::CanvasDyn for CombatGameCanvas {
    fn as_Canvas(&self) -> &crate::javax::microedition::lcdui::Canvas::Canvas { &self.base.base.base.base }
    fn as_Canvas_mut(&mut self) -> &mut crate::javax::microedition::lcdui::Canvas::Canvas { &mut self.base.base.base.base }
}
impl crate::javax::microedition::lcdui::Displayable::DisplayableDyn for CombatGameCanvas {
    fn as_Displayable(&self) -> &crate::javax::microedition::lcdui::Displayable::Displayable { &self.base.base.base.base.base }
    fn as_Displayable_mut(&mut self) -> &mut crate::javax::microedition::lcdui::Displayable::Displayable { &mut self.base.base.base.base.base }
    fn addCommand(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::addCommand(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn removeCommand(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::removeCommand(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn getWidth(&mut self) -> i32 { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::getWidth(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self)) }
    fn getHeight(&mut self) -> i32 { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::getHeight(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self)) }
    fn getTitle(&mut self) -> String { crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas::getTitle(crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn::as_AllBinaryGameCanvas_mut(self)) }
    fn setCommandListener(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::CommandListener::CommandListener>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::setCommandListener(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn setFullScreenMode(&mut self, p1: bool) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::setFullScreenMode(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn hideNotify(&mut self) { crate::javax::microedition::lcdui::Canvas::Canvas::hideNotify(crate::javax::microedition::lcdui::Canvas::CanvasDyn::as_Canvas_mut(self)) }
    fn keyPressed(&mut self, p1: i32) { crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas::keyPressed(crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn::as_AllBinaryGameCanvas_mut(self), p1) }
    fn keyRepeated(&mut self, p1: i32) { crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas::keyRepeated(crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn::as_AllBinaryGameCanvas_mut(self), p1) }
    fn keyReleased(&mut self, p1: i32) { crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas::keyReleased(crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn::as_AllBinaryGameCanvas_mut(self), p1) }
    fn pointerPressed(&mut self, p1: i32, p2: i32) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::pointerPressed(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1, p2) }
    fn pointerReleased(&mut self, p1: i32, p2: i32) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::pointerReleased(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1, p2) }
    fn pointerDragged(&mut self, p1: i32, p2: i32) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::pointerDragged(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1, p2) }
    fn paint(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas::paint(crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn::as_AllBinaryGameCanvas_mut(self), p1) }
    fn repaint(&mut self) { crate::javax::microedition::lcdui::Canvas::Canvas::repaint(crate::javax::microedition::lcdui::Canvas::CanvasDyn::as_Canvas_mut(self)) }
    fn repaintXYWH(&mut self, p1: i32, p2: i32, p3: i32, p4: i32) { crate::javax::microedition::lcdui::Canvas::Canvas::repaintXYWH(crate::javax::microedition::lcdui::Canvas::CanvasDyn::as_Canvas_mut(self), p1, p2, p3, p4) }
    fn sizeChanged(&mut self, p1: i32, p2: i32) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::sizeChanged(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1, p2) }
    fn showNotify(&mut self) { crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvas::showNotify(crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvasDyn::as_RunnableCanvas_mut(self)) }
}
impl crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn for CombatGameCanvas {
    fn as_AllBinaryGameCanvas(&self) -> &crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas { &self.base }
    fn as_AllBinaryGameCanvas_mut(&mut self) -> &mut crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas { &mut self.base }
    fn initApp(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>) { Self::initApp(self, abeClientInformation) }
    fn cleanupGame(&mut self) { Self::cleanupGame(self) }
    fn processPlayingGame(&mut self) { Self::processPlayingGame(self) }
}
impl crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvasDyn for CombatGameCanvas {
    fn as_RunnableCanvas(&self) -> &crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvas { &self.base.base }
    fn as_RunnableCanvas_mut(&mut self) -> &mut crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvas { &mut self.base.base }
    fn initCommands(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::CommandListener::CommandListener>>>) { crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas::initCommands(crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn::as_AllBinaryGameCanvas_mut(self), p1) }
    fn setRunning(&mut self, p1: bool) { crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas::setRunning(crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn::as_AllBinaryGameCanvas_mut(self), p1) }
    fn setCurrentThread(&mut self) { crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas::setCurrentThread(crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn::as_AllBinaryGameCanvas_mut(self)) }
    fn processSleep(&mut self) { crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas::processSleep(crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn::as_AllBinaryGameCanvas_mut(self)) }
    fn isPausable(&mut self) -> bool { crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas::isPausable(crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn::as_AllBinaryGameCanvas_mut(self)) }
    fn isSingleThread(&mut self) -> bool { crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas::isSingleThread(crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn::as_AllBinaryGameCanvas_mut(self)) }
    fn run(&mut self) { crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas::run(crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn::as_AllBinaryGameCanvas_mut(self)) }
    fn getType(&mut self) -> i32 { crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas::getType(crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn::as_AllBinaryGameCanvas_mut(self)) }
}
impl crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn for CombatGameCanvas {
    fn as_MyCanvas(&self) -> &crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas { &self.base.base.base }
    fn as_MyCanvas_mut(&mut self) -> &mut crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas { &mut self.base.base.base }
    fn isPaused(&mut self) -> bool { crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas::isPaused(crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn::as_AllBinaryGameCanvas_mut(self)) }
    fn pause(&mut self) { crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas::pause(crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn::as_AllBinaryGameCanvas_mut(self)) }
    fn unPause(&mut self) { crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas::unPause(crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn::as_AllBinaryGameCanvas_mut(self)) }
    fn draw(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas::draw(crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn::as_AllBinaryGameCanvas_mut(self), p1) }
    fn keyPressedByDevice(&mut self, p1: i32, p2: i32) { crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas::keyPressedByDevice(crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn::as_AllBinaryGameCanvas_mut(self), p1, p2) }
    fn keyRepeatedByDevice(&mut self, p1: i32, p2: i32) { crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas::keyRepeatedByDevice(crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn::as_AllBinaryGameCanvas_mut(self), p1, p2) }
    fn keyReleasedByDevice(&mut self, p1: i32, p2: i32) { crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas::keyReleasedByDevice(crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn::as_AllBinaryGameCanvas_mut(self), p1, p2) }
}
impl crate::org::allbinary::game::displayable::canvas::PopupMenuInterface::PopupMenuInterface for CombatGameCanvas {
    fn popupMenu(&mut self) { crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas::popupMenu(crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn::as_AllBinaryGameCanvas_mut(self)) }
    fn closeMenu(&mut self) { crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas::closeMenu(crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn::as_AllBinaryGameCanvas_mut(self)) }
    fn toggleMenu(&mut self) { crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas::toggleMenu(crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn::as_AllBinaryGameCanvas_mut(self)) }
}
impl crate::org::allbinary::game::IntermissionCompositeInterface::IntermissionCompositeInterface for CombatGameCanvas {
    fn getStartIntermissionInterface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::IntermissionInterface::IntermissionInterface>>> { crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas::getStartIntermissionInterface(crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn::as_AllBinaryGameCanvas_mut(self)) }
    fn getEndLevelIntermissionInterface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::IntermissionInterface::IntermissionInterface>>> { crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas::getEndLevelIntermissionInterface(crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn::as_AllBinaryGameCanvas_mut(self)) }
}
impl crate::org::allbinary::game::IntermissionEnableListenerInterface::IntermissionEnableListenerInterface for CombatGameCanvas {
    fn notifyIntermission(&mut self, p1: bool) { crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas::notifyIntermission(crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn::as_AllBinaryGameCanvas_mut(self), p1) }
}
impl crate::org::allbinary::graphics::displayable::DisplayableInterface::DisplayableInterface for CombatGameCanvas {
    fn addCommand(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::addCommand(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn getHeight(&mut self) -> i32 { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::getHeight(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self)) }
    fn getWidth(&mut self) -> i32 { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::getWidth(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self)) }
    fn removeCommand(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::removeCommand(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn setCommandListener(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::CommandListener::CommandListener>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::setCommandListener(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
}
impl crate::org::allbinary::graphics::displayable::command::MyCommandInterface::MyCommandInterface for CombatGameCanvas {
    fn addCommand(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::addCommand(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn removeAllCommands(&mut self) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::removeAllCommands(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self)) }
}
impl crate::org::allbinary::graphics::font::UpdateMyFontInterface::UpdateMyFontInterface for CombatGameCanvas {
    fn updateMeasurement(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas::updateMeasurement(crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn::as_AllBinaryGameCanvas_mut(self), p1) }
}


