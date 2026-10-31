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
// use crate::javax::microedition::lcdui::Command::Command;
// use crate::javax::microedition::lcdui::CommandListener::CommandListener;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::J2MEUtil::J2MEUtil;
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
use crate::org::allbinary::thread::ARunnable::ARunnable;
use crate::org::allbinary::thread::SecondaryThreadPool::SecondaryThreadPool;
use crate::org::allbinary::logic::ABSystemWrapper::ABSystemWrapper;
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
    pub currentCommand: Option<Rc<RefCell<Command>>>,
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
impl crate::org::allbinary::game::input::GameKeyEventSourceInterface::GameKeyEventSourceInterface for HighScoresCanvas {
    fn getSourceId(&mut self) -> i32 { self.base.getSourceId() }
}

impl HighScoresCanvas {
    

    pub fn new_struct(mut commandListener: Option<Rc<RefCell<CommandListener>>>, mut allBinaryGameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>, mut gameInfo: Option<Rc<RefCell<GameInfo>>>, mut paintable: Option<Rc<RefCell<HighScoresPaintable>>>, mut highScoresFactoryInterface: Option<Rc<RefCell<dyn HighScoresFactoryInterface>>>, mut highScoresCanvasInputProcessorFactoryInterface: Option<Rc<RefCell<dyn HighScoresCanvasInputProcessorFactoryInterface>>>) -> Self {
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
        this.waitPaintable = SimpleTextPaintable::new(jcall!(this.commonStrings).PLEASE_WAIT_FOR_SERVER.clone(), jcall!(BasicColorFactory::getInstance()).WHITE.clone());
        this.highScoresHelper = HighScoresHelperBase::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::score::HighScoresHelperBase::HighScoresHelperBaseDyn>>);
        this.currentCommand = jcall!(this.highScoreCommandsFactory).HIGH_SCORE_COMMANDS[(0) as usize];
        this.hasPainted = false;
        // super(...) is in the struct literal above
        jcall!(this.logUtil).putF(jcall!(this.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone());
        this.highScoresPaintable = paintable.clone();
        this.highScoresFactoryInterface = highScoresFactoryInterface.clone();
        this.gameInfo = gameInfo.clone();
        { let java_receiver = this.waitPaintable.clone(); let java_arguments = (jcall!(allBinaryGameLayerManager).getForegroundBasicColor(),); jcall!(java_receiver).setBasicColorP(java_arguments.0); };
        jcall!(this.getHighScoresPaintable()).setBasicColorP(jcall!(allBinaryGameLayerManager).getForegroundBasicColor());
        this.colorFillPaintable = jcall!(ColorFillPaintableFactory::getInstance()).getInstance2(jcall!(allBinaryGameLayerManager).getBackgroundBasicColor(), false);
        if match (&jcall!(this.highScoresHelper).getHighScoresArray(), &jcall!(NoHighScoresFactory::getInstance()).NO_HIGH_SCORES) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            this.setPaintable(this.waitPaintable.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>));
        } else {
            jcall!(this.logUtil).putF(String::from("Show HighScores that are already loaded"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone());
            this.updateCommand(this.currentCommand.clone());
            { let java_arguments = (this.getHighScoresPaintable().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>),); this.setPaintable(java_arguments.0) };
        }
        /* Java local class: move it out of the function
pub struct HighScoreRunnable {
    pub base: ARunnable,
    pub highScoresCanvas: Option<Rc<RefCell<HighScoresCanvas>>>,
}


impl HighScoreRunnable {
            

            pub fn new_struct(mut highScoresCanvas: Option<Rc<RefCell<HighScoresCanvas>>>) -> Self {
                let mut this = Self {
            base: ARunnable::new_struct(),
            highScoresCanvas: None,
                };
                this.highScoresCanvas = highScoresCanvas.clone();
                this
            }

            pub fn new(mut highScoresCanvas: Option<Rc<RefCell<HighScoresCanvas>>>) -> Option<Rc<RefCell<Self>>> {
                Some(Rc::new(RefCell::new(Self::new_struct(highScoresCanvas))))
            }
            

            pub fn run(&mut self) {
                let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
                let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
                let mut highScoresCanvas: Option<Rc<RefCell<HighScoresCanvas>>> = self.highScoresCanvas.clone();
                match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    let mut isHTML: bool = J2MEUtil::isHTML();
                    if !isHTML {
                        while !jcall!(highScoresCanvas).hasPainted {
                        }
                        jcall!(highScoresCanvas).hasPainted = false;
                    }
                    let mut stringMaker: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                    let mut systemWrapper: Option<Rc<RefCell<ABSystemWrapper>>> = ABSystemWrapper::getInstance();
                    let mut currentTimeMillis: i64 = jcall!(systemWrapper).currentTimeMillis();
                    jcall!(logUtil).putF(jcall!(stringMaker).append(String::from("HighScoresCanvas - Request repaint to be sure: ")).appendlong(currentTimeMillis.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).RUN.clone());
                    jcall!(jcall!(highScoresCanvas).repaintBehavior).onChangeRepaint(highScoresCanvas.clone());
                    if !isHTML {
                        while !jcall!(highScoresCanvas).hasPainted {
                        }
                    }
                    jcall!(stringMaker).delete(0, jcall!(stringMaker).length());
                    jcall!(logUtil).putF(jcall!(stringMaker).append(String::from("HighScoresCanvas - Now that the canvas has completed repaint go ahead and fetch the scores: ")).appendlong(currentTimeMillis.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).RUN.clone());
                    jcall!(highScoresCanvas).executeUpdate();
                })) {
                    Ok(_) => {}
                    Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

                    jcall!(logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).RUN.clone(), crate::java_into_object(e.clone()));
                    }
                }
            }
}
impl std::ops::Deref for HighScoreRunnable {
    type Target = ARunnable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for HighScoreRunnable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::thread::ARunnable::ARunnableDyn for HighScoreRunnable {
    fn as_ARunnable(&self) -> &crate::org::allbinary::thread::ARunnable::ARunnable { &self }
    fn as_ARunnable_mut(&mut self) -> &mut crate::org::allbinary::thread::ARunnable::ARunnable { &mut self }
}
impl crate::Runnable for HighScoreRunnable {
    fn run(&mut self) { Self::run(self) }
}

*/
        this
    }

    pub fn new(mut commandListener: Option<Rc<RefCell<CommandListener>>>, mut allBinaryGameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>, mut gameInfo: Option<Rc<RefCell<GameInfo>>>, mut paintable: Option<Rc<RefCell<HighScoresPaintable>>>, mut highScoresFactoryInterface: Option<Rc<RefCell<dyn HighScoresFactoryInterface>>>, mut highScoresCanvasInputProcessorFactoryInterface: Option<Rc<RefCell<dyn HighScoresCanvasInputProcessorFactoryInterface>>>) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(commandListener, allBinaryGameLayerManager, gameInfo, paintable, highScoresFactoryInterface, highScoresCanvasInputProcessorFactoryInterface))));
            this.highScoresCanvasInputProcessor = jcall!(highScoresCanvasInputProcessorFactoryInterface).getInstance((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()));
            jcall!(SecondaryThreadPool::getInstance()).runTask(HighScoreRunnable::new((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone())));
        this
    }
    

    pub fn initCommands(&mut self, mut cmdListener: Option<Rc<RefCell<CommandListener>>>) {
        self.removeAllCommands();
        { let java_arguments = (jcall!(GameCommandsFactory::getInstance()).CLOSE_AND_SHOW_GAME_CANVAS.clone(),); self.addCommand(java_arguments.0) };
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
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        jcall!(self.colorFillPaintable).paint(graphics.clone());
        jcall!(self.paintable).paint(graphics.clone());
        if !match (&self.waitPaintable, &self.paintable) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            jcall!(self.highScoresCanvasInputProcessor).paint(graphics.clone());
        }
        self.base.paint(graphics.clone());
        self.hasPainted = true;
    }
    

    pub fn executeUpdate(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            { let java_receiver = self.highScoresFactoryInterface.clone(); let java_arguments = (self.getGameInfo(), None /* Java this: a &mut self method has no Rc of itself */,); jcall!(java_receiver).fetchHighScores(java_arguments.0, java_arguments.1); };
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).UPDATE.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn getHighScoresPaintable(&mut self) -> Option<Rc<RefCell<HighScoresPaintable>>> {
        return self.highScoresPaintable.clone();
    }
    

    pub fn setHighScoresArray(&mut self, mut highScoresArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::game::score::HighScores::HighScoresDyn>>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if highScoresArray.is_some() {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(self.commonStrings).START.clone()).appendint((highScoresArray.len() as i32).clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setHighScoresArray"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            } else {
                jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setHighScoresArray"));
            }
            jcall!(self.highScoresHelper).setHighScoresArray(highScoresArray.clone());
            self.updateCommand(self.currentCommand.clone());
            { let java_arguments = (self.getHighScoresPaintable().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>),); self.setPaintable(java_arguments.0) };
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).UPDATE.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    // throws Exception
    pub fn updateCommand(&mut self, mut command: Option<Rc<RefCell<Command>>>) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(self.commonStrings).START.clone()).append(jcall!(self.stringUtil).toString(crate::java_into_object(command.clone()))).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).UPDATE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
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
                self.addCommand(jcall!(gameCommandsFactory).CLOSE_AND_SHOW_GAME_CANVAS.clone());
                self.addCommand(jcall!(self.highScoreCommandsFactory).HIGH_SCORE_COMMANDS[(nextIndex) as usize].clone());
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

