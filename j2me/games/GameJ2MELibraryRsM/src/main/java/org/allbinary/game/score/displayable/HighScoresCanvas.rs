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
// package org.allbinary.game.score.displayable
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::javax::microedition::lcdui::Command::Command;
use crate::javax::microedition::lcdui::CommandListener::CommandListener;
use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::game::GameInfo::GameInfo;
use crate::org::allbinary::game::commands::GameCommandsFactory::GameCommandsFactory;
use crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas;
use crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManager;
use crate::org::allbinary::game::paint::ColorFillBasePaintable::ColorFillBasePaintable;
use crate::org::allbinary::game::paint::ColorFillPaintableFactory::ColorFillPaintableFactory;
use crate::org::allbinary::game::score::HighScoreCommandsFactory::HighScoreCommandsFactory;
use crate::org::allbinary::game::score::HighScores::HighScores;
use crate::org::allbinary::game::score::HighScoresCanvasInputProcessor::HighScoresCanvasInputProcessor;
use crate::org::allbinary::game::score::HighScoresCanvasInputProcessorFactoryInterface::HighScoresCanvasInputProcessorFactoryInterface;
use crate::org::allbinary::game::score::HighScoresFactoryInterface::HighScoresFactoryInterface;
use crate::org::allbinary::game::score::HighScoresHelperBase::HighScoresHelperBase;
use crate::org::allbinary::game::score::HighScoresPaintable::HighScoresPaintable;
use crate::org::allbinary::game::score::HighScoresResultsListener::HighScoresResultsListener;
use crate::org::allbinary::game::score::NoHighScoresFactory::NoHighScoresFactory;
use crate::org::allbinary::game::score::NullHighScoresSingletonFactory::NullHighScoresSingletonFactory;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::graphics::paint::NullPaintable::NullPaintable;
use crate::org::allbinary::graphics::paint::Paintable::Paintable;
use crate::org::allbinary::graphics::paint::SimpleTextPaintable::SimpleTextPaintable;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::thread::SecondaryThreadPool::SecondaryThreadPool;
use crate::java::lang::Exception::*;
use super::HighScoreRunnable::HighScoreRunnable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct HighScoresCanvas {
    pub base: GameCommandCanvas,
    pub paintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>,
    pub highScoreCommandsFactory: Option<Rc<RefCell<HighScoreCommandsFactory>>>,
    pub waitPaintable: Option<Rc<RefCell<SimpleTextPaintable>>>,
    pub highScoresPaintable: Option<Rc<RefCell<HighScoresPaintable>>>,
    pub highScoresFactoryInterface: Option<Rc<RefCell<dyn HighScoresFactoryInterface>>>,
    pub colorFillPaintable: Option<Rc<RefCell<dyn crate::org::allbinary::game::paint::ColorFillBasePaintable::ColorFillBasePaintableDyn>>>,
    pub highScoresHelper: Option<Rc<RefCell<dyn crate::org::allbinary::game::score::HighScoresHelperBase::HighScoresHelperBaseDyn>>>,
    pub gameInfo: Option<Rc<RefCell<GameInfo>>>,
    pub highScoresCanvasInputProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::game::score::HighScoresCanvasInputProcessor::HighScoresCanvasInputProcessorDyn>>>,
    pub currentCommand: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>,
    pub hasPainted: bool,
}

thread_local! { pub static HighScoresCanvas_NAME: String = String::from("HighScoresCanvas"); }
impl HighScoresResultsListener for HighScoresCanvas {
    fn setHighScoresArray(&mut self, mut highScoresArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::game::score::HighScores::HighScoresDyn>>>>) { Self::setHighScoresArray(self, highScoresArray) }
}
impl crate::org::allbinary::game::displayable::canvas::MenuListener::MenuListener for HighScoresCanvas {
    fn open(&mut self) { Self::open(self) }
    fn close(&mut self) { Self::close(self) }
}
impl crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface for HighScoresCanvas {
    fn onEvent(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas::onEvent(crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn::as_GameCommandCanvas_mut(self), p1) }
}
impl crate::org::allbinary::game::input::GameKeyEventSourceInterface::GameKeyEventSourceInterface for HighScoresCanvas {
    fn getSourceId(&mut self) -> i32 { crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas::getSourceId(crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn::as_GameCommandCanvas_mut(self)) }
}
impl crate::org::allbinary::graphics::displayable::event::DisplayChangeEventListener::DisplayChangeEventListener for HighScoresCanvas {
    fn onDisplayChangeEvent(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::graphics::displayable::event::DisplayChangeEvent::DisplayChangeEvent>>>) { crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas::onDisplayChangeEvent(crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn::as_GameCommandCanvas_mut(self), p1) }
}

