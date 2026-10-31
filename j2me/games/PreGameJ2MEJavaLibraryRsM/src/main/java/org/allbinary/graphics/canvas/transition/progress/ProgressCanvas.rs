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
// package org.allbinary.graphics.canvas.transition.progress
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::CommandListener::CommandListener;
// use crate::javax::microedition::lcdui::Font::Font;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
// use crate::javax::microedition::lcdui::NullCanvas::NullCanvas;
// use crate::javax::microedition::lcdui::NullCommandListener::NullCommandListener;
use crate::org::allbinary::canvas::Processor::Processor;
use crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvas;
use crate::org::allbinary::game::commands::GameCommandsFactory::GameCommandsFactory;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::displayable::CanvasStrings::CanvasStrings;
use crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessor;
use crate::org::allbinary::graphics::font::UpdateMyFontInterface::UpdateMyFontInterface;
use crate::org::allbinary::graphics::font::UpdateMyFontProcessor::UpdateMyFontProcessor;
use crate::org::allbinary::graphics::form::item::ABCustomGaugeItem::ABCustomGaugeItem;
use crate::org::allbinary::graphics::paint::NullPaintable::NullPaintable;
use crate::org::allbinary::graphics::paint::Paintable::Paintable;
use crate::org::allbinary::graphics::paint::PaintableInterface::PaintableInterface;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::logic::math::SmallIntegerSingletonFactory::SmallIntegerSingletonFactory;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::midlet::AllBinaryMidlet::AllBinaryMidlet;
use crate::org::allbinary::thread::PathFindingThreadPool::PathFindingThreadPool;
use crate::org::allbinary::thread::ThreadPool::ThreadPool;
use super::ProgressPaintable::ProgressPaintable;
use super::ProgressEndListener::ProgressEndListener;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ProgressCanvas {
    pub base: RunnableCanvas,
    pub hasPainted: bool,
    pub backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>,
    pub GAUGE_PAINTABLE: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>,
    pub maxValue: f32,
    pub gauge: Option<Rc<RefCell<ABCustomGaugeItem>>>,
    pub TEXT: String,
    pub updateMyFontProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>>,
    pub myFontProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>>,
    pub allbinaryMidlet: Option<Rc<RefCell<dyn crate::org::allbinary::midlet::AllBinaryMidlet::AllBinaryMidletDyn>>>,
    pub value: f32,
    pub text: String,
    pub background: bool,
    pub paintable: Option<Rc<RefCell<dyn PaintableInterface>>>,
    pub inProgress: bool,
    pub IN_GAME_PROCESSOR: Option<Rc<RefCell<dyn crate::org::allbinary::canvas::Processor::ProcessorDyn>>>,
    pub inGameProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::canvas::Processor::ProcessorDyn>>>,
    pub progressEndListener: Option<Rc<RefCell<ProgressEndListener>>>,
    pub backgroundLabel: String,
    pub START_BACKGROUND: String,
    pub ADD_PORTION: String,
    pub ADD_EARLY_PORTION: String,
}

impl PaintableInterface for ProgressCanvas {
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
    fn paintThreed(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paintThreed(self, graphics) }
}
impl UpdateMyFontInterface for ProgressCanvas {
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::updateMeasurement(self, graphics) }
}
impl crate::Runnable for ProgressCanvas {
    fn run(&mut self) { self.base.run() }
}

impl ProgressCanvas {
    