impl crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn for HighScoresCanvas {
    fn as_GameCommandCanvas(&self) -> &crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas { &self.base }
    fn as_GameCommandCanvas_mut(&mut self) -> &mut crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas { &mut self.base }
    fn initCommands(&mut self, mut cmdListener: Option<Rc<RefCell<CommandListener>>>) { Self::initCommands(self, cmdListener) }
    fn open(&mut self) { Self::open(self) }
    fn close(&mut self) { Self::close(self) }
}
impl crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn for HighScoresCanvas {
    fn as_MyCanvas(&self) -> &crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas { &self.base.base }
    fn as_MyCanvas_mut(&mut self) -> &mut crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas { &mut self.base.base }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
    fn keyPressedByDevice(&mut self, p1: i32, p2: i32) { crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas::keyPressedByDevice(crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn::as_GameCommandCanvas_mut(self), p1, p2) }
    fn keyRepeatedByDevice(&mut self, p1: i32, p2: i32) { crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas::keyRepeatedByDevice(crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn::as_GameCommandCanvas_mut(self), p1, p2) }
    fn keyReleasedByDevice(&mut self, p1: i32, p2: i32) { crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas::keyReleasedByDevice(crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn::as_GameCommandCanvas_mut(self), p1, p2) }
}