impl HighScoresCanvas {
    

    pub fn new_struct(mut commandListener: Option<Rc<RefCell<dyn CommandListener>>>, mut allBinaryGameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>, mut gameInfo: Option<Rc<RefCell<GameInfo>>>, mut paintable: Option<Rc<RefCell<HighScoresPaintable>>>, mut highScoresFactoryInterface: Option<Rc<RefCell<dyn HighScoresFactoryInterface>>>, mut highScoresCanvasInputProcessorFactoryInterface: Option<Rc<RefCell<dyn HighScoresCanvasInputProcessorFactoryInterface>>>) -> Self {
        let mut this = Self {
            base: GameCommandCanvas::new_struct(commandListener.clone(), HighScoresCanvas_NAME.with(Clone::clone).clone(), jcall!(allBinaryGameLayerManager).getBackgroundBasicColor(), jcall!(allBinaryGameLayerManager).getForegroundBasicColor()),
            paintable: None,
            highScoreCommandsFactory: None,
            waitPaintable: None,
            highScoresPaintable: None,
            highScoresFactoryInterface: None,
            colorFillPaintable: None,
            highScoresHelper: None,
            gameInfo: None,
            highScoresCanvasInputProcessor: None,
            currentCommand: None,
            hasPainted: false,
        };
        this.paintable = NullPaintable::getInstance();
        this.highScoreCommandsFactory = HighScoreCommandsFactory::getInstance();
        this.waitPaintable = SimpleTextPaintable::new({ let java_field = jcall!(this.commonStrings).PLEASE_WAIT_FOR_SERVER.clone(); java_field }.clone(), { let java_field = jcall!(BasicColorFactory::getInstance()).WHITE.clone(); java_field }.clone());
        this.highScoresHelper = HighScoresHelperBase::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::score::HighScoresHelperBase::HighScoresHelperBaseDyn>>);
        this.currentCommand = jcall!(this.highScoreCommandsFactory).HIGH_SCORE_COMMANDS[(0) as usize].clone();
        this.hasPainted = false;
        // super(...) is in the struct literal above
        jcall!(this.logUtil).putF({ let java_field = jcall!(this.commonStrings).START.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(this.commonStrings).CONSTRUCTOR.clone(); java_field }.clone());
        this.highScoresPaintable = paintable.clone();
        this.highScoresFactoryInterface = highScoresFactoryInterface.clone();
        this
    }

    pub fn new(mut commandListener: Option<Rc<RefCell<dyn CommandListener>>>, mut allBinaryGameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>, mut gameInfo: Option<Rc<RefCell<GameInfo>>>, mut paintable: Option<Rc<RefCell<HighScoresPaintable>>>, mut highScoresFactoryInterface: Option<Rc<RefCell<dyn HighScoresFactoryInterface>>>, mut highScoresCanvasInputProcessorFactoryInterface: Option<Rc<RefCell<dyn HighScoresCanvasInputProcessorFactoryInterface>>>) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(commandListener.clone(), allBinaryGameLayerManager.clone(), gameInfo.clone(), paintable.clone(), highScoresFactoryInterface.clone(), highScoresCanvasInputProcessorFactoryInterface.clone()))));
            jcall!(this).highScoresCanvasInputProcessor = { let java_value = (jcall!(highScoresCanvasInputProcessorFactoryInterface).getInstance(this.clone())).clone(); java_value };
            jcall!(this).gameInfo = gameInfo.clone();
            { let java_receiver = jcall!(this).waitPaintable.clone(); let java_arguments = (jcall!(allBinaryGameLayerManager).getForegroundBasicColor(),); jcall!(java_receiver).setBasicColorP(java_arguments.0); };
            jcall!(jcall!(this).getHighScoresPaintable()).setBasicColorP(jcall!(allBinaryGameLayerManager).getForegroundBasicColor());
            jcall!(this).colorFillPaintable = { let java_value = (jcall!(ColorFillPaintableFactory::getInstance()).getInstance2(jcall!(allBinaryGameLayerManager).getBackgroundBasicColor(), false)).clone(); java_value };
            if (jcall!(jcall!(this).highScoresHelper).getHighScoresArray().len() as i32) == ({ let java_field = jcall!(NoHighScoresFactory::getInstance()).NO_HIGH_SCORES.clone(); java_field }.len() as i32) {
            { let java_arguments = (jcall!(this).waitPaintable.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>),); jcall!(this).setPaintable(java_arguments.0) };
        } else {
            jcall!(jcall!(this).logUtil).putF(String::from("Show HighScores that are already loaded"), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(jcall!(this).commonStrings).CONSTRUCTOR.clone(); java_field }.clone());
            { let java_arguments = (jcall!(this).currentCommand.clone(),); jcall!(this).updateCommand(java_arguments.0) };
            { let java_arguments = (jcall!(this).getHighScoresPaintable().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>),); jcall!(this).setPaintable(java_arguments.0) };
        }
            jcall!(SecondaryThreadPool::getInstance()).runTask(HighScoreRunnable::new(this.clone()).map(|reference| reference as Rc<RefCell<dyn crate::Runnable>>));
        this
    }
    

    pub fn initCommands(&mut self, mut cmdListener: Option<Rc<RefCell<dyn CommandListener>>>) {
        self.removeAllCommands();
        { let java_arguments = ({ let java_field = jcall!(GameCommandsFactory::getInstance()).CLOSE_AND_SHOW_GAME_CANVAS.clone(); java_field }.clone(),); self.addCommand(java_arguments.0) };
        self.setCommandListener(cmdListener.clone());
    }
    

    pub fn open(&mut self) {
        self.base.open();
        jcall!(self.highScoresCanvasInputProcessor).open();
    }
    

    // throws Exception
    pub fn close(&mut self) {
        self.base.close();
        jcall!(self.highScoresCanvasInputProcessor).close();
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
        jcall!(self.colorFillPaintable).as_ColorFillBasePaintable_mut().paint(graphics.clone());
        jcall!(self.paintable).as_Paintable_mut().paint(graphics.clone());
        if !match (&self.waitPaintable, &self.paintable) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            jcall!(self.highScoresCanvasInputProcessor).paint(graphics.clone());
        }
        { let java_arguments = (graphics.clone(),); self.base.paint(java_arguments.0) };
        self.hasPainted = true;
    }
    

    pub fn executeUpdate(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            { let java_receiver = self.highScoresFactoryInterface.clone(); let java_arguments = (self.getGameInfo(), None /* Java this: a &mut self method has no Rc of itself */,); jcall!(java_receiver).fetchHighScores(java_arguments.0, java_arguments.1); };
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).UPDATE.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn getHighScoresPaintable(&mut self) -> Option<Rc<RefCell<HighScoresPaintable>>> {
        return self.highScoresPaintable.clone();
    }
    

    pub fn setHighScoresArray(&mut self, mut highScoresArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::game::score::HighScores::HighScoresDyn>>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if !highScoresArray.is_empty() {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append({ let java_field = jcall!(self.commonStrings).START.clone(); java_field }.clone()).appendint((highScoresArray.len() as i32).clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setHighScoresArray"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            } else {
                jcall!(self.logUtil).putF({ let java_field = jcall!(self.commonStrings).START.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setHighScoresArray"));
            }
            jcall!(self.highScoresHelper).setHighScoresArray(highScoresArray.clone());
            { let java_arguments = (self.currentCommand.clone(),); self.updateCommand(java_arguments.0) };
            { let java_arguments = (self.getHighScoresPaintable().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>),); self.setPaintable(java_arguments.0) };
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).UPDATE.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    // throws Exception
    pub fn updateCommand(&mut self, mut command: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append({ let java_field = jcall!(self.commonStrings).START.clone(); java_field }.clone()).append(jcall!(self.stringUtil).toString(crate::java_into_object(command.clone()))).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).UPDATE.clone(); java_field }.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        let mut gameCommandsFactory: Option<Rc<RefCell<GameCommandsFactory>>> = GameCommandsFactory::getInstance();
        if jcall!(self.highScoreCommandsFactory).isHighScoreCommand(command.clone()) {
            let mut index: i32 = jcall!(self.highScoreCommandsFactory).getIndex(command.clone());
            let mut nextIndex: i32 = index + 1;
            let mut highScoresArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::game::score::HighScores::HighScoresDyn>>>> = jcall!(self.highScoresHelper).getHighScoresArray();
            if nextIndex >= (highScoresArray.len() as i32) {
                nextIndex = 0;
            }
            if (highScoresArray.len() as i32) > 0 {
                jcall!(self.getHighScoresPaintable()).setHighScores(highScoresArray[(index) as usize].clone());
            } else {
                jcall!(self.getHighScoresPaintable()).setHighScores(NullHighScoresSingletonFactory::getInstance());
            }
            if index != nextIndex {
                self.removeAllCommands();
                { let java_arguments = ({ let java_field = jcall!(gameCommandsFactory).CLOSE_AND_SHOW_GAME_CANVAS.clone(); java_field }.clone(),); self.addCommand(java_arguments.0) };
                { let java_arguments = (jcall!(self.highScoreCommandsFactory).HIGH_SCORE_COMMANDS[(nextIndex) as usize].clone(),); self.addCommand(java_arguments.0) };
            }
        }
        self.currentCommand = command.clone();
        self.close();
        self.base.initMenu();
        self.open();
        jcall!(self.repaintBehavior).onChangeRepaint(None /* Java this: a &mut self method has no Rc of itself */);
    }
    

    pub fn getGameInfo(&mut self) -> Option<Rc<RefCell<GameInfo>>> {
        return self.gameInfo.clone();
    }
    

    pub fn setPaintable(&mut self, mut paintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>) {
        self.paintable = paintable.clone();
        jcall!(self.repaintBehavior).onChangeRepaint(None /* Java this: a &mut self method has no Rc of itself */);
    }
}
impl std::ops::Deref for HighScoresCanvas {
    type Target = GameCommandCanvas;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for HighScoresCanvas {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::javax::microedition::lcdui::Canvas::CanvasDyn for HighScoresCanvas {
    fn as_Canvas(&self) -> &crate::javax::microedition::lcdui::Canvas::Canvas { &self.base.base.base }
    fn as_Canvas_mut(&mut self) -> &mut crate::javax::microedition::lcdui::Canvas::Canvas { &mut self.base.base.base }
}
impl crate::javax::microedition::lcdui::Displayable::DisplayableDyn for HighScoresCanvas {
    fn as_Displayable(&self) -> &crate::javax::microedition::lcdui::Displayable::Displayable { &self.base.base.base.base }
    fn as_Displayable_mut(&mut self) -> &mut crate::javax::microedition::lcdui::Displayable::Displayable { &mut self.base.base.base.base }
    fn addCommand(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::addCommand(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn removeCommand(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::removeCommand(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn getWidth(&mut self) -> i32 { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::getWidth(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self)) }
    fn getHeight(&mut self) -> i32 { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::getHeight(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self)) }
    fn setCommandListener(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::CommandListener::CommandListener>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::setCommandListener(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn setFullScreenMode(&mut self, p1: bool) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::setFullScreenMode(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn hideNotify(&mut self) { crate::javax::microedition::lcdui::Canvas::Canvas::hideNotify(crate::javax::microedition::lcdui::Canvas::CanvasDyn::as_Canvas_mut(self)) }
    fn keyPressed(&mut self, p1: i32) { crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas::keyPressed(crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn::as_GameCommandCanvas_mut(self), p1) }
    fn keyRepeated(&mut self, p1: i32) { crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas::keyRepeated(crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn::as_GameCommandCanvas_mut(self), p1) }
    fn keyReleased(&mut self, p1: i32) { crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas::keyReleased(crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn::as_GameCommandCanvas_mut(self), p1) }
    fn pointerPressed(&mut self, p1: i32, p2: i32) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::pointerPressed(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1, p2) }
    fn pointerReleased(&mut self, p1: i32, p2: i32) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::pointerReleased(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1, p2) }
    fn pointerDragged(&mut self, p1: i32, p2: i32) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::pointerDragged(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1, p2) }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::paint(self, graphics) }
    fn repaint(&mut self) { crate::javax::microedition::lcdui::Canvas::Canvas::repaint(crate::javax::microedition::lcdui::Canvas::CanvasDyn::as_Canvas_mut(self)) }
    fn repaintXYWH(&mut self, p1: i32, p2: i32, p3: i32, p4: i32) { crate::javax::microedition::lcdui::Canvas::Canvas::repaintXYWH(crate::javax::microedition::lcdui::Canvas::CanvasDyn::as_Canvas_mut(self), p1, p2, p3, p4) }
    fn sizeChanged(&mut self, p1: i32, p2: i32) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::sizeChanged(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1, p2) }
    fn showNotify(&mut self) { crate::javax::microedition::lcdui::Canvas::Canvas::showNotify(crate::javax::microedition::lcdui::Canvas::CanvasDyn::as_Canvas_mut(self)) }
}
impl crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn for HighScoresCanvas {
    fn as_GameCommandCanvas(&self) -> &crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas { &self.base }
    fn as_GameCommandCanvas_mut(&mut self) -> &mut crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas { &mut self.base }
    fn initCommands(&mut self, mut cmdListener: Option<Rc<RefCell<dyn CommandListener>>>) { Self::initCommands(self, cmdListener) }
    fn open(&mut self) { Self::open(self) }
    fn close(&mut self) { Self::close(self) }
}
impl crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn for HighScoresCanvas {
    fn as_MyCanvas(&self) -> &crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas { &self.base.base }
    fn as_MyCanvas_mut(&mut self) -> &mut crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas { &mut self.base.base }
    fn keyPressedByDevice(&mut self, p1: i32, p2: i32) { crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas::keyPressedByDevice(crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn::as_GameCommandCanvas_mut(self), p1, p2) }
    fn keyRepeatedByDevice(&mut self, p1: i32, p2: i32) { crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas::keyRepeatedByDevice(crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn::as_GameCommandCanvas_mut(self), p1, p2) }
    fn keyReleasedByDevice(&mut self, p1: i32, p2: i32) { crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas::keyReleasedByDevice(crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn::as_GameCommandCanvas_mut(self), p1, p2) }
}
impl crate::org::allbinary::graphics::displayable::DisplayableInterface::DisplayableInterface for HighScoresCanvas {
    fn addCommand(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::addCommand(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn getHeight(&mut self) -> i32 { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::getHeight(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self)) }
    fn getWidth(&mut self) -> i32 { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::getWidth(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self)) }
    fn removeCommand(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::removeCommand(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn setCommandListener(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::CommandListener::CommandListener>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::setCommandListener(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
}
impl crate::org::allbinary::graphics::font::UpdateMyFontInterface::UpdateMyFontInterface for HighScoresCanvas {
    fn updateMeasurement(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas::updateMeasurement(crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn::as_GameCommandCanvas_mut(self), p1) }
}
impl crate::org::allbinary::graphics::displayable::command::MyCommandInterface::MyCommandInterface for HighScoresCanvas {
    fn addCommand(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::addCommand(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn removeAllCommands(&mut self) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::removeAllCommands(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self)) }
}