    pub fn new_struct(mut title: String, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Self {
        let mut this = Self {
            base: RunnableCanvas::new_struct(crate::javax::microedition::lcdui::NullCommandListener::NullCommandListener_NULL_COMMAND_LISTENER.with(Clone::clone).clone(), jcall!(CanvasStrings::getInstance()).EMPTY_CHILD_NAME_LIST.clone(), false),
            hasPainted: false,
            backgroundBasicColor: None,
            GAUGE_PAINTABLE: None,
            maxValue: 0.0_f32,
            gauge: None,
            TEXT: String::new(),
            updateMyFontProcessor: None,
            myFontProcessor: None,
            allbinaryMidlet: None,
            value: 0.0_f32,
            text: String::new(),
            background: false,
            paintable: None,
            inProgress: false,
            IN_GAME_PROCESSOR: None,
            inGameProcessor: None,
            progressEndListener: None,
            backgroundLabel: String::new(),
            START_BACKGROUND: String::new(),
            ADD_PORTION: String::new(),
            ADD_EARLY_PORTION: String::new(),
        };
        this.GAUGE_PAINTABLE = ProgressPaintable::new(this);
        this.maxValue = 100.0_f32;
        this.TEXT = jcall!(this.commonStrings).LOADING.clone();
        this.updateMyFontProcessor = UpdateMyFontProcessor::new(this).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>);
        this.myFontProcessor = this.updateMyFontProcessor.clone();
        this.allbinaryMidlet = AllBinaryMidlet::getNullInstance();
        this.text = this.TEXT.clone();
        this.background = true;
        this.paintable = this.GAUGE_PAINTABLE.clone();
        this.inProgress = false;
        this.IN_GAME_PROCESSOR = Processor::new() /* Java anonymous class body: implement it as a named type
        pathFindingThreadPool: PathFindingThreadPool::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::thread::ThreadPool::ThreadPoolDyn>>),
        

        // throws Exception
        pub fn process(&mut self) {
            jcall!(this.pathFindingThreadPool).runAPriorityTask();
        }
*/;
        this.inGameProcessor = Processor::getInstance();
        this.progressEndListener = ProgressEndListener_instance.with(Clone::clone).clone();
        this.backgroundLabel = String::from("Background AI Game Loading...");
        this.START_BACKGROUND = String::from("startBackground");
        this.ADD_PORTION = String::from("addPortion");
        this.ADD_EARLY_PORTION = String::from("addEarlyPortion");
        // super(...) is in the struct literal above
        this.backgroundBasicColor = backgroundBasicColor.clone();
        this.gauge = ABCustomGaugeItem::new(jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), ((this.maxValue) as i32), 0, backgroundBasicColor.clone(), foregroundBasicColor.clone());
        this
    }

    pub fn new(mut title: String, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(title, backgroundBasicColor, foregroundBasicColor))))
    }
    

    pub fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        let mut font: Option<Rc<RefCell<Font>>> = jcall!(graphics).getFont();
        { let java_receiver = self.gauge.clone(); let java_arguments = (jcall!(font).getHeight() + 2,); jcall!(java_receiver).setHeight(java_arguments.0); };
        self.myFontProcessor = MyFontProcessor::getInstance();
    }
    

    pub fn init(&mut self, mut gameMidlet: Option<Rc<RefCell<dyn crate::org::allbinary::midlet::AllBinaryMidlet::AllBinaryMidletDyn>>>) {
        self.allbinaryMidlet = gameMidlet.clone();
    }
    

    // throws Exception
    pub fn update(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
    }
    

    pub fn initCommands(&mut self, mut cmdListener: Option<Rc<RefCell<CommandListener>>>) {
    }
    

    pub fn getMaxValue(&mut self) -> f32 {
        return self.maxValue.clone();
    }
    

    pub fn start(&mut self) {
        jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).START_METHOD_NAME.clone());
        self.setBackground(true);
        jcall!(self.gauge).setHeight(30);
        jcall!(self.gauge).setLabel(jcall!(self.commonStrings).PLEASE_WAIT.clone());
        self.setText(self.TEXT.clone());
        self.setValue(0);
        self.inGameProcessor = Processor::getInstance();
        self.paintable = self.GAUGE_PAINTABLE.clone().map(|reference| reference as Rc<RefCell<dyn PaintableInterface>>);
        self.inProgress = true;
    }
    

    pub fn startBackground(&mut self, mut background: bool) {
        jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.START_BACKGROUND.clone());
        self.setBackground(background.clone());
        self.myFontProcessor = self.updateMyFontProcessor.clone();
        jcall!(self.gauge).setLabel(self.backgroundLabel.clone());
        self.setText(self.TEXT.clone());
        self.setValue(0);
        self.inGameProcessor = Processor::getInstance();
        self.paintable = self.GAUGE_PAINTABLE.clone().map(|reference| reference as Rc<RefCell<dyn PaintableInterface>>);
    }
    

    pub fn endActual(&mut self) {
        { let java_receiver = self.allbinaryMidlet.clone(); let java_arguments = (jcall!(GameCommandsFactory::getInstance()).SHOW_GAME_CANVAS.clone(), crate::javax::microedition::lcdui::NullCanvas::NullCanvas_NULL_CANVAS.with(Clone::clone).clone(),); jcall!(java_receiver).commandAction(java_arguments.0, java_arguments.1); };
        self.inProgress = false;
        self.inGame();
    }
    

    pub fn inGame(&mut self) {
        jcall!(self.progressEndListener).onEndEvent();
        self.inGameProcessor = self.IN_GAME_PROCESSOR.clone();
    }
    

    pub fn end(&mut self) {
        jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).END_METHOD_NAME.clone());
        { let java_receiver = self.gauge.clone(); let java_arguments = (self.getMaxValue(),); jcall!(java_receiver).setValue(java_arguments.0); };
        self.endActual();
        self.paintable = NullPaintable::getInstance().map(|reference| reference as Rc<RefCell<dyn PaintableInterface>>);
    }
    

    pub fn endFromInitialLazyLoadingComplete(&mut self) {
        jcall!(self.progressEndListener).onEndEvent();
        { let java_receiver = self.gauge.clone(); let java_arguments = (self.getMaxValue(),); jcall!(java_receiver).setValue(java_arguments.0); };
        self.inGameProcessor = self.IN_GAME_PROCESSOR.clone();
    }
    

    pub fn isInGame(&mut self) -> bool {
        if match (&self.inGameProcessor, &self.IN_GAME_PROCESSOR) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return true;
        }
        return false;
    }
    

    pub fn endIfPaintedSinceStart(&mut self) {
    }
    

    pub fn addEarlyPortion(&mut self, mut value: i32, mut text: String, mut index: i32) {
        { let java_arguments = (jcall!(StringMaker::new()).append(text.clone()).append(jcall!(jcall!(SmallIntegerSingletonFactory::getInstance()).getAt(index.clone())).toString()).toString(),); self.setText(java_arguments.0) };
        { let java_receiver = self.gauge.clone(); let java_arguments = (jcall!(self.gauge).getValue() + self.getMaxValue() / ((value) as f32),); jcall!(java_receiver).setValue(java_arguments.0); };
    }
    

    pub fn addPortion(&mut self, mut value: i32, mut text: String, mut index: i32) {
        { let java_arguments = (jcall!(StringMaker::new()).append(text.clone()).append(jcall!(jcall!(SmallIntegerSingletonFactory::getInstance()).getAt(index.clone())).toString()).toString(),); self.setText(java_arguments.0) };
        PreLogUtil::put(self.text.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.ADD_PORTION.clone());
        { let java_receiver = self.gauge.clone(); let java_arguments = (jcall!(self.gauge).getValue() + self.getMaxValue() / ((value) as f32),); jcall!(java_receiver).setValue(java_arguments.0); };
    }
    

    pub fn addNormalPortion(&mut self, mut value: i32, mut text: String) {
        if self.text != text {
            PreLogUtil::put(text.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.ADD_PORTION.clone());
        }
        self.setText(text.clone());
        { let java_receiver = self.gauge.clone(); let java_arguments = (jcall!(self.gauge).getValue() + self.getMaxValue() / ((value) as f32),); jcall!(java_receiver).setValue(java_arguments.0); };
    }
    

    pub fn setValue(&mut self, mut value: i32) {
        self.value = ((value) as f32);
        jcall!(self.gauge).setValue(((value) as f32));
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        jcall!(self.paintable).paint(graphics.clone());
    }
    

    pub fn paint2(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        jcall!(self.myFontProcessor).process(graphics.clone());
        jcall!(graphics).setColor(jcall!(self.backgroundBasicColor).intValue());
        jcall!(graphics).fillRect(0, 0, jcall!(self.displayInfo).getLastWidth(), jcall!(self.displayInfo).getLastHeight());
        jcall!(self.gauge).paintXY(graphics.clone(), 0, 0);
        self.hasPainted = true;
    }
    

    pub fn paintThreed(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
    }
    

    pub fn getValue(&mut self) -> f32 {
        return self.value.clone();
    }
    

    pub fn setText(&mut self, mut text: String) {
        self.text = text.clone();
    }
    

    pub fn getText(&mut self) -> String {
        return self.text.clone();
    }
    

    pub fn setBackground(&mut self, mut background: bool) {
        self.background = background.clone();
    }
    

    pub fn isBackground(&mut self) -> bool {
        return self.background.clone();
    }
}
impl std::ops::Deref for ProgressCanvas {
    type Target = RunnableCanvas;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ProgressCanvas {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait ProgressCanvasDyn: crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvasDyn {
    fn as_ProgressCanvas(&self) -> &ProgressCanvas;
    fn as_ProgressCanvas_mut(&mut self) -> &mut ProgressCanvas;
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { ProgressCanvas::updateMeasurement(self.as_ProgressCanvas_mut(), graphics) }
    fn init(&mut self, mut gameMidlet: Option<Rc<RefCell<dyn crate::org::allbinary::midlet::AllBinaryMidlet::AllBinaryMidletDyn>>>) { ProgressCanvas::init(self.as_ProgressCanvas_mut(), gameMidlet) }
    fn update(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { ProgressCanvas::update(self.as_ProgressCanvas_mut(), graphics) }
    fn getMaxValue(&mut self) -> f32 { ProgressCanvas::getMaxValue(self.as_ProgressCanvas_mut()) }
    fn start(&mut self) { ProgressCanvas::start(self.as_ProgressCanvas_mut()) }
    fn startBackground(&mut self, mut background: bool) { ProgressCanvas::startBackground(self.as_ProgressCanvas_mut(), background) }
    fn endActual(&mut self) { ProgressCanvas::endActual(self.as_ProgressCanvas_mut()) }
    fn inGame(&mut self) { ProgressCanvas::inGame(self.as_ProgressCanvas_mut()) }
    fn end(&mut self) { ProgressCanvas::end(self.as_ProgressCanvas_mut()) }
    fn endFromInitialLazyLoadingComplete(&mut self) { ProgressCanvas::endFromInitialLazyLoadingComplete(self.as_ProgressCanvas_mut()) }
    fn isInGame(&mut self) -> bool { ProgressCanvas::isInGame(self.as_ProgressCanvas_mut()) }
    fn endIfPaintedSinceStart(&mut self) { ProgressCanvas::endIfPaintedSinceStart(self.as_ProgressCanvas_mut()) }
    fn addEarlyPortion(&mut self, mut value: i32, mut text: String, mut index: i32) { ProgressCanvas::addEarlyPortion(self.as_ProgressCanvas_mut(), value, text, index) }
    fn addPortion(&mut self, mut value: i32, mut text: String, mut index: i32) { ProgressCanvas::addPortion(self.as_ProgressCanvas_mut(), value, text, index) }
    fn addNormalPortion(&mut self, mut value: i32, mut text: String) { ProgressCanvas::addNormalPortion(self.as_ProgressCanvas_mut(), value, text) }
    fn setValue(&mut self, mut value: i32) { ProgressCanvas::setValue(self.as_ProgressCanvas_mut(), value) }
    fn paint2(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { ProgressCanvas::paint2(self.as_ProgressCanvas_mut(), graphics) }
    fn paintThreed(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { ProgressCanvas::paintThreed(self.as_ProgressCanvas_mut(), graphics) }
    fn getValue(&mut self) -> f32 { ProgressCanvas::getValue(self.as_ProgressCanvas_mut()) }
    fn setText(&mut self, mut text: String) { ProgressCanvas::setText(self.as_ProgressCanvas_mut(), text) }
    fn getText(&mut self) -> String { ProgressCanvas::getText(self.as_ProgressCanvas_mut()) }
    fn setBackground(&mut self, mut background: bool) { ProgressCanvas::setBackground(self.as_ProgressCanvas_mut(), background) }
    fn isBackground(&mut self) -> bool { ProgressCanvas::isBackground(self.as_ProgressCanvas_mut()) }
}
impl ProgressCanvasDyn for ProgressCanvas {
    fn as_ProgressCanvas(&self) -> &ProgressCanvas { self }
    fn as_ProgressCanvas_mut(&mut self) -> &mut ProgressCanvas { self }
}

impl crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvasDyn for ProgressCanvas {
    fn as_RunnableCanvas(&self) -> &crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvas { &self.base }
    fn as_RunnableCanvas_mut(&mut self) -> &mut crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvas { &mut self.base }
    fn initCommands(&mut self, mut cmdListener: Option<Rc<RefCell<CommandListener>>>) { Self::initCommands(self, cmdListener) }
}
impl crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn for ProgressCanvas {
    fn as_MyCanvas(&self) -> &crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas { &self.base.base }
    fn as_MyCanvas_mut(&mut self) -> &mut crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas { &mut self.base.base }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
}


pub struct ProgressPaintable {
    pub base: Paintable,
    pub progressCanvas: Option<Rc<RefCell<dyn ProgressCanvasDyn>>>,
}


impl ProgressPaintable {
        

        pub fn new_struct(mut progressCanvas: Option<Rc<RefCell<dyn ProgressCanvasDyn>>>) -> Self {
            let mut this = Self {
            base: Paintable::new_struct(),
            progressCanvas: None,
            };
            this.progressCanvas = progressCanvas.clone();
            this
        }

        pub fn new(mut progressCanvas: Option<Rc<RefCell<dyn ProgressCanvasDyn>>>) -> Option<Rc<RefCell<Self>>> {
            Some(Rc::new(RefCell::new(Self::new_struct(progressCanvas))))
        }
        

        pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
            jcall!(self.progressCanvas).paint2(graphics.clone());
        }
}
impl std::ops::Deref for ProgressPaintable {
    type Target = Paintable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ProgressPaintable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::paint::Paintable::PaintableDyn for ProgressPaintable {
    fn as_Paintable(&self) -> &crate::org::allbinary::graphics::paint::Paintable::Paintable { &self }
    fn as_Paintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::Paintable::Paintable { &mut self }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
}


